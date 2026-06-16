use super::{vk_handles, vk_structures};

pub type PFNvkVoidFunction = Option<unsafe extern "C" fn()>;

pub type PFNvkCreateXlibSurfaceKHR = unsafe extern "C" fn(
    instace: vk_handles::VkInstance,
    p_create_info: *const vk_structures::VkXlibSurfaceCreateInfoKHR,
    p_allocator: *const std::ffi::c_void,
    p_surface: *mut vk_handles::VkSurfaceKHR,
) -> super::VkResult;

pub type PFNvkGetPhysicalDeviceSurfaceSupportKHR = unsafe extern "C" fn(
    physical_device: vk_handles::VkPhysicalDevice,
    queue_family_index: u32,
    surface: vk_handles::VkSurfaceKHR,
    p_supported: *mut super::VkBool32,
) -> super::VkResult;
