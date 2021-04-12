use chrono::Local;
use folder_qiniu_backup::archive::zip_folder;
use folder_qiniu_backup::qiniu;
use folder_qiniu_backup::tools;
use std::{env, process};
use std::path::Path;
use tokio::fs;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("load .env failed");
    //读取环境变量配置
    let bucket_name = env::var("QINIU_BUCKET_NAME").expect("QINIU_BUCKET_NAME not found");
    let access_key = env::var("QINIU_ACCESS_KEY").expect("QINIU_ACCESS_KEY not found");
    let secret_key = env::var("QINIU_SECRET_KEY").expect("QINIU_SECRET_KEY not found");
    let src_folder = env::var("BACKUP_DIR").expect("BACKUP_DIR not found");
    let object_path_prefix = env::var("UPLOAD_PATH_PREFIX").expect("BACKUP_DIR not found");

    let src_path = Path::new(&src_folder);
    let parent_path = src_path.parent().unwrap();
    let output_path = parent_path.join("tmp_backup.zip");
    //清理过期的备份
    if let Err(clean_error) = tools::clean_old_backup_files(&bucket_name, &object_path_prefix,
                                                            &access_key, &secret_key).await {
        eprintln!("clean error: {}", clean_error);
        process::exit(-1);
    }

    let archive_result = zip_folder(Path::new(&src_folder), &output_path);
    if let Err(err) = archive_result {
        eprintln!("archive error: {}", err);
        process::exit(-1);
    }
    let time_now = Local::now();
    let date_string = time_now.format("%F").to_string();
    //上传到七牛
    qiniu::upload_file_to_server(
        &output_path,
        &bucket_name,
        &format!("{}{}.zip", object_path_prefix, &date_string),
        &access_key,
        &secret_key,
    )
        .await;
    //clean tmp zip file
    match fs::remove_file(&output_path).await {
        Ok(_) => println!("complete"),
        Err(err) => eprintln!("error: {}", err),
    }
}
