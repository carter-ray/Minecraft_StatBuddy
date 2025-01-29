use crate::{Context, Error};

#[poise::comand(prefix_command, slash_command)]
pub async fn get_ip(
    ctx: Context<'_>,
    #[description = "Shows IP address"],
) -> Result<(), Error> {

    let response = format!("insert IP here");
    ctx.say(response).await?;
    Ok(())
}