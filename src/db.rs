use sea_orm::{Database, DatabaseConnection};

// pub async fn get_db_pool() -> PgPool {
//     let db_username = env::var("DB_USER").expect("DB_USER not set");
//     let db_password = env::var("DB_PWD").expect("DB_PWD not set");
//     let db_host = env::var("DB_HOST").expect("DB_HOST not set");
//     let db_port = env::var("DB_PORT").expect("DB_PORT not set");
//     let db_name = env::var("DB_NAME").expect("DB_NAME not set");
//     let database_url = format!(
//         "postgres://{}:{}@{}:{}/{}",
//         db_username, db_password, db_host, db_port, db_name
//     );
//     PgPool::connect(&database_url).await.expect("Failed to create pool")
// }

pub async fn get_db_pool() -> Result<DatabaseConnection, sea_orm::DbErr> {
    let db_username = std::env::var("DB_USER").expect("DB_USER not set");
        let db_password = std::env::var("DB_PWD").expect("DB_PWD not set");
        let db_host = std::env::var("DB_HOST").expect("DB_HOST not set");
        let db_port = std::env::var("DB_PORT").expect("DB_PORT not set");
        let db_name = std::env::var("DB_NAME").expect("DB_NAME not set");
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            db_username, db_password, db_host, db_port, db_name
        );

    Database::connect(&database_url).await
}