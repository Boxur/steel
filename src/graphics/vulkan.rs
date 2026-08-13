mod logical_device;
mod physical_device;
mod queue_family;
mod raw;

use crate::graphics::vulkan::{
    logical_device::LogicalDevice,
    physical_device::{PhysicalDevice, PhysicalDevices},
    raw::VK_SUCCESS,
};

#[derive(Debug, Default)]
pub struct Vulkan {
    vk_instance: raw::vk_handles::VkInstance,
    vk_instance_fns: raw::vk_structures::VulkanInstanceFns,
    surface: raw::vk_handles::VkSurfaceKHR,
    physical_devices: PhysicalDevices,
    logical_device: Option<LogicalDevice>,
}

impl Vulkan {
    pub fn new(display: &*mut raw::XDisplay, window: &raw::XWindow) -> Self {
        let mut s = Self::default();
        s.create_vk_instance();
        s.create_vk_fns();
        s.create_vk_surface(*display, *window);
        s.create_physical_devices();
        s.create_logical_device();
        s
    }

    fn create_vk_instance(&mut self) {
        b"App";
        let application_info = raw::vk_structures::VkApplicationInfo::builder()
            .application_name(b"App\0")
            .engine_name(b"Steel");
        let extensions = [c"VK_KHR_surface".as_ptr(), c"VK_KHR_xlib_surface".as_ptr()];
        let layers = [c"VK_LAYER_KHRONOS_validation".as_ptr()];
        let create_instance_info = raw::vk_structures::VkInstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_extensions(&extensions)
            .enabled_layers(&layers);
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
        let surface_create_info =
            raw::vk_structures::VkXlibSurfaceCreateInfoKHR::builder(display, window);
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

    fn create_physical_devices(&mut self) {
        //let mut physical_devices = PhysicalDevices::default();
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
        let mut properties = raw::vk_structures::VkPhysicalDeviceProperties2::default();

        for d in vk_physical_devices {
            unsafe {
                raw::vkGetPhysicalDeviceProperties2(d, &raw mut properties);
            }
            self.physical_devices.add_device(PhysicalDevice {
                vk_handle: d,
                device_name: std::ffi::CStr::from_bytes_until_nul(
                    &properties.properties.get_device_name()[..],
                )
                .unwrap()
                .to_string_lossy()
                .to_string(),
                device_type: properties.properties.get_device_type(),
                graphics_queue_family_index: None,
                surface_queue_family_index: None,
            });
        }
        self.physical_devices
            .choose_device(self.vk_instance_fns, self.surface);
    }

    fn create_logical_device(&mut self) {
        self.logical_device = Some(LogicalDevice::new(
            &self.physical_devices.selected_device().unwrap(),
        ));
    }
}
