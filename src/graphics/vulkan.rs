mod logical_device;
mod physical_device;
mod queue;
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
    logical_device: LogicalDevice,
    queues: queue::Queues,
}

impl Vulkan {
    pub fn new(display: &*mut raw::XDisplay, window: &raw::XWindow) -> Self {
        let mut vulkan = Self::default();
        vulkan
            .create_vk_instance()
            .create_vk_fns()
            .create_vk_surface(*display, *window)
            .create_physical_devices()
            .create_logical_device()
            .create_queues();
        dbg!(&vulkan);
        vulkan
    }

    fn create_vk_instance(&mut self) -> &mut Self {
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
        self
    }

    fn create_vk_fns(&mut self) -> &mut Self {
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
        self
    }

    fn create_vk_surface(
        &mut self,
        display: *mut raw::XDisplay,
        window: raw::XWindow,
    ) -> &mut Self {
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
        self
    }

    fn create_physical_devices(&mut self) -> &mut Self {
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
        self
    }

    fn create_logical_device(&mut self) -> &mut Self {
        self.logical_device = LogicalDevice::new(&self.physical_devices.selected_device().unwrap());
        self
    }

    fn create_queues(&mut self) -> &mut Self {
        self.queues = queue::Queues::new(
            self.physical_devices.selected_device().unwrap(),
            &self.logical_device,
        );
        self
    }
}
