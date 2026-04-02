use entanglement_net::batch::{BatchReader, BatchWriter, read_msg};
use entanglement_net::messages::*;

/// Helper to safely read msg_type from a packed MsgHeader
fn hdr_type(h: &MsgHeader) -> u16 {
    unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(h.msg_type)) }
}
fn hdr_len(h: &MsgHeader) -> u16 {
    unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(h.msg_length)) }
}

#[test]
fn test_entity_move_roundtrip() {
    let msg_out = EntityMove {
        entity_id: 42,
        server_tick: 1000,
        x: 100.5,
        y: 0.0,
        z: -200.3,
        orientation: 1.57,
        vx: 5.0,
        vy: 0.0,
        vz: -3.0,
    };

    let mut buf = [0u8; 512];
    let written = {
        let mut writer = BatchWriter::new(&mut buf);
        writer.write_msg(msg_type::ENTITY_MOVE, &msg_out).unwrap();
        writer.bytes_written()
    };

    let reader = BatchReader::new(&buf[..written]);
    for item in reader {
        let (header, payload) = item.unwrap();
        assert_eq!(hdr_type(&header), msg_type::ENTITY_MOVE);
        assert_eq!(hdr_len(&header) as usize, core::mem::size_of::<EntityMove>());

        let msg_in: EntityMove = read_msg(payload).unwrap();
        assert_eq!(msg_in, msg_out);
    }
}

#[test]
fn test_session_open_roundtrip() {
    let msg_out = SessionOpen {
        protocol_version: PROTOCOL_VERSION,
        player_id: 12345,
        shard_id: 1,
        origin_x: -500.0,
        origin_z: 1200.0,
        server_tick: 99999,
        tick_rate_hz: 30,
    };

    let mut buf = [0u8; 128];
    let written = {
        let mut writer = BatchWriter::new(&mut buf);
        writer.write_msg(msg_type::SESSION_OPEN, &msg_out).unwrap();
        writer.bytes_written()
    };

    let reader = BatchReader::new(&buf[..written]);
    let (header, payload) = reader.into_iter().next().unwrap().unwrap();
    assert_eq!(hdr_type(&header), msg_type::SESSION_OPEN);
    let msg_in: SessionOpen = read_msg(payload).unwrap();
    assert_eq!(msg_in, msg_out);
}

#[test]
fn test_batch_multiple_messages() {
    let mut buf = [0u8; MAX_PAYLOAD_BYTES];
    let (count, written) = {
        let mut writer = BatchWriter::new(&mut buf);
        let msg = EntityMove {
            entity_id: 1, server_tick: 100,
            x: 0.0, y: 0.0, z: 0.0, orientation: 0.0,
            vx: 0.0, vy: 0.0, vz: 0.0,
        };
        let mut count = 0;
        while writer.write_msg(msg_type::ENTITY_MOVE, &msg).is_ok() {
            count += 1;
        }
        (count, writer.bytes_written())
    };

    // EntityMove = 36 bytes + 6 header = 42 each; 1154 / 42 = 27
    assert!(count >= 27, "Expected >=27 messages, got {count}");

    let reader = BatchReader::new(&buf[..written]);
    let read_count = reader.filter_map(|r| r.ok()).count();
    assert_eq!(read_count, count);
}

#[test]
fn test_mixed_batch() {
    let mut buf = [0u8; 512];
    let mut writer = BatchWriter::new(&mut buf);

    let spawn = EntitySpawn {
        entity_id: 10, entity_type: 1,
        x: 50.0, y: 0.0, z: -30.0, orientation: 0.0, initial_state: 0,
    };
    writer.write_msg(msg_type::ENTITY_SPAWN, &spawn).unwrap();

    let mv = EntityMove {
        entity_id: 10, server_tick: 1,
        x: 51.0, y: 0.0, z: -30.0, orientation: 0.1,
        vx: 1.0, vy: 0.0, vz: 0.0,
    };
    writer.write_msg(msg_type::ENTITY_MOVE, &mv).unwrap();

    let health = EntityHealth { entity_id: 10, hp: 100, max_hp: 100 };
    writer.write_msg(msg_type::ENTITY_HEALTH, &health).unwrap();

    assert_eq!(writer.message_count(), 3);

    let reader = BatchReader::new(writer.as_bytes());
    let msgs: Vec<_> = reader.filter_map(|r| r.ok()).collect();
    assert_eq!(msgs.len(), 3);
    assert_eq!(hdr_type(&msgs[0].0), msg_type::ENTITY_SPAWN);
    assert_eq!(hdr_type(&msgs[1].0), msg_type::ENTITY_MOVE);
    assert_eq!(hdr_type(&msgs[2].0), msg_type::ENTITY_HEALTH);
}

#[test]
fn test_player_input_batch_variable_length() {
    let inputs = [
        PlayerInput { input_sequence: 1, estimated_server_tick: 100, move_x: 1.0, move_z: 0.0, orientation: 0.0, buttons: 0 },
        PlayerInput { input_sequence: 2, estimated_server_tick: 101, move_x: 0.5, move_z: 0.5, orientation: 1.0, buttons: 1 },
        PlayerInput { input_sequence: 3, estimated_server_tick: 102, move_x: 0.0, move_z: 1.0, orientation: 2.0, buttons: 0 },
    ];

    let mut buf = [0u8; 256];
    let written = write_player_input_batch(&mut buf, &inputs).unwrap();
    assert_eq!(written, 1 + 3 * core::mem::size_of::<PlayerInput>());

    let payload = read_player_input_batch(&buf[..written]).unwrap();
    assert_eq!(payload.len(), 3 * core::mem::size_of::<PlayerInput>());

    // read_msg handles from_wire conversion for each entry
    let entry_size = core::mem::size_of::<PlayerInput>();
    for i in 0..3 {
        let input: PlayerInput = read_msg(&payload[i * entry_size..]).unwrap();
        assert_eq!(input, inputs[i]);
    }
}

#[test]
fn test_player_input_batch_max_entries() {
    let inputs: Vec<PlayerInput> = (0..12).map(|i| PlayerInput {
        input_sequence: i, estimated_server_tick: 100 + i,
        move_x: 0.0, move_z: 0.0, orientation: 0.0, buttons: 0,
    }).collect();

    let mut buf = [0u8; 512];
    let written = write_player_input_batch(&mut buf, &inputs).unwrap();
    let expected = 1 + 8 * core::mem::size_of::<PlayerInput>();
    assert_eq!(written, expected);
    assert_eq!(buf[0], 8);
}

#[test]
fn test_wire_sizes() {
    assert_eq!(core::mem::size_of::<MsgHeader>(), 6);
    assert_eq!(core::mem::size_of::<SessionOpen>(), 24);
    assert_eq!(core::mem::size_of::<SessionClose>(), 1);
    assert_eq!(core::mem::size_of::<Ping>(), 12);
    assert_eq!(core::mem::size_of::<Pong>(), 28);
    assert_eq!(core::mem::size_of::<ShardHandoff>(), 22);
    assert_eq!(core::mem::size_of::<EntitySpawn>(), 26);
    assert_eq!(core::mem::size_of::<EntityDespawn>(), 5);
    assert_eq!(core::mem::size_of::<EntityMove>(), 36);
    assert_eq!(core::mem::size_of::<EntityState>(), 18);
    assert_eq!(core::mem::size_of::<EntityHealth>(), 12);
    assert_eq!(core::mem::size_of::<PlayerInput>(), 24);
    assert_eq!(core::mem::size_of::<StateAck>(), 36);
}

#[test]
fn test_batch_full_error() {
    let mut buf = [0u8; 10];
    let mut writer = BatchWriter::new(&mut buf);
    let msg = EntityMove {
        entity_id: 1, server_tick: 1,
        x: 0.0, y: 0.0, z: 0.0, orientation: 0.0,
        vx: 0.0, vy: 0.0, vz: 0.0,
    };
    assert!(writer.write_msg(msg_type::ENTITY_MOVE, &msg).is_err());
}

#[test]
fn test_empty_batch() {
    let buf = [0u8; 0];
    let reader = BatchReader::new(&buf);
    assert_eq!(reader.count(), 0);
}

#[test]
fn test_malformed_batch() {
    // Manually write a LE header claiming 100 bytes, but buffer only 10
    let mut buf = [0u8; 10];
    let header = MsgHeader {
        msg_type: msg_type::ENTITY_MOVE,
        msg_length: 100,
        msg_flags: 0,
        reserved: 0,
    }.to_wire();
    unsafe {
        core::ptr::write_unaligned(buf.as_mut_ptr() as *mut MsgHeader, header);
    }
    let mut reader = BatchReader::new(&buf);
    let result = reader.next().unwrap();
    assert!(result.is_err());
}

#[test]
fn test_dispatcher_routes_messages() {
    use entanglement_net::dispatcher::{Dispatcher, MessageContext};
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    let counter = Arc::new(AtomicU32::new(0));
    let c = counter.clone();

    let mut dispatcher = Dispatcher::new();
    dispatcher.on(msg_type::ENTITY_MOVE, move |_payload, _ctx| {
        c.fetch_add(1, Ordering::Relaxed);
        Ok(())
    });

    let mut buf = [0u8; 512];
    let written = {
        let mut writer = BatchWriter::new(&mut buf);
        let msg = EntityMove {
            entity_id: 1, server_tick: 1,
            x: 0.0, y: 0.0, z: 0.0, orientation: 0.0,
            vx: 0.0, vy: 0.0, vz: 0.0,
        };
        for _ in 0..3 {
            writer.write_msg(msg_type::ENTITY_MOVE, &msg).unwrap();
        }
        writer.bytes_written()
    };

    let ctx = MessageContext { sender_id: 1, server_tick: 100, timestamp_us: 0 };
    let dispatched = dispatcher.dispatch_batch(&buf[..written], &ctx).unwrap();
    assert_eq!(dispatched, 3);
    assert_eq!(counter.load(Ordering::Relaxed), 3);
}

#[test]
fn test_dispatcher_unknown_type() {
    use entanglement_net::dispatcher::{Dispatcher, MessageContext};

    let dispatcher = Dispatcher::new();
    let header = MsgHeader { msg_type: 0xFFFF, msg_length: 0, msg_flags: 0, reserved: 0 };
    let ctx = MessageContext { sender_id: 1, server_tick: 0, timestamp_us: 0 };
    assert!(dispatcher.dispatch(&header, &[], &ctx).is_err());
}

#[test]
fn test_session_coordinate_transform() {
    use entanglement_net::session::Session;

    let session = Session::new(1, 1, 1000.0, 2000.0, PROTOCOL_VERSION);

    let (wx, wz) = session.world_to_wire(1050.0, 2100.0);
    assert!((wx - 50.0).abs() < 0.01);
    assert!((wz - 100.0).abs() < 0.01);

    let (world_x, world_z) = session.wire_to_world(wx, wz);
    assert!((world_x - 1050.0).abs() < 0.01);
    assert!((world_z - 2100.0).abs() < 0.01);
}

/// Verify the wire format is explicitly little-endian at the byte level.
/// This test would catch a broken to_wire() on a big-endian platform.
#[test]
fn test_wire_format_is_little_endian() {
    // Write an EntityHealth with known values via write_msg (LE conversion)
    let msg = EntityHealth { entity_id: 0x01020304, hp: 0x05060708, max_hp: 0x090A0B0C };

    let mut buf = [0u8; 64];
    let written = {
        let mut writer = BatchWriter::new(&mut buf);
        writer.write_msg(msg_type::ENTITY_HEALTH, &msg).unwrap();
        writer.bytes_written()
    };
    assert_eq!(written, 6 + 12); // header + payload

    // ── Verify header bytes (LE) ──
    // msg_type: ENTITY_HEALTH = 0x0104 → LE bytes: [0x04, 0x01]
    assert_eq!(buf[0], 0x04);
    assert_eq!(buf[1], 0x01);
    // msg_length: 12 = 0x000C → LE bytes: [0x0C, 0x00]
    assert_eq!(buf[2], 0x0C);
    assert_eq!(buf[3], 0x00);
    // msg_flags: 0, reserved: 0
    assert_eq!(buf[4], 0x00);
    assert_eq!(buf[5], 0x00);

    // ── Verify payload bytes (LE) ──
    // entity_id: 0x01020304 → LE: [04, 03, 02, 01]
    assert_eq!(&buf[6..10], &[0x04, 0x03, 0x02, 0x01]);
    // hp: 0x05060708 → LE: [08, 07, 06, 05]
    assert_eq!(&buf[10..14], &[0x08, 0x07, 0x06, 0x05]);
    // max_hp: 0x090A0B0C → LE: [0C, 0B, 0A, 09]
    assert_eq!(&buf[14..18], &[0x0C, 0x0B, 0x0A, 0x09]);

    // ── Verify round-trip via read_msg recovers original values ──
    let reader = BatchReader::new(&buf[..written]);
    let (_, payload) = reader.into_iter().next().unwrap().unwrap();
    let recovered: EntityHealth = read_msg(payload).unwrap();
    assert_eq!(recovered, msg);
}

/// Verify f32 fields are stored as LE IEEE 754 bits on the wire.
#[test]
fn test_wire_format_float_le() {
    // f32 1.0 = 0x3F800000 → LE bytes: [00, 00, 80, 3F]
    let msg = EntityMove {
        entity_id: 1, server_tick: 2,
        x: 1.0, y: 0.0, z: 0.0, orientation: 0.0,
        vx: 0.0, vy: 0.0, vz: 0.0,
    };

    let mut buf = [0u8; 64];
    let written = {
        let mut writer = BatchWriter::new(&mut buf);
        writer.write_msg(msg_type::ENTITY_MOVE, &msg).unwrap();
        writer.bytes_written()
    };

    // x field starts at offset 6 (header) + 4 (entity_id) + 4 (server_tick) = 14
    assert_eq!(&buf[14..18], &[0x00, 0x00, 0x80, 0x3F], "f32 1.0 should be LE 0x3F800000");

    // Full round-trip
    let reader = BatchReader::new(&buf[..written]);
    let (_, payload) = reader.into_iter().next().unwrap().unwrap();
    let recovered: EntityMove = read_msg(payload).unwrap();
    assert_eq!(recovered, msg);
}

/// Verify u64 fields are LE on the wire.
#[test]
fn test_wire_format_u64_le() {
    let msg = Ping { client_frame: 1, client_time_us: 0x0102030405060708 };

    let mut buf = [0u8; 32];
    let written = {
        let mut writer = BatchWriter::new(&mut buf);
        writer.write_msg(msg_type::PING, &msg).unwrap();
        writer.bytes_written()
    };

    // client_time_us at offset 6 + 4 = 10, 8 bytes
    assert_eq!(
        &buf[10..18],
        &[0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01],
        "u64 should be LE"
    );

    let reader = BatchReader::new(&buf[..written]);
    let (_, payload) = reader.into_iter().next().unwrap().unwrap();
    let recovered: Ping = read_msg(payload).unwrap();
    assert_eq!(recovered, msg);
}
