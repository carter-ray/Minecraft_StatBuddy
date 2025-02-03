
use std::collections::HashMap;

use regex::Regex;
use sqlx::{sqlite::SqliteRow, SqlitePool};
use once_cell::sync::Lazy;
use tokio::sync::OnceCell;

use crate::{constants::{get_stat_vec, StatCategory}, rconfuncs::{get_whitelist, query_rcon_server}};

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
}

pub fn get_db_pool() -> &'static SqlitePool {
    DB_POOL.get().expect("Database pool not initialized")
}

pub async fn update_db() {
    
    let players: Vec<String> = get_whitelist().await;
    
    let mut cmds: HashMap<String, String> = HashMap::new();
    for username in players {
        let val = format!("scoreboard players list {}", username);
        cmds.insert(username, val);
    }
    query_rcon_server(&mut cmds).await;
    
    let re: Regex = Regex::new(r"\[([\w ]+)\]: *([\w\d]+)").unwrap();

    for (user, result) in cmds {
        let mut col_stat_map: HashMap<String, String> = HashMap::new();
    
        for cap in re.captures_iter(&result) {
            if let (Some(name), Some(stat)) = (cap.get(1), cap.get(2)) {
                col_stat_map.insert(name.as_str().to_string().replace(" ", "_").replace("'", "").to_lowercase(), stat.as_str().parse().unwrap());
            }
        }

        
        let cols: Vec<String> = get_stat_vec(StatCategory::All)
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        let vals: Vec<String> = cols.iter().map(|col| {
            col_stat_map.get(col).map(|val| val.to_string()).unwrap_or_else(|| col.to_owned())
        }).collect();

        let query = format!(
            "INSERT OR REPLACE INTO statistics (username, {}) VALUES ({}, {})",
            cols.join(", "),
            user,
            vals.join(", ")
        );

        post_db(&query).await;
    }
    

}

pub async fn query_db(query: &str) -> Vec<SqliteRow> {
    sqlx::query(query)
        .fetch_all(get_db_pool()).await
        .unwrap()
}

pub async fn post_db(cmd: &str) {
    let _ = sqlx::query(&cmd)
        .execute(get_db_pool());
}

