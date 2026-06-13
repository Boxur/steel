#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkStructureType {
    VkStructureTypeApplicationInfo = 0,
    VkStructureTypeInstanceCreateInfo = 1,
    _VkStructureTypeXlibSurfaceCreateInfoKHR = 1000004000,
}
