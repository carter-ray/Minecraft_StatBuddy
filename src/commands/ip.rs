
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use tokio::net::lookup_host;

use crate::CONFIG;

pub async fn run(_options: &[ResolvedOption<'_>]) -> Option<String> {
    let server_addr = &CONFIG.global_server_addr;
    if let Some("localhost") = server_addr.split(":").next() {
        let ip_api: &str = "https://api.ipify.org?format=text"; 
        match reqwest::get(ip_api).await {
            Ok(response) => {
                match response.text().await {
                    Ok(ip_addr) => Some(format!("Public IP Addrss: {}", ip_addr)),
                    Err(e) => {
                        eprintln!("Failed to parse response from IP API: {:?}", e);
                        Some("Could not get Public IP Address".to_string())
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to fetch public IP Address: {:?}", e);
                Some("Could not get Public IP Address".to_string())
            }
        }
    } else {
        match lookup_host(&CONFIG.global_server_addr).await {
            Ok(mut ip) => {
                let ip_addr: String = ip.next().unwrap().to_string();
                let output = format!("Resolved {:#?} to {:#?}", &CONFIG.global_server_addr, ip_addr);
                Some(output)
            },
            Err(e) => {
                eprintln!("Error in hostname resolution {:?}", e);
                Some("Could not get Public IP Address".to_string())
            }
        }
    }
}

pub async  fn register() -> CreateCommand {
    CreateCommand::new("ip")
        .description("Gets IP of server")
}