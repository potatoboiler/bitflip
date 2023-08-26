use async_trait::async_trait;
use serenity::{
    framework::{
        standard::{
            macros::{command, group},
            CommandResult,
        },
        StandardFramework,
    },
    model::prelude::Message,
    prelude::{Context, EventHandler, GatewayIntents},
    Client,
};
use tokio::sync::OnceCell;

#[group]
#[commands(
    ping,
    create_party,
    join_party,
    start_party,
    run_tests,
    submit,
    forfeit
)]
struct CodeParty;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

static GAMESTATE: OnceCell<game_state::GameState> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    GAMESTATE
        .set(game_state::GameState::new())
        .expect("Failed to initialize GAMESTATE");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&CODEPARTY_GROUP);

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    if let Err(error) = client.start().await {
        println!("Client error: {:?}", error);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
async fn create_party(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
async fn join_party(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
async fn start_party(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
async fn run_tests(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
async fn submit(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
async fn forfeit(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

mod game_state {
    use std::{borrow::Cow, sync::Arc};

    use dashmap::DashMap;
    use uuid_b64::UuidB64;

    #[derive(Clone, Debug)]
    pub(crate) struct GameState {
        room_map: Arc<DashMap<UuidB64, Room>>, // Maps room id to room
        player_map: Arc<DashMap<Cow<'static, str>, UuidB64>>, // Checks if user is already in a game
    }
    impl GameState {
        pub(crate) fn new() -> Self {
            Self {
                room_map: Arc::new(DashMap::new()),
                player_map: Arc::new(DashMap::new()),
            }
        }

        async fn create_party(&self, creator: Player, n_problems: usize) {
            let room_id = UuidB64::new();
            self.room_map
                .insert(room_id, Room::new(creator, n_problems));
        }
    }

    #[derive(Debug)]
    struct Room {
        players: Vec<Player>,
        n_problems: usize,
        problems_played: usize,
    }
    impl Room {
        fn new(creator: Player, n_problems: usize) -> Self {
            Self {
                players: Vec::new(),
                n_problems,
                problems_played: 0,
            }
        }
    }

    #[derive(Debug)]
    struct Player {}
}
