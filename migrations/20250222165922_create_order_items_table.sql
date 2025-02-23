-- Order items table (linking products and orders)
CREATE TABLE order_items (
    id SERIAL PRIMARY KEY,
    order_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    quantity_sold INTEGER NOT NULL CHECK (quantity_sold > 0),
    discount NUMERIC(4, 2) CHECK (discount BETWEEN 0.0 AND 1.0),
    shipping_cost NUMERIC(10, 2) NOT NULL,
    CONSTRAINT unique_order_product_item UNIQUE (order_id, product_id),
    FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE CASCADE,
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE
);