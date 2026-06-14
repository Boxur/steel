mod raw;
mod vk_handles;
mod vk_pfn_types;
mod vk_structure_types;
mod vk_structures;

use crate::graphics::vulkan::{
    raw::{VK_API_VERSION_1_3, VK_SUCCESS},
    vk_structure_types::VkStructureType,
};

fn get_vk_functions(instance: vk_handles::VkInstance) {
    unsafe {
        let create_xlib_surface =
            raw::vkGetInstanceProcAddr(instance, c"vkCreateXlibSurfaceKHR".as_ptr()).unwrap();
        let enumerate_physical_devices =
            raw::vkGetInstanceProcAddr(instance, c"vkEnumeratePhysicalDevices".as_ptr()).unwrap();
        let get_physical_device_properties2 =
            raw::vkGetInstanceProcAddr(instance, c"vkGetPhysicalDeviceProperties2".as_ptr())
                .unwrap();
        let get_physical_device_queue_family_properties = raw::vkGetInstanceProcAddr(
            instance,
            c"vkGetPhysicalDeviceQueueFamilyProperties".as_ptr(),
        )
        .unwrap();
        let get_physical_device_surface_support_khr =
            raw::vkGetInstanceProcAddr(instance, c"vkGetPhysicalDeviceSurfaceSupportKHR".as_ptr())
                .unwrap();
        raw::VK = Some(vk_structures::VulkanInstanceFns {
            create_xlib_surface: *(&raw const create_xlib_surface
                as *const vk_pfn_types::PFNvkCreateXlibSurfaceKHR),
            enumerate_physical_devices: *(&raw const enumerate_physical_devices
                as *const vk_pfn_types::PFNvkEnumeratePhysicalDevices),
            _get_physical_device_properties2: *(&raw const get_physical_device_properties2
                as *const vk_pfn_types::PFNvkGetPhysicalDeviceProperties2),
            get_physical_device_queue_family_properties:
                *(&raw const get_physical_device_queue_family_properties
                    as *const vk_pfn_types::PFNvkGetPhysicalDeviceQueueFamilyProperties),
            get_physical_device_surface_support_khr:
                *(&raw const get_physical_device_surface_support_khr
                    as *const vk_pfn_types::PFNvkGetPhysicalDeviceSurfaceSupportKHR),
        });
    }
}

fn create_vk_instance() -> vk_handles::VkInstance {
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
    let mut instance: vk_handles::VkInstance = core::ptr::null_mut();
    unsafe {
        let result = raw::vkCreateInstance(
            &raw const create_instance_info,
            core::ptr::null(),
            &raw mut instance,
        );
        assert_eq!(result, VK_SUCCESS);
    }
    get_vk_functions(instance);
    instance
}

fn create_vk_surface(
    instance: vk_handles::VkInstance,
    display: *mut raw::XDisplay,
    window: raw::XWindow,
) -> vk_handles::VkSurfaceKHR {
    let surface_create_info = vk_structures::VkXlibSurfaceCreateInfoKHR {
        s_type: VkStructureType::VkStructureTypeXlibSurfaceCreateInfoKHR,
        p_next: core::ptr::null(),
        flags: 0,
        dpy: display,
        window: window,
    };
    let mut surface: vk_handles::VkSurfaceKHR = core::ptr::null_mut();
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

fn create_vk_devices(instance: vk_handles::VkInstance) -> Vec<vk_handles::VkPhysicalDevice> {
    let mut count = 0u32;

    let mut devices: Vec<vk_handles::VkPhysicalDevice> = Vec::new();
    unsafe {
        let result = (raw::VK.unwrap().enumerate_physical_devices)(
            instance,
            &raw mut count,
            core::ptr::null_mut(),
        );
        assert_eq!(result, VK_SUCCESS);
        devices.resize(count as usize, core::ptr::null_mut());
        let result = (raw::VK.unwrap().enumerate_physical_devices)(
            instance,
            &raw mut count,
            devices.as_mut_ptr(),
        );
        assert_eq!(result, VK_SUCCESS);

        let devices = Vec::from(devices);
        devices
    }
}

fn get_valid_queues(
    devices: &Vec<vk_handles::VkPhysicalDevice>,
    surface: vk_handles::VkSurfaceKHR,
) -> Vec<vk_structures::VkQueueFamilyProperties> {
    let mut count = 0_u32;
    let mut queues: Vec<vk_structures::VkQueueFamilyProperties> = Vec::new();

    for device in &*devices {
        let mut cur: Vec<vk_structures::VkQueueFamilyProperties> = Vec::new();
        unsafe {
            (raw::VK.unwrap().get_physical_device_queue_family_properties)(
                *device,
                &raw mut count,
                core::ptr::null_mut(),
            );
            cur.resize(
                count as usize,
                vk_structures::VkQueueFamilyProperties::default(),
            );
            (raw::VK.unwrap().get_physical_device_queue_family_properties)(
                *device,
                &raw mut count,
                cur.as_mut_ptr(),
            )
        }
        let mut supports_surface = raw::VkFalse;
        for (index, &q) in cur.iter().enumerate() {
            unsafe {
                let result = (raw::VK.unwrap().get_physical_device_surface_support_khr)(
                    *device,
                    index as u32,
                    surface,
                    &raw mut supports_surface,
                );
                assert_eq!(result, raw::VK_SUCCESS);
            }
            if vk_structure_types::VkQueueFlagBits::VkQueueGraphicsBit & q.queue_flags
                == vk_structure_types::VkQueueFlags(1)
                && supports_surface == raw::VkTrue
            {
                queues.push(q);
            }
        }
    }
    queues
}

pub fn init(display: *mut raw::XDisplay, window: raw::XWindow) {
    let instance = create_vk_instance();
    let surface = create_vk_surface(instance, display, window);
    let devices = create_vk_devices(instance);
    let queues = get_valid_queues(&devices, surface);
    dbg!(&queues);
}
