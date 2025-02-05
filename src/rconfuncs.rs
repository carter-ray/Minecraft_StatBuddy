use std::collections::HashMap;

use rcon::{AsyncStdStream, Connection};

use crate::CONFIG;

pub async fn query_rcon_server(cmds: &mut HashMap<String, String>) {    
    let connection = <Connection<AsyncStdStream>>::builder()
        .enable_minecraft_quirks(true)
        .connect(&CONFIG.rcon_addr_port, &CONFIG.rcon_pw)
        .await;
    
    match connection {
        Ok(mut conn) => {
            println!("Connected to RCON server");
            for (_, cmd) in cmds.iter_mut() {
                let resp: Result<String, rcon::Error> = Connection::cmd(&mut conn, &cmd).await;
                match resp {
                    Ok(returned_data) => {
                        *cmd = returned_data;
                    },
                    Err(e) => {
                        println!("Error in rcon server connection: {}", e);
                        *cmd = "NULL".to_string();
                    },
                }
            }
        },
        Err(_) => panic!("Could not connect to RCON server {}", &CONFIG.rcon_addr_port)
    }
}

pub async fn get_whitelist() -> Vec<String> {
    let mut cmd: HashMap<String, String> = HashMap::new();
    cmd.insert("0".to_string(), "whitelist list".to_string());

    println!("Getting current whitelist");
    query_rcon_server(&mut cmd).await;

    let players: Vec<String> = cmd.get("0")
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>()[1]
        .split(",")
        .map(|item| item.trim().to_string())
        .collect();
    players
}