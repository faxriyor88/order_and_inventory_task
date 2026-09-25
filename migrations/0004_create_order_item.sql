CREATE TABLE order_item
(
    id         BIGSERIAL PRIMARY KEY,
    order_id   BIGINT         NOT NULL REFERENCES orders (id),
    product_id BIGINT         NOT NULL REFERENCES product (id),
    quantity   INTEGER        NOT NULL,
    unit_price NUMERIC(17, 4) NOT NULL,
    line_total NUMERIC(17, 4) NOT NULL,
    UNIQUE (order_id, product_id)
)