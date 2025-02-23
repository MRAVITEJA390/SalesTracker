mod api;
mod config;
mod scripts;

use actix_web::{App, HttpServer, web::Data};
use scripts::start_data_refresh_job;
use tokio::spawn as tokio_spawn;

use api::{
   health_check_route, 
   hello_route, 
   refresh_route, 
   total_revenue_route, 
   revenue_by_category_route,
   revenue_by_product_route,
   revenue_by_region_route,
};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
   let csv_path = config::APP_CONFIG.csv_path.clone();

   let pool = config::DB_CONFIG.get_pool().await.expect("Failed to connect to database");
   
   config::LOGGER_CONFIG.init();

   {
      let pool_clone = pool.clone();
      let csv_path_clone = csv_path.clone();
      tokio_spawn( async move {
         start_data_refresh_job(pool_clone, csv_path_clone).await
      });
   }
   
   HttpServer::new(move || {
      App::new()
      .app_data(Data::new(pool.clone()))
      .app_data(Data::new(csv_path.to_string().clone()))
      .service(hello_route)
      .service(health_check_route)
      .service(refresh_route)
      .service(total_revenue_route)
      .service(revenue_by_product_route)
      .service(revenue_by_category_route)
      .service(revenue_by_region_route)
   }).bind(("0.0.0.0", 8080))?.run().await
}
