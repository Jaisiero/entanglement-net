// AUTO-GENERATED — do not edit manually
// Source: schemas/messages.toml

pub const PROTOCOL_VERSION: u16 = 1;
pub const MSG_HEADER_SIZE: usize = 6;
pub const MAX_PAYLOAD_BYTES: usize = 1154;

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
    pub const PLAYER_INPUT: u16 = 0x0200;
    pub const PLAYER_INPUT_BATCH: u16 = 0x0201;
    pub const STATE_ACK: u16 = 0x0202;
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MsgHeader {
    pub msg_type: u16,
    pub msg_length: u16,
    pub msg_flags: u8,
    pub reserved: u8,
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

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SessionClose {
    pub reason: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ping {
    pub client_frame: u32,
    pub client_time_us: u64,
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

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntityDespawn {
    pub entity_id: u32,
    pub reason: u8,
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

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntityState {
    pub entity_id: u32,
    pub server_tick: u32,
    pub state_id: u16,
    pub param_a: u32,
    pub param_b: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntityHealth {
    pub entity_id: u32,
    pub hp: u32,
    pub max_hp: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerInput {
    pub input_sequence: u32,
    pub estimated_server_tick: u32,
    pub move_x: f32,
    pub move_z: f32,
    pub orientation: f32,
    pub buttons: u32,
}

/// Variable-length batch: count (u8) + count × PlayerInput
/// Max entries: 8
pub const PLAYER_INPUT_BATCH_MAX_ENTRIES: usize = 8;

/// Write a PlayerInputBatch into a buffer. Returns bytes written.
pub fn write_player_input_batch(buf: &mut [u8], inputs: &[PlayerInput]) -> Result<usize, ()> {
    let count = inputs.len().min(PLAYER_INPUT_BATCH_MAX_ENTRIES);
    let entry_size = core::mem::size_of::<PlayerInput>();
    let total = 1 + count * entry_size; // 1 byte count + entries
    if total > buf.len() { return Err(()); }
    buf[0] = count as u8;
    for i in 0..count {
        unsafe {
            core::ptr::write_unaligned(
                buf[1 + i * entry_size..].as_mut_ptr() as *mut PlayerInput,
                inputs[i],
            );
        }
    }
    Ok(total)
}

/// Read a PlayerInputBatch from a buffer.
pub fn read_player_input_batch(payload: &[u8]) -> Option<&[u8]> {
    if payload.is_empty() { return None; }
    let count = payload[0] as usize;
    let entry_size = core::mem::size_of::<PlayerInput>();
    let total = 1 + count * entry_size;
    if total > payload.len() { return None; }
    Some(&payload[1..total])
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
const _: () = assert!(core::mem::size_of::<PlayerInput>() == 24);
const _: () = assert!(core::mem::size_of::<StateAck>() == 36);
