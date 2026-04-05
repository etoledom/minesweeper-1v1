#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn new(height: usize, width: usize) -> Self {
        Self { width, height }
    }
}
