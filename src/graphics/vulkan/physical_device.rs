use super::raw;
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PhysicalDevice {
    pub vk_handle: raw::vk_handles::VkPhysicalDevice,
    pub device_name: String,
    pub device_type: raw::vk_structure_types::VkPhysicalDeviceType,
    pub graphics_queue_family_index: Option<usize>,
    pub surface_queue_family_index: Option<usize>,
}

#[derive(Debug, Default)]
pub struct PhysicalDevices {
    devices: Vec<PhysicalDevice>,
    selected_device_index: Option<usize>,
}

impl PhysicalDevices {
    pub fn add_device(&mut self, physical_device: PhysicalDevice) {
        self.devices.push(physical_device);
    }

    pub fn selected_device(&self) -> Option<&PhysicalDevice> {
        match self.selected_device_index {
            None => None,
            Some(device_index) => Some(&self.devices[device_index]),
        }
    }

    pub fn choose_device(
        &mut self,
        vk_instance_fns: raw::vk_structures::VulkanInstanceFns,
        surface: raw::vk_handles::VkSurfaceKHR,
    ) {
        let mut count = 0_u32;

        for (i, physical_device) in self.devices.iter_mut().enumerate() {
            let mut supports_graphics = false;
            let mut supports_surface = false;
            let vk_physical_device = &physical_device.vk_handle;
            let mut cur: Vec<raw::vk_structures::VkQueueFamilyProperties> = Vec::new();
            unsafe {
                raw::vkGetPhysicalDeviceQueueFamilyProperties(
                    *vk_physical_device,
                    &raw mut count,
                    core::ptr::null_mut(),
                );
                cur.resize(
                    count as usize,
                    raw::vk_structures::VkQueueFamilyProperties::default(),
                );
                raw::vkGetPhysicalDeviceQueueFamilyProperties(
                    *vk_physical_device,
                    &raw mut count,
                    cur.as_mut_ptr(),
                )
            }
            let mut queue_family_supports_surface = raw::VK_FALSE;
            for (j, &q) in cur.iter().enumerate() {
                unsafe {
                    let result = (vk_instance_fns
                        .get_physical_device_surface_support_khr
                        .unwrap())(
                        *vk_physical_device,
                        j as u32,
                        surface,
                        &raw mut queue_family_supports_surface,
                    );
                    assert_eq!(result, raw::VK_SUCCESS);
                }
                if supports_surface == false && queue_family_supports_surface == raw::VK_TRUE {
                    supports_surface = true;
                    physical_device.surface_queue_family_index = Some(j);
                }
                if supports_graphics == false
                    && raw::vk_structure_types::VkQueueFlagBits::VkQueueGraphicsBit & q.queue_flags
                        == raw::vk_structure_types::VkQueueFlags(1)
                {
                    supports_graphics = true;
                    physical_device.graphics_queue_family_index = Some(j);
                }
                if supports_surface && supports_graphics {
                    self.selected_device_index = Some(i);
                    return;
                }
            }
        }
        panic!("Correct device not found");
    }
}
