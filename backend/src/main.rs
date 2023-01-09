mod blog;

use blog::BlogRef;
use salvo::prelude::*;
use salvo::serve_static::StaticDir;
use git2::Repository;
use std::io::{self, Write};
use std::str;
use std::time::Duration;
use tokio::{task, time};

const remote_name : &str = "origin";
const remote_branch : &str = "main";
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
            pull();
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

//Probably not the most secure FML
fn pull() {
    println!("\nStarting new Pull!");
    let repo = match Repository::open("./content") {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open: {}", e),
    };

    let mut remote = repo.find_remote(remote_name).expect("Error: Remote 'Origin' doesn't exist.");

    let fetch_commit = do_fetch(&repo, &[remote_branch], &mut remote).expect("Unable to fetch changes");

    do_merge(&repo, remote_branch, fetch_commit).expect("Unable to pull from the branch.")
}

//This is an a Example from the git2 repo
//https://github.com/rust-lang/git2-rs/blob/master/examples/pull.rs
fn do_fetch<'a>(
    repo: &'a git2::Repository,
    refs: &[&str],
    remote: &'a mut git2::Remote,
) -> Result<git2::AnnotatedCommit<'a>, git2::Error> {
    let mut cb = git2::RemoteCallbacks::new();

    // Print out our transfer progress.
    cb.transfer_progress(|stats| {
        if stats.received_objects() == stats.total_objects() {
            print!(
                "  Resolving deltas {}/{}\r",
                stats.indexed_deltas(),
                stats.total_deltas()
            );
        } else if stats.total_objects() > 0 {
            print!(
                "  Received {}/{} objects ({}) in {} bytes\r",
                stats.received_objects(),
                stats.total_objects(),
                stats.indexed_objects(),
                stats.received_bytes()
            );
        }
        io::stdout().flush().unwrap();
        true
    });

    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(cb);
    // Always fetch all tags.
    // Perform a download and also update tips
    fo.download_tags(git2::AutotagOption::All);
    println!("  Fetching {} for repo", remote.name().unwrap());
    remote.fetch(refs, Some(&mut fo), None)?;

    // If there are local objects (we got a thin pack), then tell the user
    // how many objects we saved from having to cross the network.
    let stats = remote.stats();
    if stats.local_objects() > 0 {
        println!(
            "\r  Received {}/{} objects in {} bytes (used {} local \
             objects)",
            stats.indexed_objects(),
            stats.total_objects(),
            stats.received_bytes(),
            stats.local_objects()
        );
    } else {
        println!(
            "\r  Received {}/{} objects in {} bytes",
            stats.indexed_objects(),
            stats.total_objects(),
            stats.received_bytes()
        );
    }

    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    Ok(repo.reference_to_annotated_commit(&fetch_head)?)
}

fn fast_forward(
    repo: &Repository,
    lb: &mut git2::Reference,
    rc: &git2::AnnotatedCommit,
) -> Result<(), git2::Error> {
    let name = match lb.name() {
        Some(s) => s.to_string(),
        None => String::from_utf8_lossy(lb.name_bytes()).to_string(),
    };
    let msg = format!("  Fast-Forward: Setting {} to id: {}", name, rc.id());
    println!("{}", msg);
    lb.set_target(rc.id(), &msg)?;
    repo.set_head(&name)?;
    repo.checkout_head(Some(
        git2::build::CheckoutBuilder::default()
            // For some reason the force is required to make the working directory actually get updated
            // I suspect we should be adding some logic to handle dirty working directory states
            // but this is just an example so maybe not.
            .force(),
    ))?;
    Ok(())
}

fn normal_merge(
    repo: &Repository,
    local: &git2::AnnotatedCommit,
    remote: &git2::AnnotatedCommit,
) -> Result<(), git2::Error> {
    let local_tree = repo.find_commit(local.id())?.tree()?;
    let remote_tree = repo.find_commit(remote.id())?.tree()?;
    let ancestor = repo
        .find_commit(repo.merge_base(local.id(), remote.id())?)?
        .tree()?;
    let mut idx = repo.merge_trees(&ancestor, &local_tree, &remote_tree, None)?;

    if idx.has_conflicts() {
        println!("  Merge conflicts detected...");
        repo.checkout_index(Some(&mut idx), None)?;
        return Ok(());
    }
    let result_tree = repo.find_tree(idx.write_tree_to(repo)?)?;
    // now create the merge commit
    let msg = format!("  Merge: {} into {}", remote.id(), local.id());
    let sig = repo.signature()?;
    let local_commit = repo.find_commit(local.id())?;
    let remote_commit = repo.find_commit(remote.id())?;
    // Do our merge commit and set current branch head to that commit.
    let _merge_commit = repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &msg,
        &result_tree,
        &[&local_commit, &remote_commit],
    )?;
    // Set working tree to match head.
    repo.checkout_head(None)?;
    Ok(())
}

fn do_merge<'a>(
    repo: &'a Repository,
    remote_branch_var: &'a str,
    fetch_commit: git2::AnnotatedCommit<'a>,
) -> Result<(), git2::Error> {
    // 1. do a merge analysis
    let analysis = repo.merge_analysis(&[&fetch_commit])?;

    // 2. Do the appropriate merge
    if analysis.0.is_fast_forward() {
        println!("  Doing a fast forward");
        // do a fast forward
        let refname = format!("refs/heads/{}", remote_branch_var);
        match repo.find_reference(&refname) {
            Ok(mut r) => {
                fast_forward(repo, &mut r, &fetch_commit)?;
            }
            Err(_) => {
                // The branch doesn't exist so just set the reference to the
                // commit directly. Usually this is because you are pulling
                // into an empty repository.
                repo.reference(
                    &refname,
                    fetch_commit.id(),
                    true,
                    &format!("  Setting {} to {}", remote_branch_var, fetch_commit.id()),
                )?;
                repo.set_head(&refname)?;
                repo.checkout_head(Some(
                    git2::build::CheckoutBuilder::default()
                        .allow_conflicts(true)
                        .conflict_style_merge(true)
                        .force(),
                ))?;
            }
        };
    } else if analysis.0.is_normal() {
        // do a normal merge
        let head_commit = repo.reference_to_annotated_commit(&repo.head()?)?;
        normal_merge(&repo, &head_commit, &fetch_commit)?;
    } else {
        println!("  Nothing to do...");
    }
    Ok(())
}