use super::*;

#[derive(Debug)]
pub struct WindowData {
    pub key_states: KeyStates,
    pub button_states: ButtonStates,
    pub mouse_position: MousePos,
    pub window_size: WindowSize,
}
