type XAtom = usize;
#[derive(Debug, Clone)]
pub struct XAtoms {
    pub wm_delete_window: XAtom,
}

impl Default for XAtoms {
    fn default() -> Self {
        XAtoms {
            wm_delete_window: 0,
        }
    }
}
