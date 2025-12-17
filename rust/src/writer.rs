use std::collections::VecDeque;

#[derive(Debug)]
#[allow(dead_code)]
pub struct State {
    pub len: usize,
    pub buf: Vec<u8>,
}

pub struct WriterImpl {
    buf: Vec<u8>,
    states: VecDeque<State>,
}

impl WriterImpl {
    pub fn new() -> Self {
        WriterImpl {
            buf: Vec::new(),
            states: VecDeque::new(),
        }
    }

    #[inline]
    pub fn write_varint32(&mut self, mut value: u32) {
        while value >= 0x80 {
            self.buf.push((value & 0x7F | 0x80) as u8);
            value >>= 7;
        }
        self.buf.push(value as u8);
    }

    #[inline]
    pub fn write_varint64(&mut self, mut value: u64) {
        while value >= 0x80 {
            self.buf.push((value & 0x7F | 0x80) as u8);
            value >>= 7;
        }
        self.buf.push(value as u8);
    }

    #[inline]
    pub fn write_sint32(&mut self, value: i32) {
        let encoded = ((value << 1) ^ (value >> 31)) as u32;
        self.write_varint32(encoded);
    }

    #[inline]
    pub fn write_sint64(&mut self, value: i64) {
        let encoded = ((value << 1) ^ (value >> 63)) as u64;
        self.write_varint64(encoded);
    }

    #[inline]
    pub fn write_fixed32(&mut self, value: u32) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    #[inline]
    pub fn write_fixed64(&mut self, value: u64) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    #[inline]
    pub fn write_float(&mut self, value: f32) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    #[inline]
    pub fn write_double(&mut self, value: f64) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    #[inline]
    pub fn write_bytes(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data);
    }

    pub fn fork(&mut self) {
        let state = State {
            len: self.buf.len(),
            buf: self.buf.clone(),
        };
        self.states.push_back(state);
        self.buf.clear();
    }

    pub fn reset(&mut self) {
        if let Some(state) = self.states.pop_back() {
            self.buf = state.buf;
        } else {
            self.buf.clear();
        }
    }

    pub fn ldelim(&mut self) {
        let fork_len = self.buf.len();
        let fork_data = self.buf.clone();
        
        // Reset to parent state
        self.reset();
        
        // Write length as varint
        self.write_varint32(fork_len as u32);
        
        // Append fork data
        if fork_len > 0 {
            self.buf.extend_from_slice(&fork_data);
        }
    }

    pub fn finish(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.buf)
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }
}
