use std::env;

use sqlx::Row;

use crate::rconfuncs::query_rcon_server;


pub async fn update_db(database: &sqlx::SqlitePool) {
    let addr_port: String = env::var("LOCAL_ADDR")
        .expect("Expected an rcon address:port in the environment");
    
    let rcon_pw= env::var("RCON_PW")
        .expect("Expected an rcon password in the environment");

    let result = sqlx::query("SELECT username FROM statistics;")
        .fetch_all(database).await
        .unwrap();

    let mut cmds: Vec<String> = Vec::with_capacity(result.len());
    for row in result.iter() {
        let username: &str = row.try_get("username").unwrap();
        println!("username = {}", username);
        if username != "" {
            cmds.push(format!("scoreboard players list {username}"));
        }
    }

    println!("Commands: {:#?}", cmds);
    
    query_rcon_server(addr_port, rcon_pw, cmds).await;

}



