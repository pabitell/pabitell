use actix::prelude::*;
use actix_web_actors::ws;
use rand::{self, rngs::ThreadRng, Rng};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use tracing::{debug, info};
use uuid::Uuid;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Message which is sent to connected clients
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Messages which are sent between Connection and Manager actors
/// New client is connected
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub world_id: Uuid,
    pub addr: Recipient<Message>,
}

/// Client is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub world_id: Uuid,
    pub id: usize,
}

/// A message from client was recieved
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub world_id: Uuid,
    pub data: String,
}

/// Should manage connected clients
#[derive(Debug, Default)]
pub struct WsManager {
    clients: HashMap<Uuid, HashMap<usize, Recipient<Message>>>,
    rng: ThreadRng,
}

impl WsManager {
    pub fn new() -> WsManager {
        Self::default()
    }
}

impl WsManager {
    /// Sends messages to all users in the same world
    fn send_message(&self, id: &Uuid, message: &str) {
        debug!("client={:?}", self.clients.get(id));
        self.clients.get(id).iter().for_each(|clients| {
            clients.values().for_each(|addr| {
                debug!("sending_message; to={:?}", addr);
                addr.do_send(Message(message.to_owned()));
            });
        });
    }
}

impl Actor for WsManager {
    type Context = Context<Self>;
}

/// Register a new client
impl Handler<Connect> for WsManager {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        debug!("Connect in Manager");
        // register session with random id
        let id = self.rng.gen::<usize>();
        self.clients
            .entry(msg.world_id)
            .or_insert_with(HashMap::new)
            .insert(id, msg.addr);

        debug!("clients={:?}", self.clients);

        // send id back
        id
    }
}

/// Handle a situation when client disconnects
impl Handler<Disconnect> for WsManager {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        info!("Disconnected;world={},mid={}", msg.world_id, msg.id);
        if let Some(clients) = self.clients.get_mut(&msg.world_id) {
            clients.remove(&msg.id);
            if clients.is_empty() {
                // Remove world if no clients are connected
                self.clients.remove(&msg.world_id);
            }
        }
    }
}

/// A message from client was recieved
/// send it to all connected clients
/// (including the one from which the message was recieved)
impl Handler<ClientMessage> for WsManager {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        debug!("Handling message");
        self.send_message(&msg.world_id, &msg.data);
    }
}

#[derive(Debug)]
pub struct WsConnection {
    /// unique id of the connection
    /// it should be obtained from WsManager on start()
    id: Option<usize>,
    /// Client must send ping
    hb: Instant,
    /// Id of a world
    world_id: Uuid,
    /// Manager callback (e.g. to report disconnects)
    addr: Addr<WsManager>,
}

impl Actor for WsConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        debug!("Started");
        // start heartbeat
        self.hb(ctx);

        // register self in Manager. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        let addr = ctx.address();
        self.addr
            .send(Connect {
                world_id: self.world_id,
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = Some(res),
                    // something is wrong with the server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify server
        debug!("Stopping");
        self.addr.do_send(Disconnect {
            id: self.id.unwrap_or_default(),
            world_id: self.world_id,
        });
        Running::Stop
    }
}

/// Handle messages from Manager, we simply send it to peer websocket
impl Handler<Message> for WsConnection {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        debug!("message={:?}", msg);
        match msg {
            ws::Message::Ping(msg) => {
                // ping recieved the connection is alive
                self.hb = Instant::now();
                // reply with pong
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                // pong recieved the connection is alive
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                // Forward message between clients
                self.addr.do_send(ClientMessage {
                    world_id: self.world_id,
                    data: text.to_string(),
                });
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Binary(_) => (),
            ws::Message::Nop => (),
        }
    }
}

impl WsConnection {
    /// ping to client every second and also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let world_id = self.world_id;
        ctx.run_interval(HEARTBEAT_INTERVAL, move |act, ctx| {
            debug!("heartbeat;world={},act={:?}", world_id.clone(), act.id);
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                debug!("Websocket Client heartbeat failed, disconnecting!");

                // notify chat server
                act.addr.do_send(Disconnect {
                    id: act.id.unwrap_or_default(),
                    world_id,
                });

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            // Send ping to connected client
            ctx.ping(b"");
        });
    }

    pub fn new(hb: Instant, world_id: Uuid, addr: Addr<WsManager>) -> Self {
        debug!("new connection;world={}", world_id);
        Self {
            id: None,
            hb,
            world_id,
            addr,
        }
    }
}
