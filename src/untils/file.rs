use std::{
    fs::{create_dir_all, File, OpenOptions},
    path::Path,
};

pub fn create_directory(file_path: String) -> Result<(File, String), Box<dyn std::error::Error>> {
    let path = Path::new(&file_path);

    // Vérifier si le chemin est trop long (limite de 255 caractères, typique sur certains systèmes)
    if file_path.len() > 255 {
        let truncated_path = truncate_path(&file_path);
        return create_directory(truncated_path);  // Retenter avec le chemin tronqué
    }

    // Vérifier et créer les répertoires si nécessaire
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    // Ouvrir ou créer le fichier
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)?;

    Ok((file, file_path))
}

fn truncate_path(file_path: &str) -> String {
    // Limiter la longueur du fichier à 255 caractères
    if file_path.len() > 255 {
        let suffix = "-truncated";
        let truncated_length = 255 - suffix.len();
        let truncated_file_path = &file_path[..truncated_length];
        format!("{}{}", truncated_file_path, suffix)  // Ajouter un suffixe pour indiquer la troncature
    } else {
        file_path.to_string()
    }
}

