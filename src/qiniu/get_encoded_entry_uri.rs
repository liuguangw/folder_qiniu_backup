pub fn get_encoded_entry_uri(bucket_name: &str, save_path: &str) -> String {
    let entry = format!("{}:{}", bucket_name, save_path);
    base64::encode_config(entry.as_bytes(), base64::URL_SAFE)
}
