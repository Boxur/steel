#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowSize {
    pub width: i32,
    pub height: i32,
}

impl Default for WindowSize {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
        }
    }
}
