use actix_web::{get, web, HttpResponse, Responder};
use pulldown_cmark::{html, Parser};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn get_content_dir() -> PathBuf {
    if let Ok(path) = std::env::var("CARGO_MANIFEST_DIR") {
        return Path::new(&path).join("../content");
    }
    PathBuf::from("../content")
}

#[derive(Serialize)]
struct Post {
    slug: String,
    title: String,
}

#[get("/posts")]
async fn get_posts() -> impl Responder {
    let mut posts = Vec::new();
    let content_dir = get_content_dir();

    if !content_dir.exists() {
        let msg = format!("Content directory not found at: {}", content_dir.display());
        return HttpResponse::InternalServerError().body(msg);
    }

    for entry in WalkDir::new(&content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            if let Some(slug) = path.file_stem().and_then(|s| s.to_str()) {
                if let Ok(content) = fs::read_to_string(path) {
                    let title = content
                        .lines()
                        .next()
                        .unwrap_or("")
                        .trim_start_matches('#')
                        .trim()
                        .to_string();
                    posts.push(Post {
                        slug: slug.to_string(),
                        title,
                    });
                }
            }
        }
    }

    HttpResponse::Ok().json(posts)
}

#[get("/posts/{slug}")]
async fn get_post(slug: web::Path<String>) -> impl Responder {
    let slug_str = slug.into_inner();
    let content_dir = get_content_dir();
    let file_path = content_dir.join(format!("{}.md", slug_str));

    match fs::read_to_string(&file_path) {
        Ok(markdown) => {
            let parser = Parser::new(&markdown);
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html_output)
        }
        Err(_) => {
            let msg = format!("Post not found at: {}", file_path.display());
            HttpResponse::NotFound().body(msg)
        }
    }
}
