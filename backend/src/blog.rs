use std::{io, path};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct BlogRef {
    id : usize,
}

impl BlogRef {

    pub fn new(id : usize) -> BlogRef {
        BlogRef{
            id
        }
    }

    pub fn load_md(&self) -> Option<String> {
        let paths = std::fs::read_dir("content/blogs");

        if let Ok(mut dir) = paths {
            if let Some(path) = dir.nth(self.id) {
                if let Ok(blog) = path {
                    let mut buf = blog.path();
                    buf.push("content.md");
                    let content = std::fs::read_to_string(buf);
                    if let Ok(content) = content {
                        return Some(content)
                    }
                }
            }
        }
        
        None
    }
}

