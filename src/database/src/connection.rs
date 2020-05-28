use diesel::Connection;
use dotenv::dotenv;

pub fn establish() -> diesel::PgConnection {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");
    return diesel::PgConnection::establish(&database_url).expect("Failed to connect database");
}
