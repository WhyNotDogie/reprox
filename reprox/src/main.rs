mod server;
mod arg;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    log!("Starting server on port: {}", 1024);
    server::run(1024)
}