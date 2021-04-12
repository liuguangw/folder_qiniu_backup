use reqwest::Client;

pub async fn delete_file(
    bucket_name: &str,
    save_path: &str,
    access_key: &str,
    secret_key: &str,
) -> Result<u16, String> {
    let method = "POST";
    let host_name = "rs.qiniu.com";
    let path = format!(
        "/delete/{}",
        super::get_encoded_entry_uri(bucket_name, save_path)
    );
    let url = format!("https://{}{}", host_name, path);
    let content_type = "application/x-www-form-urlencoded";
    let admin_token = super::get_admin_token(
        method,
        host_name,
        &path,
        content_type,
        access_key,
        secret_key,
    );
    let authorization = format!("Qiniu {}", admin_token);
    //println!("{} {}\nAuthorization: {}", method, url, authorization);
    let client = Client::new();
    let response_result = client
        .post(&url)
        .header("Authorization", authorization)
        .header("Content-Type", content_type)
        .send()
        .await;
    match response_result {
        Ok(response) => Ok(response.status().as_u16()),
        Err(err) => Err(err.to_string())
    }
}
