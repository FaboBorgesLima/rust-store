use std::{
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

pub struct RequestReader {}
impl RequestReader {
    pub fn read(tcp_stream: &TcpStream) -> HttpRequest {
        let mut buf_reader = BufReader::new(tcp_stream);
        let header = Self::read_header(&mut buf_reader);
        let body = Self::read_body(&mut buf_reader, &header);

        HttpRequest { header, body }
    }
    fn read_header(buf_reader: &mut BufReader<&TcpStream>) -> HttpHeader {
        let mut line = String::new();

        let _ = buf_reader.read_line(&mut line).unwrap();

        let mut method_path_params = line.split(&[' ', '?'][..]);

        let method = method_path_params.nth(0).unwrap_or("GET");

        let method = match method {
            "DELETE" => RequestMethod::DELETE,
            "PATH" => RequestMethod::PATH,
            "PUT" => RequestMethod::PUT,
            "POST" => RequestMethod::POST,
            _ => RequestMethod::GET,
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

        HttpHeader {
            method,
            path,
            length,
            url_params,
        }
    }
    fn read_body(buf_reader: &mut BufReader<&TcpStream>, header: &HttpHeader) -> String {
        let mut buf = vec![0; header.length];

        buf_reader.read_exact(&mut buf).unwrap();

        String::from_utf8(buf).unwrap()
    }
}
pub struct HttpRequest {
    pub header: HttpHeader,
    pub body: String,
}

pub struct HttpHeader {
    pub length: usize,
    pub path: String,
    pub method: RequestMethod,
    pub url_params: String,
}

pub enum RequestMethod {
    GET,
    POST,
    DELETE,
    PUT,
    PATH,
}
