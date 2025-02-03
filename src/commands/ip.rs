
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use tokio::net::lookup_host;

use crate::CONFIG;

pub async fn run(_options: &[ResolvedOption<'_>]) -> Option<String> {
    let x: Option<String> = match lookup_host(&CONFIG.global_server_addr).await {
        Ok(mut ip) => {
            let ip_addr: String = ip.next().unwrap().to_string();
            let output = format!("Resolved {:#?} to {:#?}", &CONFIG.global_server_addr, ip_addr);
            Some(output)
        },
        Err(e) => {
            println!("Error in hostname resolution {:?}", e);
            Some(format!("Failed to resolve Hostname {:?}: to IP", &CONFIG.global_server_addr).to_string())
        }
    };
    x
}

pub async  fn register() -> CreateCommand {
    CreateCommand::new("ip")
        .description("Gets IP of server")
}