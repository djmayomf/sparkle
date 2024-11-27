# PowerShell script to set up the database

$env:PGPASSWORD = "Kang23dJ"

# Create database
psql -U postgres -c "DROP DATABASE IF EXISTS sparkle;"
psql -U postgres -c "CREATE DATABASE sparkle;"

# Run schema setup
psql -U postgres -d sparkle -f migrations/20240101000000_initial_schema.sql

Write-Host "Database setup complete!" 