type Atom = u64;
pub struct Atoms {
    pub wm_delete_window: Atom,
}

impl Default for Atoms {
    fn default() -> Self {
        Atoms {
            wm_delete_window: 0,
        }
    }
}
