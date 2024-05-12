use core::usize;

const BIG_STRIDE: usize = 0xFFFF;

#[derive(Clone, Copy)]
pub struct StrideScheduler {
    pub stride: usize,
    priority: isize,
}

impl StrideScheduler {
    pub fn new() -> Self {
        Self {
            stride: 0,
            priority: 16,
        }
    }
    pub fn set_priority(&mut self, p: isize) {
        self.priority = p;
    }
    pub fn step(&mut self) {
        self.stride += BIG_STRIDE / self.priority as usize;
    }
}
