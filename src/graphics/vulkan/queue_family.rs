use crate::graphics::vulkan::raw::{vk_structure_types, vk_structures};

#[derive(Debug, Default, Clone, Copy)]
pub struct _QueueFamily {
    pub vk_queue_family_properties: vk_structures::VkQueueFamilyProperties,
    pub physical_device_type: vk_structure_types::VkPhysicalDeviceType,
    pub vk_queue_family_index: usize,
    pub supports_surface: bool,
    pub supports_graphic: bool,
}
