CREATE TABLE orders
(
    id          BIGSERIAL PRIMARY KEY,
    total_price NUMERIC(17, 4),
    comment     TEXT
);