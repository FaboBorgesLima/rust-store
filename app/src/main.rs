mod store;

use mysql::{OptsBuilder, Pool};
use store::store_product::StoreProduct;

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

    let conn = pool.get_conn().unwrap();

    let res = StoreProduct::all(conn).unwrap();

    let first_item = res.get(0).unwrap();

    println!("{}", first_item.to_json());
}
