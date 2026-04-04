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
    pub const ENTITY_SPAWN: u16 = 0x0100;
    pub const ENTITY_DESPAWN: u16 = 0x0101;
    pub const ENTITY_MOVE: u16 = 0x0102;
    pub const ENTITY_STATE: u16 = 0x0103;
    pub const ENTITY_HEALTH: u16 = 0x0104;
    pub const HIT_CONFIRM: u16 = 0x0105;
    pub const ACTION_REJECTED: u16 = 0x0106;
    pub const PLAYER_MOVE: u16 = 0x0200;
    pub const PLAYER_MOVE_BATCH: u16 = 0x0201;
    pub const PLAYER_ACTION: u16 = 0x0202;
    pub const STATE_ACK: u16 = 0x0203;
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
}

impl WireMessage for PlayerMove {
    fn to_wire(self) -> Self {
        Self {
            input_sequence: self.input_sequence.to_le(),
            estimated_server_tick: self.estimated_server_tick.to_le(),
            move_x: f32::from_bits(self.move_x.to_bits().to_le()),
            move_z: f32::from_bits(self.move_z.to_bits().to_le()),
            orientation: f32::from_bits(self.orientation.to_bits().to_le()),
        }
    }
    fn from_wire(self) -> Self {
        Self {
            input_sequence: u32::from_le(self.input_sequence),
            estimated_server_tick: u32::from_le(self.estimated_server_tick),
            move_x: f32::from_bits(u32::from_le(self.move_x.to_bits())),
            move_z: f32::from_bits(u32::from_le(self.move_z.to_bits())),
            orientation: f32::from_bits(u32::from_le(self.orientation.to_bits())),
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
        }
    }
}

const _: () = assert!(core::mem::size_of::<MsgHeader>() == 6);
const _: () = assert!(core::mem::size_of::<SessionOpen>() == 24);
const _: () = assert!(core::mem::size_of::<SessionClose>() == 1);
const _: () = assert!(core::mem::size_of::<Ping>() == 12);
const _: () = assert!(core::mem::size_of::<Pong>() == 28);
const _: () = assert!(core::mem::size_of::<ShardHandoff>() == 22);
const _: () = assert!(core::mem::size_of::<EntitySpawn>() == 26);
const _: () = assert!(core::mem::size_of::<EntityDespawn>() == 5);
const _: () = assert!(core::mem::size_of::<EntityMove>() == 36);
const _: () = assert!(core::mem::size_of::<EntityState>() == 18);
const _: () = assert!(core::mem::size_of::<EntityHealth>() == 12);
const _: () = assert!(core::mem::size_of::<HitConfirm>() == 20);
const _: () = assert!(core::mem::size_of::<ActionRejected>() == 8);
const _: () = assert!(core::mem::size_of::<PlayerMove>() == 20);
const _: () = assert!(core::mem::size_of::<PlayerAction>() == 20);
const _: () = assert!(core::mem::size_of::<StateAck>() == 36);
