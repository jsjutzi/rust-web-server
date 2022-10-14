use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        //TODO: Expand to real structure
        let stream = stream.unwrap();

        println!("Connection established");
    }
}