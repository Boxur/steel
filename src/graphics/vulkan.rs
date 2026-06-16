mod physical_device;
mod queue_family;
mod raw;

use crate::graphics::vulkan::{
    physical_device::{PhysicalDevice, PhysicalDevices},
    raw::{VK_API_VERSION_1_3, VK_SUCCESS},
};

#[derive(Debug, Default)]
pub struct Vulkan {
    vk_instance: raw::vk_handles::VkInstance,
    vk_instance_fns: raw::vk_structures::VulkanInstanceFns,
    surface: raw::vk_handles::VkSurfaceKHR,
    devices: PhysicalDevices,
}

impl Vulkan {
    pub fn new(display: &*mut raw::XDisplay, window: &raw::XWindow) -> Self {
        let mut s = Self::default();
        s.create_vk_instance();
        s.create_vk_fns();
        s.create_vk_surface(*display, *window);
        s.create_devices();
        dbg!(&s.devices);
        s
    }

    fn create_vk_instance(&mut self) {
        let application_info = raw::vk_structures::VkApplicationInfo {
            s_type: raw::vk_structure_types::VkStructureType::VkStructureTypeApplicationInfo,
            p_next: core::ptr::null(),
            p_application_name: c"App".as_ptr(),
            application_version: 0,
            p_engine_name: c"Steel".as_ptr(),
            engine_version: 0,
            api_version: VK_API_VERSION_1_3,
        };
        let extensions = [c"VK_KHR_surface".as_ptr(), c"VK_KHR_xlib_surface".as_ptr()];
        let layers = [c"VK_LAYER_KHRONOS_validation".as_ptr()];
        let create_instance_info = raw::vk_structures::VkInstanceCreateInfo {
            s_type: raw::vk_structure_types::VkStructureType::VkStructureTypeInstanceCreateInfo,
            p_next: core::ptr::null(),
            flags: 0,
            p_application_info: &raw const application_info,
            enabled_extension_count: extensions.len() as u32,
            pp_enabled_extension_names: extensions.as_ptr(),
            enabled_layer_count: layers.len() as u32,
            pp_enabled_layer_names: layers.as_ptr(),
        };
        let mut instance: raw::vk_handles::VkInstance = core::ptr::null_mut();
        unsafe {
            let result = raw::vkCreateInstance(
                &raw const create_instance_info,
                core::ptr::null(),
                &raw mut instance,
            );
            assert_eq!(result, VK_SUCCESS);
        }
        self.vk_instance = instance;
    }

    fn create_vk_fns(&mut self) {
        unsafe {
            let create_xlib_surface =
                raw::vkGetInstanceProcAddr(self.vk_instance, c"vkCreateXlibSurfaceKHR".as_ptr())
                    .unwrap();
            let get_physical_device_surface_support_khr = raw::vkGetInstanceProcAddr(
                self.vk_instance,
                c"vkGetPhysicalDeviceSurfaceSupportKHR".as_ptr(),
            )
            .unwrap();
            self.vk_instance_fns = raw::vk_structures::VulkanInstanceFns {
                create_xlib_surface: Some(
                    *(&raw const create_xlib_surface
                        as *const raw::vk_pfn_types::PFNvkCreateXlibSurfaceKHR),
                ),
                get_physical_device_surface_support_khr: Some(
                    *(&raw const get_physical_device_surface_support_khr
                        as *const raw::vk_pfn_types::PFNvkGetPhysicalDeviceSurfaceSupportKHR),
                ),
            };
        }
    }

    fn create_vk_surface(&mut self, display: *mut raw::XDisplay, window: raw::XWindow) {
        let surface_create_info = raw::vk_structures::VkXlibSurfaceCreateInfoKHR {
            s_type:
                raw::vk_structure_types::VkStructureType::VkStructureTypeXlibSurfaceCreateInfoKHR,
            p_next: core::ptr::null(),
            flags: 0,
            dpy: display,
            window: window,
        };
        let mut surface: raw::vk_handles::VkSurfaceKHR = core::ptr::null_mut();
        unsafe {
            let result = (self.vk_instance_fns.create_xlib_surface.unwrap())(
                self.vk_instance,
                &raw const surface_create_info,
                core::ptr::null(),
                &raw mut surface,
            );
            assert_eq!(result, VK_SUCCESS);
        }
        self.surface = surface;
    }

    fn create_devices(&mut self) {
        let mut physical_devices = PhysicalDevices::default();
        let mut count = 0u32;

        let mut vk_physical_devices: Vec<raw::vk_handles::VkPhysicalDevice> = Vec::new();
        unsafe {
            let result = raw::vkEnumeratePhysicalDevices(
                self.vk_instance,
                &raw mut count,
                core::ptr::null_mut(),
            );
            assert_eq!(result, VK_SUCCESS);
            vk_physical_devices.resize(count as usize, core::ptr::null_mut());
            let result = raw::vkEnumeratePhysicalDevices(
                self.vk_instance,
                &raw mut count,
                vk_physical_devices.as_mut_ptr(),
            );
            assert_eq!(result, VK_SUCCESS);
        }
        let mut properties = raw::vk_structures::VkPhysicalDeviceProperties2 {
            s_type:
                raw::vk_structure_types::VkStructureType::VkStructureTypePhysicalDeviceProperties2,
            p_next: core::ptr::null(),
            properties: raw::vk_structures::VkPhysicalDeviceProperties {
                api_version: 0,
                driver_version: 0,
                vendor_id: 0,
                device_id: 0,
                device_type: raw::vk_structure_types::VkPhysicalDeviceType::default(),
                device_name: [0; 256],
                pipeline_cache_uuid: [0; 16],
                limits: raw::vk_structures::VkPhysicalDeviceLimits::default(),
                sparse_properties: raw::vk_structures::VkPhysicalDeviceSparseProperties::default(),
            },
        };

        for d in vk_physical_devices {
            unsafe {
                raw::vkGetPhysicalDeviceProperties2(d, &raw mut properties);
            }
            physical_devices.add_device(PhysicalDevice {
                vk_handle: d,
                device_name: std::ffi::CStr::from_bytes_until_nul(
                    &properties.properties.device_name[..],
                )
                .unwrap()
                .to_string_lossy()
                .to_string(),
                device_type: properties.properties.device_type,
                graphics_queue_family_index: None,
                surface_queue_family_index: None,
            });
        }
        self.devices = physical_devices;
        self.devices
            .choose_device(self.vk_instance_fns, self.surface);
    }
}
