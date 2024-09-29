use std::io::{copy, Write};
use reqwest;
use std::fs::{File, create_dir_all};

use crate::untils::file::create_directory;

pub fn mirror_url(url: &String, reject: bool, reject_value: String, convert_links: bool) {
    let path = url.split("//").last().unwrap().to_string();
    create_dir_all(&path).expect("unable to create directory");
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    let indexpath = format!("./{path}/index.html");
    let mut dest_file = File::create(indexpath).expect("unable to create index file");
    
    let mut updated_response = response.clone();

    let lines = response.split(">");
    for s in lines {
        if s.contains("url(") {
            let vec = s.split("url(").collect::<Vec<_>>();
            for v in vec {
                if v.contains("'") {
                    let name = v.split("'").collect::<Vec<_>>()[1];
                    let relative_url = format!("{}/{}", url, name);
                    let destf = format!("{path}{name}");
                    download_a_file(relative_url.clone(), destf.clone());

                    if convert_links {
                        // Remplacer les liens absolus par des liens relatifs dans le HTML
                        updated_response = updated_response.replace(&name, &format!(".{}",&name));

                    }
                }
            }
        }

        if (s.contains("img ") && s.contains("src=")) || s.contains("img src=") {
            let vec = s.split_whitespace().collect::<Vec<_>>();
            for v in vec {
                let x: &[_] = &['\'', '"'];
                if v.contains("src=") && (!reject || (reject && !v.contains(&reject_value))) {
                    let mut res = v.split("src=").collect::<Vec<_>>()[1];
                    if res.split("/").collect::<Vec<_>>().len() > 1 {
                        let mut vec = res.split("/").collect::<Vec<_>>();
                        let len = vec.len() - 1;
                        vec[len] = vec[len].trim_matches(x);
                        for i in 0..vec.len() {
                            if vec[i].chars().all(char::is_alphanumeric) {
                                let dir = format!("{}{}", path, vec[i]);
                                create_dir_all(&dir).expect("unable to create directory");
                                let relative_url = format!("{}/{}/{}", url, vec[i], vec[i + 1]);
                                let destf = format!("{path}{}/{}", vec[i], vec[i + 1]);
                                download_a_file(relative_url.clone(), destf.clone());

                                if convert_links {
                                    // Convertir les liens absolus en relatifs
                                    updated_response = updated_response.replace(&relative_url, &format!("{}/{}", vec[i], vec[i + 1]));
                                }
                                break;
                            }
                        }
                    } else {
                        res = res.trim_matches(x);
                        let relative_url = format!("{}/{}", url, res);
                        let destf = format!("{path}{}", res);
                        download_a_file(relative_url.clone(), destf.clone());

                        if convert_links {
                            // Conversion du lien absolu en lien relatif
                            updated_response = updated_response.replace(&relative_url, res);
                        }
                    }
                }
            }
        }

        if ((s.contains("a href") || s.contains("link href")) && !s.contains("http") && !s.contains(".com"))|| (s.contains("<script") && s.contains("src")) {
            let mut link = "";
            let mut dened=false; 
            if s.contains("href="){
                if s.trim().split("href=").collect::<Vec<_>>().len() > 1{
                    link= s.split("href=").collect::<Vec<_>>()[1];
                }
            }else if s.contains("src="){
                if s.trim().split("src=").collect::<Vec<_>>().len() > 1{
                    link= s.split("src=").collect::<Vec<_>>()[1];
                }
            }
            if !link.is_empty(){
            //    println!("le link ====== {} ======", link);
               link = link.split("\"").collect::<Vec<_>>()[1];
                // destf;
               // if link.split("/").collect::<Vec<_>>().len() > 1 {
               //     let res = link.split("/").collect::<Vec<_>>();
               //     let dir = format!("{}{}", path, res[0]);
               //     create_dir_all(&dir).expect("unable to create directory");
               //     destf = format!("./{}{}/{}", path, res[0], res[1]);
               // } else {
               //     destf = format!("./{}/{}", path, link);
               // }
               // println!("link : {}", link);
               let mut destf = format!("./{}{}", path.trim_end_matches("/"), link);
               // println!("dest {}",destf);
               match create_directory(format!("{}/{}", path, link)){
                   Ok((_file, filname)) =>{
                    destf = filname;
                    dened=false;
                },
                   Err(error) =>{
                    // println!("destf =***** {} *****=\n path=***** {} ******=",destf,path );
                       dened=true;
                    //    destf = "index.html".to_string();
                       println!("erreur lors de la creation de dossier : {}", error);
                }
               }
               let relative_url = format!("{}{}", url.clone(), link);
               if !dened{
                   download_a_file(relative_url.clone(), destf.clone());
       
                   if convert_links {
                       // Conversion du lien dans le HTML
                       updated_response = updated_response.replace(link, &format!(".{}",&link));
                   }
               }
           }
        }
    }

    // Écriture du HTML mis à jour (avec les liens convertis, si applicable)
    write!(dest_file, "{}", updated_response).expect("unable to write to index file");
}

pub fn download_a_file(path: String, destf: String) {
    let mut resp = reqwest::blocking::get(&path).unwrap();
    let mut dest_file = File::create(&destf).expect("error creating file");
    // println!("dest_file $$$$$$$$ {} $$$$$$",destf);
    // Copie le contenu de la réponse dans le fichier de destination
    copy(&mut resp, &mut dest_file).expect("error copying response");
}
