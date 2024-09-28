pub fn is_flag(flag: &str) -> Option<&str> {
    if flag.starts_with("-P") {
        return Some("-P");
    } else if flag.starts_with("-B") {
        return Some("-B");
    } else if flag.starts_with("-O") {
        return Some("-O");
    } else if flag.starts_with("--rate-limit") {
        return Some("--rate-limit");
    } else if flag.starts_with("-i") {
        return Some("-i");
    } else if flag.starts_with("--mirror") {
        return Some("--mirror");
    }else if flag.starts_with("--convert-links"){
        return Some("--convert-links");
    }else if flag.starts_with("-R") || flag.starts_with("--reject"){
        if flag.starts_with("-R"){
            return Some("-R");
        }else if flag.starts_with("--reject"){
            return Some("--reject");
        }
    }else if flag.starts_with("--exclude") || flag.starts_with("-X") {
        if flag.starts_with("-X"){
            return Some("-X");
        }else if flag.starts_with("--exclude"){
            return Some("--exclude");
        }
    }
    None
}

pub fn find_name_url(url : String) -> Option<String> {
    let urls = add_http_if_missing(url.as_str());
    let parts : Vec<&str> = urls.split("https://").collect();
    if parts.len() == 2 && !parts[1].is_empty(){
        return Some(parts[1].to_string());
    }
    None
}

pub fn add_http_if_missing(url: &str) -> String {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return format!("https://{}", url);
    }
    url.to_string()
}

