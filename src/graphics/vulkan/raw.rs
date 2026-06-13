use super::vk_structures;
pub type VkInstance = *mut std::ffi::c_void;
pub type VkSurfaceKHR = *mut std::ffi::c_void;
pub type VkResult = i32;
pub type PFNvkVoidFunction = Option<unsafe extern "C" fn()>;

pub const VK_SUCCESS: VkResult = 0;
pub const VK_API_VERSION_1_3: u32 = (0 << 29) | (1 << 22) | (3 << 12) | 0;

pub type XDisplay = std::ffi::c_void;
pub type XWindow = usize;

//pub static mut vkCreateXlibSurfaceKHR

pub static mut VK: Option<vk_structures::VulkanInstanceFns> = None;

#[link(name = "vulkan")]
unsafe extern "C" {
    pub fn vkGetInstanceProcAddr(
        instance: VkInstance,
        p_name: *const std::ffi::c_char,
    ) -> PFNvkVoidFunction;

    pub fn vkCreateInstance(
        p_create_info: *const vk_structures::VkInstanceCreateInfo,
        p_allocator: *const std::ffi::c_void,
        p_instance: *mut VkInstance,
    ) -> VkResult;
}
