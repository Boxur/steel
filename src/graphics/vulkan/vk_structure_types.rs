use std::ops::{BitAnd, BitOr};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VkStructureType {
    #[default]
    VkStructureTypeApplicationInfo = 0,
    VkStructureTypeInstanceCreateInfo = 1,
    VkStructureTypeXlibSurfaceCreateInfoKHR = 1000004000,
    VkStructureTypePhysicalDeviceProperties2 = 1000059001,
}

#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VkPhysicalDeviceType {
    #[default]
    VkPhysicalDeviceTypeOther = 0,
    VkPhysicalDeviceTypeIntegratedGpu = 1,
    VkPhysicalDeviceTypeDiscreteGpu = 2,
    VkPhysicalDeviceTypeVirtualGpu = 3,
    VkPhysicalDeviceTypeCpu = 4,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VkQueueFlagBits {
    #[default]
    None = 0,
    VkQueueGraphicsBit = 1 << 0,
    VkQueueComputeBit = 1 << 1,
    VkQueueTransferBit = 1 << 2,
    VkQueueSparseBindingBit = 1 << 3,
    VkQueueProtectedBit = 1 << 4,
    VkQueueVideoDecodeBitKhr = 1 << 5,
    VkQueueVideoEncodeBitKhr = 1 << 6,
    VkQueueOpticalFlowBitNv = 1 << 8,
    VkQueueDataGraphBitArm = 1 << 10,
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
