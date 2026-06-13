use super::{raw, vk_pfn_types, vk_structure_type::VkStructureType};

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
pub struct VkXlibSurfaceCreateInfoKHR {
    pub s_type: VkStructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: u32,
    pub dpy: *mut raw::XDisplay,
    pub window: raw::XWindow,
}

#[derive(Debug, Clone, Copy)]
pub struct VulkanInstanceFns {
    pub create_xlib_surface: vk_pfn_types::PFNvkCreateXlibSurfaceKHR,
}
