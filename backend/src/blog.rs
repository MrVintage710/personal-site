use std::{path};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BlogMeta {
    tags : Vec<String>,
    desc : String,
    title : String,
    pic : String,
    path : String,
    author : String,
    show_title : bool
}

#[derive(Deserialize, Serialize)]
pub struct Blog {
    id : usize,
    meta : BlogMeta,
    content : String,
}

#[derive(Deserialize, Serialize)]
pub struct BlogRef {
    id : usize,
}

impl BlogRef {

    pub fn new(id : usize) -> BlogRef {
        BlogRef {
            id
        }
    }

    pub fn load_blog(&self) -> Option<Blog> {
        let content = self.load_md();
        let meta = self.load_meta();

        if content.is_some() && meta.is_some() {
            let content = content.unwrap();
            let meta = meta.unwrap();
            return Some(Blog { id: self.id, meta: meta, content: content })
        }

        None
    }

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

    pub fn exists(&self) -> bool {
        match self.get_path() {
            Some(_) => true,
            None => false,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct BlogBatch {
    offset: usize,
    count: usize,
}

impl BlogBatch {
    pub fn load_meta(&self) -> Vec<BlogMeta> {
        let mut metas = Vec::new();

        for i in self.offset..(self.count + self.offset) {
            let blog_ref = BlogRef::new(i);
            if let Some(meta) = blog_ref.load_meta() {
                metas.push(meta)
            }
        }

        metas
    }
}

pub fn is_blog(id : usize) -> bool {
    let blog_ref = BlogRef::new(id);
    match blog_ref.get_path() {
        Some(_) => true,
        None => false,
    }
}