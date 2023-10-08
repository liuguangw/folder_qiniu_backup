use base64::{engine::general_purpose, Engine};

pub fn get_encoded_entry_uri(bucket_name: &str, save_path: &str) -> String {
    let entry = format!("{bucket_name}:{save_path}");
    general_purpose::URL_SAFE.encode(entry.as_bytes())
}
