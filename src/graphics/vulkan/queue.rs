use crate::graphics::vulkan::{vk_structure_types, vk_structures};

#[derive(Debug, Default, Clone, Copy)]
pub struct Queue {
    pub vk_queue: vk_structures::VkQueueFamilyProperties,
    pub device_type: vk_structure_types::VkPhysicalDeviceType,
    pub index: usize,
    pub supports_surface: bool,
    pub supports_graphic: bool,
}
