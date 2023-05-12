use crate::{Context, Error, utils::database::retrieve_database};

#[poise::command(prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong").await?;
    Ok(())
}

#[poise::command(prefix_command)]
pub async fn read_db(ctx: Context<'_>) -> Result<(), Error> {
    let db_guard = ctx.data().file_lock.lock().await;
    let db = retrieve_database(db_guard.as_str());
    ctx.say(format!("```rs\n{:#?}```", db)).await?;
    Ok(())
}