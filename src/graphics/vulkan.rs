pub mod raw;
mod vk_structure_type;

use crate::graphics::vulkan::{
    raw::{VK_API_VERSION_1_3, VK_SUCCESS},
    vk_structure_type::VkStructureType,
};

pub fn init() {
    let application_info = raw::VkApplicationInfo {
        s_type: VkStructureType::VkStructureTypeApplicationInfo,
        p_next: core::ptr::null(),
        p_application_name: c"App".as_ptr(),
        application_version: 0,
        p_engine_name: c"Steel".as_ptr(),
        engine_version: 0,
        api_version: VK_API_VERSION_1_3,
    };
    let extensions = [c"VK_KHR_surface".as_ptr(), c"VK_KHR_xlib_surface".as_ptr()];
    let layers = [c"VK_LAYER_KHRONOS_validation".as_ptr()];
    let create_instance_info = raw::VkInstanceCreateInfo {
        s_type: VkStructureType::VkStructureTypeInstanceCreateInfo,
        p_next: core::ptr::null(),
        flags: 0,
        p_application_info: &raw const application_info,
        enabled_extension_count: extensions.len() as u32,
        pp_enabled_extension_names: extensions.as_ptr(),
        enabled_layer_count: layers.len() as u32,
        pp_enabled_layer_names: layers.as_ptr(),
    };
    let mut instance: raw::VkInstance = core::ptr::null_mut();
    unsafe {
        let result = raw::vkCreateInstance(
            &raw const create_instance_info,
            core::ptr::null(),
            &raw mut instance,
        );
        assert_eq!(result, VK_SUCCESS);
    }

    let name = c"vkCreateXlibSurfaceKHR";
    unsafe {
        let func = raw::vkGetInstanceProcAddr(instance, name.as_ptr()).unwrap();
        // raw::VK = Some(VulkanInstanceFns {
        //     create_xlib_surface: vk_create_xlib_surface,
        // });
    }
}
