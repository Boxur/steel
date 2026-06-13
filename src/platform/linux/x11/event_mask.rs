use std::ops;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventMaskFlags {
    Empty = 0,
    KeyPress = 1 << 0,
    KeyRelease = 1 << 1,
    ButtonPress = 1 << 2,
    ButtonRelease = 1 << 3,
    EnterWindow = 1 << 4,
    LeaveWindow = 1 << 5,
    PointerMotion = 1 << 6,
    PointerMotionHint = 1 << 7,
    Button1Motion = 1 << 8,
    Button2Motion = 1 << 9,
    Button3Motion = 1 << 10,
    Button4Motion = 1 << 11,
    Button5Motion = 1 << 12,
    ButtonMotion = 1 << 13,
    KeymapState = 1 << 14,
    Exposure = 1 << 15,
    VisibilityChange = 1 << 16,
    StructureNotify = 1 << 17,
    ResizeRedirect = 1 << 18,
    SubstructureNotify = 1 << 19,
    SubstructureRedirect = 1 << 20,
    FocusChange = 1 << 21,
    PropertyChange = 1 << 22,
    ColormapChange = 1 << 23,
    OwnerGrabButton = 1 << 24,
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
