use std::env;

use sqlx::{Row, SqlitePool};
use once_cell::sync::Lazy;
use tokio::sync::OnceCell;

use crate::rconfuncs::query_rcon_server;

static DB_POOL: Lazy<OnceCell<SqlitePool>> = Lazy::new(OnceCell::new);

pub async fn init_db() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("db/statistics.sqlite")
                .create_if_missing(true)
        ).await;

    match pool {
        Ok(db) => {
            sqlx::migrate!("db/migrations").run(&db)
                .await
                .expect("Couldn't run database migrations");
        
            let _ = DB_POOL.set(db);
        },
        Err(e) => panic!("{e}")
    }
    // Run migrations, which updates the database's schema to the latest version.
}

pub fn get_db_pool() -> &'static SqlitePool {
    DB_POOL.get().expect("Database pool not initialized")
}

pub async fn update_db() {
    let addr_port: String = env::var("LOCAL_ADDR")
        .expect("Expected an rcon address:port in the environment");
    
    let rcon_pw= env::var("RCON_PW")
        .expect("Expected an rcon password in the environment");

    let result = sqlx::query("SELECT username FROM statistics;")
        .fetch_all(get_db_pool()).await
        .unwrap();

    let mut cmds: Vec<String> = Vec::with_capacity(result.len());
    for row in result.iter() {
        let username: &str = row.try_get("username").unwrap();
        println!("username = {}", username);
        if username != "" {
            cmds.push(format!("scoreboard players list {username}"));
        }
    }
    
    query_rcon_server(addr_port, rcon_pw, cmds).await;

}

// pub async fn query_db(query: String) -> String {
//     println!("{}", &query);
//     query
// }


