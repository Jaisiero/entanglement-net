use crate::error::NetError;
use crate::messages::MsgHeader;

/// Context passed to message handlers
#[derive(Debug, Clone)]
pub struct MessageContext {
    /// Opaque sender identifier (e.g., Entanglement endpoint index)
    pub sender_id: u64,
    /// Server tick at time of dispatch
    pub server_tick: u32,
    /// Timestamp in microseconds
    pub timestamp_us: u64,
}

/// Handler for a specific known message type
pub trait MessageHandler: Send + Sync {
    fn handle(&self, payload: &[u8], context: &MessageContext) -> Result<(), NetError>;
}

/// Handler for game-defined messages (0x1000 - 0xFFFF)
pub trait GameMessageHandler: Send + Sync {
    fn handle_game_message(
        &self,
        msg_type: u16,
        payload: &[u8],
        context: &MessageContext,
    ) -> Result<(), NetError>;
}

/// Message dispatcher — routes incoming bytes to typed handlers
pub struct Dispatcher {
    handlers: std::collections::HashMap<u16, Box<dyn MessageHandler>>,
    game_handler: Option<Box<dyn GameMessageHandler>>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            handlers: std::collections::HashMap::new(),
            game_handler: None,
        }
    }

    /// Register a handler for a specific message type.
    pub fn register(&mut self, msg_type: u16, handler: Box<dyn MessageHandler>) {
        self.handlers.insert(msg_type, handler);
    }

    /// Register a handler for game-defined messages (0x1000+).
    pub fn set_game_handler(&mut self, handler: Box<dyn GameMessageHandler>) {
        self.game_handler = Some(handler);
    }

    /// Register a closure as a handler for a specific message type.
    pub fn on<F>(&mut self, msg_type: u16, f: F)
    where
        F: Fn(&[u8], &MessageContext) -> Result<(), NetError> + Send + Sync + 'static,
    {
        self.handlers.insert(msg_type, Box::new(ClosureHandler(f)));
    }

    /// Dispatch a single message to its handler.
    pub fn dispatch(
        &self,
        header: &MsgHeader,
        payload: &[u8],
        ctx: &MessageContext,
    ) -> Result<(), NetError> {
        let msg_type = header.msg_type;

        if msg_type >= 0x1000 {
            if let Some(h) = &self.game_handler {
                return h.handle_game_message(msg_type, payload, ctx);
            }
            return Err(NetError::UnknownMessageType(msg_type));
        }

        match self.handlers.get(&msg_type) {
            Some(h) => h.handle(payload, ctx),
            None => Err(NetError::UnknownMessageType(msg_type)),
        }
    }

    /// Dispatch all messages in a batch buffer.
    pub fn dispatch_batch(
        &self,
        buffer: &[u8],
        ctx: &MessageContext,
    ) -> Result<usize, NetError> {
        let mut reader = crate::batch::BatchReader::new(buffer);
        let mut count = 0;
        for item in &mut reader {
            let (header, payload) = item?;
            self.dispatch(&header, payload, ctx)?;
            count += 1;
        }
        Ok(count)
    }

    /// Returns the number of registered handlers.
    pub fn handler_count(&self) -> usize {
        self.handlers.len()
    }
}

impl Default for Dispatcher {
    fn default() -> Self {
        Self::new()
    }
}

struct ClosureHandler<F>(F);

impl<F> MessageHandler for ClosureHandler<F>
where
    F: Fn(&[u8], &MessageContext) -> Result<(), NetError> + Send + Sync + 'static,
{
    fn handle(&self, payload: &[u8], context: &MessageContext) -> Result<(), NetError> {
        (self.0)(payload, context)
    }
}
