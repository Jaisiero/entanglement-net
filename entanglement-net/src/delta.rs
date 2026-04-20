// ── Delta encoding for IntershardEntityUpdate ────────────────────────
//
// NOTE: this module is HAND-WRITTEN and must not be regenerated. Lives
// outside `messages.rs` (which is auto-generated) precisely so the
// codegen's overwrite can't wipe it.
//
// Wire format (variable-length, manual serialization):
//   entity_id:  u32  (4 bytes, always present)
//   seq:        u16  (2 bytes, sender tick counter for stale-detection)
//   bitfield:   u16  (2 bytes, which fields follow)
//   [fields...]:      only present fields, in bitfield order, LE
//
// Bitfield bits:
//   bit 0: x           (f32, 4B)
//   bit 1: y           (f32, 4B)
//   bit 2: z           (f32, 4B)
//   bit 3: orientation (f32, 4B)
//   bit 4: vx          (f32, 4B)
//   bit 5: vy          (f32, 4B)
//   bit 6: vz          (f32, 4B)
//   bit 7: hp          (u32, 4B)
//   bit 8: combat_state (u8, 1B)
//
// Header = 8 bytes.  Max payload = 8 + 33 = 41 bytes.
// Min payload (zero dirty) = 8 bytes (skip entirely is also valid).

use crate::messages::IntershardEntityUpdate;

pub const BIT_X: u16           = 1 << 0;
pub const BIT_Y: u16           = 1 << 1;
pub const BIT_Z: u16           = 1 << 2;
pub const BIT_ORIENTATION: u16 = 1 << 3;
pub const BIT_VX: u16          = 1 << 4;
pub const BIT_VY: u16          = 1 << 5;
pub const BIT_VZ: u16          = 1 << 6;
pub const BIT_HP: u16          = 1 << 7;
pub const BIT_COMBAT_STATE: u16 = 1 << 8;

pub const ALL_BITS: u16 = BIT_X | BIT_Y | BIT_Z | BIT_ORIENTATION
    | BIT_VX | BIT_VY | BIT_VZ | BIT_HP | BIT_COMBAT_STATE;

/// Header size: entity_id(4) + seq(2) + bitfield(2) = 8 bytes
pub const HEADER_SIZE: usize = 8;
/// Maximum delta payload: header + all 9 fields (7×4 + 4 + 1 = 33)
pub const MAX_PAYLOAD: usize = HEADER_SIZE + 33;

/// Threshold for float field "unchanged" detection.
pub const FLOAT_EPS: f32 = 0.001;

/// Compare two update states and return the dirty bitfield.
#[inline]
pub fn compute_bitfield(prev: &IntershardEntityUpdate, cur: &IntershardEntityUpdate) -> u16 {
    let mut bits: u16 = 0;
    if (cur.x - prev.x).abs() > FLOAT_EPS { bits |= BIT_X; }
    if (cur.y - prev.y).abs() > FLOAT_EPS { bits |= BIT_Y; }
    if (cur.z - prev.z).abs() > FLOAT_EPS { bits |= BIT_Z; }
    if (cur.orientation - prev.orientation).abs() > FLOAT_EPS { bits |= BIT_ORIENTATION; }
    if (cur.vx - prev.vx).abs() > FLOAT_EPS { bits |= BIT_VX; }
    if (cur.vy - prev.vy).abs() > FLOAT_EPS { bits |= BIT_VY; }
    if (cur.vz - prev.vz).abs() > FLOAT_EPS { bits |= BIT_VZ; }
    if cur.hp != prev.hp { bits |= BIT_HP; }
    if cur.combat_state != prev.combat_state { bits |= BIT_COMBAT_STATE; }
    bits
}

/// Encode a delta message into `buf`. Returns number of bytes written.
/// All multi-byte fields are little-endian.
#[inline]
pub fn encode(entity_id: u32, seq: u16, bitfield: u16,
              cur: &IntershardEntityUpdate, buf: &mut [u8]) -> usize {
    let mut pos = 0;
    // entity_id
    buf[pos..pos + 4].copy_from_slice(&entity_id.to_le_bytes());
    pos += 4;
    // seq
    buf[pos..pos + 2].copy_from_slice(&seq.to_le_bytes());
    pos += 2;
    // bitfield
    buf[pos..pos + 2].copy_from_slice(&bitfield.to_le_bytes());
    pos += 2;

    macro_rules! write_f32 {
        ($bit:expr, $field:expr) => {
            if bitfield & $bit != 0 {
                buf[pos..pos + 4].copy_from_slice(&$field.to_bits().to_le_bytes());
                pos += 4;
            }
        };
    }
    write_f32!(BIT_X, cur.x);
    write_f32!(BIT_Y, cur.y);
    write_f32!(BIT_Z, cur.z);
    write_f32!(BIT_ORIENTATION, cur.orientation);
    write_f32!(BIT_VX, cur.vx);
    write_f32!(BIT_VY, cur.vy);
    write_f32!(BIT_VZ, cur.vz);
    if bitfield & BIT_HP != 0 {
        buf[pos..pos + 4].copy_from_slice(&cur.hp.to_le_bytes());
        pos += 4;
    }
    if bitfield & BIT_COMBAT_STATE != 0 {
        buf[pos] = cur.combat_state;
        pos += 1;
    }
    pos
}

/// Decode a delta message from `payload`, applying changes onto `baseline`.
/// Returns (entity_id, seq) on success, or None if payload is malformed.
#[inline]
pub fn decode(payload: &[u8], baseline: &mut IntershardEntityUpdate) -> Option<(u32, u16)> {
    if payload.len() < HEADER_SIZE { return None; }
    let entity_id = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]);
    let seq = u16::from_le_bytes([payload[4], payload[5]]);
    let bitfield = u16::from_le_bytes([payload[6], payload[7]]);

    // Validate expected length
    let expected = HEADER_SIZE + payload_size_from_bitfield(bitfield);
    if payload.len() < expected { return None; }

    let mut pos = HEADER_SIZE;

    macro_rules! read_f32 {
        ($bit:expr, $field:ident) => {
            if bitfield & $bit != 0 {
                let v = u32::from_le_bytes([payload[pos], payload[pos+1], payload[pos+2], payload[pos+3]]);
                baseline.$field = f32::from_bits(v);
                pos += 4;
            }
        };
    }
    read_f32!(BIT_X, x);
    read_f32!(BIT_Y, y);
    read_f32!(BIT_Z, z);
    read_f32!(BIT_ORIENTATION, orientation);
    read_f32!(BIT_VX, vx);
    read_f32!(BIT_VY, vy);
    read_f32!(BIT_VZ, vz);
    if bitfield & BIT_HP != 0 {
        baseline.hp = u32::from_le_bytes([payload[pos], payload[pos+1], payload[pos+2], payload[pos+3]]);
        pos += 4;
    }
    if bitfield & BIT_COMBAT_STATE != 0 {
        baseline.combat_state = payload[pos];
        // pos += 1;  // not needed, last field
    }

    baseline.entity_id = entity_id;
    Some((entity_id, seq))
}

/// Compute the payload byte count (excluding header) for a given bitfield.
#[inline]
pub fn payload_size_from_bitfield(bitfield: u16) -> usize {
    let mut size = 0usize;
    // 7 f32 fields (bits 0-6) + 1 u32 field (bit 7)
    for bit in 0..=7 {
        if bitfield & (1 << bit) != 0 { size += 4; }
    }
    // bit 8: combat_state u8
    if bitfield & BIT_COMBAT_STATE != 0 { size += 1; }
    size
}
