#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MousePos {
    pub x: i32,
    pub y: i32,
}

impl Default for MousePos {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}
