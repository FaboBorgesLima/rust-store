mod store;

use mysql::{OptsBuilder, Pool};
use rust_decimal::Decimal;
use store::store_product::StoreProduct;

fn main() {
    let db_user_name = std::env::var("MARIADB_USER").unwrap();
    let db_user_password =
        std::fs::read_to_string(std::env::var("MARIADB_PASSWORD_FILE").unwrap()).unwrap();
    let db_name = std::env::var("MARIADB_DATABASE").unwrap();
    let db_address = std::env::var("MARIADB_ADDRESS").unwrap();
    let db_port: u16 = std::env::var("MARIADB_PORT").unwrap().parse().unwrap();

    let builder = OptsBuilder::new();

    let pool = Pool::new(
        builder
            .db_name(Some(db_name))
            .ip_or_hostname(Some(db_address))
            .pass(Some(db_user_password))
            .user(Some(db_user_name))
            .tcp_port(db_port),
    )
    .unwrap();

    let mut conn = pool.get_conn().unwrap();
}
