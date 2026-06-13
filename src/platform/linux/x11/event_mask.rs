use std::ops;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventMaskFlags {
    empty = 0,
    key_press = 1 << 0,
    key_release = 1 << 1,
    button_press = 1 << 2,
    button_release = 1 << 3,
    enter_window = 1 << 4,
    leave_window = 1 << 5,
    pointer_motion = 1 << 6,
    pointer_motion_hint = 1 << 7,
    button_1_motion = 1 << 8,
    button_2_motion = 1 << 9,
    button_3_motion = 1 << 10,
    button_4_motion = 1 << 11,
    button_5_motion = 1 << 12,
    button_motion = 1 << 13,
    keymap_state = 1 << 14,
    exposure = 1 << 15,
    visibility_change = 1 << 16,
    structure_notify = 1 << 17,
    resize_redirect = 1 << 18,
    substructure_notify = 1 << 19,
    substructure_redirect = 1 << 20,
    focus_change = 1 << 21,
    property_change = 1 << 22,
    colormap_change = 1 << 23,
    owner_grab_button = 1 << 24,
}

pub struct EventMask(i32);

impl ops::BitOr for EventMaskFlags {
    type Output = EventMask;
    fn bitor(self, rhs: Self) -> Self::Output {
        EventMask(self as i32 | rhs as i32)
    }
}

impl ops::BitOr<EventMaskFlags> for EventMask {
    type Output = EventMask;
    fn bitor(self, rhs: EventMaskFlags) -> Self::Output {
        EventMask(self.0 | rhs as i32)
    }
}

impl From<EventMask> for isize {
    fn from(value: EventMask) -> Self {
        value.0 as isize
    }
}
