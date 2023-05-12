use crate::{Context, Error};
use std::fs::read_to_string;

#[poise::command(prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong").await?;
    Ok(())
}

#[poise::command(prefix_command)]
pub async fn test(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Acquiring File Lock").await?;
    let file = ctx.data().file_lock.lock().await;
    let contents = read_to_string(file.as_str())?;
    let output = format!("Found {contents} @ path {}", file.as_str());
    ctx.say(output).await?;
    Ok(())
}