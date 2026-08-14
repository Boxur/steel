use crate::graphics::vulkan::{
    physical_device::PhysicalDevice,
    raw::{self, vk_structures},
};

use super::raw::vk_handles;

#[derive(Debug, Default)]
struct LogicalDeviceData {
    queue_count: u32,
    graphics_queue_priorities: Vec<f32>,
    surface_queue_priorities: Vec<f32>,
    queue_create_infos: Vec<vk_structures::VkDeviceQueueCreateInfo>,
    extensions: Vec<*const i8>,
}

#[derive(Debug, Default)]
pub struct LogicalDevice {
    vk_device: vk_handles::VkDevice,
}

impl LogicalDevice {
    pub fn new(physical_device: &PhysicalDevice) -> LogicalDevice {
        let mut logical_device = LogicalDevice::default();
        let (create_info, logical_create_data) =
            logical_device.generate_create_info(physical_device);
        unsafe {
            raw::vkCreateDevice(
                physical_device.vk_handle,
                &raw const create_info,
                core::ptr::null(),
                &raw mut logical_device.vk_device,
            );
        }
        std::mem::drop(logical_create_data); //manually destruct late to make sure it isnt deleted before creation of logical device
        logical_device
    }

    fn generate_create_info(
        &mut self,
        physical_device: &PhysicalDevice,
    ) -> (vk_structures::VkDeviceCreateInfo, LogicalDeviceData) {
        if physical_device.graphics_queue_family_index.unwrap()
            == physical_device.surface_queue_family_index.unwrap()
        {
            return self.generate_single_queue_create_info(
                physical_device.graphics_queue_family_index.unwrap(),
            );
        }
        self.generate_multiple_queue_create_info(
            physical_device.graphics_queue_family_index.unwrap(),
            physical_device.surface_queue_family_index.unwrap(),
        )
    }

    fn generate_single_queue_create_info(
        &mut self,
        queue_family_index: u32,
    ) -> (vk_structures::VkDeviceCreateInfo, LogicalDeviceData) {
        let mut logical_device_data = LogicalDeviceData::default();
        logical_device_data.queue_count = 1;
        logical_device_data.graphics_queue_priorities = vec![1.0_f32];
        logical_device_data.queue_create_infos = vec![
            vk_structures::VkDeviceQueueCreateInfo::builder(
                queue_family_index,
                &logical_device_data.graphics_queue_priorities,
                1,
            ),
            vk_structures::VkDeviceQueueCreateInfo::default(),
        ];

        (
            vk_structures::VkDeviceCreateInfo::builder().queue_create_info(
                &logical_device_data.queue_create_infos,
                logical_device_data.queue_count,
            ),
            logical_device_data,
        )
    }

    fn generate_multiple_queue_create_info(
        &mut self,
        graphics_queue_family_index: u32,
        surface_queue_family_index: u32,
    ) -> (vk_structures::VkDeviceCreateInfo, LogicalDeviceData) {
        let mut logical_device_data = LogicalDeviceData::default();
        logical_device_data.extensions = vec![c"VK_KHR_swapchain".as_ptr()];
        logical_device_data.queue_count = 2;
        logical_device_data.graphics_queue_priorities = vec![1.0_f32];
        logical_device_data.surface_queue_priorities = vec![1.0_f32];
        logical_device_data.queue_create_infos = vec![
            vk_structures::VkDeviceQueueCreateInfo::builder(
                graphics_queue_family_index,
                &logical_device_data.graphics_queue_priorities,
                1,
            ),
            vk_structures::VkDeviceQueueCreateInfo::builder(
                surface_queue_family_index,
                &logical_device_data.surface_queue_priorities,
                1,
            ),
        ];

        (
            vk_structures::VkDeviceCreateInfo::builder()
                .queue_create_info(
                    &logical_device_data.queue_create_infos,
                    logical_device_data.queue_count,
                )
                .enabled_extensions(&logical_device_data.extensions),
            logical_device_data,
        )
    }

    pub fn get_vk_device(&self) -> vk_handles::VkDevice {
        self.vk_device
    }
}
