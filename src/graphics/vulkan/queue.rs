use crate::graphics::vulkan::{
    logical_device, physical_device,
    queue::Queues::{Double, Single},
    raw::{self, vk_handles},
};

#[derive(Debug, Default)]
pub enum Queues {
    #[default]
    None,
    Single(Queue),
    Double(Queue, Queue),
}

impl Queues {
    pub fn new(
        physical_device: &physical_device::PhysicalDevice,
        logical_device: &logical_device::LogicalDevice,
    ) -> Self {
        let graphics_queue_family_index = physical_device.graphics_queue_family_index.unwrap();
        let surface_queue_family_index = physical_device.surface_queue_family_index.unwrap();
        if graphics_queue_family_index == surface_queue_family_index {
            return Single(Queue::new(&logical_device, graphics_queue_family_index, 0));
        }
        Double(
            Queue::new(&logical_device, graphics_queue_family_index, 0),
            Queue::new(&logical_device, surface_queue_family_index, 0),
        )
    }
}

#[derive(Debug, Default)]
pub struct Queue {
    vk_queue: vk_handles::VkQueue,
}

impl Queue {
    pub fn new(
        logical_device: &logical_device::LogicalDevice,
        queue_family_index: u32,
        queue_index: u32,
    ) -> Self {
        let mut queue = Self::default();
        unsafe {
            raw::vkGetDeviceQueue(
                logical_device.get_vk_device(),
                queue_family_index,
                queue_index,
                &raw mut queue.vk_queue,
            );
        }
        queue
    }
}
