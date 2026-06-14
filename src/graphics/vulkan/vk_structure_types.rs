#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkStructureType {
    VkStructureTypeApplicationInfo = 0,
    VkStructureTypeInstanceCreateInfo = 1,
    VkStructureTypeXlibSurfaceCreateInfoKHR = 1000004000,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkPhysicalDeviceType {
    VkPhysicalDeviceTypeOther = 0,
    VkPhysicalDeviceTypeIntegratedGpu = 1,
    VkPhysicalDeviceTypeDiscreteGpu = 2,
    VkPhysicalDeviceTypeVirtualGpu = 3,
    VkPhysicalDeviceTypeCpu = 4,
}
