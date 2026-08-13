use std::ops::{BitAnd, BitOr};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VkStructureType {
    #[default]
    ApplicationInfo = 0,
    InstanceCreateInfo = 1,
    DeviceQueueCreateInfo = 2,
    DeviceCreateInfo = 3,
    XlibSurfaceCreateInfoKHR = 1000004000,
    PhysicalDeviceProperties2 = 1000059001,
}

#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VkPhysicalDeviceType {
    #[default]
    Other = 0,
    IntegratedGpu = 1,
    DiscreteGpu = 2,
    VirtualGpu = 3,
    Cpu = 4,
}

#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VkQueueFlagBits {
    #[default]
    None = 0,
    GraphicsBit = 1 << 0,
    ComputeBit = 1 << 1,
    TransferBit = 1 << 2,
    SparseBindingBit = 1 << 3,
    ProtectedBit = 1 << 4,
    VideoDecodeBitKhr = 1 << 5,
    VideoEncodeBitKhr = 1 << 6,
    OpticalFlowBitNv = 1 << 8,
    DataGraphBitArm = 1 << 10,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkQueueFlags(pub u32);

impl BitOr for VkQueueFlagBits {
    type Output = VkQueueFlags;
    fn bitor(self, rhs: Self) -> Self::Output {
        VkQueueFlags(self as u32 | rhs as u32)
    }
}

impl BitOr<VkQueueFlags> for VkQueueFlagBits {
    type Output = VkQueueFlags;
    fn bitor(self, rhs: VkQueueFlags) -> Self::Output {
        VkQueueFlags(self as u32 | rhs.0)
    }
}

impl BitOr<VkQueueFlagBits> for VkQueueFlags {
    type Output = VkQueueFlags;
    fn bitor(self, rhs: VkQueueFlagBits) -> Self::Output {
        VkQueueFlags(self.0 | rhs as u32)
    }
}

impl BitAnd for VkQueueFlagBits {
    type Output = VkQueueFlags;
    fn bitand(self, rhs: Self) -> Self::Output {
        VkQueueFlags(self as u32 & rhs as u32)
    }
}

impl BitAnd<VkQueueFlags> for VkQueueFlagBits {
    type Output = VkQueueFlags;
    fn bitand(self, rhs: VkQueueFlags) -> Self::Output {
        VkQueueFlags(self as u32 & rhs.0)
    }
}

impl BitAnd<VkQueueFlagBits> for VkQueueFlags {
    type Output = VkQueueFlags;
    fn bitand(self, rhs: VkQueueFlagBits) -> Self::Output {
        VkQueueFlags(self.0 & rhs as u32)
    }
}

#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VkDeviceQueueCreateFlagBits {
    #[default]
    None = 0,
    ProtectedBit = 1 << 0,
    InternallySynchronizedBitKhr = 1 << 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkDeviceQueueCreateFlags(pub u32);

impl BitOr for VkDeviceQueueCreateFlagBits {
    type Output = VkDeviceQueueCreateFlags;
    fn bitor(self, rhs: Self) -> Self::Output {
        VkDeviceQueueCreateFlags(self as u32 | rhs as u32)
    }
}

impl BitOr<VkDeviceQueueCreateFlags> for VkDeviceQueueCreateFlagBits {
    type Output = VkDeviceQueueCreateFlags;
    fn bitor(self, rhs: VkDeviceQueueCreateFlags) -> Self::Output {
        VkDeviceQueueCreateFlags(self as u32 | rhs.0)
    }
}

impl BitOr<VkDeviceQueueCreateFlagBits> for VkDeviceQueueCreateFlags {
    type Output = VkDeviceQueueCreateFlags;
    fn bitor(self, rhs: VkDeviceQueueCreateFlagBits) -> Self::Output {
        VkDeviceQueueCreateFlags(self.0 | rhs as u32)
    }
}

impl BitAnd for VkDeviceQueueCreateFlagBits {
    type Output = VkDeviceQueueCreateFlags;
    fn bitand(self, rhs: Self) -> Self::Output {
        VkDeviceQueueCreateFlags(self as u32 & rhs as u32)
    }
}

impl BitAnd<VkDeviceQueueCreateFlags> for VkDeviceQueueCreateFlagBits {
    type Output = VkDeviceQueueCreateFlags;
    fn bitand(self, rhs: VkDeviceQueueCreateFlags) -> Self::Output {
        VkDeviceQueueCreateFlags(self as u32 & rhs.0)
    }
}

impl BitAnd<VkDeviceQueueCreateFlagBits> for VkDeviceQueueCreateFlags {
    type Output = VkDeviceQueueCreateFlags;
    fn bitand(self, rhs: VkDeviceQueueCreateFlagBits) -> Self::Output {
        VkDeviceQueueCreateFlags(self.0 & rhs as u32)
    }
}

impl From<VkDeviceQueueCreateFlagBits> for VkDeviceQueueCreateFlags {
    fn from(value: VkDeviceQueueCreateFlagBits) -> Self {
        VkDeviceQueueCreateFlags(value as u32)
    }
}
