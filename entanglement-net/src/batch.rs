use crate::error::NetError;
use crate::messages::{MsgHeader, MSG_HEADER_SIZE};

/// Packs multiple messages into a single Entanglement payload
pub struct BatchWriter<'a> {
    buffer: &'a mut [u8],
    position: usize,
    count: usize,
}

impl<'a> BatchWriter<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self { buffer, position: 0, count: 0 }
    }

    /// Write a fixed-size message. Returns Err if the message doesn't fit.
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
        };
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
        };
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

/// Iterates over messages in a received buffer
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
        };
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
