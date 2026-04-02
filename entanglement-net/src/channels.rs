/// Entanglement channel IDs.
/// Must match the channel registration order in Entanglement C++
/// (see channel_manager.h `namespace channels`).
pub mod channel {
    /// Control channel — RELIABLE_ORDERED, priority 255.
    /// SessionClose, ShardHandoff.  **Not for app-layer data.**
    pub const CONTROL:              u8 = 0;
    /// Unreliable — positions, StateAck, pings, PlayerMove.
    /// High frequency, loss tolerated, no ordering guarantee.
    pub const UNRELIABLE:           u8 = 1;
    /// Reliable — spawn, despawn, PlayerAction, SessionOpen, events.
    /// Guaranteed delivery, no ordering guarantee (no head-of-line blocking).
    pub const RELIABLE:             u8 = 2;
    /// Ordered — RELIABLE_ORDERED, priority 128.
    /// Ordered app-layer messages.
    pub const ORDERED:              u8 = 3;
    /// Unreliable coalesced — batched position updates.
    pub const UNRELIABLE_COALESCED: u8 = 4;
    /// Reliable coalesced.
    pub const RELIABLE_COALESCED:   u8 = 5;
    /// Ordered coalesced.
    pub const ORDERED_COALESCED:    u8 = 6;
}
