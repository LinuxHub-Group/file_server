use std::{os::windows::fs::MetadataExt, time::SystemTime};

use chrono::Local;

#[derive(Debug)]
pub enum FileType {
    File,
    Dir,
}
#[derive(Debug)]
pub struct FileEntry {
    pub path: String,
    pub file_name: String,
    pub file_type: FileType,
    pub file_size: String,
    pub file_modified_time: String,
}
impl FileEntry {
    pub fn new(
        path: String,
        file_name: String,
        file_type: FileType,
        file_size: u64,
        file_modified_time: SystemTime,
    ) -> Self {
        // use chrono to transform timestamp to readable time string.
        let datetime: chrono::DateTime<Local> = file_modified_time.into();
        FileEntry {
            path,
            file_name,
            file_size: match file_type {
                FileType::Dir => String::from(""),
                FileType::File => {
                    if file_size < 1024 {
                        format!("{}b", file_size)
                    } else if file_size >= 1024 && file_size < 1024 * 1024 {
                        format!("{:.2}Kb", file_size as f64 / 1024 as f64)
                    } else if file_size >= 1024 * 1024 && file_size < 1024 * 1024 * 1024 {
                        format!("{:.2}Mb", file_size as f64 / (1024 * 1024) as f64)
                    } else {
                        format!("{:.2}Gb", file_size as f64 / (1024 * 1024 * 1024) as f64)
                    }
                }
            },
            file_type,
            file_modified_time: datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
    pub async fn get_files(path: &String) -> Vec<Self> {
        let mut files = tokio::fs::read_dir(&path).await.unwrap();
        let mut result = Vec::new();
        loop {
            if let Some(entry) = files.next_entry().await.unwrap() {
                let file_size = entry.metadata().await.unwrap().file_size();
                let file_type = if entry.metadata().await.unwrap().is_dir() {
                    FileType::Dir
                } else {
                    FileType::File
                };
                // append slash to file name to indicate this is a dir.
                let file_name = match file_type {
                    FileType::Dir => format!("{}/", entry.file_name().into_string().unwrap()),
                    FileType::File => entry.file_name().into_string().unwrap(),
                };
                // concat origin path and file name to structure a new path.
                let path = format!("{}{}", path, file_name);
                let file_modified_time = entry.metadata().await.unwrap().modified().unwrap();
                result.push(FileEntry::new(
                    path,
                    file_name,
                    file_type,
                    file_size,
                    file_modified_time,
                ));
            } else {
                break;
            }
        }
        result
    }
}
