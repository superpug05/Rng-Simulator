use crate::{Context, Error, utils::database::{retrieve_database, User, write_database}};

const SPLASHES: [&'static str; 5] = [
    "RNG simulator on top",
    "its goon time",
    "Touch Grass!",
    "Made with Rust 🦀",
    "Don't you have something better to do than play this?",
];

#[poise::command(prefix_command)]
pub async fn start(ctx: Context<'_>) -> Result<(), Error> {
    let database_guard = ctx.data().file_lock.lock().await;
    let mut database = retrieve_database(database_guard.as_str());

    if database.contains_key(ctx.author().id.as_u64()) {
        ctx.send(|cr| {
            cr.reply(true);
            cr.embed(|em| {
                em.title("You already have an account!");
                em.description("```ini\n[Additional Notes]\ngoon time```")
            })
        }).await?;
        return Ok(());
    }

    database.insert(*ctx.author().id.as_u64(), User::default());
    write_database(database_guard.as_str(), database);
    drop(database_guard);

    ctx.send(|cr| {
        cr.reply(true);
        cr.embed(|em| {
            em.title("");
            let rand_splash = SPLASHES[fastrand::usize(..5)];
            let description = format!("\
                ```ini\n\
                [[RNG SIMULATOR]]\n\
                ; {rand_splash}\n\
                ```\n\
                ```ini\n\
                [Welcome!]\n\
                Hi this is a super cool and radical \
                bot where you can spend many hours \
                gambling to try and get low numbers \
                because LETS GO RANDOM NUMBER GENERATOR!!!!!!!\n\
                ```\n\
                ```ini\n\
                [How to play]\n\
                You can start playing by using the \
                [{}roll] command to create your \
                first item!\n\
                ```\
            ", ctx.prefix());
            em.description(description)
        })
    }).await?;
    Ok(())
}