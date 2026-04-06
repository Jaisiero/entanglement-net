/* AUTO-GENERATED — do not edit manually */
/* Source: schemas/messages.toml              */
/*                                            */
/* Wire format: all multi-byte fields are LITTLE-ENDIAN.  */
/* Use ent_net_htole* / ent_net_letoh* macros to convert. */

#ifndef ENTANGLEMENT_NET_H
#define ENTANGLEMENT_NET_H

#include <stdint.h>

/* ── Little-endian conversion helpers ─────────────────── */
#if defined(__BYTE_ORDER__) && __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
  #define ENT_NET_HTOLE16(x) __builtin_bswap16(x)
  #define ENT_NET_HTOLE32(x) __builtin_bswap32(x)
  #define ENT_NET_HTOLE64(x) __builtin_bswap64(x)
#else
  #define ENT_NET_HTOLE16(x) (x)
  #define ENT_NET_HTOLE32(x) (x)
  #define ENT_NET_HTOLE64(x) (x)
#endif
#define ENT_NET_LETOH16(x) ENT_NET_HTOLE16(x)
#define ENT_NET_LETOH32(x) ENT_NET_HTOLE32(x)
#define ENT_NET_LETOH64(x) ENT_NET_HTOLE64(x)

#define ENT_NET_PROTOCOL_VERSION 2
#define ENT_NET_MSG_HEADER_SIZE 6
#define ENT_NET_MAX_PAYLOAD_BYTES 1154

#define ENT_NET_MSG_SESSION_OPEN 0x0001
#define ENT_NET_MSG_SESSION_CLOSE 0x0002
#define ENT_NET_MSG_PING 0x0003
#define ENT_NET_MSG_PONG 0x0004
#define ENT_NET_MSG_SHARD_HANDOFF 0x0005
#define ENT_NET_MSG_SESSION_AUTH 0x0007
#define ENT_NET_MSG_SESSION_AUTH_FAILED 0x0008
#define ENT_NET_MSG_ENTITY_SPAWN 0x0100
#define ENT_NET_MSG_ENTITY_DESPAWN 0x0101
#define ENT_NET_MSG_ENTITY_MOVE 0x0102
#define ENT_NET_MSG_ENTITY_STATE 0x0103
#define ENT_NET_MSG_ENTITY_HEALTH 0x0104
#define ENT_NET_MSG_HIT_CONFIRM 0x0105
#define ENT_NET_MSG_ACTION_REJECTED 0x0106
#define ENT_NET_MSG_PLAYER_MOVE 0x0200
#define ENT_NET_MSG_PLAYER_MOVE_BATCH 0x0201
#define ENT_NET_MSG_PLAYER_ACTION 0x0202
#define ENT_NET_MSG_STATE_ACK 0x0203

#pragma pack(push, 1)

typedef struct {
    uint16_t msg_type;
    uint16_t msg_length;
    uint8_t  msg_flags;
    uint8_t  reserved;
} ent_net_msg_header_t;

typedef struct {
    uint16_t protocol_version;
    uint32_t player_id;
    uint32_t shard_id;
    float origin_x;
    float origin_z;
    uint32_t server_tick;
    uint16_t tick_rate_hz;
} ent_net_session_open_t;

typedef struct {
    uint8_t reason;
} ent_net_session_close_t;

typedef struct {
    uint32_t client_frame;
    uint64_t client_time_us;
} ent_net_ping_t;

typedef struct {
    uint32_t client_frame;
    uint64_t client_time_us;
    uint32_t server_tick;
    uint64_t server_time_us;
    uint32_t tick_delta_us;
} ent_net_pong_t;

typedef struct {
    uint32_t new_shard_ip_packed;
    uint16_t new_shard_port;
    uint32_t new_shard_id;
    float new_origin_x;
    float new_origin_z;
    uint32_t handoff_tick;
} ent_net_shard_handoff_t;

typedef struct {
    uint16_t jwt_length;
} ent_net_session_auth_t;

typedef struct {
    uint8_t reason;
    uint8_t pad_a;
    uint8_t pad_b;
    uint8_t pad_c;
} ent_net_session_auth_failed_t;

typedef struct {
    uint32_t entity_id;
    uint16_t entity_type;
    float x;
    float y;
    float z;
    float orientation;
    uint32_t initial_state;
} ent_net_entity_spawn_t;

typedef struct {
    uint32_t entity_id;
    uint8_t reason;
} ent_net_entity_despawn_t;

typedef struct {
    uint32_t entity_id;
    uint32_t server_tick;
    float x;
    float y;
    float z;
    float orientation;
    float vx;
    float vy;
    float vz;
} ent_net_entity_move_t;

typedef struct {
    uint32_t entity_id;
    uint32_t server_tick;
    uint16_t state_id;
    uint32_t param_a;
    uint32_t param_b;
} ent_net_entity_state_t;

typedef struct {
    uint32_t entity_id;
    uint32_t hp;
    uint32_t max_hp;
} ent_net_entity_health_t;

typedef struct {
    uint32_t input_sequence;
    uint32_t target_id;
    uint32_t damage_dealt;
    uint32_t target_hp;
    uint32_t server_tick;
} ent_net_hit_confirm_t;

typedef struct {
    uint32_t input_sequence;
    uint8_t reason;
    uint8_t pad_a;
    uint8_t pad_b;
    uint8_t pad_c;
} ent_net_action_rejected_t;

typedef struct {
    uint32_t input_sequence;
    uint32_t estimated_server_tick;
    float move_x;
    float move_z;
    float orientation;
    uint32_t buttons;
} ent_net_player_move_t;

typedef struct {
    uint32_t input_sequence;
    uint32_t server_tick;
    uint8_t action_type;
    uint8_t pad_a;
    uint8_t pad_b;
    uint8_t pad_c;
    uint32_t param_a;
    uint32_t param_b;
} ent_net_player_action_t;

typedef struct {
    uint32_t input_sequence_acked;
    uint32_t server_tick;
    uint32_t tick_delta_us;
    float x;
    float y;
    float z;
    float vx;
    float vy;
    float vz;
} ent_net_state_ack_t;

#pragma pack(pop)

#if defined(__cplusplus)
  #define ENT_NET_STATIC_ASSERT(expr, msg) static_assert(expr, msg)
#elif defined(_Static_assert)
  #define ENT_NET_STATIC_ASSERT(expr, msg) _Static_assert(expr, msg)
#elif __STDC_VERSION__ >= 201112L
  #define ENT_NET_STATIC_ASSERT(expr, msg) _Static_assert(expr, msg)
#else
  #define ENT_NET_STATIC_ASSERT(expr, msg) typedef char _ent_static_assert_##__LINE__[(expr)?1:-1]
#endif

ENT_NET_STATIC_ASSERT(sizeof(ent_net_msg_header_t) == 6, "MsgHeader size");
ENT_NET_STATIC_ASSERT(sizeof(ent_net_session_open_t) == 24, "SessionOpen size");
ENT_NET_STATIC_ASSERT(sizeof(ent_net_session_close_t) == 1, "SessionClose size");
ENT_NET_STATIC_ASSERT(sizeof(ent_net_ping_t) == 12, "Ping size");
ENT_NET_STATIC_ASSERT(sizeof(ent_net_pong_t) == 28, "Pong size");
ENT_NET_STATIC_ASSERT(sizeof(ent_net_shard_handoff_t) == 22, "ShardHandoff size");
ENT_NET_STATIC_ASSERT(sizeof(ent_net_session_auth_t) == 2, "SessionAuth size");
ENT_NET_STATIC_ASSERT(sizeof(ent_net_session_auth_failed_t) == 4, "SessionAuthFailed size");
ENT_NET_STATIC_ASSERT(sizeof(ent_net_entity_spawn_t) == 26, "EntitySpawn size");
ENT_NET_STATIC_ASSERT(sizeof(ent_net_entity_despawn_t) == 5, "EntityDespawn size");
ENT_NET_STATIC_ASSERT(sizeof(ent_net_entity_move_t) == 36, "EntityMove size");
ENT_NET_STATIC_ASSERT(sizeof(ent_net_entity_state_t) == 18, "EntityState size");
ENT_NET_STATIC_ASSERT(sizeof(ent_net_entity_health_t) == 12, "EntityHealth size");
ENT_NET_STATIC_ASSERT(sizeof(ent_net_hit_confirm_t) == 20, "HitConfirm size");
ENT_NET_STATIC_ASSERT(sizeof(ent_net_action_rejected_t) == 8, "ActionRejected size");
ENT_NET_STATIC_ASSERT(sizeof(ent_net_player_move_t) == 24, "PlayerMove size");
ENT_NET_STATIC_ASSERT(sizeof(ent_net_player_action_t) == 20, "PlayerAction size");
ENT_NET_STATIC_ASSERT(sizeof(ent_net_state_ack_t) == 36, "StateAck size");

#endif /* ENTANGLEMENT_NET_H */
