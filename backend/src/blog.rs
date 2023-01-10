use std::{path};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BlogRef {
    id : usize,
}

#[derive(Deserialize, Serialize)]
pub struct BlogMeta {
    tags : Vec<String>
}

#[derive(Deserialize, Serialize)]
pub struct Blog {
    id : usize,
    meta : BlogMeta,
    content : String
}

impl BlogRef {

    pub fn new(id : usize) -> BlogRef {
        BlogRef{
            id
        }
    }

    pub fn load_blog() -> Blog {}

    pub fn load_meta(&self) -> Option<BlogMeta> {
        if let Some(mut path) = self.get_path() {
            path.push("meta.json");
            let content = std::fs::read_to_string(path);
            if let Ok(content) = content {
                let meta = serde_json::from_str::<BlogMeta>(content.as_str());
                if let Ok(meta) = meta {
                    return Some(meta);
                }
            }
        }

        None
    }

    pub fn load_md(&self) -> Option<String> {
        if let Some(mut path) = self.get_path() {
            path.push("content.md");
            let content = std::fs::read_to_string(path);
            if let Ok(content) = content {
                return Some(content)
            }
        }
                
        None
    }

    pub fn get_path(&self) -> Option<path::PathBuf>{
        let paths = std::fs::read_dir("content/blogs");

        if let Ok(mut dir) = paths {
            if let Some(path) = dir.nth(self.id) {
                if let Ok(blog) = path {
                    let buf = blog.path();
                    return Some(buf);
                }
            }
        }

        None
    }
}

