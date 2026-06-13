use crate::graphics::vulkan::vk_structure_type::VkStructureType;

pub type VkInstance = *mut std::ffi::c_void;
type _VkSurfaceKHR = *mut std::ffi::c_void;
type VkResult = i32;
type PFNvkVoidFunction = Option<unsafe extern "C" fn()>;

pub const VK_SUCCESS: VkResult = 0;
pub const VK_API_VERSION_1_3: u32 = (0 << 29) | (1 << 22) | (3 << 12) | 0;

pub type _XDisplay = std::ffi::c_void;
pub type _XWindow = usize;

//pub static mut vkCreateXlibSurfaceKHR

#[repr(C)]
pub struct VkInstanceCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: u32,
    pub p_application_info: *const VkApplicationInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const std::ffi::c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const std::ffi::c_char,
}

#[repr(C)]
pub struct VkApplicationInfo {
    pub s_type: VkStructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_application_name: *const i8,
    pub application_version: u32,
    pub p_engine_name: *const i8,
    pub engine_version: u32,
    pub api_version: u32,
}

#[repr(C)]
pub struct _VkXlibSurfaceCreateInfoKHR {
    pub s_type: VkStructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: u32,
    pub dpy: *mut _XDisplay,
    pub window: _XWindow,
}

type _PFNvkCreateXlibSurfaceKHR = unsafe extern "C" fn(
    instace: VkInstance,
    p_create_info: *const _VkXlibSurfaceCreateInfoKHR,
    p_allocator: *const std::ffi::c_void,
    p_surface: *mut _VkSurfaceKHR,
);

pub struct _VulkanInstanceFns {
    create_xlib_surface: _PFNvkCreateXlibSurfaceKHR,
}

pub static mut _VK: Option<_VulkanInstanceFns> = None;

#[link(name = "vulkan")]
unsafe extern "C" {
    pub fn vkGetInstanceProcAddr(
        instance: VkInstance,
        p_name: *const std::ffi::c_char,
    ) -> PFNvkVoidFunction;

    pub fn vkCreateInstance(
        p_create_info: *const VkInstanceCreateInfo,
        p_allocator: *const std::ffi::c_void,
        p_instance: *mut VkInstance,
    ) -> VkResult;
}
