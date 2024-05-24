use std::path::PathBuf;

use crate::local_file::FileEntry;

pub fn gen_page(file_list: Vec<FileEntry>) -> String {
    // start content of target html page.
    // I just think this page would'nt be TOO BIG.
    let start = r#"<!DOCTYPE html>
    <html lang="en">
    
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>File Server</title>
        <style>
            table {
                text-align: left;
                width: 800;
            }
    
            th.file_name {
                width: 30%;
            }
    
            th.file_size {
                width: 30%;
            }
    
            th.modified_time {
                width: 40%;
            }
        </style>
    </head>
    
    <body>
        <h1>Welcome to file_server</h1>
        <table>
            <thead>
                <tr>
                    <th class="file_name">File Name</th>
                    <th class="file_size">File Size</th>
                    <th class="modified_time">Modified Time</th>
                </tr>
            </thead>
            <tbody>"#;
    // end of this page.
    let end = r#"
    </tbody>
    </table>
    </body>
    </html>"#;
    // insert file entry to page.
    let mut file_entry = String::new();
    for entry in file_list {
        let tr = format!(
            "<tr><td><a href=\"{}\">{}</a></td><td>{}</td><td>{}</td></tr>",
            entry.file_name, entry.file_name, entry.file_size, entry.file_modified_time
        );
        file_entry.push_str(&tr);
    }

    String::new() + start + &file_entry + end
}
