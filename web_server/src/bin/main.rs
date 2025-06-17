use std::io::prelude::*; // ler e escrever em streams
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;
use std::thread;
use std::time::Duration;

extern crate web_server;
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    //                              ip lookback:porta
    // bind é como new
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(5) {
    // incoming gera um iterador de streams
    // um stream representa uma conexão aberta
    // entre cliente e servidor.
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
        // acessar 127.0.0.1 no navegador
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    // 1024 bytes de memoria
    let mut buffer = [0; 1024];
    // le bytes e armazena no buffer
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // converte os bytes pra uma string
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
