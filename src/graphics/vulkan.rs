pub mod raw;
mod vk_pfn_types;
mod vk_structure_type;
mod vk_structures;

use crate::graphics::vulkan::{
    raw::{VK_API_VERSION_1_3, VK_SUCCESS},
    vk_structure_type::VkStructureType,
};

fn create_vk_instance() -> raw::VkInstance {
    let application_info = vk_structures::VkApplicationInfo {
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
    let create_instance_info = vk_structures::VkInstanceCreateInfo {
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
    instance
}

fn create_vk_surface(
    instance: raw::VkInstance,
    display: *mut raw::XDisplay,
    window: raw::XWindow,
) -> raw::VkSurfaceKHR {
    let name = c"vkCreateXlibSurfaceKHR";
    unsafe {
        let func = raw::vkGetInstanceProcAddr(instance, name.as_ptr()).unwrap();
        raw::VK = Some(vk_structures::VulkanInstanceFns {
            create_xlib_surface: *(&raw const func
                as *const vk_pfn_types::PFNvkCreateXlibSurfaceKHR),
        });
    }
    let surface_create_info = vk_structures::VkXlibSurfaceCreateInfoKHR {
        s_type: VkStructureType::VkStructureTypeXlibSurfaceCreateInfoKHR,
        p_next: core::ptr::null(),
        flags: 0,
        dpy: display,
        window: window,
    };
    let mut surface: raw::VkSurfaceKHR = core::ptr::null_mut();
    unsafe {
        let result = (raw::VK.unwrap().create_xlib_surface)(
            instance,
            &raw const surface_create_info,
            core::ptr::null(),
            &raw mut surface,
        );
        assert_eq!(result, VK_SUCCESS);
    }
    surface
}

pub fn init(display: *mut raw::XDisplay, window: raw::XWindow) {
    let instance = create_vk_instance();
    let _surface = create_vk_surface(instance, display, window);
}
