use std::env;

use features::change_filename::change_filename;
use features::change_path::change_path;
use features::download::{ found_file_name, SpeedUnit};
use features::multiple_link::take_all_link;
use features::rate_limit::rate_limit;
// use untils::file::create_directory;
use untils::is_flag::add_http_if_missing;
mod features {
    pub mod change_filename;
    pub mod change_path;
    pub mod download;
    pub mod mirror;
    pub mod multiple_link;
    pub mod rate_limit;
    pub mod redirect_file;
}
mod untils {
    pub mod file;
    pub mod is_flag;
}
use features::mirror::mirror_url;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Récupérer un itérateur sur les arguments
    let args: Vec<String> = env::args().collect();
    let mut file_name: Option<String> = None;
    let mut directory_saved: Option<String> = None;
    let mut is_redirect = false;
    let mut is_multilink = false;
    let mut is_limitation = false;
    let is_mirror = false;
    let mut limit: (usize, SpeedUnit) = (0, SpeedUnit::K);

    // Trouver tous les flags et les enregistrer
    for arg in &args[1..] {
        let mut content_size: Vec<usize> = Vec::new();
        let flag = untils::is_flag::is_flag(arg);
        if let Some(flag) = flag {
            match flag {
                "-P" => {
                    // Handle -P flag change directory
                    directory_saved = change_path(arg.to_string());
                }
                "-B" => {
                    // Handle -B flag
                    is_redirect = true;
                }
                "-O" => {
                    // Mettre à jour file_name avec la valeur après "="
                    file_name = change_filename(arg.to_string());
                }
                "--rate-limit" => {
                    // Handle --rate-limit flag
                    match rate_limit(arg.as_str()) {
                        Some(v) => {
                            limit = v;
                            is_limitation = true
                        }
                        None => println!("this limit is incorrect"),
                    }
                }
                "-i" => {
                    // Handle -i flag
                    let filename = change_filename(arg.to_string());
                    let res = take_all_link(filename.unwrap(), is_mirror);
                    let mut display = String::new();
                    if res.is_ok() {
                        is_multilink = true;
                        for v in res.unwrap() {
                            // Ajouter http:// ou https:// si nécessaire
                            let url = add_http_if_missing(v.as_str());
                            display.push_str(
                                &format!("finished {:?}\n", found_file_name(url.as_str()).unwrap())
                                    .as_str()
                                    .replace("\"", ""),
                            );
                            match features::download::download(
                                url.as_str(),
                                file_name.clone(),
                                directory_saved.clone(),
                                is_redirect,
                                is_multilink,
                                is_limitation,
                                &mut content_size,
                                limit,
                            ) {
                                Ok(_path) => {}
                                Err(error) => {
                                    println!("Une erreur est survenue: {:?}", error);
                                }
                            }
                        }
                        println!("content size : {:?}", content_size);

                        println!("{}", display);
                    }
                }
                "--mirror" => {
                    let mut is_reject = (false, String::new());
                    //let mut is_exclude = (false, String::new());
                    let is_convert = false;
                    let mut actif = false;
                    let argument: Vec<String> = env::args().collect();
                    if argument.len() == 3 {
                        let url = argument[argument.len() - 1].clone();
                        if url.contains("http://") || url.contains("https://") {
                            mirror_url(&url, false, "".to_string(), false);
                            break;
                        }
                    } else {
                        for v in argument.clone() {
                            if actif {
                                match untils::is_flag::is_flag(&v) {
                                    Some(res) => {
                                        match res {
                                            "--reject" | "-R" | "--exclude" | "-X" => {
                                                is_reject = (
                                                    true,
                                                    v.split("=").collect::<Vec<&str>>()[1]
                                                        .to_owned(),
                                                );
                                                let url = argument[argument.len() - 1].clone();
                                                if url.contains("http://")
                                                    || url.contains("https://")
                                                {
                                                    mirror_url(
                                                        &url,
                                                        is_reject.0,
                                                        is_reject.1,
                                                        is_convert,
                                                    );
                                                    break;
                                                }
                                            }

                                            // "--exclude" | "-X"=>{is_exclude=(true, v.split("=").collect::<Vec<&str>>()[1].to_owned())},
                                            "--convert-links" => {
                                                let url = argument[argument.len() - 1].clone();
                                                if url.contains("http://")
                                                    || url.contains("https://")
                                                {
                                                    mirror_url(&url, false, "".to_string(), true);
                                                    break;
                                                }
                                            }
                                            _ => {}
                                        }
                                        {}
                                    }
                                    None => {}
                                }
                            }
                            if v == "--mirror" {
                                actif = true;
                            }
                        }
                    }

                    break;
                }
                _ => {
                    println!("Flag non reconnu : {}", flag);
                }
            }
        } else {
            // Ajouter http:// ou https:// si nécessaire
            let url = add_http_if_missing(arg.as_str());
            match features::download::download(
                url.as_str(),
                file_name.clone(),
                directory_saved.clone(),
                is_redirect,
                is_multilink,
                is_limitation,
                &mut content_size,
                limit,
            ) {
                Ok(_path) => {}
                Err(error) => {
                    println!("Une erreur est survenue: {:?}", error);
                }
            }
        }
    }

    Ok(())
}
