use std::fs;
use std::path::{Path, PathBuf};

pub fn web_authentication_login() -> Vec<u8> {
    println!("Starting web server on port 8081");
    
    // Spawn a thread to run the web server
    std::thread::spawn(|| {
        // Create a server listening on port 8081
        let server = tiny_http::Server::http("0.0.0.0:8081").unwrap();
        println!("Server running at http://localhost:8081/");

        // Get the path to the static directory
        let static_dir = get_static_dir();
        println!("Serving static files from: {}", static_dir.display());

        // Handle incoming requests
        for request in server.incoming_requests() {
            let url = request.url();
            let path = if url == "/" {
                static_dir.join("index.html")
            } else {
                static_dir.join(url.trim_start_matches('/'))
            };

            let response = if path.exists() {
                let content_type = get_content_type(&path);
                match fs::read_to_string(&path) {
                    Ok(content) => tiny_http::Response::from_string(content)
                        .with_header(tiny_http::Header::from_str(&format!("Content-Type: {}", content_type)).unwrap()),
                    Err(_) => tiny_http::Response::from_string("Error reading file")
                        .with_status_code(tiny_http::StatusCode(500))
                }
            } else {
                tiny_http::Response::from_string("Not Found")
                    .with_status_code(tiny_http::StatusCode(404))
            };

            let _ = request.respond(response);
        }
    });

    // Return empty response to Flutter
    let response = vec![];
    response
}

fn get_static_dir() -> PathBuf {
    // Try to find the static directory relative to the current executable
    let mut dir = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    dir.pop(); // Remove executable name
    
    // Try different relative paths to find the static directory
    let possible_paths = [
        "static",
        "../static",
        "../../static",
        "../../../static",
        "linux/static",
    ];
    
    for path in possible_paths {
        let test_path = dir.join(path);
        if test_path.exists() && test_path.is_dir() {
            return test_path;
        }
    }
    
    // Fallback to a hardcoded path if we can't find it
    PathBuf::from("linux/static")
}

fn get_content_type(path: &Path) -> String {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8".to_string(),
        Some("js") => "application/javascript; charset=utf-8".to_string(),
        Some("css") => "text/css; charset=utf-8".to_string(),
        Some("png") => "image/png".to_string(),
        Some("jpg") | Some("jpeg") => "image/jpeg".to_string(),
        Some("svg") => "image/svg+xml".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}

pub fn web_authentication_logout(
    _domain: String,
    _client_id: String,
    _return_to: Option<String>,
) -> Vec<u8> {
    let response = vec![];
    response
}
