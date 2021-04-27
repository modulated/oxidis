SET DATABASE_URL = 'sqlite:data.db'
sqlx database create
sqlx migrate run