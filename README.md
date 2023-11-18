# CRUD using sqlx and Postgres

Simple Rust code to perform CRUD operations 

## Setting up the project
1. Clone the repository 
2. Create a .env file in the root directory and add the following data to it
```
DATABASE_URL=postgres://<username>:<password>@<host>/<db_name>
```
3. Install sqlx-cli using cmd
```
cargo install sqlx-cli
```
4. Create the db using sqlx-cli
```
sqlx database creaet
```
5. Run the migrations
```
sqlx migration run 
```
6. Run the cargo file
```
cargo run
```
