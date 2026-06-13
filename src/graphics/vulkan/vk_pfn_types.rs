use super::{raw, vk_handles, vk_structures};

pub type PFNvkVoidFunction = Option<unsafe extern "C" fn()>;

pub type PFNvkCreateXlibSurfaceKHR = unsafe extern "C" fn(
    instace: vk_handles::VkInstance,
    p_create_info: *const vk_structures::VkXlibSurfaceCreateInfoKHR,
    p_allocator: *const std::ffi::c_void,
    p_surface: *mut vk_handles::VkSurfaceKHR,
) -> raw::VkResult;

pub type PFNvkEnumeratePhysicalDevices = unsafe extern "C" fn(
    instance: vk_handles::VkInstance,
    p_physical_device_count: *mut u32,
    p_physical_devices: *mut vk_handles::VkPhysicalDevice,
) -> raw::VkResult;
