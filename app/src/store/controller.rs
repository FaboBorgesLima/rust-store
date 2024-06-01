use mysql::{Error, Pool, PooledConn};

use crate::http::response::Response;

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
            Err(_) => {
                return Response::new(
                    crate::http::response::content_type::ContentType::ApplicationJson,
                    500,
                    String::new(),
                )
            }
        };

        let products = products.iter().map(|product| product.to_json());

        let mut body = String::from("{\"products\":[");

        for product in products {
            body.push_str(&product);
        }

        body.push_str("]}");

        Response {
            body,
            content_type: crate::http::response::content_type::ContentType::ApplicationJson,
            code: 200,
        }
    }
}
