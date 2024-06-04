use std::{
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use super::{header::Header, method::Method, Request};

pub struct Reader {}
impl Reader {
    pub fn read(tcp_stream: &TcpStream) -> Request {
        let mut buf_reader = BufReader::new(tcp_stream);
        let header = Self::read_header(&mut buf_reader);
        let body = Self::read_body(&mut buf_reader, &header);

        Request { header, body }
    }
    fn read_header(buf_reader: &mut BufReader<&TcpStream>) -> Header {
        let mut line = String::new();

        let _ = buf_reader.read_line(&mut line).unwrap();

        let mut method_path_params = line.split(&[' ', '?'][..]);

        let method = method_path_params.nth(0).unwrap_or("GET");

        let method = match method {
            "DELETE" => Method::DELETE,
            "PATH" => Method::PATH,
            "PUT" => Method::PUT,
            "POST" => Method::POST,
            "OPTIONS" => Method::OPTIONS,
            _ => Method::GET,
        };

        let path = method_path_params.nth(0).unwrap_or("/").to_string();

        let mut length = 0;

        let url_params = method_path_params.nth(0).unwrap_or("").to_string();

        loop {
            let mut line = String::new();

            let n_bytes = buf_reader.read_line(&mut line).unwrap();

            if n_bytes < 3 {
                break;
            }

            if line.starts_with("Content-Length:") {
                length = line
                    .split(":")
                    .nth(1)
                    .unwrap_or("0")
                    .trim()
                    .parse()
                    .unwrap_or(0);
            }
        }

        Header {
            method,
            path,
            length,
            url_params,
        }
    }
    fn read_body(buf_reader: &mut BufReader<&TcpStream>, header: &Header) -> String {
        let mut buf = vec![0; header.length];

        buf_reader.read_exact(&mut buf).unwrap();

        String::from_utf8(buf).unwrap()
    }
}
