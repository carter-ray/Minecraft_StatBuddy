use std::env;

use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use tokio::net::lookup_host;

pub async fn run(_options: &[ResolvedOption<'_>]) -> Option<String> {
    let hostname: String = env::var("GLOBAL_SERVER_ADDR")
        .expect("Expected a server address in the environment");

    let x: Option<String> = match lookup_host(&hostname).await {
        Ok(mut ip) => {
            let ip_addr: String = ip.next().unwrap().to_string();
            println!("{:#?}", ip_addr);
            let output = format!("Resolved {:#?} to {:#?}", hostname, ip_addr);
            Some(output)
        },
        Err(e) => {
            println!("{:?}", e);
            Some(format!("Failed to resolve Hostname {:?}: to IP", hostname).to_string())
        }
    };
    x
}

pub async  fn register() -> CreateCommand {
    CreateCommand::new("ip")
        .description("Gets IP of server")
}