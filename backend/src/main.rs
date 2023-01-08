use salvo::prelude::*;
use salvo::serve_static::StaticDir;

#[handler]
async fn main_page(res : &mut Response) {
    let html = std::fs::read_to_string("dist/index.html").expect("File not found");
    res.render(Text::Html(html));
}

#[tokio::main]
async fn main() {

    

    let router = Router::new()
        .get(main_page)
        .push(
            Router::with_path("assets")
            .push(Router::with_path("<**path>")
            .get(
                StaticDir::new(["dist/assets"]).with_defaults("index.html").with_listing(true)
            ))
        );
    Server::new(TcpListener::bind("127.0.0.1:7878")).serve(router).await;
}