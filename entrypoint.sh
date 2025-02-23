#!/bin/sh
set -e  # Exit immediately if any command fails

echo  "DATABASE_URL: $DATABASE_URL"

sqlx migrate run

# Execute the Rust application
/app/sales_tracker
