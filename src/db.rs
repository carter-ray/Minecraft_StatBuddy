
use std::env;

use rcon::{AsyncStdStream, Connection};
use sqlx::Row;


pub async fn update_db(database: &sqlx::SqlitePool) {
    let addr_port: String = env::var("LOCAL_ADDR")
        .expect("Expected an rcon address:port in the environment");
    
    let rcon_pw= env::var("RCON_PW")
        .expect("Expected an rcon password in the environment");


    let result = sqlx::query("SELECT username FROM statistics;")
        .fetch_all(database).await
        .unwrap();

    let mut usernames: Vec<&str> = Vec::with_capacity(result.len());
    for row in result.iter() {
        let username: &str = row.try_get("username").unwrap();
        usernames.push(username);
    }

    println!("Usernames: {:#?}", usernames);

    let connection = <Connection<AsyncStdStream>>::builder()
        .enable_minecraft_quirks(true)
        .connect(&addr_port, rcon_pw.as_str())
        .await;
    
    match connection {
        Ok(mut conn) => {
            println!("Connected to RCON server");    

            for user in usernames {
                let cmd: String= format!("scoreboard players list {user}");
                let _resp = Connection::cmd(&mut conn, &cmd).await;
            }
        },
        Err(_) => panic!("Could not connect to RCON server {}", &addr_port)
    }


    // let task_description = task_description.trim();
    // // That's how we are going to use a sqlite command.
    // // We are inserting into the todo table, our task_description in task column and our
    // // user_id in user_Id column.
    // sqlx::query!(
    //     "INSERT INTO todo (task, user_id) VALUES (?, ?)",
    //     task_description,
    //     user_id,
    // )
    // .execute(&self.database) // < Where the command will be executed
    // .await
    // .unwrap();

    // let response = format!("Successfully added `{task_description}` to your todo list");
    // msg.channel_id.say(&ctx, response).await.unwrap();
}
