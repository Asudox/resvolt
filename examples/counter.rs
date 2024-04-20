use {
    anyhow::Result,
    async_trait::async_trait,
    resvolt::{
        models::{events::ReadyEvent, Presence, UserStatus},
        Client, Context, RevoltCommand, RevoltEventHandler,
    }, std::env,
};

#[derive(Clone, Default)]
struct Counter(u32);

impl Counter {
    fn increment(&mut self) {
        self.0 += 1;
    }
}

struct CountCommand;

#[async_trait]
impl RevoltCommand for CountCommand {
    async fn execute(&self, ctx: &Context) -> Result<()> {
        ctx.state.update(Counter::increment).await;

        let Counter(current_count) = ctx.state.get().await.unwrap();
        let content = format!("#### Counted!\nCurrent count is **{current_count}**");

        ctx.msg().reply(ctx, content, false).await.ok();

        Ok(())
    }
}

struct EventHandler;

#[async_trait]
impl RevoltEventHandler for EventHandler {
    async fn on_ready(&self, ctx: &Context, _: ReadyEvent) -> Result<()> {
        ctx.state.insert(Counter::default()).await;

        let status = UserStatus::new("Counting ...", Presence::Busy);

        ctx.edit(status).await.ok();

        println!("Bot is READY");

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("TOKEN").expect("No TOKEN environment variable found!");
    let mut client = Client::new(token, "!", EventHandler, None).await.unwrap();

    client.register_command("count", CountCommand).await;
    client.listen().await.unwrap();
}
