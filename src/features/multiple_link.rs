use std::fs::File;
use std::io::{self, Read};
use regex::Regex;

pub fn take_all_link(filename: String, is_mirror: bool) -> io::Result<Vec<String>> {
    let mut res: Vec<String> = Vec::new();
    //println!("le filename : {} ", filename);
    
    // Ouvrir le fichier
    let mut file = File::open(filename.clone())?;
    
    let mut content = Vec::new(); // Lire en tant que bytes
    
    // Lire le contenu du fichier en bytes
    file.read_to_end(&mut content)?;

    // Convertir en UTF-8 en remplaçant les caractères non valides
    let content_string = String::from_utf8_lossy(&content);

    // Si is_mirror est activé, on extrait les liens
    if is_mirror {
        // Expression régulière pour capturer les liens des balises <a>, <link>, et <img> avec href ou src
        let re = Regex::new(r#"<(a|link|img)[^>]*?\s(?:href|src)="([^"]+)""#).unwrap();
    
        // Parcourir toutes les correspondances dans le contenu du fichier
        for cap in re.captures_iter(&content_string) {
            // Extraire le lien (second groupe capturé, href ou src)
            if let Some(link) = cap.get(2) {
                res.push(link.as_str().to_string());
            }
        }
    } else {
        // Si is_mirror est désactivé, on retourne le contenu ligne par ligne
        res = content_string
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
    }
    
    Ok(res)
}
