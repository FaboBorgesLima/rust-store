use mysql::{params, prelude::Queryable, Conn, Error, PooledConn};
use rust_decimal::Decimal;

#[derive(PartialEq, Eq)]
pub struct StoreProduct {
    pub product_id: i32,
    pub price: Decimal,
    pub quantity: i32,
    pub product_name: String,
}

impl Clone for StoreProduct {
    fn clone(&self) -> Self {
        Self {
            product_id: self.product_id.clone(),
            price: self.price.clone(),
            quantity: self.quantity.clone(),
            product_name: self.product_name.clone(),
        }
    }
}

impl StoreProduct {
    pub fn to_json(&self) -> String {
        return format!(
            "{{\"product_id\":{product_id},\"price\":{price},\"quantity\":{quantity},\"product_name\":\"{product_name}\"}}",
            product_id = self.product_id,
            product_name = self.product_name,
            price = self.price,
            quantity = self.quantity,

        );
    }
    pub fn new(price: Decimal, quantity: i32, product_name: String) -> StoreProduct {
        StoreProduct {
            price,
            quantity,
            product_id: 0,
            product_name,
        }
    }
    pub fn all(conn: &mut PooledConn) -> Result<Vec<StoreProduct>, Error> {
        conn.query_map(
            "SELECT product_id, price, quantity, product_name FROM product",
            |(product_id, price, quantity, product_name)| StoreProduct {
                product_id,
                price,
                product_name,
                quantity,
            },
        )
    }
    pub fn delete(conn: &mut PooledConn, store_prod: &StoreProduct) {
        let _ = conn.exec_drop(
            "DELETE FROM product WHERE product_id=:product_id",
            params!("product_id"=> store_prod.product_id),
        );
    }
    pub fn insert(&self, conn: &mut PooledConn) {
        let _ = conn.exec_drop(
            "INSERT INTO product (price,quantity,product_name) VALUES (:price, :quantity, :product_name)",
            params!("price"=>self.price,"quantity"=>self.quantity,"product_name"=>&self.product_name),
        );
    }

    pub fn update(&self, conn: &mut PooledConn) {
        let _ = conn.exec_drop(
            "UPDATE product SET price = :price,quantity = :quantity ,product_name = :product_name WHERE product_id = :product_id",
            params!(":product_id"=>self.product_id,"price"=>self.price,"quantity"=>self.quantity,"product_name"=>&self.product_name),
        );
    }
}
