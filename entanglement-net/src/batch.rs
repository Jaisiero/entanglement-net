use crate::error::NetError;
use crate::messages::{MsgHeader, WireMessage, MSG_HEADER_SIZE};

/// Packs multiple messages into a single Entanglement payload.
/// All multi-byte fields are written in little-endian wire format.
pub struct BatchWriter<'a> {
    buffer: &'a mut [u8],
    position: usize,
    count: usize,
}

impl<'a> BatchWriter<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self { buffer, position: 0, count: 0 }
    }

    /// Write a fixed-size message with automatic LE conversion.
    pub fn write_msg<T: WireMessage>(&mut self, msg_type: u16, msg: &T) -> Result<(), NetError> {
        let payload_size = core::mem::size_of::<T>();
        let total = MSG_HEADER_SIZE + payload_size;

        if self.position + total > self.buffer.len() {
            return Err(NetError::BatchFull {
                needed: total,
                available: self.buffer.len() - self.position,
            });
        }

        let header = MsgHeader {
            msg_type,
            msg_length: payload_size as u16,
            msg_flags: 0,
            reserved: 0,
        }.to_wire();
        unsafe {
            core::ptr::write_unaligned(
                self.buffer[self.position..].as_mut_ptr().cast::<MsgHeader>(),
                header,
            );
        }
        self.position += MSG_HEADER_SIZE;

        let wire_msg = msg.to_wire();
        unsafe {
            core::ptr::write_unaligned(
                self.buffer[self.position..].as_mut_ptr().cast::<T>(),
                wire_msg,
            );
        }
        self.position += payload_size;
        self.count += 1;
        Ok(())
    }

    /// Write a fixed-size message WITHOUT endianness conversion (raw copy).
    /// Use only when the caller has already ensured correct byte order.
    pub fn write<T: Copy>(&mut self, msg_type: u16, msg: &T) -> Result<(), NetError> {
        let payload_size = core::mem::size_of::<T>();
        let total = MSG_HEADER_SIZE + payload_size;

        if self.position + total > self.buffer.len() {
            return Err(NetError::BatchFull {
                needed: total,
                available: self.buffer.len() - self.position,
            });
        }

        let header = MsgHeader {
            msg_type,
            msg_length: payload_size as u16,
            msg_flags: 0,
            reserved: 0,
        }.to_wire();
        unsafe {
            core::ptr::write_unaligned(
                self.buffer[self.position..].as_mut_ptr().cast::<MsgHeader>(),
                header,
            );
        }
        self.position += MSG_HEADER_SIZE;

        unsafe {
            core::ptr::write_unaligned(
                self.buffer[self.position..].as_mut_ptr().cast::<T>(),
                *msg,
            );
        }
        self.position += payload_size;
        self.count += 1;
        Ok(())
    }

    /// Write raw bytes as a message payload (for variable-length messages).
    pub fn write_raw(&mut self, msg_type: u16, payload: &[u8]) -> Result<(), NetError> {
        let total = MSG_HEADER_SIZE + payload.len();

        if self.position + total > self.buffer.len() {
            return Err(NetError::BatchFull {
                needed: total,
                available: self.buffer.len() - self.position,
            });
        }

        let header = MsgHeader {
            msg_type,
            msg_length: payload.len() as u16,
            msg_flags: 0,
            reserved: 0,
        }.to_wire();
        unsafe {
            core::ptr::write_unaligned(
                self.buffer[self.position..].as_mut_ptr() as *mut MsgHeader,
                header,
            );
        }
        self.position += MSG_HEADER_SIZE;

        self.buffer[self.position..self.position + payload.len()].copy_from_slice(payload);
        self.position += payload.len();
        self.count += 1;
        Ok(())
    }

    /// Number of bytes written so far.
    pub fn bytes_written(&self) -> usize { self.position }

    /// Number of messages written so far.
    pub fn message_count(&self) -> usize { self.count }

    /// Remaining capacity in bytes.
    pub fn remaining(&self) -> usize { self.buffer.len() - self.position }

    /// Returns the written portion of the buffer.
    pub fn as_bytes(&self) -> &[u8] { &self.buffer[..self.position] }
}

/// Iterates over messages in a received buffer.
/// Headers are automatically converted from LE wire format to native.
pub struct BatchReader<'a> {
    buffer: &'a [u8],
    position: usize,
}

impl<'a> BatchReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer, position: 0 }
    }

    /// Returns remaining unread bytes.
    pub fn remaining(&self) -> usize {
        self.buffer.len().saturating_sub(self.position)
    }
}

/// Read a typed message from a payload slice, converting from LE wire format.
pub fn read_msg<T: WireMessage>(payload: &[u8]) -> Result<T, NetError> {
    let size = core::mem::size_of::<T>();
    if payload.len() < size {
        return Err(NetError::PayloadTooSmall { expected: size, actual: payload.len() });
    }
    let raw = unsafe { core::ptr::read_unaligned(payload.as_ptr() as *const T) };
    Ok(raw.from_wire())
}

impl<'a> Iterator for BatchReader<'a> {
    type Item = Result<(MsgHeader, &'a [u8]), NetError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position + MSG_HEADER_SIZE > self.buffer.len() {
            return None;
        }

        let header = unsafe {
            core::ptr::read_unaligned(
                self.buffer[self.position..].as_ptr() as *const MsgHeader,
            )
        }.from_wire();
        self.position += MSG_HEADER_SIZE;

        let payload_len = header.msg_length as usize;
        if self.position + payload_len > self.buffer.len() {
            return Some(Err(NetError::MalformedBatch { offset: self.position - MSG_HEADER_SIZE }));
        }

        let payload = &self.buffer[self.position..self.position + payload_len];
        self.position += payload_len;

        Some(Ok((header, payload)))
    }
}

/// Max JWT size in bytes for SessionAuth.
pub const SESSION_AUTH_MAX_JWT: usize = 512;

/// Write a SessionAuth payload: [jwt_length: u16 LE][jwt_bytes: N].
/// Returns the number of payload bytes written (excluding MsgHeader).
/// Use with `BatchWriter::write_raw(msg_type::SESSION_AUTH, &buf[..n])`.
pub fn write_session_auth(jwt: &str, buf: &mut [u8]) -> Result<usize, NetError> {
    let jwt_bytes = jwt.as_bytes();
    if jwt_bytes.len() > SESSION_AUTH_MAX_JWT {
        return Err(NetError::PayloadTooSmall {
            expected: SESSION_AUTH_MAX_JWT,
            actual: jwt_bytes.len(),
        });
    }
    let total = 2 + jwt_bytes.len();
    if total > buf.len() {
        return Err(NetError::BatchFull { needed: total, available: buf.len() });
    }
    let len_le = (jwt_bytes.len() as u16).to_le_bytes();
    buf[0] = len_le[0];
    buf[1] = len_le[1];
    buf[2..total].copy_from_slice(jwt_bytes);
    Ok(total)
}

/// Read the JWT string from a SessionAuth payload.
/// Payload format: [jwt_length: u16 LE][jwt_bytes: N].
pub fn read_session_auth_jwt(payload: &[u8]) -> Option<&str> {
    if payload.len() < 2 { return None; }
    let jwt_len = u16::from_le_bytes([payload[0], payload[1]]) as usize;
    if jwt_len == 0 || jwt_len > SESSION_AUTH_MAX_JWT { return None; }
    if 2 + jwt_len > payload.len() { return None; }
    core::str::from_utf8(&payload[2..2 + jwt_len]).ok()
}

/// Size of a HandoffAuth payload (v2):
/// entity_id: u32 + handoff_token: u64 + client_current_sequence: u32
/// + client_action_sequence: u32 = 20 bytes.
pub const HANDOFF_AUTH_SIZE: usize = 20;

/// Legacy (v1) HandoffAuth payload size (entity_id + handoff_token = 12 bytes).
/// Kept so servers can accept legacy clients that haven't been updated yet.
pub const HANDOFF_AUTH_SIZE_V1: usize = 12;

/// Write a HandoffAuth payload (v2):
/// [entity_id: u32 LE][handoff_token: u64 LE]
/// [client_current_sequence: u32 LE][client_action_sequence: u32 LE].
/// Returns the number of payload bytes written.
pub fn write_handoff_auth(
    entity_id: u32,
    token: u64,
    client_current_sequence: u32,
    client_action_sequence: u32,
    buf: &mut [u8],
) -> Result<usize, NetError> {
    if buf.len() < HANDOFF_AUTH_SIZE {
        return Err(NetError::BatchFull { needed: HANDOFF_AUTH_SIZE, available: buf.len() });
    }
    buf[0..4].copy_from_slice(&entity_id.to_le_bytes());
    buf[4..12].copy_from_slice(&token.to_le_bytes());
    buf[12..16].copy_from_slice(&client_current_sequence.to_le_bytes());
    buf[16..20].copy_from_slice(&client_action_sequence.to_le_bytes());
    Ok(HANDOFF_AUTH_SIZE)
}

/// Read a HandoffAuth payload. Accepts both v1 (12 bytes, no sequences) and
/// v2 (20 bytes, with client sequences) wire formats. For v1 payloads the two
/// sequence fields default to 0, which makes the learner shard reset its
/// per-player dedup state — the safe fallback behavior.
///
/// Returns `(entity_id, handoff_token, client_current_sequence, client_action_sequence)`.
pub fn read_handoff_auth(payload: &[u8]) -> Option<(u32, u64, u32, u32)> {
    if payload.len() < HANDOFF_AUTH_SIZE_V1 { return None; }
    let entity_id = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]);
    let token = u64::from_le_bytes([
        payload[4], payload[5], payload[6], payload[7],
        payload[8], payload[9], payload[10], payload[11],
    ]);
    if payload.len() >= HANDOFF_AUTH_SIZE {
        let client_current_sequence = u32::from_le_bytes([
            payload[12], payload[13], payload[14], payload[15],
        ]);
        let client_action_sequence = u32::from_le_bytes([
            payload[16], payload[17], payload[18], payload[19],
        ]);
        Some((entity_id, token, client_current_sequence, client_action_sequence))
    } else {
        // v1 legacy payload — no sequences. Default to 0 so the server resets
        // dedup on the learner shard (Fix 4 fallback).
        Some((entity_id, token, 0, 0))
    }
}
