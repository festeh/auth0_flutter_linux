pub fn web_authentication_login() -> Vec<u8> {
    println!("Starting web server on port 8081");
    
    // Spawn a thread to run the web server
    std::thread::spawn(|| {
        // Create a server listening on port 8081
        let server = tiny_http::Server::http("0.0.0.0:8081").unwrap();
        println!("Server running at http://localhost:8081/");

        // HTML content with Tailwind CSS
        let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Auth0 Flutter Linux</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <style type="text/tailwindcss">
        @layer components {
            .card {
                @apply bg-white rounded-lg shadow-lg p-6 max-w-md mx-auto mt-10;
            }
            .title {
                @apply text-2xl font-bold text-center text-blue-600 mb-4;
            }
        }
    </style>
</head>
<body class="bg-gray-100 min-h-screen flex items-center justify-center">
    <div class="card">
        <h1 class="title">Hello World</h1>
        <p class="text-gray-700 text-center">Welcome to Auth0 Flutter Linux</p>
        <div class="mt-4 flex justify-center">
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                Click Me
            </button>
        </div>
    </div>

    <script>
        console.log("foo");
        document.querySelector('button').addEventListener('click', () => {
            alert('Button clicked!');
        });
    </script>
</body>
</html>"#;

        // Handle incoming requests
        for request in server.incoming_requests() {
            let response = tiny_http::Response::from_string(html)
                .with_header(tiny_http::Header::from_str("Content-Type: text/html").unwrap());
            let _ = request.respond(response);
        }
    });

    // Return empty response to Flutter
    let response = vec![];
    response
}

pub fn web_authentication_logout(
    _domain: String,
    _client_id: String,
    _return_to: Option<String>,
) -> Vec<u8> {
    let response = vec![];
    response
}
