// Generates table of contents for API schemas

use std::fs;
use std::path::Path;

pub fn generate_toc(api_dir: &str) -> String {
    let mut toc = String::new();
    toc.push_str("<h1>API Table of Contents</h1>\n<ul>\n");

    for entry in fs::read_dir(api_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap();
            toc.push_str(&format!("<li>{}</li>\n", dir_name));
            toc.push_str("<ul>\n");
            for file in fs::read_dir(path).unwrap() {
                let file = file.unwrap();
                let file_name = file.file_name().to_str().unwrap().to_string();
                toc.push_str(&format!("<li>{}</li>\n", file_name));
            }
            toc.push_str("</ul>\n");
        }
    }

    toc.push_str("</ul>\n");
    toc
}
