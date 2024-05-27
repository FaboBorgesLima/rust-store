use mysql::{prelude::Queryable, Error, PooledConn};
use rust_decimal::Decimal;

#[derive(PartialEq, Eq)]
pub struct StoreProduct {
    pub product_id: i32,
    pub price: Decimal,
    pub quantity: i32,
    pub product_name: String,
}

impl StoreProduct {
    pub fn to_json(&self) -> String {
        return format!(
            "{{product_id:{product_id},price:{price},quantity:{quantity},product_name:\"{product_name}\"}}",
            product_id = self.product_id,
            product_name = self.product_name,
            price = self.price,
            quantity = self.quantity,

        );
    }
    pub fn all(mut conn: PooledConn) -> Result<Vec<StoreProduct>, Error> {
        conn.query_map(
            "SELECT product_id,price , quantity, product_name FROM product",
            |(product_id, price, quantity, product_name)| StoreProduct {
                product_id,
                price,
                product_name,
                quantity,
            },
        )
    }
}
