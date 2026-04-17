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
        persistent_id: 12345,
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
fn test_player_move_batch_variable_length() {
    let inputs = [
        PlayerMove { input_sequence: 1, estimated_server_tick: 100, move_x: 1.0, move_z: 0.0, orientation: 0.0, buttons: 0 },
        PlayerMove { input_sequence: 2, estimated_server_tick: 101, move_x: 0.5, move_z: 0.5, orientation: 1.0, buttons: 0 },
        PlayerMove { input_sequence: 3, estimated_server_tick: 102, move_x: 0.0, move_z: 1.0, orientation: 2.0, buttons: 0 },
    ];

    let mut buf = [0u8; 256];
    let written = write_player_move_batch(&mut buf, &inputs).unwrap();
    assert_eq!(written, 1 + 3 * core::mem::size_of::<PlayerMove>());

    let payload = read_player_move_batch(&buf[..written]).unwrap();
    assert_eq!(payload.len(), 3 * core::mem::size_of::<PlayerMove>());

    let entry_size = core::mem::size_of::<PlayerMove>();
    for i in 0..3 {
        let input: PlayerMove = read_msg(&payload[i * entry_size..]).unwrap();
        assert_eq!(input, inputs[i]);
    }
}

#[test]
fn test_player_move_batch_max_entries() {
    let inputs: Vec<PlayerMove> = (0..12).map(|i| PlayerMove {
        input_sequence: i, estimated_server_tick: 100 + i,
        move_x: 0.0, move_z: 0.0, orientation: 0.0, buttons: 0,
    }).collect();

    let mut buf = [0u8; 512];
    let written = write_player_move_batch(&mut buf, &inputs).unwrap();
    let expected = 1 + 8 * core::mem::size_of::<PlayerMove>();
    assert_eq!(written, expected);
    assert_eq!(buf[0], 8);
}

#[test]
fn test_wire_sizes() {
    assert_eq!(core::mem::size_of::<MsgHeader>(), 6);
    assert_eq!(core::mem::size_of::<SessionOpen>(), 28);
    assert_eq!(core::mem::size_of::<SessionClose>(), 1);
    assert_eq!(core::mem::size_of::<Ping>(), 12);
    assert_eq!(core::mem::size_of::<Pong>(), 28);
    assert_eq!(core::mem::size_of::<ShardHandoff>(), 30);
    assert_eq!(core::mem::size_of::<EntitySpawn>(), 26);
    assert_eq!(core::mem::size_of::<EntityDespawn>(), 5);
    assert_eq!(core::mem::size_of::<EntityMove>(), 36);
    assert_eq!(core::mem::size_of::<EntityState>(), 18);
    assert_eq!(core::mem::size_of::<EntityHealth>(), 12);
    assert_eq!(core::mem::size_of::<HitConfirm>(), 20);
    assert_eq!(core::mem::size_of::<ActionRejected>(), 8);
    assert_eq!(core::mem::size_of::<PlayerMove>(), 24);
    assert_eq!(core::mem::size_of::<PlayerAction>(), 20);
    assert_eq!(core::mem::size_of::<StateAck>(), 44);
    assert_eq!(core::mem::size_of::<EntityMoveBatch>(), 4);
    assert_eq!(core::mem::size_of::<EntityMoveCompact>(), 32);
    assert_eq!(core::mem::size_of::<SessionAuth>(), 2);
    assert_eq!(core::mem::size_of::<SessionAuthFailed>(), 4);
    // Inter-shard messages (0x0300+)
    assert_eq!(core::mem::size_of::<IntershardHandshake>(), 24);
    assert_eq!(core::mem::size_of::<IntershardHandshakeAck>(), 12);
    assert_eq!(core::mem::size_of::<IntershardHeartbeat>(), 16);
    assert_eq!(core::mem::size_of::<IntershardEntityEnter>(), 48);
    assert_eq!(core::mem::size_of::<IntershardEntityUpdate>(), 40);
    assert_eq!(core::mem::size_of::<IntershardEntityLeave>(), 8);
    assert_eq!(core::mem::size_of::<IntershardEntityState>(), 96);
    assert_eq!(core::mem::size_of::<IntershardHandoffReq>(), 12);
    assert_eq!(core::mem::size_of::<IntershardHandoffAck>(), 12);
    assert_eq!(core::mem::size_of::<IntershardAttack>(), 28);
    assert_eq!(core::mem::size_of::<IntershardHitResult>(), 24);
    assert_eq!(core::mem::size_of::<IntershardCombatState>(), 12);
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

#[test]
fn test_player_move_roundtrip() {
    let msg_out = PlayerMove {
        input_sequence: 42,
        estimated_server_tick: 1000,
        move_x: 0.5,
        move_z: -0.5,
        orientation: 1.57,
        buttons: 0,
    };

    let mut buf = [0u8; 64];
    let written = {
        let mut writer = BatchWriter::new(&mut buf);
        writer.write_msg(msg_type::PLAYER_MOVE, &msg_out).unwrap();
        writer.bytes_written()
    };

    let reader = BatchReader::new(&buf[..written]);
    let (header, payload) = reader.into_iter().next().unwrap().unwrap();
    assert_eq!(hdr_type(&header), msg_type::PLAYER_MOVE);
    assert_eq!(hdr_len(&header) as usize, core::mem::size_of::<PlayerMove>());
    let msg_in: PlayerMove = read_msg(payload).unwrap();
    assert_eq!(msg_in, msg_out);
}

#[test]
fn test_player_action_roundtrip() {
    let msg_out = PlayerAction {
        input_sequence: 99,
        server_tick: 5000,
        action_type: 0x01, // Attack (light)
        pad_a: 0,
        pad_b: 0,
        pad_c: 0,
        param_a: 12345,
        param_b: 0,
    };

    let mut buf = [0u8; 64];
    let written = {
        let mut writer = BatchWriter::new(&mut buf);
        writer.write_msg(msg_type::PLAYER_ACTION, &msg_out).unwrap();
        writer.bytes_written()
    };

    let reader = BatchReader::new(&buf[..written]);
    let (header, payload) = reader.into_iter().next().unwrap().unwrap();
    assert_eq!(hdr_type(&header), msg_type::PLAYER_ACTION);
    assert_eq!(hdr_len(&header) as usize, core::mem::size_of::<PlayerAction>());
    let msg_in: PlayerAction = read_msg(payload).unwrap();
    assert_eq!(msg_in, msg_out);
}

#[test]
fn test_state_ack_roundtrip() {
    let msg_out = StateAck {
        input_sequence_acked: 42,
        server_tick: 1000,
        tick_delta_us: 16666,
        x: 100.5,
        y: 0.0,
        z: -200.3,
        vx: 5.0,
        vy: 0.0,
        vz: -3.0,
        hp: 100,
        stamina: 100.0,
    };

    let mut buf = [0u8; 64];
    let written = {
        let mut writer = BatchWriter::new(&mut buf);
        writer.write_msg(msg_type::STATE_ACK, &msg_out).unwrap();
        writer.bytes_written()
    };

    let reader = BatchReader::new(&buf[..written]);
    let (header, payload) = reader.into_iter().next().unwrap().unwrap();
    assert_eq!(hdr_type(&header), msg_type::STATE_ACK);
    assert_eq!(hdr_len(&header) as usize, core::mem::size_of::<StateAck>());
    let msg_in: StateAck = read_msg(payload).unwrap();
    assert_eq!(msg_in, msg_out);
}

#[test]
fn test_channels_constants() {
    use entanglement_net::channel;
    assert_eq!(channel::CONTROL, 0);
    assert_eq!(channel::UNRELIABLE, 1);
    assert_eq!(channel::RELIABLE, 2);
    assert_eq!(channel::ORDERED, 3);
    assert_eq!(channel::UNRELIABLE_COALESCED, 4);
    assert_eq!(channel::RELIABLE_COALESCED, 5);
    assert_eq!(channel::ORDERED_COALESCED, 6);
}

#[test]
fn test_hit_confirm_roundtrip() {
    let msg_out = HitConfirm {
        input_sequence: 42,
        target_id: 7,
        damage_dealt: 15,
        target_hp: 85,
        server_tick: 12000,
    };

    let mut buf = [0u8; 64];
    let written = {
        let mut writer = BatchWriter::new(&mut buf);
        writer.write_msg(msg_type::HIT_CONFIRM, &msg_out).unwrap();
        writer.bytes_written()
    };

    let reader = BatchReader::new(&buf[..written]);
    let (header, payload) = reader.into_iter().next().unwrap().unwrap();
    assert_eq!(hdr_type(&header), msg_type::HIT_CONFIRM);
    assert_eq!(hdr_len(&header) as usize, core::mem::size_of::<HitConfirm>());
    let msg_in: HitConfirm = read_msg(payload).unwrap();
    assert_eq!(msg_in, msg_out);
}

#[test]
fn test_hit_confirm_wire_le() {
    let msg = HitConfirm {
        input_sequence: 0x01020304,
        target_id: 0x05060708,
        damage_dealt: 15,
        target_hp: 85,
        server_tick: 1000,
    };

    let mut buf = [0u8; 64];
    let written = {
        let mut writer = BatchWriter::new(&mut buf);
        writer.write_msg(msg_type::HIT_CONFIRM, &msg).unwrap();
        writer.bytes_written()
    };
    assert_eq!(written, 6 + 20);

    // msg_type: HIT_CONFIRM = 0x0105 → LE: [0x05, 0x01]
    assert_eq!(buf[0], 0x05);
    assert_eq!(buf[1], 0x01);
    // input_sequence: 0x01020304 → LE: [04, 03, 02, 01]
    assert_eq!(&buf[6..10], &[0x04, 0x03, 0x02, 0x01]);
    // target_id: 0x05060708 → LE: [08, 07, 06, 05]
    assert_eq!(&buf[10..14], &[0x08, 0x07, 0x06, 0x05]);
}

#[test]
fn test_action_rejected_roundtrip() {
    let msg_out = ActionRejected {
        input_sequence: 99,
        reason: 0x00, // NoStamina
        pad_a: 0,
        pad_b: 0,
        pad_c: 0,
    };

    let mut buf = [0u8; 32];
    let written = {
        let mut writer = BatchWriter::new(&mut buf);
        writer.write_msg(msg_type::ACTION_REJECTED, &msg_out).unwrap();
        writer.bytes_written()
    };

    let reader = BatchReader::new(&buf[..written]);
    let (header, payload) = reader.into_iter().next().unwrap().unwrap();
    assert_eq!(hdr_type(&header), msg_type::ACTION_REJECTED);
    assert_eq!(hdr_len(&header) as usize, core::mem::size_of::<ActionRejected>());
    let msg_in: ActionRejected = read_msg(payload).unwrap();
    assert_eq!(msg_in, msg_out);
}

#[test]
fn test_action_rejected_all_reasons() {
    let reasons = [0x00u8, 0x01, 0x02, 0x03, 0x04]; // NoStamina..NotInPvpZone
    for &reason in &reasons {
        let msg = ActionRejected {
            input_sequence: reason as u32,
            reason,
            pad_a: 0, pad_b: 0, pad_c: 0,
        };
        let mut buf = [0u8; 32];
        let written = {
            let mut writer = BatchWriter::new(&mut buf);
            writer.write_msg(msg_type::ACTION_REJECTED, &msg).unwrap();
            writer.bytes_written()
        };
        let reader = BatchReader::new(&buf[..written]);
        let (_, payload) = reader.into_iter().next().unwrap().unwrap();
        let recovered: ActionRejected = read_msg(payload).unwrap();
        assert_eq!(recovered.reason, reason);
    }
}

#[test]
fn test_combat_messages_in_mixed_batch() {
    let mut buf = [0u8; 256];
    let mut writer = BatchWriter::new(&mut buf);

    let hit = HitConfirm {
        input_sequence: 1, target_id: 5,
        damage_dealt: 15, target_hp: 85, server_tick: 100,
    };
    writer.write_msg(msg_type::HIT_CONFIRM, &hit).unwrap();

    let reject = ActionRejected {
        input_sequence: 2, reason: 0x01,
        pad_a: 0, pad_b: 0, pad_c: 0,
    };
    writer.write_msg(msg_type::ACTION_REJECTED, &reject).unwrap();

    let health = EntityHealth { entity_id: 5, hp: 85, max_hp: 100 };
    writer.write_msg(msg_type::ENTITY_HEALTH, &health).unwrap();

    assert_eq!(writer.message_count(), 3);

    let reader = BatchReader::new(writer.as_bytes());
    let msgs: Vec<_> = reader.filter_map(|r| r.ok()).collect();
    assert_eq!(msgs.len(), 3);
    assert_eq!(hdr_type(&msgs[0].0), msg_type::HIT_CONFIRM);
    assert_eq!(hdr_type(&msgs[1].0), msg_type::ACTION_REJECTED);
    assert_eq!(hdr_type(&msgs[2].0), msg_type::ENTITY_HEALTH);
}

// ─── SessionAuth / SessionAuthFailed tests ───

#[test]
fn test_session_auth_roundtrip_128() {
    use entanglement_net::batch::{write_session_auth, read_session_auth_jwt};

    let jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkphaW1lIiwiaWF0IjoxNTE2MjM5MDIyfQ";
    assert!(jwt.len() <= 128);

    let mut payload_buf = [0u8; 520];
    let written = write_session_auth(jwt, &mut payload_buf).unwrap();
    assert_eq!(written, 2 + jwt.len());

    let recovered = read_session_auth_jwt(&payload_buf[..written]).unwrap();
    assert_eq!(recovered, jwt);
}

#[test]
fn test_session_auth_roundtrip_512() {
    use entanglement_net::batch::{write_session_auth, read_session_auth_jwt, SESSION_AUTH_MAX_JWT};

    let jwt: String = (0..SESSION_AUTH_MAX_JWT).map(|i| (b'A' + (i % 26) as u8) as char).collect();
    assert_eq!(jwt.len(), 512);

    let mut payload_buf = [0u8; 520];
    let written = write_session_auth(&jwt, &mut payload_buf).unwrap();
    assert_eq!(written, 2 + 512);

    let recovered = read_session_auth_jwt(&payload_buf[..written]).unwrap();
    assert_eq!(recovered, jwt);
}

#[test]
fn test_session_auth_too_long() {
    use entanglement_net::batch::write_session_auth;

    let jwt: String = (0..513).map(|_| 'X').collect();
    let mut buf = [0u8; 600];
    assert!(write_session_auth(&jwt, &mut buf).is_err());
}

#[test]
fn test_session_auth_via_batch_writer() {
    use entanglement_net::batch::write_session_auth;

    let jwt = "test.jwt.token";
    let mut payload_buf = [0u8; 128];
    let payload_len = write_session_auth(jwt, &mut payload_buf).unwrap();

    let mut buf = [0u8; 256];
    let written = {
        let mut writer = BatchWriter::new(&mut buf);
        writer.write_raw(msg_type::SESSION_AUTH, &payload_buf[..payload_len]).unwrap();
        writer.bytes_written()
    };

    let reader = BatchReader::new(&buf[..written]);
    let (header, payload) = reader.into_iter().next().unwrap().unwrap();
    assert_eq!(hdr_type(&header), msg_type::SESSION_AUTH);
    assert_eq!(hdr_len(&header), payload_len as u16);

    let recovered = entanglement_net::read_session_auth_jwt(payload).unwrap();
    assert_eq!(recovered, jwt);
}

#[test]
fn test_session_auth_failed_size() {
    assert_eq!(core::mem::size_of::<SessionAuthFailed>(), 4);
}

#[test]
fn test_session_auth_failed_roundtrip() {
    use entanglement_net::session_auth_fail_reason;

    let msg_out = SessionAuthFailed {
        reason: session_auth_fail_reason::EXPIRED,
        pad_a: 0, pad_b: 0, pad_c: 0,
    };

    let mut buf = [0u8; 32];
    let written = {
        let mut writer = BatchWriter::new(&mut buf);
        writer.write_msg(msg_type::SESSION_AUTH_FAILED, &msg_out).unwrap();
        writer.bytes_written()
    };

    let reader = BatchReader::new(&buf[..written]);
    let (header, payload) = reader.into_iter().next().unwrap().unwrap();
    assert_eq!(hdr_type(&header), msg_type::SESSION_AUTH_FAILED);
    assert_eq!(hdr_len(&header) as usize, 4);
    let msg_in: SessionAuthFailed = read_msg(payload).unwrap();
    assert_eq!(msg_in.reason, session_auth_fail_reason::EXPIRED);
}

#[test]
fn test_session_auth_fail_reason_constants() {
    use entanglement_net::session_auth_fail_reason;
    assert_eq!(session_auth_fail_reason::INVALID_TOKEN, 0x00);
    assert_eq!(session_auth_fail_reason::EXPIRED, 0x01);
    assert_eq!(session_auth_fail_reason::SERVER_FULL, 0x02);
    assert_eq!(session_auth_fail_reason::ALREADY_CONNECTED, 0x03);
}

// ─── Inter-shard message tests ───

#[test]
fn test_intershard_handshake_roundtrip() {
    let msg_out = IntershardHandshake {
        shard_id: 42,
        sequence: 1,
        hmac_0: 0x0102030405060708,
        hmac_1: 0x090A0B0C0D0E0F10,
    };

    let mut buf = [0u8; 64];
    let written = {
        let mut writer = BatchWriter::new(&mut buf);
        writer.write_msg(msg_type::INTERSHARD_HANDSHAKE, &msg_out).unwrap();
        writer.bytes_written()
    };

    let reader = BatchReader::new(&buf[..written]);
    let (header, payload) = reader.into_iter().next().unwrap().unwrap();
    assert_eq!(hdr_type(&header), msg_type::INTERSHARD_HANDSHAKE);
    assert_eq!(hdr_len(&header) as usize, core::mem::size_of::<IntershardHandshake>());
    let msg_in: IntershardHandshake = read_msg(payload).unwrap();
    assert_eq!(msg_in, msg_out);
}

#[test]
fn test_intershard_entity_enter_roundtrip() {
    let msg_out = IntershardEntityEnter {
        entity_id: 100,
        entity_type: 1,
        pad_a: 0,
        x: 50.5, y: 0.0, z: -100.3,
        orientation: 1.57,
        vx: 5.0, vy: 0.0, vz: -3.0,
        hp: 100, max_hp: 100,
        combat_state: 0, pvp_flag: 1,
        pad_b: 0, pad_c: 0,
    };

    let mut buf = [0u8; 128];
    let written = {
        let mut writer = BatchWriter::new(&mut buf);
        writer.write_msg(msg_type::INTERSHARD_ENTITY_ENTER, &msg_out).unwrap();
        writer.bytes_written()
    };

    let reader = BatchReader::new(&buf[..written]);
    let (header, payload) = reader.into_iter().next().unwrap().unwrap();
    assert_eq!(hdr_type(&header), msg_type::INTERSHARD_ENTITY_ENTER);
    assert_eq!(hdr_len(&header) as usize, 48);
    let msg_in: IntershardEntityEnter = read_msg(payload).unwrap();
    assert_eq!(msg_in, msg_out);
}

#[test]
fn test_intershard_entity_state_f64_roundtrip() {
    let msg_out = IntershardEntityState {
        entity_id: 42,
        x: 1234.567890123456,
        y: 0.0,
        z: -5678.901234567890,
        vx: 5.123456789,
        vy: 0.0,
        vz: -3.987654321,
        orientation: 3.14159265358979,
        hp: 85,
        stamina_x100: 7500,
        combat_state: 0,
        pvp_flag: 1,
        pad_a: 0,
        pad_b: 0,
        combat_state_param: 0,
        group_id: 0,
        last_sequence: 0,
        last_action_sequence: 0,
        handoff_token: 0,
    };

    let mut buf = [0u8; 128];
    let written = {
        let mut writer = BatchWriter::new(&mut buf);
        writer.write_msg(msg_type::INTERSHARD_ENTITY_STATE, &msg_out).unwrap();
        writer.bytes_written()
    };

    let reader = BatchReader::new(&buf[..written]);
    let (header, payload) = reader.into_iter().next().unwrap().unwrap();
    assert_eq!(hdr_type(&header), msg_type::INTERSHARD_ENTITY_STATE);
    assert_eq!(hdr_len(&header) as usize, 96);
    let msg_in: IntershardEntityState = read_msg(payload).unwrap();
    assert_eq!(msg_in, msg_out);
    // Verify f64 precision preserved (copy to avoid packed struct alignment issue)
    let x = { msg_in.x };
    let ori = { msg_in.orientation };
    assert_eq!(x, 1234.567890123456);
    assert_eq!(ori, 3.14159265358979);
}

#[test]
fn test_intershard_attack_roundtrip() {
    let msg_out = IntershardAttack {
        attacker_entity_id: 10,
        target_entity_id: 20,
        attack_sequence: 5,
        action_type: 0x01,
        pad_a: 0, pad_b: 0, pad_c: 0,
        attacker_x: 100.0,
        attacker_z: 200.0,
        attacker_orientation: 1.57,
    };

    let mut buf = [0u8; 64];
    let written = {
        let mut writer = BatchWriter::new(&mut buf);
        writer.write_msg(msg_type::INTERSHARD_ATTACK, &msg_out).unwrap();
        writer.bytes_written()
    };

    let reader = BatchReader::new(&buf[..written]);
    let (header, payload) = reader.into_iter().next().unwrap().unwrap();
    assert_eq!(hdr_type(&header), msg_type::INTERSHARD_ATTACK);
    assert_eq!(hdr_len(&header) as usize, 28);
    let msg_in: IntershardAttack = read_msg(payload).unwrap();
    assert_eq!(msg_in, msg_out);
}

#[test]
fn test_intershard_mixed_batch() {
    let mut buf = [0u8; 512];
    let mut writer = BatchWriter::new(&mut buf);

    let heartbeat = IntershardHeartbeat {
        shard_id: 1, server_tick: 1000, player_count: 50, ghost_count: 5,
    };
    writer.write_msg(msg_type::INTERSHARD_HEARTBEAT, &heartbeat).unwrap();

    let update = IntershardEntityUpdate {
        entity_id: 42,
        x: 10.0, y: 0.0, z: 20.0,
        orientation: 1.0,
        vx: 1.0, vy: 0.0, vz: -1.0,
        hp: 100, combat_state: 0,
        pad_a: 0, pad_b: 0, pad_c: 0,
    };
    writer.write_msg(msg_type::INTERSHARD_ENTITY_UPDATE, &update).unwrap();

    let leave = IntershardEntityLeave {
        entity_id: 99, reason: 1, pad_a: 0, pad_b: 0, pad_c: 0,
    };
    writer.write_msg(msg_type::INTERSHARD_ENTITY_LEAVE, &leave).unwrap();

    assert_eq!(writer.message_count(), 3);

    let reader = BatchReader::new(writer.as_bytes());
    let msgs: Vec<_> = reader.filter_map(|r| r.ok()).collect();
    assert_eq!(msgs.len(), 3);
    assert_eq!(hdr_type(&msgs[0].0), msg_type::INTERSHARD_HEARTBEAT);
    assert_eq!(hdr_type(&msgs[1].0), msg_type::INTERSHARD_ENTITY_UPDATE);
    assert_eq!(hdr_type(&msgs[2].0), msg_type::INTERSHARD_ENTITY_LEAVE);

    // Verify update has same layout as EntityMoveCompact + hp/combat (40 bytes)
    assert_eq!(core::mem::size_of::<IntershardEntityUpdate>(), 40);
}