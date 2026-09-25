CREATE TABLE product
(
    id             BIGSERIAL PRIMARY KEY,
    name           TEXT NOT NULL,
    price          NUMERIC(17, 4),
    stock_quantity INTEGER
);