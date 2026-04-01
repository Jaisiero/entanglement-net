/// Session state for a connected client
#[derive(Debug, Clone)]
pub struct Session {
    pub player_id: u32,
    pub shard_id: u32,
    pub origin_x: f32,
    pub origin_z: f32,
    pub protocol_version: u16,
    pub last_input_sequence: u32,
    pub last_state_ack_tick: u32,
    pub rtt_us: u32,
    pub clock_offset_ticks: i32,
}

impl Session {
    pub fn new(player_id: u32, shard_id: u32, origin_x: f32, origin_z: f32, protocol_version: u16) -> Self {
        Self {
            player_id,
            shard_id,
            origin_x,
            origin_z,
            protocol_version,
            last_input_sequence: 0,
            last_state_ack_tick: 0,
            rtt_us: 0,
            clock_offset_ticks: 0,
        }
    }

    /// Convert world coordinates to wire coordinates (relative to shard origin)
    pub fn world_to_wire(&self, world_x: f64, world_z: f64) -> (f32, f32) {
        (
            (world_x - self.origin_x as f64) as f32,
            (world_z - self.origin_z as f64) as f32,
        )
    }

    /// Convert wire coordinates to world coordinates
    pub fn wire_to_world(&self, wire_x: f32, wire_z: f32) -> (f64, f64) {
        (
            wire_x as f64 + self.origin_x as f64,
            wire_z as f64 + self.origin_z as f64,
        )
    }

    /// Update RTT and clock offset from Ping/Pong exchange
    pub fn update_clock(
        &mut self,
        _client_time_us: u64,
        _server_time_us: u64,
        server_tick: u32,
        rtt_us: u32,
        tick_rate_hz: u16,
    ) {
        self.rtt_us = rtt_us;
        let ticks_per_us = tick_rate_hz as f64 / 1_000_000.0;
        let half_rtt_ticks = (rtt_us as f64 / 2.0) * ticks_per_us;
        self.clock_offset_ticks = (server_tick as f64 + half_rtt_ticks) as i32;
    }
}
