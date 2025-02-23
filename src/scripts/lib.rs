use sqlx::{PgPool, types::{BigDecimal, time::Date}};
use time::macros::format_description;
use std::path::Path;
use std::fs::File;
use std::str::FromStr;
use csv::Reader;
use serde::Deserialize;
use log::info;


#[derive(Debug, Deserialize, Clone)]
struct SaleRecord {
    #[serde(rename = "Order ID")]
    order_id: String,
    #[serde(rename = "Product ID")]
    product_id: String,
    #[serde(rename = "Customer ID")]
    customer_id: String,
    #[serde(rename = "Product Name")]
    product_name: String,
    #[serde(rename = "Category")]
    category: String,
    #[serde(rename = "Region")]
    region: String,
    #[serde(rename = "Date of Sale")]
    date_of_sale: String,
    #[serde(rename = "Quantity Sold")]
    quantity_sold: i32,
    #[serde(rename = "Unit Price")]
    unit_price: f64,
    #[serde(rename = "Discount")]
    discount: f64,
    #[serde(rename = "Shipping Cost")]
    shipping_cost: f64,
    #[serde(rename = "Payment Method")]
    payment_method: String,
    #[serde(rename = "Customer Name")]
    customer_name: String,
    #[serde(rename = "Customer Email")]
    customer_email: String,
    #[serde(rename = "Customer Address")]
    customer_address: String,
}

pub async fn load_csv<P: AsRef<Path> + std::fmt::Display>(file_path: P, pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(&file_path);
    
    if file.is_err() {
        info!("Error opening file {file_path}");
        let err_msg = format!("Error opening file {0}", file.unwrap_err().to_string());
        info!("{err_msg}");

        return Err("Error opening file {file.unwrap_err()}".into());
    }

    let file = file.unwrap();
    let mut csv_reader = Reader::from_reader(file);

    for result in csv_reader.deserialize::<SaleRecord>() {
        if result.is_err() {
            info!("Skipping invalid record {result:?}");
            continue
        }
        
        let record = result.unwrap();

        info!("Processing record: {record:?}");

        // Insert into `products`
        sqlx::query!(
            r#"INSERT INTO products (id, name, category, unit_price)
               VALUES ($1, $2, $3, $4)
               ON CONFLICT (id) DO NOTHING"#,
            record.product_id,
            record.product_name,
            record.category,
            BigDecimal::from_str(record.unit_price.to_string().as_str())?
        ).execute(pool).await?;

        // Insert into `customers`
        sqlx::query!(
            r#"INSERT INTO customers (id, name, email, address)
               VALUES ($1, $2, $3, $4)
               ON CONFLICT (id) DO NOTHING"#,
            record.customer_id,
            record.customer_name,
            record.customer_email,
            record.customer_address
        ).execute(pool).await?;

        // Insert into `orders`
        let description = format_description!("[year]-[month]-[day]");
        let Some(date_of_sale) = Date::parse(record.date_of_sale.as_str(),  &description).ok() else {
            info!("Skipping invalid date of sale {0}", record.date_of_sale);
            continue
        };
        sqlx::query!(
            r#"INSERT INTO orders (id, customer_id, date_of_sale, payment_method, total_amount)
               VALUES ($1, $2, $3, $4, $5)
               ON CONFLICT (id) DO NOTHING"#,
            record.order_id,
            record.customer_id,
            date_of_sale,
            record.payment_method,
            BigDecimal::from_str(
                ((record.quantity_sold as f64 * record.unit_price) * (1.0 - record.discount) + record.shipping_cost).to_string().as_str()
            )?
        ).execute(pool).await?;

        // Insert into `order_items`
        sqlx::query!(
            r#"INSERT INTO order_items (order_id, product_id, quantity_sold, discount, shipping_cost)
               VALUES ($1, $2, $3, $4, $5)
               ON CONFLICT (order_id, product_id) DO NOTHING"#,
            record.order_id,
            record.product_id,
            record.quantity_sold,
            BigDecimal::from_str(record.discount.to_string().as_str())?,
            BigDecimal::from_str(record.shipping_cost.to_string().as_str())?
        ).execute(pool).await?;

        // Insert into `regions` and link to order
        let region_id = sqlx::query_scalar!(
            r#"INSERT INTO regions (name)
               VALUES ($1)
               ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
               RETURNING id"#,
            record.region
        )
        .fetch_one(pool)
        .await?;

        sqlx::query!(
            r#"INSERT INTO order_regions (order_id, region_id)
               VALUES ($1, $2)
               ON CONFLICT DO NOTHING"#,
            record.order_id,
            region_id
        ).execute(pool).await?;
    }

    info!("CSV data successfully loaded!");
    Ok(())
}
