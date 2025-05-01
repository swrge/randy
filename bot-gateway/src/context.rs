use crate::cache::RedisConfig;
use crate::SHUTDOWN;
use bb8_redis::redis::AsyncCommands;
use randy_gateway::error::{ReceiveMessageError, ReceiveMessageErrorType};
use randy_gateway::{CloseFrame, Event, MessageSender, Session, Shard, ShardId, StreamExt};
use randy_model::gateway::payload::incoming::{
    GuildCreate, Hello, MemberAdd, MemberChunk, MemberUpdate, MessageCreate, MessageDelete,
    MessageUpdate, PresenceUpdate, ReactionAdd, ReactionRemove, Ready,
};
use randy_model::gateway::payload::outgoing::RequestGuildMembers;
use randy_model::guild::{Guild, UnavailableGuild};
use randy_rest::Client;
use redlight::cache::RedisCache;
use redlight::config::CacheConfig;
use std::collections::HashMap;
use std::hash::RandomState;
use std::pin::Pin;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub type ShardInfo = (Option<Session>, Option<String>);
type GatewayEvent = Result<Event, ReceiveMessageError>;

#[derive(Clone)]
pub struct SharedContext {
    pub sender: Option<MessageSender>,
    pub client: Arc<Client>,
    pub cache: Arc<RedisCache<RedisConfig>>,
}

pub struct Context {
    pub shard: Pin<Box<Shard>>,
    pub shared: SharedContext,
}

impl Context {
    pub fn new_boxed(
        shard: Box<Shard>,
        client: Arc<Client>,
        cache: Arc<RedisCache<RedisConfig>>,
    ) -> Self {
        Self {
            shard: Pin::from(shard),
            shared: SharedContext {
                sender: None,
                client,
                cache,
            },
        }
    }

    /// Dumps session and resume info
    pub fn dump_info(&self) -> (Option<Session>, Option<String>) {
        println!("Dumping session and resume info");
        let result = (
            self.shard.session().cloned(),
            self.shard.resume_url().map(String::from),
        );
        println!("Dumping done");
        result
    }

    pub fn cache(&self) -> &RedisCache<RedisConfig> {
        &self.shared.cache
    }

    pub async fn freeze(&self, info: (Option<Session>, Option<String>)) -> anyhow::Result<()> {
        let mut conn = self.shared.cache.connection().await?;
        if let Some(url) = info.1 {
            println!("Freezing resume URL: {}", url);
            conn.set::<_, _, ()>("resume_url", url).await?;
        }
        if let Some(session) = info.0 {
            println!("Freezing session ID: {}", session.id());
            let mut sessions = HashMap::new();
            sessions.insert(self.shard.id().number(), session);
            // Assuming freeze takes ownership of sessions
            self.cache().freeze(&sessions, None).await?;
        }
        Ok(())
    }

    pub async fn thaw<C: CacheConfig>(
        cache: &RedisCache<C>,
    ) -> anyhow::Result<(Option<Session>, Option<String>)> {
        let sessions = match cache.defrost(*super::DEBUG).await {
            Ok(Some(sessions)) => sessions,
            _ => {
                println!("No sessions found for thawing");
                return Ok((None, None));
            }
        };
        println!("Found sessions for thawing: {:?}", &sessions);
        let session = sessions.get(&ShardId::ONE.number()).cloned();
        if let Some(ref session) = session {
            println!("Selected session: {}", session.id());
        }

        let mut conn = cache.pool().get().await?;
        let url: Option<String> = conn.get("resume_url").await?;
        if let Some(ref url) = url {
            println!("Found resume URL: {}", url);
        }

        Ok((session, url))
    }

    /// Get a reference to the shared part of the context
    pub fn shared(&self) -> &SharedContext {
        &self.shared
    }

    /// Create a shared context for passing to handlers
    pub fn clone_shared(&self) -> SharedContext {
        let mut result = self.shared.clone();
        result.sender = Some(self.shard.sender());
        result
    }

    /// Handles errors raised in the shard runner.
    /// Returns true if the user explicitly requested a shutdown.
    async fn on_error(&self, error: ReceiveMessageError) -> bool {
        println!(
            "GATEWAY: Shard `{}` raised the following error:",
            self.shard.id()
        );

        let requested_shutdown = SHUTDOWN.load(Ordering::Relaxed)
            && matches!(error.kind(), ReceiveMessageErrorType::Reconnect);

        match requested_shutdown {
            true => println!("GATEWAY: Explicit shutdown requested, raised error: {error:#?}"),
            false => println!("{error:#?}"),
        }

        requested_shutdown
    }

    async fn on_ready(&self, r: Box<Ready>) {
        println!(
            "Ready as {} ({}) in {} guilds !",
            r.user.name,
            r.user.id,
            r.guilds.len()
        );
    }

    async fn on_close(&self, event: Option<CloseFrame<'_>>) {
        println!("GATEWAY: Gateway closed for shard `{}`.", self.shard.id());
        if let Some(data) = event {
            println!("GATEWAY: CODE `{}` - Reason: {}", data.code, data.reason);
        }
    }

    async fn on_hello(&self, data: Hello) {
        todo!()
    }

    async fn on_heartbeat(&self, data: u64) {
        todo!()
    }

    async fn on_heartbeat_ack(&self) {
        todo!()
    }

    async fn on_reaction_remove(&self, data: Box<ReactionRemove>) {
        todo!()
    }

    async fn on_reaction_add(&self, data: Box<ReactionAdd>) {
        todo!()
    }

    async fn on_message_update(&self, data: Box<MessageUpdate>) {
        todo!()
    }

    async fn on_message_delete(&self, data: MessageDelete) {
        todo!()
    }

    async fn on_resumed(&self) {
        println!("Resumed shard!");
        // Show the last time the shard was active and the time it resumed
        //println!("Last active time: {}", chrono::Utc::now());
        println!(
            "Current time: {}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
    }
    #[rustfmt::skip]
    async fn on_guild_create(&self, data: Box<GuildCreate>) {
        match data.as_ref() {
            GuildCreate::Unavailable(guild) => {
                if guild.unavailable {
                    println!("Guild {} is unavailable", guild.id);
                } else {
                    println!("Guild {} is available and will soon receive it's data", guild.id);
                }
            }
            GuildCreate::Available(guild) => {
                println!("Guild {} ({}) is available", guild.name, guild.id);
                let request = RequestGuildMembers::builder(data.id())
                    .presences(true)
                    .query("", None);
                self.shard.command(&request);
            }
        }


        // Assuming command doesn't return a future now
    }

    async fn on_member_add(&self, _data: Box<MemberAdd>) {
        println!("Member added!");
    }

    async fn on_member_update(&self, _data: Box<MemberUpdate>) {
        println!("Member updated!");
    }

    async fn on_member_chunk(&self, _data: MemberChunk) {
        println!("Member chunk received!");
    }

    async fn on_message_create(&self, data: Box<MessageCreate>) {
        if let Some(guild_id) = data.guild_id {
            println!(
                "Message created from user {} ({}) in guild {} in channel {}",
                data.author.name, data.author.id, guild_id, data.channel_id
            );
            //process_message(&self.shard, data, Arc::clone(&self.shared.client)).await;
        } else {
            println!(
                "DM message created from user {} ({}) in channel {}",
                data.author.name, data.author.id, data.channel_id
            );
        }
    }

    async fn on_presence_update(&self, _data: Box<PresenceUpdate>) {
        println!("Presence updated!");
    }
    async fn on_invalid_session(&self, can_reconnect: bool) {
        println!("Invalid session received!");
        if can_reconnect {
            println!("Can reconnect!");
        } else {
            println!("Cannot reconnect!");
        }
    }

    async fn on_reconnect(&self) {
        println!("Gateway requested shard to reconnect");
    }

    async fn on_dispatch(&self, event: Event) {
        match event {
            Event::GatewayClose(event) => self.on_close(event).await,
            Event::Ready(data) => self.on_ready(data).await,
            Event::Resumed => self.on_resumed().await,
            Event::GuildCreate(data) => self.on_guild_create(data).await,
            Event::MemberAdd(data) => self.on_member_add(data).await,
            Event::MemberUpdate(data) => self.on_member_update(data).await,
            Event::MemberChunk(data) => self.on_member_chunk(data).await,
            Event::MessageCreate(data) => self.on_message_create(data).await,
            Event::PresenceUpdate(data) => self.on_presence_update(data).await,
            Event::GatewayHello(data) => self.on_hello(data).await,
            Event::GatewayHeartbeat(data) => self.on_heartbeat(data).await,
            Event::GatewayHeartbeatAck => self.on_heartbeat_ack().await,
            Event::GatewayReconnect => self.on_reconnect().await,
            Event::MessageDelete(data) => self.on_message_delete(data).await,
            Event::MessageUpdate(data) => self.on_message_update(data).await,
            Event::ReactionAdd(data) => self.on_reaction_add(data).await,
            Event::ReactionRemove(data) => self.on_reaction_remove(data).await,
            Event::GatewayInvalidateSession(can_reconnect) => {
                self.on_invalid_session(can_reconnect).await
            }
            _ => {
                println!("Unhandled event: {:?}", event.kind());
            }
        }
    }

    pub async fn run(mut self) -> Option<(Option<Session>, Option<String>)> {
        while let Some(item) = self.shard.next_event(*super::FLAGS).await {
            match item {
                Ok(event) => self.on_dispatch(event).await,
                Err(error) => {
                    if self.on_error(error).await {
                        println!("GATEWAY: Exiting event loop");
                        SHUTDOWN.store(true, Ordering::Relaxed);
                    }
                }
            }
        }
        Some(self.dump_info())
    }
}
