use mysql::{params, prelude::Queryable, OptsBuilder, Pool, Row};
use rust_decimal::Decimal;
#[derive(PartialEq, Eq)]
struct Product {
    product_id: i32,
    price: Decimal,
    quantity: i32,
    product_name: String,
}

impl Product {
    pub fn to_json(&self) -> String {
        return format!(
            "{{product_id:{product_id},price:{price},quantity:{quantity},product_name:\"{product_name}\"}}",
            product_id = self.product_id,
            product_name = self.product_name,
            price = self.price,
            quantity = self.quantity,

        );
    }
}
fn main() {
    let db_user_name = std::env::var("MARIADB_USER").unwrap();
    let db_user_password =
        std::fs::read_to_string(std::env::var("MARIADB_PASSWORD_FILE").unwrap()).unwrap();
    let db_name = std::env::var("MARIADB_DATABASE").unwrap();
    let db_socket_address = std::env::var("MARIADB_SOCKET_ADDRESS").unwrap();

    let builder = OptsBuilder::new();

    let pool = Pool::new(
        builder
            .db_name(Some(db_name))
            .ip_or_hostname(Some(db_socket_address))
            .pass(Some(db_user_password))
            .user(Some(db_user_name)),
    )
    .unwrap();

    let mut conn = pool.get_conn().unwrap();

    let _: Vec<Row> = conn.exec(
        r"INSERT INTO product (price,quantity,product_name) VALUES (:price,:quantity,:product_name)",
        params! {
            "price" => 1,
            "quantity" => 1,
            "product_name" => "miojo"
        },
    ).unwrap();

    let res = conn
        .query_map(
            "SELECT product_id,price , quantity, product_name FROM product",
            |(product_id, price, quantity, product_name)| Product {
                product_id,
                price,
                product_name,
                quantity,
            },
        )
        .unwrap();

    let first_item = res.get(0).unwrap();

    println!("{}", first_item.to_json());
}
