use {
    anyhow::Result,
    async_trait::async_trait,
    resvolt::{models::events::ReadyEvent, Client, Context, RevoltCommand, RevoltEventHandler},
    std::{env, time::Duration},
};

struct PingCommand;

#[async_trait]
impl RevoltCommand for PingCommand {
    async fn execute(&self, ctx: &Context) -> Result<()> {
        println!("{:?}", ctx.latency().await);
        let msg_content = format!("Pong!\nWS Latency: {:?}", ctx.latency().await);

        if let Ok(msg) = ctx.msg().reply(ctx, msg_content, true).await {
            msg.delete_after(ctx, Duration::from_secs(3)).await.ok();
        }

        Ok(())
    }
}

struct EventHandler;

#[async_trait]
impl RevoltEventHandler for EventHandler {
    async fn on_ready(&self, _ctx: &Context, _: ReadyEvent) -> Result<()> {
        println!("Bot is READY");

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("TOKEN").expect("No TOKEN environment variable found!");
    let mut client = Client::new(token, "!".to_string(), EventHandler, None)
        .await
        .unwrap();

    client
        .register_command("ping".to_string(), PingCommand)
        .await;
    client.listen().await.unwrap();
}
