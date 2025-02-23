-- Order-Region mapping table (because one order is placed in a specific region)
CREATE TABLE order_regions (
    order_id TEXT NOT NULL,
    region_id INTEGER NOT NULL,
    PRIMARY KEY (order_id, region_id),
    FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE CASCADE,
    FOREIGN KEY (region_id) REFERENCES regions(id) ON DELETE CASCADE
);