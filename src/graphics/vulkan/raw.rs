pub mod vk_handles;
pub mod vk_pfn_types;
pub mod vk_structure_types;
pub mod vk_structures;

pub type VkResult = i32;
pub type VkDeviceSize = u64;
pub type VkFlags = u32;
pub type VkSampleCountFlags = VkFlags;

pub type VkBool32 = u32;
pub const VK_FALSE: VkBool32 = 0;
pub const VK_TRUE: VkBool32 = 0;

pub const VK_SUCCESS: VkResult = 0;
pub const VK_API_VERSION_1_3: u32 = (0 << 29) | (1 << 22) | (3 << 12) | 0;

pub type XDisplay = std::ffi::c_void;
pub type XWindow = usize;

//pub static mut vkCreateXlibSurfaceKHR

#[link(name = "vulkan")]
unsafe extern "C" {
    pub fn vkGetInstanceProcAddr(
        instance: vk_handles::VkInstance,
        p_name: *const std::ffi::c_char,
    ) -> vk_pfn_types::PFNvkVoidFunction;

    pub fn vkCreateInstance(
        p_create_info: *const vk_structures::VkInstanceCreateInfo,
        p_allocator: *const std::ffi::c_void,
        p_instance: *mut vk_handles::VkInstance,
    ) -> VkResult;

    pub fn vkEnumeratePhysicalDevices(
        instance: vk_handles::VkInstance,
        p_physical_device_count: *mut u32,
        p_physical_devices: *mut vk_handles::VkPhysicalDevice,
    ) -> VkResult;

    pub fn vkGetPhysicalDeviceProperties2(
        physical_device: vk_handles::VkPhysicalDevice,
        p_properties: *mut vk_structures::VkPhysicalDeviceProperties2,
    );

    pub fn vkGetPhysicalDeviceQueueFamilyProperties(
        physical_device: vk_handles::VkPhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut vk_structures::VkQueueFamilyProperties,
    );

    pub fn vkCreateDevice(
        physical_device: vk_handles::VkPhysicalDevice,
        p_create_info: *const vk_structures::VkDeviceCreateInfo,
        p_allocator: *const std::ffi::c_void,
        p_device: *mut vk_handles::VkDevice,
    ) -> VkResult;

    pub fn vkGetDeviceQueue(
        device: vk_handles::VkDevice,
        queue_family_index: u32,
        queue_index: u32,
        p_queue: *mut vk_handles::VkQueue,
    ) -> VkResult;
}
