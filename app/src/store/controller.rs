use mysql::{Error, Pool, PooledConn};
use rust_decimal::Decimal;

use crate::http::{request::Request, response::Response, url_params_decoder};

use super::store_product::StoreProduct;

pub struct Controller {
    conn: PooledConn,
}

impl Controller {
    pub fn new(pool: Pool) -> Result<Self, Error> {
        let conn = pool.get_conn();

        match conn {
            Ok(conn) => Ok(Self { conn }),
            Err(er) => Err(er),
        }
    }

    pub fn all(&mut self) -> Response {
        let products = StoreProduct::all(&mut self.conn);

        let products = match products {
            Ok(products) => products,
            Err(_) => return Response::server_error(),
        };

        let products = products.iter().map(|product| product.to_json());

        let mut body = String::from("{\"products\":[");

        for product in products {
            body.push_str(&product);
            body.push_str(",");
        }

        body.pop();

        body.push_str("]}");

        Response {
            body,
            content_type: crate::http::response::content_type::ContentType::ApplicationJson,
            code: 200,
        }
    }

    pub fn create(&mut self, request: &Request) -> Response {
        let decoded = url_params_decoder::UrlParamsDecoder::decode(&request.body);

        let product_name = decoded.get("product_name");

        let quantity = decoded.get("quantity");

        let price = decoded.get("price");

        if product_name.is_none() || quantity.is_none() || price.is_none() {
            return Response::bad_request();
        }

        let product_name = product_name.unwrap();

        let quantity = quantity.unwrap().parse::<i32>();

        let price = price.unwrap().parse::<Decimal>();

        if quantity.is_err() || price.is_err() {
            return Response::bad_request();
        }

        let price = price.unwrap();

        let quantity = quantity.unwrap();

        let product = StoreProduct::new(price, quantity, product_name.to_string());

        let result = product.insert(&mut self.conn);

        match result {
            Ok(_) => Response::new(
                crate::http::response::content_type::ContentType::ApplicationJson,
                200,
                "{\"result\":true}".to_string(),
            ),
            Err(_) => Response::server_error(),
        }
    }
}
