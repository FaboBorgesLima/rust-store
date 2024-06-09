use std::collections::HashMap;

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

    pub fn delete(&mut self, request: &Request) -> Response {
        let decoded = url_params_decoder::UrlParamsDecoder::decode(&request.header.url_params);

        let product_id = decoded.get("product_id");

        let product_id = match product_id {
            Some(id) => id,
            None => return Response::bad_request(),
        };

        let product_id = match product_id.parse::<i32>() {
            Ok(id) => id,
            Err(_) => return Response::bad_request(),
        };

        match StoreProduct::delete(&mut self.conn, product_id) {
            Ok(_) => Response::ok(None),
            Err(_) => Response::bad_request(),
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

        let products_len = products.len();

        for product in products {
            body.push_str(&product);
            body.push_str(",");
        }

        if products_len > 0 {
            body.pop();
        }

        body.push_str("]}");

        Response::new(
            crate::http::response::content_type::ContentType::ApplicationJson,
            200,
            body,
        )
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

        if price.is_zero() || price.is_sign_negative() {
            return Response::bad_request();
        };
        if product_name.clone().chars().count() < 3 {
            return Response::bad_request();
        }

        let result = product.insert(&mut self.conn);

        match result {
            Ok(_) => Response::ok(None),
            Err(_) => Response::server_error(),
        }
    }

    pub fn update(&mut self, request: &Request) -> Response {
        let decoded = url_params_decoder::UrlParamsDecoder::decode(&request.body);

        let store_product = hash_map_get_store_product(&decoded);

        let store_product = match store_product {
            Some(store_product) => store_product,
            None => return Response::bad_request(),
        };

        match store_product.update(&mut self.conn) {
            Ok(_) => Response::ok(None),
            Err(_) => Response::server_error(),
        }
    }
}

fn hash_map_get_store_product_no_id(hash_map: &HashMap<String, String>) -> Option<StoreProduct> {
    let product_name = hash_map.get("product_name");

    let quantity = hash_map.get("quantity");

    let price = hash_map.get("price");

    let (quantity, price, product_name) = match (quantity, price, product_name) {
        (Some(quantity), Some(price), Some(product_name)) => (quantity, price, product_name),
        _ => return None,
    };

    let quantity = match quantity.parse() {
        Ok(quantity) => quantity,
        Err(_) => return None,
    };

    let price = match price.parse() {
        Ok(price) => price,
        Err(_) => return None,
    };

    Some(StoreProduct::new(price, quantity, product_name.clone()))
}

fn hash_map_get_store_product(hash_map: &HashMap<String, String>) -> Option<StoreProduct> {
    let store_product_no_id = hash_map_get_store_product_no_id(hash_map);

    let store_product_no_id = match store_product_no_id {
        Some(store_product_no_id) => store_product_no_id,
        None => return None,
    };

    let product_id = hash_map.get("product_id");

    let product_id = match product_id {
        Some(product_id) => product_id,
        None => return None,
    };
    let product_id = match product_id.parse() {
        Ok(product_id) => product_id,
        Err(_) => return None,
    };

    let mut store_product = store_product_no_id;

    store_product.product_id = product_id;

    Some(store_product)
}
