CREATE TABLE product (
    product_id INT NOT NULL AUTO_INCREMENT,
    price DECIMAL(7,2) NOT NULL,
    quantity INT NOT NULL,
    product_name VARCHAR(255) NOT NULL,
    PRIMARY KEY (product_id)
);