pub fn change_path(name : String)-> Option<String>{
    //verification des caracteres ~
    if name.contains("="){
      return  Some(expand_tilde(name.split('=').nth(1).unwrap_or("")));
    }
    None
}

fn expand_tilde(path: &str) -> String {
    if path.starts_with("~") {
        if let Some(home_dir) = dirs::home_dir() {
            let path_remainder = &path[1..]; // Le reste du chemin après le ~
            return format!("{}{}", home_dir.display(), path_remainder);
        }
    }
    path.to_string()
}

pub fn find_first_path_url(arg:String ,url : String)->String{
    if url.contains("https://") || url.contains("http://"){
        return url.split("https://").collect::<Vec<&str>>()[1].to_string();
    }
    return "".to_owned();
}

pub fn find_path_url(arg:String, url : String)->String{
    let domain = find_first_path_url(arg.clone(),url);
    if domain.is_empty(){
       return "".to_owned();
    }
    let urls: Vec<&str> = domain.split("/").collect();
    let mut new_url = String::new();
    new_url.push_str(&format!("{}/", arg));
    for (i,v) in urls.iter().enumerate(){
        new_url.push_str(v);
        if i != urls.len()-1{
            new_url.push('/');
        }
    }
    return new_url;
}