// AUTO-GENERATED — do not edit manually
// Source: schemas/messages.toml
//
// Wire format: all multi-byte fields are LITTLE-ENDIAN.
// Use WireMessage::to_wire() before sending and WireMessage::from_wire() after receiving
// to ensure correct byte order on all platforms (x86, ARM, big-endian, etc.).

pub const PROTOCOL_VERSION: u16 = 2;
pub const MSG_HEADER_SIZE: usize = 6;
pub const MAX_PAYLOAD_BYTES: usize = 1154;

/// Trait for converting between native and little-endian wire format.
/// On little-endian platforms (x86, ARM LE), these are compiled to no-ops.
pub trait WireMessage: Copy {
    /// Convert from native byte order to little-endian wire format.
    fn to_wire(self) -> Self;
    /// Convert from little-endian wire format to native byte order.
    fn from_wire(self) -> Self;
}

pub mod msg_type {
    pub const SESSION_OPEN: u16 = 0x0001;
    pub const SESSION_CLOSE: u16 = 0x0002;
    pub const PING: u16 = 0x0003;
    pub const PONG: u16 = 0x0004;
    pub const SHARD_HANDOFF: u16 = 0x0005;
    pub const HANDOFF_AUTH: u16 = 0x0006;
    pub const SESSION_AUTH: u16 = 0x0007;
    pub const SESSION_AUTH_FAILED: u16 = 0x0008;
    pub const ENTITY_SPAWN: u16 = 0x0100;
    pub const ENTITY_DESPAWN: u16 = 0x0101;
    pub const ENTITY_MOVE: u16 = 0x0102;
    pub const ENTITY_MOVE_BATCH: u16 = 0x0107;
    pub const ENTITY_MOVE_COMPACT: u16 = 0x0108;
    pub const ENTITY_MOVE_DELTA_BATCH: u16 = 0x0109;
    pub const ENTITY_STATE: u16 = 0x0103;
    pub const ENTITY_HEALTH: u16 = 0x0104;
    pub const HIT_CONFIRM: u16 = 0x0105;
    pub const ACTION_REJECTED: u16 = 0x0106;
    pub const PLAYER_MOVE: u16 = 0x0200;
    pub const PLAYER_MOVE_BATCH: u16 = 0x0201;
    pub const PLAYER_ACTION: u16 = 0x0202;
    pub const STATE_ACK: u16 = 0x0203;
    pub const INTERSHARD_HANDSHAKE: u16 = 0x0300;
    pub const INTERSHARD_HANDSHAKE_ACK: u16 = 0x0301;
    pub const INTERSHARD_HEARTBEAT: u16 = 0x0302;
    pub const INTERSHARD_ENTITY_ENTER: u16 = 0x0310;
    pub const INTERSHARD_ENTITY_UPDATE: u16 = 0x0311;
    pub const INTERSHARD_ENTITY_LEAVE: u16 = 0x0312;
    pub const INTERSHARD_ENTITY_UPDATE_DELTA: u16 = 0x0314;
    pub const INTERSHARD_ENTITY_STATE: u16 = 0x0313;
    pub const INTERSHARD_HANDOFF_REQ: u16 = 0x0320;
    pub const INTERSHARD_HANDOFF_ACK: u16 = 0x0321;
    pub const INTERSHARD_HANDOFF_COMPLETE: u16 = 0x0322;
    pub const INTERSHARD_ATTACK: u16 = 0x0330;
    pub const INTERSHARD_HIT_RESULT: u16 = 0x0331;
    pub const INTERSHARD_COMBAT_STATE: u16 = 0x0332;
    pub const INTERSHARD_FORWARD_ACTION: u16 = 0x0340;
    pub const INTERSHARD_OVERLAP_ENROL: u16 = 0x0350;
    pub const INTERSHARD_OVERLAP_SAMPLE: u16 = 0x0351;
    pub const INTERSHARD_AUTHORITY_TRANSFER: u16 = 0x0352;
    pub const INTERSHARD_AUTHORITY_ACK: u16 = 0x0353;
    pub const INTERSHARD_OVERLAP_LEAVE: u16 = 0x0354;
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MsgHeader {
    pub msg_type: u16,
    pub msg_length: u16,
    pub msg_flags: u8,
    pub reserved: u8,
}

impl WireMessage for MsgHeader {
    fn to_wire(self) -> Self {
        Self {
            msg_type: self.msg_type.to_le(),
            msg_length: self.msg_length.to_le(),
            msg_flags: self.msg_flags,
            reserved: self.reserved,
        }
    }
    fn from_wire(self) -> Self {
        Self {
            msg_type: u16::from_le(self.msg_type),
            msg_length: u16::from_le(self.msg_length),
            msg_flags: self.msg_flags,
            reserved: self.reserved,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SessionOpen {
    pub protocol_version: u16,
    pub player_id: u32,
    pub shard_id: u32,
    pub origin_x: f32,
    pub origin_z: f32,
    pub server_tick: u32,
    pub tick_rate_hz: u16,
    pub persistent_id: u32,
}

impl WireMessage for SessionOpen {
    fn to_wire(self) -> Self {
        Self {
            protocol_version: self.protocol_version.to_le(),
            player_id: self.player_id.to_le(),
            shard_id: self.shard_id.to_le(),
            origin_x: f32::from_bits(self.origin_x.to_bits().to_le()),
            origin_z: f32::from_bits(self.origin_z.to_bits().to_le()),
            server_tick: self.server_tick.to_le(),
            tick_rate_hz: self.tick_rate_hz.to_le(),
            persistent_id: self.persistent_id.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            protocol_version: u16::from_le(self.protocol_version),
            player_id: u32::from_le(self.player_id),
            shard_id: u32::from_le(self.shard_id),
            origin_x: f32::from_bits(u32::from_le(self.origin_x.to_bits())),
            origin_z: f32::from_bits(u32::from_le(self.origin_z.to_bits())),
            server_tick: u32::from_le(self.server_tick),
            tick_rate_hz: u16::from_le(self.tick_rate_hz),
            persistent_id: u32::from_le(self.persistent_id),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SessionClose {
    pub reason: u8,
}

impl WireMessage for SessionClose {
    fn to_wire(self) -> Self {
        Self {
            reason: self.reason,
        }
    }
    fn from_wire(self) -> Self {
        Self {
            reason: self.reason,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ping {
    pub client_frame: u32,
    pub client_time_us: u64,
}

impl WireMessage for Ping {
    fn to_wire(self) -> Self {
        Self {
            client_frame: self.client_frame.to_le(),
            client_time_us: self.client_time_us.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            client_frame: u32::from_le(self.client_frame),
            client_time_us: u64::from_le(self.client_time_us),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pong {
    pub client_frame: u32,
    pub client_time_us: u64,
    pub server_tick: u32,
    pub server_time_us: u64,
    pub tick_delta_us: u32,
}

impl WireMessage for Pong {
    fn to_wire(self) -> Self {
        Self {
            client_frame: self.client_frame.to_le(),
            client_time_us: self.client_time_us.to_le(),
            server_tick: self.server_tick.to_le(),
            server_time_us: self.server_time_us.to_le(),
            tick_delta_us: self.tick_delta_us.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            client_frame: u32::from_le(self.client_frame),
            client_time_us: u64::from_le(self.client_time_us),
            server_tick: u32::from_le(self.server_tick),
            server_time_us: u64::from_le(self.server_time_us),
            tick_delta_us: u32::from_le(self.tick_delta_us),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShardHandoff {
    pub new_shard_ip_packed: u32,
    pub new_shard_port: u16,
    pub new_shard_id: u32,
    pub new_origin_x: f32,
    pub new_origin_z: f32,
    pub handoff_tick: u32,
    pub handoff_token: u64,
}

impl WireMessage for ShardHandoff {
    fn to_wire(self) -> Self {
        Self {
            new_shard_ip_packed: self.new_shard_ip_packed.to_le(),
            new_shard_port: self.new_shard_port.to_le(),
            new_shard_id: self.new_shard_id.to_le(),
            new_origin_x: f32::from_bits(self.new_origin_x.to_bits().to_le()),
            new_origin_z: f32::from_bits(self.new_origin_z.to_bits().to_le()),
            handoff_tick: self.handoff_tick.to_le(),
            handoff_token: self.handoff_token.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            new_shard_ip_packed: u32::from_le(self.new_shard_ip_packed),
            new_shard_port: u16::from_le(self.new_shard_port),
            new_shard_id: u32::from_le(self.new_shard_id),
            new_origin_x: f32::from_bits(u32::from_le(self.new_origin_x.to_bits())),
            new_origin_z: f32::from_bits(u32::from_le(self.new_origin_z.to_bits())),
            handoff_tick: u32::from_le(self.handoff_tick),
            handoff_token: u64::from_le(self.handoff_token),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HandoffAuth {
    pub entity_id: u32,
    pub handoff_token: u64,
    pub client_current_sequence: u32,
    pub client_action_sequence: u32,
}

impl WireMessage for HandoffAuth {
    fn to_wire(self) -> Self {
        Self {
            entity_id: self.entity_id.to_le(),
            handoff_token: self.handoff_token.to_le(),
            client_current_sequence: self.client_current_sequence.to_le(),
            client_action_sequence: self.client_action_sequence.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            entity_id: u32::from_le(self.entity_id),
            handoff_token: u64::from_le(self.handoff_token),
            client_current_sequence: u32::from_le(self.client_current_sequence),
            client_action_sequence: u32::from_le(self.client_action_sequence),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SessionAuth {
    pub jwt_length: u16,
}

impl WireMessage for SessionAuth {
    fn to_wire(self) -> Self {
        Self {
            jwt_length: self.jwt_length.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            jwt_length: u16::from_le(self.jwt_length),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SessionAuthFailed {
    pub reason: u8,
    pub pad_a: u8,
    pub pad_b: u8,
    pub pad_c: u8,
}

impl WireMessage for SessionAuthFailed {
    fn to_wire(self) -> Self {
        Self {
            reason: self.reason,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
        }
    }
    fn from_wire(self) -> Self {
        Self {
            reason: self.reason,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntitySpawn {
    pub entity_id: u32,
    pub entity_type: u16,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub orientation: f32,
    pub initial_state: u32,
}

impl WireMessage for EntitySpawn {
    fn to_wire(self) -> Self {
        Self {
            entity_id: self.entity_id.to_le(),
            entity_type: self.entity_type.to_le(),
            x: f32::from_bits(self.x.to_bits().to_le()),
            y: f32::from_bits(self.y.to_bits().to_le()),
            z: f32::from_bits(self.z.to_bits().to_le()),
            orientation: f32::from_bits(self.orientation.to_bits().to_le()),
            initial_state: self.initial_state.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            entity_id: u32::from_le(self.entity_id),
            entity_type: u16::from_le(self.entity_type),
            x: f32::from_bits(u32::from_le(self.x.to_bits())),
            y: f32::from_bits(u32::from_le(self.y.to_bits())),
            z: f32::from_bits(u32::from_le(self.z.to_bits())),
            orientation: f32::from_bits(u32::from_le(self.orientation.to_bits())),
            initial_state: u32::from_le(self.initial_state),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntityDespawn {
    pub entity_id: u32,
    pub reason: u8,
}

impl WireMessage for EntityDespawn {
    fn to_wire(self) -> Self {
        Self {
            entity_id: self.entity_id.to_le(),
            reason: self.reason,
        }
    }
    fn from_wire(self) -> Self {
        Self {
            entity_id: u32::from_le(self.entity_id),
            reason: self.reason,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntityMove {
    pub entity_id: u32,
    pub server_tick: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub orientation: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    /// FNV-1a hash of the shard_id string (`peer::shard_id_hash`) of
    /// the shard that currently holds broadcast authority over this
    /// entity. The sending shard fills this in for every entity it
    /// broadcasts:
    ///   - Local entities (this shard's authority): own shard_id_hash.
    ///   - Forwarded ghosts (received via INTERSHARD_GHOST_ZONE from
    ///     a neighbour): the source shard's hash, propagated through
    ///     the ghost record.
    /// Receivers (notably the viewer) use this to colour-code each
    /// entity by its true authority owner instead of inferring it
    /// from `(x, z)` ↔ region geometry — which lies whenever a
    /// handoff is mid-flight or has failed silently.
    /// 0 means "unknown" (older sender that hasn't been upgraded yet,
    /// or a path that lost the source shard ID en route); receivers
    /// fall back to their previous behaviour for that case.
    pub source_shard_hash: u32,
}

impl WireMessage for EntityMove {
    fn to_wire(self) -> Self {
        Self {
            entity_id: self.entity_id.to_le(),
            server_tick: self.server_tick.to_le(),
            x: f32::from_bits(self.x.to_bits().to_le()),
            y: f32::from_bits(self.y.to_bits().to_le()),
            z: f32::from_bits(self.z.to_bits().to_le()),
            orientation: f32::from_bits(self.orientation.to_bits().to_le()),
            vx: f32::from_bits(self.vx.to_bits().to_le()),
            vy: f32::from_bits(self.vy.to_bits().to_le()),
            vz: f32::from_bits(self.vz.to_bits().to_le()),
            source_shard_hash: self.source_shard_hash.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            entity_id: u32::from_le(self.entity_id),
            server_tick: u32::from_le(self.server_tick),
            x: f32::from_bits(u32::from_le(self.x.to_bits())),
            y: f32::from_bits(u32::from_le(self.y.to_bits())),
            z: f32::from_bits(u32::from_le(self.z.to_bits())),
            orientation: f32::from_bits(u32::from_le(self.orientation.to_bits())),
            vx: f32::from_bits(u32::from_le(self.vx.to_bits())),
            vy: f32::from_bits(u32::from_le(self.vy.to_bits())),
            vz: f32::from_bits(u32::from_le(self.vz.to_bits())),
            source_shard_hash: u32::from_le(self.source_shard_hash),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntityMoveBatch {
    pub server_tick: u32,
}

impl WireMessage for EntityMoveBatch {
    fn to_wire(self) -> Self {
        Self {
            server_tick: self.server_tick.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            server_tick: u32::from_le(self.server_tick),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntityMoveCompact {
    pub entity_id: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub orientation: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
}

impl WireMessage for EntityMoveCompact {
    fn to_wire(self) -> Self {
        Self {
            entity_id: self.entity_id.to_le(),
            x: f32::from_bits(self.x.to_bits().to_le()),
            y: f32::from_bits(self.y.to_bits().to_le()),
            z: f32::from_bits(self.z.to_bits().to_le()),
            orientation: f32::from_bits(self.orientation.to_bits().to_le()),
            vx: f32::from_bits(self.vx.to_bits().to_le()),
            vy: f32::from_bits(self.vy.to_bits().to_le()),
            vz: f32::from_bits(self.vz.to_bits().to_le()),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            entity_id: u32::from_le(self.entity_id),
            x: f32::from_bits(u32::from_le(self.x.to_bits())),
            y: f32::from_bits(u32::from_le(self.y.to_bits())),
            z: f32::from_bits(u32::from_le(self.z.to_bits())),
            orientation: f32::from_bits(u32::from_le(self.orientation.to_bits())),
            vx: f32::from_bits(u32::from_le(self.vx.to_bits())),
            vy: f32::from_bits(u32::from_le(self.vy.to_bits())),
            vz: f32::from_bits(u32::from_le(self.vz.to_bits())),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntityState {
    pub entity_id: u32,
    pub server_tick: u32,
    pub state_id: u16,
    pub param_a: u32,
    pub param_b: u32,
}

impl WireMessage for EntityState {
    fn to_wire(self) -> Self {
        Self {
            entity_id: self.entity_id.to_le(),
            server_tick: self.server_tick.to_le(),
            state_id: self.state_id.to_le(),
            param_a: self.param_a.to_le(),
            param_b: self.param_b.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            entity_id: u32::from_le(self.entity_id),
            server_tick: u32::from_le(self.server_tick),
            state_id: u16::from_le(self.state_id),
            param_a: u32::from_le(self.param_a),
            param_b: u32::from_le(self.param_b),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntityHealth {
    pub entity_id: u32,
    pub hp: u32,
    pub max_hp: u32,
}

impl WireMessage for EntityHealth {
    fn to_wire(self) -> Self {
        Self {
            entity_id: self.entity_id.to_le(),
            hp: self.hp.to_le(),
            max_hp: self.max_hp.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            entity_id: u32::from_le(self.entity_id),
            hp: u32::from_le(self.hp),
            max_hp: u32::from_le(self.max_hp),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitConfirm {
    pub input_sequence: u32,
    pub target_id: u32,
    pub damage_dealt: u32,
    pub target_hp: u32,
    pub server_tick: u32,
}

impl WireMessage for HitConfirm {
    fn to_wire(self) -> Self {
        Self {
            input_sequence: self.input_sequence.to_le(),
            target_id: self.target_id.to_le(),
            damage_dealt: self.damage_dealt.to_le(),
            target_hp: self.target_hp.to_le(),
            server_tick: self.server_tick.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            input_sequence: u32::from_le(self.input_sequence),
            target_id: u32::from_le(self.target_id),
            damage_dealt: u32::from_le(self.damage_dealt),
            target_hp: u32::from_le(self.target_hp),
            server_tick: u32::from_le(self.server_tick),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ActionRejected {
    pub input_sequence: u32,
    pub reason: u8,
    pub pad_a: u8,
    pub pad_b: u8,
    pub pad_c: u8,
}

impl WireMessage for ActionRejected {
    fn to_wire(self) -> Self {
        Self {
            input_sequence: self.input_sequence.to_le(),
            reason: self.reason,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
        }
    }
    fn from_wire(self) -> Self {
        Self {
            input_sequence: u32::from_le(self.input_sequence),
            reason: self.reason,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerMove {
    pub input_sequence: u32,
    pub estimated_server_tick: u32,
    pub move_x: f32,
    pub move_z: f32,
    pub orientation: f32,
    pub buttons: u32,
}

impl WireMessage for PlayerMove {
    fn to_wire(self) -> Self {
        Self {
            input_sequence: self.input_sequence.to_le(),
            estimated_server_tick: self.estimated_server_tick.to_le(),
            move_x: f32::from_bits(self.move_x.to_bits().to_le()),
            move_z: f32::from_bits(self.move_z.to_bits().to_le()),
            orientation: f32::from_bits(self.orientation.to_bits().to_le()),
            buttons: self.buttons.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            input_sequence: u32::from_le(self.input_sequence),
            estimated_server_tick: u32::from_le(self.estimated_server_tick),
            move_x: f32::from_bits(u32::from_le(self.move_x.to_bits())),
            move_z: f32::from_bits(u32::from_le(self.move_z.to_bits())),
            orientation: f32::from_bits(u32::from_le(self.orientation.to_bits())),
            buttons: u32::from_le(self.buttons),
        }
    }
}

/// Variable-length batch: count (u8) + count × PlayerMove
/// Max entries: 8
pub const PLAYER_MOVE_BATCH_MAX_ENTRIES: usize = 8;

/// Write a PlayerMoveBatch into a buffer (little-endian). Returns bytes written.
pub fn write_player_move_batch(buf: &mut [u8], inputs: &[PlayerMove]) -> Result<usize, ()> {
    let count = inputs.len().min(PLAYER_MOVE_BATCH_MAX_ENTRIES);
    let entry_size = core::mem::size_of::<PlayerMove>();
    let total = 1 + count * entry_size;
    if total > buf.len() { return Err(()); }
    buf[0] = count as u8;
    for i in 0..count {
        unsafe {
            core::ptr::write_unaligned(
                buf[1 + i * entry_size..].as_mut_ptr() as *mut PlayerMove,
                inputs[i].to_wire(),
            );
        }
    }
    Ok(total)
}

/// Read a PlayerMoveBatch from a buffer (raw LE bytes). Use WireMessage::from_wire() on each entry.
pub fn read_player_move_batch(payload: &[u8]) -> Option<&[u8]> {
    if payload.is_empty() { return None; }
    let count = payload[0] as usize;
    let entry_size = core::mem::size_of::<PlayerMove>();
    let total = 1 + count * entry_size;
    if total > payload.len() { return None; }
    Some(&payload[1..total])
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerAction {
    pub input_sequence: u32,
    pub server_tick: u32,
    pub action_type: u8,
    pub pad_a: u8,
    pub pad_b: u8,
    pub pad_c: u8,
    pub param_a: u32,
    pub param_b: u32,
}

impl WireMessage for PlayerAction {
    fn to_wire(self) -> Self {
        Self {
            input_sequence: self.input_sequence.to_le(),
            server_tick: self.server_tick.to_le(),
            action_type: self.action_type,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
            param_a: self.param_a.to_le(),
            param_b: self.param_b.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            input_sequence: u32::from_le(self.input_sequence),
            server_tick: u32::from_le(self.server_tick),
            action_type: self.action_type,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
            param_a: u32::from_le(self.param_a),
            param_b: u32::from_le(self.param_b),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StateAck {
    pub input_sequence_acked: u32,
    pub server_tick: u32,
    pub tick_delta_us: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub hp: u32,
    pub stamina: f32,
}

impl WireMessage for StateAck {
    fn to_wire(self) -> Self {
        Self {
            input_sequence_acked: self.input_sequence_acked.to_le(),
            server_tick: self.server_tick.to_le(),
            tick_delta_us: self.tick_delta_us.to_le(),
            x: f32::from_bits(self.x.to_bits().to_le()),
            y: f32::from_bits(self.y.to_bits().to_le()),
            z: f32::from_bits(self.z.to_bits().to_le()),
            vx: f32::from_bits(self.vx.to_bits().to_le()),
            vy: f32::from_bits(self.vy.to_bits().to_le()),
            vz: f32::from_bits(self.vz.to_bits().to_le()),
            hp: self.hp.to_le(),
            stamina: f32::from_bits(self.stamina.to_bits().to_le()),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            input_sequence_acked: u32::from_le(self.input_sequence_acked),
            server_tick: u32::from_le(self.server_tick),
            tick_delta_us: u32::from_le(self.tick_delta_us),
            x: f32::from_bits(u32::from_le(self.x.to_bits())),
            y: f32::from_bits(u32::from_le(self.y.to_bits())),
            z: f32::from_bits(u32::from_le(self.z.to_bits())),
            vx: f32::from_bits(u32::from_le(self.vx.to_bits())),
            vy: f32::from_bits(u32::from_le(self.vy.to_bits())),
            vz: f32::from_bits(u32::from_le(self.vz.to_bits())),
            hp: u32::from_le(self.hp),
            stamina: f32::from_bits(u32::from_le(self.stamina.to_bits())),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntershardHandshake {
    pub shard_id: u32,
    pub sequence: u32,
    pub hmac_0: u64,
    pub hmac_1: u64,
}

impl WireMessage for IntershardHandshake {
    fn to_wire(self) -> Self {
        Self {
            shard_id: self.shard_id.to_le(),
            sequence: self.sequence.to_le(),
            hmac_0: self.hmac_0.to_le(),
            hmac_1: self.hmac_1.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            shard_id: u32::from_le(self.shard_id),
            sequence: u32::from_le(self.sequence),
            hmac_0: u64::from_le(self.hmac_0),
            hmac_1: u64::from_le(self.hmac_1),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntershardHandshakeAck {
    pub shard_id: u32,
    pub sequence: u32,
    pub ok: u8,
    pub pad_a: u8,
    pub pad_b: u8,
    pub pad_c: u8,
}

impl WireMessage for IntershardHandshakeAck {
    fn to_wire(self) -> Self {
        Self {
            shard_id: self.shard_id.to_le(),
            sequence: self.sequence.to_le(),
            ok: self.ok,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
        }
    }
    fn from_wire(self) -> Self {
        Self {
            shard_id: u32::from_le(self.shard_id),
            sequence: u32::from_le(self.sequence),
            ok: self.ok,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntershardHeartbeat {
    pub shard_id: u32,
    pub server_tick: u32,
    pub player_count: u32,
    pub ghost_count: u32,
}

impl WireMessage for IntershardHeartbeat {
    fn to_wire(self) -> Self {
        Self {
            shard_id: self.shard_id.to_le(),
            server_tick: self.server_tick.to_le(),
            player_count: self.player_count.to_le(),
            ghost_count: self.ghost_count.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            shard_id: u32::from_le(self.shard_id),
            server_tick: u32::from_le(self.server_tick),
            player_count: u32::from_le(self.player_count),
            ghost_count: u32::from_le(self.ghost_count),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntershardEntityEnter {
    pub entity_id: u32,
    pub entity_type: u16,
    pub pad_a: u16,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub orientation: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub hp: u32,
    pub max_hp: u32,
    pub combat_state: u8,
    pub pvp_flag: u8,
    pub pad_b: u8,
    pub pad_c: u8,
}

impl WireMessage for IntershardEntityEnter {
    fn to_wire(self) -> Self {
        Self {
            entity_id: self.entity_id.to_le(),
            entity_type: self.entity_type.to_le(),
            pad_a: self.pad_a.to_le(),
            x: f32::from_bits(self.x.to_bits().to_le()),
            y: f32::from_bits(self.y.to_bits().to_le()),
            z: f32::from_bits(self.z.to_bits().to_le()),
            orientation: f32::from_bits(self.orientation.to_bits().to_le()),
            vx: f32::from_bits(self.vx.to_bits().to_le()),
            vy: f32::from_bits(self.vy.to_bits().to_le()),
            vz: f32::from_bits(self.vz.to_bits().to_le()),
            hp: self.hp.to_le(),
            max_hp: self.max_hp.to_le(),
            combat_state: self.combat_state,
            pvp_flag: self.pvp_flag,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
        }
    }
    fn from_wire(self) -> Self {
        Self {
            entity_id: u32::from_le(self.entity_id),
            entity_type: u16::from_le(self.entity_type),
            pad_a: u16::from_le(self.pad_a),
            x: f32::from_bits(u32::from_le(self.x.to_bits())),
            y: f32::from_bits(u32::from_le(self.y.to_bits())),
            z: f32::from_bits(u32::from_le(self.z.to_bits())),
            orientation: f32::from_bits(u32::from_le(self.orientation.to_bits())),
            vx: f32::from_bits(u32::from_le(self.vx.to_bits())),
            vy: f32::from_bits(u32::from_le(self.vy.to_bits())),
            vz: f32::from_bits(u32::from_le(self.vz.to_bits())),
            hp: u32::from_le(self.hp),
            max_hp: u32::from_le(self.max_hp),
            combat_state: self.combat_state,
            pvp_flag: self.pvp_flag,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntershardEntityUpdate {
    pub entity_id: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub orientation: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub hp: u32,
    pub combat_state: u8,
    pub pad_a: u8,
    pub pad_b: u8,
    pub pad_c: u8,
}

impl WireMessage for IntershardEntityUpdate {
    fn to_wire(self) -> Self {
        Self {
            entity_id: self.entity_id.to_le(),
            x: f32::from_bits(self.x.to_bits().to_le()),
            y: f32::from_bits(self.y.to_bits().to_le()),
            z: f32::from_bits(self.z.to_bits().to_le()),
            orientation: f32::from_bits(self.orientation.to_bits().to_le()),
            vx: f32::from_bits(self.vx.to_bits().to_le()),
            vy: f32::from_bits(self.vy.to_bits().to_le()),
            vz: f32::from_bits(self.vz.to_bits().to_le()),
            hp: self.hp.to_le(),
            combat_state: self.combat_state,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
        }
    }
    fn from_wire(self) -> Self {
        Self {
            entity_id: u32::from_le(self.entity_id),
            x: f32::from_bits(u32::from_le(self.x.to_bits())),
            y: f32::from_bits(u32::from_le(self.y.to_bits())),
            z: f32::from_bits(u32::from_le(self.z.to_bits())),
            orientation: f32::from_bits(u32::from_le(self.orientation.to_bits())),
            vx: f32::from_bits(u32::from_le(self.vx.to_bits())),
            vy: f32::from_bits(u32::from_le(self.vy.to_bits())),
            vz: f32::from_bits(u32::from_le(self.vz.to_bits())),
            hp: u32::from_le(self.hp),
            combat_state: self.combat_state,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntershardEntityLeave {
    pub entity_id: u32,
    pub reason: u8,
    pub pad_a: u8,
    pub pad_b: u8,
    pub pad_c: u8,
}

impl WireMessage for IntershardEntityLeave {
    fn to_wire(self) -> Self {
        Self {
            entity_id: self.entity_id.to_le(),
            reason: self.reason,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
        }
    }
    fn from_wire(self) -> Self {
        Self {
            entity_id: u32::from_le(self.entity_id),
            reason: self.reason,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntershardEntityState {
    pub entity_id: u32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub orientation: f64,
    pub hp: u32,
    pub stamina_x100: u32,
    pub combat_state: u8,
    pub pvp_flag: u8,
    pub pad_a: u8,
    pub pad_b: u8,
    pub combat_state_param: u32,
    pub group_id: u32,
    pub last_sequence: u32,
    pub last_action_sequence: u32,
    pub handoff_token: u64,
}

impl WireMessage for IntershardEntityState {
    fn to_wire(self) -> Self {
        Self {
            entity_id: self.entity_id.to_le(),
            x: f64::from_bits(self.x.to_bits().to_le()),
            y: f64::from_bits(self.y.to_bits().to_le()),
            z: f64::from_bits(self.z.to_bits().to_le()),
            vx: f64::from_bits(self.vx.to_bits().to_le()),
            vy: f64::from_bits(self.vy.to_bits().to_le()),
            vz: f64::from_bits(self.vz.to_bits().to_le()),
            orientation: f64::from_bits(self.orientation.to_bits().to_le()),
            hp: self.hp.to_le(),
            stamina_x100: self.stamina_x100.to_le(),
            combat_state: self.combat_state,
            pvp_flag: self.pvp_flag,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            combat_state_param: self.combat_state_param.to_le(),
            group_id: self.group_id.to_le(),
            last_sequence: self.last_sequence.to_le(),
            last_action_sequence: self.last_action_sequence.to_le(),
            handoff_token: self.handoff_token.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            entity_id: u32::from_le(self.entity_id),
            x: f64::from_bits(u64::from_le(self.x.to_bits())),
            y: f64::from_bits(u64::from_le(self.y.to_bits())),
            z: f64::from_bits(u64::from_le(self.z.to_bits())),
            vx: f64::from_bits(u64::from_le(self.vx.to_bits())),
            vy: f64::from_bits(u64::from_le(self.vy.to_bits())),
            vz: f64::from_bits(u64::from_le(self.vz.to_bits())),
            orientation: f64::from_bits(u64::from_le(self.orientation.to_bits())),
            hp: u32::from_le(self.hp),
            stamina_x100: u32::from_le(self.stamina_x100),
            combat_state: self.combat_state,
            pvp_flag: self.pvp_flag,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            combat_state_param: u32::from_le(self.combat_state_param),
            group_id: u32::from_le(self.group_id),
            last_sequence: u32::from_le(self.last_sequence),
            last_action_sequence: u32::from_le(self.last_action_sequence),
            handoff_token: u64::from_le(self.handoff_token),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntershardHandoffReq {
    pub entity_id: u32,
    pub sequence: u32,
    pub handoff_tick: u32,
}

impl WireMessage for IntershardHandoffReq {
    fn to_wire(self) -> Self {
        Self {
            entity_id: self.entity_id.to_le(),
            sequence: self.sequence.to_le(),
            handoff_tick: self.handoff_tick.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            entity_id: u32::from_le(self.entity_id),
            sequence: u32::from_le(self.sequence),
            handoff_tick: u32::from_le(self.handoff_tick),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntershardHandoffAck {
    pub entity_id: u32,
    pub sequence: u32,
    pub ok: u8,
    pub pad_a: u8,
    pub pad_b: u8,
    pub pad_c: u8,
}

impl WireMessage for IntershardHandoffAck {
    fn to_wire(self) -> Self {
        Self {
            entity_id: self.entity_id.to_le(),
            sequence: self.sequence.to_le(),
            ok: self.ok,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
        }
    }
    fn from_wire(self) -> Self {
        Self {
            entity_id: u32::from_le(self.entity_id),
            sequence: u32::from_le(self.sequence),
            ok: self.ok,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntershardHandoffComplete {
    pub entity_id: u32,
    pub handoff_token: u64,
}

impl WireMessage for IntershardHandoffComplete {
    fn to_wire(self) -> Self {
        Self {
            entity_id: self.entity_id.to_le(),
            handoff_token: self.handoff_token.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            entity_id: u32::from_le(self.entity_id),
            handoff_token: u64::from_le(self.handoff_token),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntershardAttack {
    pub attacker_entity_id: u32,
    pub target_entity_id: u32,
    pub attack_sequence: u32,
    pub action_type: u8,
    pub pad_a: u8,
    pub pad_b: u8,
    pub pad_c: u8,
    pub attacker_x: f32,
    pub attacker_z: f32,
    pub attacker_orientation: f32,
}

impl WireMessage for IntershardAttack {
    fn to_wire(self) -> Self {
        Self {
            attacker_entity_id: self.attacker_entity_id.to_le(),
            target_entity_id: self.target_entity_id.to_le(),
            attack_sequence: self.attack_sequence.to_le(),
            action_type: self.action_type,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
            attacker_x: f32::from_bits(self.attacker_x.to_bits().to_le()),
            attacker_z: f32::from_bits(self.attacker_z.to_bits().to_le()),
            attacker_orientation: f32::from_bits(self.attacker_orientation.to_bits().to_le()),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            attacker_entity_id: u32::from_le(self.attacker_entity_id),
            target_entity_id: u32::from_le(self.target_entity_id),
            attack_sequence: u32::from_le(self.attack_sequence),
            action_type: self.action_type,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
            attacker_x: f32::from_bits(u32::from_le(self.attacker_x.to_bits())),
            attacker_z: f32::from_bits(u32::from_le(self.attacker_z.to_bits())),
            attacker_orientation: f32::from_bits(u32::from_le(self.attacker_orientation.to_bits())),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntershardHitResult {
    pub attacker_entity_id: u32,
    pub target_entity_id: u32,
    pub attack_sequence: u32,
    pub hit: u8,
    pub pad_a: u8,
    pub pad_b: u8,
    pub pad_c: u8,
    pub damage_dealt: u32,
    pub target_hp: u32,
}

impl WireMessage for IntershardHitResult {
    fn to_wire(self) -> Self {
        Self {
            attacker_entity_id: self.attacker_entity_id.to_le(),
            target_entity_id: self.target_entity_id.to_le(),
            attack_sequence: self.attack_sequence.to_le(),
            hit: self.hit,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
            damage_dealt: self.damage_dealt.to_le(),
            target_hp: self.target_hp.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            attacker_entity_id: u32::from_le(self.attacker_entity_id),
            target_entity_id: u32::from_le(self.target_entity_id),
            attack_sequence: u32::from_le(self.attack_sequence),
            hit: self.hit,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
            damage_dealt: u32::from_le(self.damage_dealt),
            target_hp: u32::from_le(self.target_hp),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntershardCombatState {
    pub entity_id: u32,
    pub combat_state: u8,
    pub pad_a: u8,
    pub state_param: u16,
    pub server_tick: u32,
}

impl WireMessage for IntershardCombatState {
    fn to_wire(self) -> Self {
        Self {
            entity_id: self.entity_id.to_le(),
            combat_state: self.combat_state,
            pad_a: self.pad_a,
            state_param: self.state_param.to_le(),
            server_tick: self.server_tick.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            entity_id: u32::from_le(self.entity_id),
            combat_state: self.combat_state,
            pad_a: self.pad_a,
            state_param: u16::from_le(self.state_param),
            server_tick: u32::from_le(self.server_tick),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntershardOverlapEnrol {
    pub entity_id: u32,
    pub source_tick: u32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub orientation: f64,
    pub hp: u32,
    pub stamina_x100: u32,
    pub combat_state: u8,
    pub pvp_flag: u8,
    pub pad_a: u8,
    pub pad_b: u8,
    pub combat_state_param: u32,
    pub group_id: u32,
    pub last_sequence: u32,
    pub last_action_sequence: u32,
    pub overlap_token: u64,
}

impl WireMessage for IntershardOverlapEnrol {
    fn to_wire(self) -> Self {
        Self {
            entity_id: self.entity_id.to_le(),
            source_tick: self.source_tick.to_le(),
            x: f64::from_bits(self.x.to_bits().to_le()),
            y: f64::from_bits(self.y.to_bits().to_le()),
            z: f64::from_bits(self.z.to_bits().to_le()),
            vx: f64::from_bits(self.vx.to_bits().to_le()),
            vy: f64::from_bits(self.vy.to_bits().to_le()),
            vz: f64::from_bits(self.vz.to_bits().to_le()),
            orientation: f64::from_bits(self.orientation.to_bits().to_le()),
            hp: self.hp.to_le(),
            stamina_x100: self.stamina_x100.to_le(),
            combat_state: self.combat_state,
            pvp_flag: self.pvp_flag,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            combat_state_param: self.combat_state_param.to_le(),
            group_id: self.group_id.to_le(),
            last_sequence: self.last_sequence.to_le(),
            last_action_sequence: self.last_action_sequence.to_le(),
            overlap_token: self.overlap_token.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            entity_id: u32::from_le(self.entity_id),
            source_tick: u32::from_le(self.source_tick),
            x: f64::from_bits(u64::from_le(self.x.to_bits())),
            y: f64::from_bits(u64::from_le(self.y.to_bits())),
            z: f64::from_bits(u64::from_le(self.z.to_bits())),
            vx: f64::from_bits(u64::from_le(self.vx.to_bits())),
            vy: f64::from_bits(u64::from_le(self.vy.to_bits())),
            vz: f64::from_bits(u64::from_le(self.vz.to_bits())),
            orientation: f64::from_bits(u64::from_le(self.orientation.to_bits())),
            hp: u32::from_le(self.hp),
            stamina_x100: u32::from_le(self.stamina_x100),
            combat_state: self.combat_state,
            pvp_flag: self.pvp_flag,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            combat_state_param: u32::from_le(self.combat_state_param),
            group_id: u32::from_le(self.group_id),
            last_sequence: u32::from_le(self.last_sequence),
            last_action_sequence: u32::from_le(self.last_action_sequence),
            overlap_token: u64::from_le(self.overlap_token),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntershardOverlapSample {
    pub entity_id: u32,
    pub source_tick: u32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub last_sequence: u32,
    pub last_action_sequence: u32,
    pub overlap_token: u64,
}

impl WireMessage for IntershardOverlapSample {
    fn to_wire(self) -> Self {
        Self {
            entity_id: self.entity_id.to_le(),
            source_tick: self.source_tick.to_le(),
            x: f64::from_bits(self.x.to_bits().to_le()),
            y: f64::from_bits(self.y.to_bits().to_le()),
            z: f64::from_bits(self.z.to_bits().to_le()),
            vx: f64::from_bits(self.vx.to_bits().to_le()),
            vy: f64::from_bits(self.vy.to_bits().to_le()),
            vz: f64::from_bits(self.vz.to_bits().to_le()),
            last_sequence: self.last_sequence.to_le(),
            last_action_sequence: self.last_action_sequence.to_le(),
            overlap_token: self.overlap_token.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            entity_id: u32::from_le(self.entity_id),
            source_tick: u32::from_le(self.source_tick),
            x: f64::from_bits(u64::from_le(self.x.to_bits())),
            y: f64::from_bits(u64::from_le(self.y.to_bits())),
            z: f64::from_bits(u64::from_le(self.z.to_bits())),
            vx: f64::from_bits(u64::from_le(self.vx.to_bits())),
            vy: f64::from_bits(u64::from_le(self.vy.to_bits())),
            vz: f64::from_bits(u64::from_le(self.vz.to_bits())),
            last_sequence: u32::from_le(self.last_sequence),
            last_action_sequence: u32::from_le(self.last_action_sequence),
            overlap_token: u64::from_le(self.overlap_token),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntershardAuthorityTransfer {
    pub entity_id: u32,
    pub target_tick: u32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub orientation: f64,
    pub hp: u32,
    pub stamina_x100: u32,
    pub combat_state: u8,
    pub pad_a: u8,
    pub pad_b: u8,
    pub pad_c: u8,
    pub combat_state_param: u32,
    pub overlap_token: u64,
}

impl WireMessage for IntershardAuthorityTransfer {
    fn to_wire(self) -> Self {
        Self {
            entity_id: self.entity_id.to_le(),
            target_tick: self.target_tick.to_le(),
            x: f64::from_bits(self.x.to_bits().to_le()),
            y: f64::from_bits(self.y.to_bits().to_le()),
            z: f64::from_bits(self.z.to_bits().to_le()),
            vx: f64::from_bits(self.vx.to_bits().to_le()),
            vy: f64::from_bits(self.vy.to_bits().to_le()),
            vz: f64::from_bits(self.vz.to_bits().to_le()),
            orientation: f64::from_bits(self.orientation.to_bits().to_le()),
            hp: self.hp.to_le(),
            stamina_x100: self.stamina_x100.to_le(),
            combat_state: self.combat_state,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
            combat_state_param: self.combat_state_param.to_le(),
            overlap_token: self.overlap_token.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            entity_id: u32::from_le(self.entity_id),
            target_tick: u32::from_le(self.target_tick),
            x: f64::from_bits(u64::from_le(self.x.to_bits())),
            y: f64::from_bits(u64::from_le(self.y.to_bits())),
            z: f64::from_bits(u64::from_le(self.z.to_bits())),
            vx: f64::from_bits(u64::from_le(self.vx.to_bits())),
            vy: f64::from_bits(u64::from_le(self.vy.to_bits())),
            vz: f64::from_bits(u64::from_le(self.vz.to_bits())),
            orientation: f64::from_bits(u64::from_le(self.orientation.to_bits())),
            hp: u32::from_le(self.hp),
            stamina_x100: u32::from_le(self.stamina_x100),
            combat_state: self.combat_state,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
            combat_state_param: u32::from_le(self.combat_state_param),
            overlap_token: u64::from_le(self.overlap_token),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntershardAuthorityAck {
    pub entity_id: u32,
    pub applied_tick: u32,
    pub ok: u8,
    pub pad_a: u8,
    pub pad_b: u8,
    pub pad_c: u8,
    pub overlap_token: u64,
}

impl WireMessage for IntershardAuthorityAck {
    fn to_wire(self) -> Self {
        Self {
            entity_id: self.entity_id.to_le(),
            applied_tick: self.applied_tick.to_le(),
            ok: self.ok,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
            overlap_token: self.overlap_token.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            entity_id: u32::from_le(self.entity_id),
            applied_tick: u32::from_le(self.applied_tick),
            ok: self.ok,
            pad_a: self.pad_a,
            pad_b: self.pad_b,
            pad_c: self.pad_c,
            overlap_token: u64::from_le(self.overlap_token),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntershardOverlapLeave {
    pub entity_id: u32,
    pub overlap_token: u64,
}

impl WireMessage for IntershardOverlapLeave {
    fn to_wire(self) -> Self {
        Self {
            entity_id: self.entity_id.to_le(),
            overlap_token: self.overlap_token.to_le(),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            entity_id: u32::from_le(self.entity_id),
            overlap_token: u64::from_le(self.overlap_token),
        }
    }
}

const _: () = assert!(core::mem::size_of::<MsgHeader>() == 6);
const _: () = assert!(core::mem::size_of::<SessionOpen>() == 28);
const _: () = assert!(core::mem::size_of::<SessionClose>() == 1);
const _: () = assert!(core::mem::size_of::<Ping>() == 12);
const _: () = assert!(core::mem::size_of::<Pong>() == 28);
const _: () = assert!(core::mem::size_of::<ShardHandoff>() == 30);
const _: () = assert!(core::mem::size_of::<HandoffAuth>() == 20);
const _: () = assert!(core::mem::size_of::<SessionAuth>() == 2);
const _: () = assert!(core::mem::size_of::<SessionAuthFailed>() == 4);
const _: () = assert!(core::mem::size_of::<EntitySpawn>() == 26);
const _: () = assert!(core::mem::size_of::<EntityDespawn>() == 5);
// Bumped from 36 to 40 when `source_shard_hash: u32` was appended
// (Phase 4 Option A v2 — viewer ring colours by true authority owner
// instead of inferring from `(x, z)` ↔ region geometry).
const _: () = assert!(core::mem::size_of::<EntityMove>() == 40);
const _: () = assert!(core::mem::size_of::<EntityMoveBatch>() == 4);
const _: () = assert!(core::mem::size_of::<EntityMoveCompact>() == 32);
const _: () = assert!(core::mem::size_of::<EntityState>() == 18);
const _: () = assert!(core::mem::size_of::<EntityHealth>() == 12);
const _: () = assert!(core::mem::size_of::<HitConfirm>() == 20);
const _: () = assert!(core::mem::size_of::<ActionRejected>() == 8);
const _: () = assert!(core::mem::size_of::<PlayerMove>() == 24);
const _: () = assert!(core::mem::size_of::<PlayerAction>() == 20);
const _: () = assert!(core::mem::size_of::<StateAck>() == 44);
const _: () = assert!(core::mem::size_of::<IntershardHandshake>() == 24);
const _: () = assert!(core::mem::size_of::<IntershardHandshakeAck>() == 12);
const _: () = assert!(core::mem::size_of::<IntershardHeartbeat>() == 16);
const _: () = assert!(core::mem::size_of::<IntershardEntityEnter>() == 48);
const _: () = assert!(core::mem::size_of::<IntershardEntityUpdate>() == 40);
const _: () = assert!(core::mem::size_of::<IntershardEntityLeave>() == 8);
const _: () = assert!(core::mem::size_of::<IntershardEntityState>() == 96);
const _: () = assert!(core::mem::size_of::<IntershardHandoffReq>() == 12);
const _: () = assert!(core::mem::size_of::<IntershardHandoffAck>() == 12);
const _: () = assert!(core::mem::size_of::<IntershardHandoffComplete>() == 12);
const _: () = assert!(core::mem::size_of::<IntershardAttack>() == 28);
const _: () = assert!(core::mem::size_of::<IntershardHitResult>() == 24);
const _: () = assert!(core::mem::size_of::<IntershardCombatState>() == 12);
const _: () = assert!(core::mem::size_of::<IntershardOverlapEnrol>() == 100);
const _: () = assert!(core::mem::size_of::<IntershardOverlapSample>() == 72);
const _: () = assert!(core::mem::size_of::<IntershardAuthorityTransfer>() == 88);
const _: () = assert!(core::mem::size_of::<IntershardAuthorityAck>() == 20);
const _: () = assert!(core::mem::size_of::<IntershardOverlapLeave>() == 12);

/// Delta encoding for `IntershardEntityUpdate`. Re-exported for
/// backwards compatibility — the canonical path is `crate::delta`.
pub use crate::delta;
