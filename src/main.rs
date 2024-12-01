// Module declarations
mod calculator;
mod routes;

// External imports
use actix_web::{App, HttpServer};
use routes::{main_routes, calculator_routes};

/// Main entry point for the calculator web service
/// Sets up and runs the HTTP server with configured routes
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Server configuration
    let host = "127.0.0.1";
    let port = 8080;

    println!("Server running at http://{}:{}. Try it!", host, port);

    // Create and configure the HTTP server
    HttpServer::new(|| {
        App::new()
            // Register route configurations
            .service(main_routes())        // Adds the health check endpoint
            .service(calculator_routes())   // Adds all calculator endpoints
    })
        .bind((host, port))?  // Bind server to specified host:port
        .run()               // Start the server
        .await              // Wait for server completion
}