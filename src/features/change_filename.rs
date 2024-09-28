pub fn change_filename(name: String) -> Option<String> {
    Some(name.split('=').nth(1).unwrap_or("").to_string())
}
