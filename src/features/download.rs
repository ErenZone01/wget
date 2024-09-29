use crate::features::redirect_file;
use crate::untils::file::create_directory;
use crate::untils::is_flag::add_http_if_missing;
use anyhow::Context;
use chrono::Local;
use reqwest::blocking::Response;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Url;
use std::{fs::File, io::Read, io::Write};
use std::path::Path;
use std::error::Error;
use std::time::{Duration, Instant};


#[derive(Clone, Copy, Debug)]
pub enum SpeedUnit {
    B,
    K,
    M,
    G,
}

// Fonction principale de téléchargement
 pub fn download(
    url: &str,
    filename: Option<String>,
    directory: Option<String>,
    is_redirect: bool,
    is_multilink: bool,
    is_limitation: bool,
    content_size: &mut Vec<usize>,
    limit: (usize, SpeedUnit),
) -> Result<String, Box<dyn Error>> {
    let mut display = String::new();

    let filename = get_filename(url, filename)?;
    let directory = directory.unwrap_or_else(|| ".".to_string());
    let file_path = format!("{}/{}", directory, filename);

    create_dir_if_needed(&directory)?;

    // Affichage de l'heure de début
    let start_time = Local::now();
    if is_redirect {
        display.push_str(&format!(
            "start at {}",
            start_time.format("%Y-%m-%d %H:%M:%S")
        ));
    } else if !is_multilink {
        eprintln!("start at {}", start_time.format("%Y-%m-%d %H:%M:%S"));
    }

    // Initialisation du client HTTP et envoi de la requête
    let response = send_request(url)?;
    if !response.status().is_success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Échec du téléchargement : {}", response.status()),
        )));
    }

    // Afficher les informations de début de téléchargement
    if is_redirect {
        display.push_str(&format!(
            "\nsending request, awaiting response... status {}\ncontent size: {} [~{:.2}MB]\nsaving file to: {}",
            response.status(), 
            response.content_length().unwrap_or(0), 
            response.content_length().unwrap_or(0) as f64 / (1024.0 * 1024.0),
            file_path
        ));
        content_size.push(response.content_length().unwrap_or(0) as usize);
    } else if !is_multilink {
        eprintln!(
            "sending request, awaiting response... status {}",
            response.status()
        );
        eprintln!(
            "content size: {} [~{:.2}MB]",
            response.content_length().unwrap_or(0),
            response.content_length().unwrap_or(0) as f64 / (1024.0 * 1024.0)
        );
        content_size.push(response.content_length().unwrap_or(0) as usize);
        eprintln!("saving file to: {}", file_path);
    }

    // Initialiser la barre de progression
    let pb = create_progress_bar(response.content_length());

    // Créer le fichier
    let (mut file, final_file_path) = match create_directory(file_path.clone()) {
        Ok(res) => res,
        Err(err) => {
            println!("la creation de fichier est incorrect : {} ", err);
            return Err(err);
        }
    };

    // Téléchargement avec ou sans limitation
    if is_limitation {
        download_with_limitation(limit, response, &mut file, pb)?;
    } else {
        if  is_redirect {
             download_file_redirect(response, &mut file)?;
        }else{
            download_file(response, &mut file, pb)?;
        }
    }

    // Affichage de la fin de téléchargement
    let end_time = Local::now();
    if is_redirect {
        display.push_str(&format!(
            "\nDownloaded [{}]\nfinished at {}",
            url,
            end_time.format("%Y-%m-%d %H:%M:%S")
        ));
        eprintln!("Output will be written to \"wget-log\".");
        redirect_file::redirect_file(display)
    } else if !is_multilink {
        eprintln!(" ");
        eprintln!("Downloaded [{}]", url);
        eprintln!("finished at {}", end_time.format("%Y-%m-%d %H:%M:%S"));
    }

    Ok(final_file_path)
}

// Récupérer le nom du fichier à partir de l'URL ou utiliser celui fourni
fn get_filename(url: &str, filename: Option<String>) -> Result<String, Box<dyn Error>> {
    if let Some(name) = filename {
        return Ok(name);
    }
    found_file_name(url).ok_or_else(|| "Impossible de déterminer le nom du fichier.".into())
}

// Créer le répertoire si nécessaire
fn create_dir_if_needed(directory: &str) -> std::io::Result<()> {
    if !Path::new(directory).exists() {
        std::fs::create_dir_all(directory)?;
    }
    Ok(())
}

// Fonction pour envoyer une requête HTTP bloquante
fn send_request(url: &str) -> Result<reqwest::blocking::Response, anyhow::Error> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0")
        .danger_accept_invalid_certs(true)
        .build()
        .context("Failed to build client")?;

    let response = client
        .get(add_http_if_missing(url))
        .send()
        .context("Failed to send request")?;

    Ok(response)
}


// Créer une barre de progression
fn create_progress_bar(content_length: Option<u64>) -> ProgressBar {
    match content_length {
        Some(len) => ProgressBar::new(len),
        None => ProgressBar::new_spinner(),
    }
    .with_style(
        ProgressStyle::default_bar()
            .template(
                " {bytes} / {total_bytes} [{wide_bar}] [{percent}%] {bytes_per_sec} {elapsed_precise}",
            )
            .unwrap_or_else(|_| ProgressStyle::default_spinner())
            .progress_chars("=>-"),
    )
}

// Télécharger sans limitation
fn download_file(mut response: reqwest::blocking::Response, file: &mut File, pb: ProgressBar) -> Result<(), Box<dyn Error>> {
    let mut downloaded = 0;
    let mut buffer = [0; 8192];
    loop {
        let bytes_read = response
            .read(&mut buffer)
            .context("Failed to read response")?;
        if bytes_read == 0 {
            break;
        }

        file.write_all(&buffer[..bytes_read])
            .context("Failed to write to file")?;
        downloaded += bytes_read as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("done");
    Ok(())
}

fn download_file_redirect(mut response: reqwest::blocking::Response, file: &mut File) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 8192];
    loop {
        let bytes_read = response
            .read(&mut buffer)
            .context("Failed to read response")?;
        if bytes_read == 0 {
            break;
        }

        file.write_all(&buffer[..bytes_read])
            .context("Failed to write to file")?;
    }

    Ok(())
}
// Télécharger avec limitation
fn download_with_limitation(
    limit: (usize, SpeedUnit),
    mut response: Response,
    file: &mut File,
    pb: ProgressBar,
) -> Result<(), Box<dyn Error>> {
    let limit_bytes_per_sec = get_limit_in_bytes(limit);
    let mut buffer = [0; 8192]; // Taille du buffer
    let mut downloaded = 0;
    let mut bytes_this_second = 0;
    let mut last_print = Instant::now();

    loop {
        let bytes_read = response
            .read(&mut buffer)
            .context("Failed to read response")?;
        if bytes_read == 0 {
            break;
        }
        bytes_this_second += bytes_read;

        if bytes_this_second >= limit_bytes_per_sec as usize {
            let elapsed = last_print.elapsed();
            if elapsed < Duration::from_secs(1) {
                std::thread::sleep(Duration::from_secs(1) - elapsed);
            }
            bytes_this_second = 0;
            last_print = Instant::now();
        }
        
        file.write_all(&buffer[..bytes_read])
        .context("Failed to write to file")?;
        pb.set_position(downloaded);
        downloaded += bytes_read as u64;
    }
    pb.finish_with_message("done");
    Ok(())
}

// Convertir la limite en octets par seconde
fn get_limit_in_bytes(limit: (usize, SpeedUnit)) -> usize {
    match limit.1 {
        SpeedUnit::B => limit.0,
        SpeedUnit::K => limit.0 * 1024,
        SpeedUnit::M => limit.0 * 1024 * 1024,
        SpeedUnit::G => limit.0 * 1024 * 1024 * 1024,
    }
}

// Trouver le nom du fichier à partir de l'URL
pub fn found_file_name(url: &str) -> Option<String> {
    let parsed_url = Url::parse(url).ok()?;
    let file_name = parsed_url
        .path_segments()
        .and_then(|segments| segments.last())
        .unwrap_or("index.html");

    if file_name.is_empty() || file_name.ends_with('/') {
        return Some("index.html".to_string());
    }

    Some(file_name.to_string())
}

