mod http;
mod store;
mod thread_pool;

use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use mysql::{OptsBuilder, Pool};

fn main() {
    let db_user_name = std::env::var("MARIADB_USER").unwrap();
    let db_user_password =
        std::fs::read_to_string(std::env::var("MARIADB_PASSWORD_FILE").unwrap()).unwrap();
    let db_name = std::env::var("MARIADB_DATABASE").unwrap();
    let db_address = std::env::var("MARIADB_ADDRESS").unwrap();
    let db_port: u16 = std::env::var("MARIADB_PORT").unwrap().parse().unwrap();

    let builder = OptsBuilder::new();

    let db_pool = Pool::new(
        builder
            .db_name(Some(db_name))
            .ip_or_hostname(Some(db_address))
            .pass(Some(db_user_password))
            .user(Some(db_user_name))
            .tcp_port(db_port),
    )
    .unwrap();

    let server_poll = thread_pool::ThreadPool::new(3);

    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("started");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        server_poll.execute(move || handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let request = http::request_reader::RequestReader::read(&stream);

    stream.write_all(request.header.path.as_bytes()).unwrap();
}
