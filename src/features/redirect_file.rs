use std::io::Write;
use std::fs::OpenOptions;
pub fn redirect_file(txt : String) {
    //ouvrir ou creer un fichier
    let mut file = OpenOptions::new().write(true).create(true).append(true).open("wget-log.log").expect("Unable to open file");

    //Ecrire dans le fichier
    if let Err(e) = writeln!(file, "{}", txt){
        eprintln!("Couldn't write to file : {}", e);
    }
}