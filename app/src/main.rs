mod http;
mod store;
mod thread_pool;

use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use http::response;
use mysql::{OptsBuilder, Pool};
use store::store_product::StoreProduct;

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

    let server_poll = thread_pool::ThreadPool::new(3, &db_pool);

    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("started");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        server_poll.execute(move |pool| handle_connection(stream, pool));
    }
}

fn handle_connection(mut stream: TcpStream, pool: Pool) {
    let request = http::request::reader::Reader::read(&stream);

    let mut conn = pool.get_conn().unwrap();

    let all = StoreProduct::all(&mut conn).unwrap();

    let all = all
        .iter()
        .map(|prod| prod.to_json())
        .collect::<Vec<_>>()
        .join(",");

    let mut body = String::from("{products:[");
    body.push_str(&all);

    body.push_str("]}");

    let response =
        response::Response::new(response::content_type::ContentType::Text, 200, request.body);

    stream.write_all(response.to_string().as_bytes()).unwrap();
}
