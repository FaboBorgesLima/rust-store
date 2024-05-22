fn main() {
    let db_name = std::env::var("db_name");
    let db_password = std::fs::read_to_string("/run/secrets/db_password.txt");

    let db_name = match db_name {
        Ok(name) => name,
        Err(_) => panic!("no db_name env var"),
    };

    let db_password = match db_password {
        Ok(passsword) => passsword,
        Err(_) => panic!("no db_password"),
    };

    println!("db name : {}", db_name);
    println!("db password : {}", db_password);
}
