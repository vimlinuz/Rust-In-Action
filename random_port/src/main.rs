use std::net::TcpListener;
fn get_free_port() -> std::io::Result<u16> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    Ok(port)
}
fn main() {
    for _ in 0..10 {
        match get_free_port() {
            Ok(port) => println!("Found free port: {}", port),
            Err(e) => eprintln!("Error finding free port: {}", e),
        }
    }
}
