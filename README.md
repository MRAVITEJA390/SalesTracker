## Setup Instructions  

**Pre-Requisites:** git, docker, docker compose  

1. Clone the repository  
2. Change directory into the clone directory  
3. Update the placeholder values in both `app.env` and `db.env`  
4. Run the following command to start the services:  
   
   ```sh
   docker compose up -d
5. The default path for the sample data is `sample_data.csv`


## Technical Specifications

- **Language**: Rust
- **Database**: PostgreSQL
- **ORM**: SQLx
- **Web Framework**: Actix-web
- **Build Tool**: Cargo
- **Containerization**: Docker


## API Endpoints

Below is a list of available API endpoints, their methods, expected request parameters, and sample responses.

Default URL: `http://localhost:8001`
Example: `http://localhost:8001/api/v1/health_check` etc

| Route | Method | Query Params / Body | Description | Sample Response | Sample Request |
|--------|--------|----------------------|-------------|-----------------|----------------|
| `/api/v1/health_check` | GET | None | Returns a simple health check status | `"Healthy"` | `http://localhost:8001/api/v1/health_check`
| `/api/v1/hello` | GET | None | Returns a simple greeting | `"Hello world!"` | `http://localhost:8001/api/v1/hello`
| `/api/v1/refresh` | GET | None | Triggers a scheduled data refresh asynchronously | `"Scheduled data refresh successful."` | `http://localhost:8001/api/v1/refresh`
| `/api/v1/revenue/by-category` | GET | `start_date`, `end_date` (YYYY-MM-DD) | Returns revenue breakdown by product category within the given date range | `[{ "name": "Electronics", "revenue": 50000.00 }]` | `http://localhost:8001/api/v1/revenue/by-category?start_date=2022-01-01&end_date=2022-12-31`
| `/api/v1/revenue/by-product` | GET | `start_date`, `end_date` (YYYY-MM-DD) | Returns revenue breakdown by product within the given date range | `[{ "name": "Laptop", "revenue": 25000.00 }]` | `http://localhost:8001/api/v1/revenue/by-product?start_date=2022-01-01&end_date=2022-12-31`
| `/api/v1/revenue/by-region` | GET | `start_date`, `end_date` (YYYY-MM-DD) | Returns revenue breakdown by region within the given date range | `[{ "name": "North America", "revenue": 60000.00 }]` | `http://localhost:8001/api/v1/revenue/by-region?start_date=2022-01-01&end_date=2022-12-31`
| `/api/v1/revenue` | GET | `start_date`, `end_date` (YYYY-MM-DD) | Returns the total revenue within the given date range | `{ "total_revenue": 150000.00 }` | `http://localhost:8001/api/v1/revenue?start_date=2022-01-01&end_date=2022-12-31`

### Notes:
- All date-based queries require `start_date` and `end_date` in the `YYYY-MM-DD` format.
- The revenue endpoints return revenue values as `BigDecimal`, formatted as JSON numbers.
- The `/api/v1/refresh` endpoint triggers a scheduled refresh and returns immediately; the actual operation runs asynchronously.
- Database Schema diagrams are provided in diagrams directory.
- The sample data file is located at `sample_data.csv` in the root directory.
