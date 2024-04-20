use {
    anyhow::Result,
    async_trait::async_trait,
    resvolt::{
        models::{events::ReadyEvent, Content, Message},
        state::State,
        Client, Context, RevoltEventHandler,
    },
    std::{env, sync::Arc},
};

struct EventHandler;

#[async_trait]
impl RevoltEventHandler for EventHandler {
    async fn on_ready(&self, _ctx: &Context, _: ReadyEvent) -> Result<()> {
        println!("Bot is READY");

        Ok(())
    }

    async fn on_message(&self, ctx: &Context, msg: Message) -> Result<()> {
        println!("Got new message");
        if let Content::Text(content) = &msg.content {
            if let Some(blacklist) = ctx.state.get::<Vec<String>>().await {
                for word in blacklist {
                    if content.contains(word.as_str()) {
                        println!("Deleting message");
                        msg.delete(ctx).await.unwrap();
                        println!("Deleted messsage");
                        msg.create_from_self(ctx, "Sorry! Your message contained blacklisted words and has been removed accordingly.").await.ok();
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let blacklisted_words: Vec<String> = vec![
        "fuck".into(),
        "kys".into(),
        "asshole".into(),
        "faggot".into(),
    ];
    let state = Arc::new(State::default());
    state.insert(blacklisted_words).await;
    let token = env::var("TOKEN").expect("No TOKEN environment variable found!");
    let mut client = Client::new(token, "!".to_string(), EventHandler, Some(state.clone()))
        .await
        .unwrap();

    client.listen().await.unwrap();
}
