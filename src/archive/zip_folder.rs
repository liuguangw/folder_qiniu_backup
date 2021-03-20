use super::ArchiveResult;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use zip::write::FileOptions;
use zip::ZipWriter;

pub fn zip_folder<P: AsRef<Path>>(src_folder: P, output_path: P) -> ArchiveResult {
    let out_file = File::create(output_path)?;
    let mut zip = zip::ZipWriter::new(out_file);
    ad_items_to_zip(&mut zip, src_folder, "")?;
    zip.finish()?;
    Ok(())
}

fn ad_items_to_zip<P: AsRef<Path>>(
    zip: &mut ZipWriter<File>,
    src_folder: P,
    context_prefix: &str,
) -> ArchiveResult {
    let directory_name = src_folder.as_ref().file_name().unwrap();
    let directory_name = directory_name.to_str().unwrap();
    let sub_context_prefix = format!("{}{}/", context_prefix, directory_name);
    println!("add dir {}", sub_context_prefix);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Bzip2)
        .unix_permissions(0o755);
    zip.add_directory(&sub_context_prefix, options)?;
    for entry in fs::read_dir(src_folder)? {
        if let Ok(item_entry) = entry {
            if let Ok(file_type) = item_entry.file_type() {
                let item_name = item_entry.file_name();
                let item_name = item_name.to_str().unwrap();
                let item_path = item_entry.path();
                if file_type.is_file() {
                    let zip_path_name = format!("{}{}", sub_context_prefix, item_name);
                    println!("add file {}", zip_path_name);
                    zip.start_file(&zip_path_name, options)?;
                    let file_content = fs::read(item_path)?;
                    zip.write_all(&file_content)?;
                } else if file_type.is_dir() {
                    ad_items_to_zip(zip, item_path, &sub_context_prefix)?;
                    //println!("{:?}", item_path);
                    //dir_list.push(item_path);
                }
                // Now let's show our entry's file type!
                //println!("{:?}: {:?}", item_entry.path(), file_type);
            }
            //else {
            //println!("Couldn't get file type for {:?}", item_entry.path());
            //}
        }
        //println!("===== {:?}", src_folder.as_ref());
    }
    //
    /*for sub_dir in &dir_list {
        ad_items_to_zip(zip, sub_dir.as_path(), &sub_context_prefix)?;
    }*/
    Ok(())
}
