use chrono::{Duration, Local};

pub async fn clean_old_backup_files(
    bucket_name: &str,
    path_prefix: &str,
    access_key: &str,
    secret_key: &str,
) {
    println!("clean expired backup files ...");
    let time_now = Local::now();
    for i in (10..=20i64).rev() {
        let time_point = time_now - Duration::days(i);
        let date_string = time_point.format("%F").to_string();
        println!("check {}", &date_string);
        let backup_path = format!("{}{}.zip", path_prefix, date_string);
        let backup_status =
            super::super::qiniu::get_file_status(bucket_name, &backup_path, access_key, secret_key)
                .await;
        match backup_status {
            200 => {
                println!("delete file: {}", backup_path);
                let delete_status = super::super::qiniu::delete_file(
                    bucket_name,
                    &backup_path,
                    access_key,
                    secret_key,
                )
                .await;
                match delete_status {
                    200 => println!("delete success"),
                    612 => println!("not exists, skip..."),
                    _ => eprintln!("error: {}", backup_status),
                }
            }
            612 => println!("not exists, skip"),
            _ => eprintln!("error: {}", backup_status),
        }
    }
}
