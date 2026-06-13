use super::raw;
use super::vk_structures;
pub type PFNvkCreateXlibSurfaceKHR = unsafe extern "C" fn(
    instace: raw::VkInstance,
    p_create_info: *const vk_structures::VkXlibSurfaceCreateInfoKHR,
    p_allocator: *const std::ffi::c_void,
    p_surface: *mut raw::VkSurfaceKHR,
) -> raw::VkResult;
