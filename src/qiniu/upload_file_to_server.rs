use reqwest::multipart::{Form, Part};
use reqwest::Client;
use std::path::Path;
use tokio::fs;

pub async fn upload_file_to_server<P: AsRef<Path>>(
    local_path: P,
    bucket_name: &str,
    save_path: &str,
    access_key: &str,
    secret_key: &str,
) {
    let token = super::get_upload_token(bucket_name, save_path, access_key, secret_key);
    //println!("{}", token);
    let file_content = fs::read(&local_path).await.expect("read file error");
    let file_name = {
        let path = Path::new(save_path);
        path.file_name().unwrap().to_str().unwrap().to_string()
    };
    let file_part = Part::bytes(file_content)
        .file_name(file_name)
        .mime_str("application/zip")
        .expect("make part error");
    let client = Client::new();
    let form = Form::new()
        .text("key", save_path.to_string())
        .text("token", token)
        .part("file", file_part);
    let response_result = client
        .post("https://up-z2.qiniup.com/")
        .header("User-Agent", "folder_qiniu_backup/1.1")
        .multipart(form)
        .send()
        .await;
    match response_result {
        Ok(response) => match response.error_for_status_ref() {
            Ok(_res) => {
                println!("content: {}", response.text().await.unwrap());
            }
            Err(err) => {
                eprintln!("error: {}\n{}", err, response.text().await.unwrap())
            }
        },
        Err(err) => eprintln!("error: {err}"),
    }
}
