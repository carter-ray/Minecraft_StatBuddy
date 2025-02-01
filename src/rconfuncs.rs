use rcon::{AsyncStdStream, Connection};

pub async fn query_rcon_server(addr_port: String, rcon_pw: String, cmds: Vec<String>) -> Vec<String> {
    let mut responses: Vec<String> = Vec::with_capacity(cmds.len());
    
    let connection = <Connection<AsyncStdStream>>::builder()
        .enable_minecraft_quirks(true)
        .connect(&addr_port, rcon_pw.as_str())
        .await;
    
    match connection {
        Ok(mut conn) => {
            println!("Connected to RCON server");     
            for cmd in cmds.iter() {
                let resp: Result<String, rcon::Error> = Connection::cmd(&mut conn, &cmd).await;
                match resp {
                    Ok(returned_data) => {
                        println!("{}", &returned_data);
                        responses.push(returned_data);
                    },
                    Err(e) => {
                        println!("{}", e);
                        responses.push("".to_string());
                    },
                }
            }
            responses
        },
        Err(_) => panic!("Could not connect to RCON server {}", &addr_port)
    }
}