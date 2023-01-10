mod blog;
mod git;

use blog::BlogRef;
use salvo::prelude::*;
use salvo::serve_static::StaticDir;
use git2::Repository;
use std::io::{self, Write};
use std::str;
use std::time::Duration;
use tokio::{task, time};

const refresh_time : u64 = 10*60;

#[handler]
async fn main_page(res : &mut Response) {
    let html = std::fs::read_to_string("dist/src/routes/home/index.html").expect("File not found");
    res.render(Text::Html(html));
}

#[handler]
async fn serve_blog_page(request : &mut Request, res : &mut Response) {
    let html = std::fs::read_to_string("dist/src/routes/blog/index.html").expect("File not found");
    res.render(Text::Html(html));
}

#[handler]
async fn blog_batch(request : &mut Request, res : &mut Response) {
    let html = std::fs::read_to_string("dist/src/routes/blog/index.html").expect("File not found");
    res.render(Text::Html(html));
}

#[handler]
async fn blog_single(req : &mut Request, res : &mut Response) {
    //TODO add error handling
    println!("{:?}", req.queries());
    if let Ok(blog_ref) = req.parse_queries::<BlogRef>() {
        if let Some(source) = blog_ref.load_md() {
            res.render(Text::Plain(source))
        } else {
            res.set_status_code(StatusCode::NOT_FOUND)
        }
    } else {
        res.set_status_code(StatusCode::BAD_REQUEST)
    }


    let blog_id = req.queries().get("id").clone();
    let blog_id = blog_id.unwrap().parse::<usize>();
    let blog_ref = BlogRef::new(blog_id.unwrap());

    res.render(Text::Plain(blog_ref.load_md().unwrap()));
}

#[tokio::main]
async fn main() {

    let update_task = task::spawn(async {
        let mut interval = time::interval(Duration::from_secs(refresh_time));

        loop {
            crate::git::pull();
            interval.tick().await;
        }
    });

    let router = Router::new()
        .get(main_page)
        .push(
            Router::with_path("assets")
            .push(Router::with_path("<**path>")
                .get(
                    StaticDir::new(["dist/assets"]).with_defaults("index.html").with_listing(true)
                )
            )
        )
        .push(
            Router::with_path("blog")
            .get(serve_blog_page)
            .push(
                Router::with_path("batch")
            )
            .push(
                Router::with_path("single")
                .get(blog_single)
            )
        );
    
    Server::new(TcpListener::bind("127.0.0.1:7878")).serve(router).await;
}