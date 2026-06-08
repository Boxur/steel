use std::ops;

pub struct EventMask {
    bits: i64,
}

impl EventMask {
    pub fn empty() -> EventMask {
        EventMask { bits: 0 }
    }

    pub fn key_press() -> EventMask {
        EventMask { bits: 1 << 0 }
    }

    pub fn key_release() -> EventMask {
        EventMask { bits: 1 << 1 }
    }

    pub fn button_press() -> EventMask {
        EventMask { bits: 1 << 2 }
    }

    pub fn button_release() -> EventMask {
        EventMask { bits: 1 << 3 }
    }

    pub fn enter_window() -> EventMask {
        EventMask { bits: 1 << 4 }
    }

    pub fn leave_window() -> EventMask {
        EventMask { bits: 1 << 5 }
    }

    pub fn pointer_motion() -> EventMask {
        EventMask { bits: 1 << 6 }
    }

    pub fn pointer_motion_hint() -> EventMask {
        EventMask { bits: 1 << 7 }
    }

    pub fn button_1_motion() -> EventMask {
        EventMask { bits: 1 << 8 }
    }

    pub fn button_2_motion() -> EventMask {
        EventMask { bits: 1 << 9 }
    }

    pub fn button_3_motion() -> EventMask {
        EventMask { bits: 1 << 10 }
    }

    pub fn button_4_motion() -> EventMask {
        EventMask { bits: 1 << 11 }
    }

    pub fn button_5_motion() -> EventMask {
        EventMask { bits: 1 << 12 }
    }

    pub fn button_motion() -> EventMask {
        EventMask { bits: 1 << 13 }
    }

    pub fn keymap_state() -> EventMask {
        EventMask { bits: 1 << 14 }
    }

    pub fn exposure() -> EventMask {
        EventMask { bits: 1 << 15 }
    }

    pub fn visibility_change() -> EventMask {
        EventMask { bits: 1 << 16 }
    }

    pub fn structure_notify() -> EventMask {
        EventMask { bits: 1 << 17 }
    }

    pub fn resize_redirect() -> EventMask {
        EventMask { bits: 1 << 18 }
    }

    pub fn substructure_notify() -> EventMask {
        EventMask { bits: 1 << 19 }
    }

    pub fn substructure_redirect() -> EventMask {
        EventMask { bits: 1 << 20 }
    }

    pub fn focus_change() -> EventMask {
        EventMask { bits: 1 << 21 }
    }

    pub fn property_change() -> EventMask {
        EventMask { bits: 1 << 22 }
    }

    pub fn colormap_change() -> EventMask {
        EventMask { bits: 1 << 23 }
    }

    pub fn owner_grab_button() -> EventMask {
        EventMask { bits: 1 << 24 }
    }
}

impl ops::BitOr for EventMask {
    type Output = EventMask;
    fn bitor(self, rhs: Self) -> Self::Output {
        EventMask {
            bits: self.bits | rhs.bits,
        }
    }
}

impl From<EventMask> for i64 {
    fn from(value: EventMask) -> Self {
        value.bits
    }
}
