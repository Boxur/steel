use crate::graphics::vulkan::{vk_structure_types, vk_structures};

use super::{raw, vk_pfn_types};

#[repr(C)]
#[derive(Debug, Default)]
pub struct VkInstanceCreateInfo {
    pub s_type: vk_structure_types::VkStructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: u32,
    pub p_application_info: *const VkApplicationInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const std::ffi::c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const std::ffi::c_char,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct VkApplicationInfo {
    pub s_type: vk_structure_types::VkStructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_application_name: *const i8,
    pub application_version: u32,
    pub p_engine_name: *const i8,
    pub engine_version: u32,
    pub api_version: u32,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct VkXlibSurfaceCreateInfoKHR {
    pub s_type: vk_structure_types::VkStructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: u32,
    pub dpy: *mut raw::XDisplay,
    pub window: raw::XWindow,
}

#[repr(C)]
#[derive(Debug)]
pub struct VkPhysicalDeviceProperties2 {
    pub s_type: vk_structure_types::VkStructureType,
    pub p_next: *const std::ffi::c_void,
    pub properties: VkPhysicalDeviceProperties,
}

#[repr(C)]
#[derive(Debug)]
pub struct VkPhysicalDeviceProperties {
    pub api_version: u32,
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: vk_structure_types::VkPhysicalDeviceType,
    pub device_name: [i8; 256_usize],
    pub pipeline_cache_uuid: [u8; 16_usize],
    pub limits: VkPhysicalDeviceLimits,
    pub sparse_properties: VkPhysicalDeviceSparseProperties,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct VkPhysicalDeviceLimits {
    pub max_image_dimension_1d: u32,
    pub max_image_dimension_2d: u32,
    pub max_image_dimension_3d: u32,
    pub max_image_dimension_cube: u32,
    pub max_image_array_layers: u32,
    pub max_texel_buffer_elements: u32,
    pub max_uniform_buffer_range: u32,
    pub max_storage_buffer_range: u32,
    pub max_push_constants_size: u32,
    pub max_memory_allocation_count: u32,
    pub max_sampler_allocation_count: u32,
    pub buffer_image_granularity: raw::VkDeviceSize,
    pub sparse_address_space_size: raw::VkDeviceSize,
    pub max_bound_descriptor_sets: u32,
    pub max_per_stage_descriptor_samplers: u32,
    pub max_per_stage_descriptor_uniform_buffers: u32,
    pub max_per_stage_descriptor_storage_buffers: u32,
    pub max_per_stage_descriptor_sampled_images: u32,
    pub max_per_stage_descriptor_storage_images: u32,
    pub max_per_stage_descriptor_input_attachments: u32,
    pub max_per_stage_resources: u32,
    pub max_descriptor_set_samplers: u32,
    pub max_descriptor_set_uniform_buffers: u32,
    pub max_descriptor_set_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_storage_buffers: u32,
    pub max_descriptor_set_storage_buffers_dynamic: u32,
    pub max_descriptor_set_sampled_images: u32,
    pub max_descriptor_set_storage_images: u32,
    pub max_descriptor_set_input_attachments: u32,
    pub max_vertex_input_attributes: u32,
    pub max_vertex_input_bindings: u32,
    pub max_vertex_input_attribute_offset: u32,
    pub max_vertex_input_binding_stride: u32,
    pub max_vertex_output_components: u32,
    pub max_tessellation_generation_level: u32,
    pub max_tessellation_patch_size: u32,
    pub max_tessellation_control_per_vertex_input_components: u32,
    pub max_tessellation_control_per_vertex_output_components: u32,
    pub max_tessellation_control_per_patch_output_components: u32,
    pub max_tessellation_control_total_output_components: u32,
    pub max_tessellation_evaluation_input_components: u32,
    pub max_tessellation_evaluation_output_components: u32,
    pub max_geometry_shader_invocations: u32,
    pub max_geometry_input_components: u32,
    pub max_geometry_output_components: u32,
    pub max_geometry_output_vertices: u32,
    pub max_geometry_total_output_components: u32,
    pub max_fragment_input_components: u32,
    pub max_fragment_output_attachments: u32,
    pub max_fragment_dual_src_attachments: u32,
    pub max_fragment_combined_output_resources: u32,
    pub max_compute_shared_memory_size: u32,
    pub max_compute_work_group_count: [u32; 3],
    pub max_compute_work_group_invocations: u32,
    pub max_compute_work_group_size: [u32; 3],
    pub sub_pixel_precision_bits: u32,
    pub sub_texel_precision_bits: u32,
    pub mipmap_precision_bits: u32,
    pub max_draw_indexed_index_value: u32,
    pub max_draw_indirect_count: u32,
    pub max_sampler_lod_bias: f32,
    pub max_sampler_anisotropy: f32,
    pub max_viewports: u32,
    pub max_viewport_dimensions: [u32; 2],
    pub viewport_bounds_range: [f32; 2],
    pub viewport_sub_pixel_bits: u32,
    pub min_memory_map_alignment: isize,
    pub min_texel_buffer_offset_alignment: raw::VkDeviceSize,
    pub min_uniform_buffer_offset_alignment: raw::VkDeviceSize,
    pub min_storage_buffer_offset_alignment: raw::VkDeviceSize,
    pub min_texel_offset: i32,
    pub max_texel_offset: u32,
    pub min_texel_gather_offset: i32,
    pub max_texel_gather_offset: u32,
    pub min_interpolation_offset: f32,
    pub max_interpolation_offset: f32,
    pub sub_pixel_interpolation_offset_bits: u32,
    pub max_framebuffer_width: u32,
    pub max_framebuffer_height: u32,
    pub max_framebuffer_layers: u32,
    pub framebuffer_color_sample_counts: raw::VkSampleCountFlags,
    pub framebuffer_depth_sample_counts: raw::VkSampleCountFlags,
    pub framebuffer_stencil_sample_counts: raw::VkSampleCountFlags,
    pub framebuffer_no_attachments_sample_counts: raw::VkSampleCountFlags,
    pub max_color_attachments: u32,
    pub sampled_image_color_sample_counts: raw::VkSampleCountFlags,
    pub sampled_image_integer_sample_counts: raw::VkSampleCountFlags,
    pub sampled_image_depth_sample_counts: raw::VkSampleCountFlags,
    pub sampled_image_stencil_sample_counts: raw::VkSampleCountFlags,
    pub storage_image_sample_counts: raw::VkSampleCountFlags,
    pub max_sample_mask_words: u32,
    pub timestamp_compute_and_graphics: raw::VkBool32,
    pub timestamp_period: f32,
    pub max_clip_distances: u32,
    pub max_cull_distances: u32,
    pub max_combined_clip_and_cull_distances: u32,
    pub discrete_queue_priorities: u32,
    pub point_size_range: [f32; 2],
    pub line_width_range: [f32; 2],
    pub point_size_granularity: f32,
    pub line_width_granularity: f32,
    pub strict_lines: raw::VkBool32,
    pub standard_sample_locations: raw::VkBool32,
    pub optimal_buffer_copy_offset_alignment: raw::VkDeviceSize,
    pub optimal_buffer_copy_row_pitch_alignment: raw::VkDeviceSize,
    pub non_coherent_atom_size: raw::VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct VkPhysicalDeviceSparseProperties {
    pub residency_standard_2d_block_shape: raw::VkBool32,
    pub residency_standard_2d_multisample_block_shape: raw::VkBool32,
    pub residency_standard_3d_block_shape: raw::VkBool32,
    pub residency_aligned_mip_size: raw::VkBool32,
    pub residency_non_resident_strict: raw::VkBool32,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct VkQueueFamilyProperties {
    pub queue_flags: vk_structure_types::VkQueueFlags,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: VkExtent3D,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct VkExtent3D {
    width: u32,
    height: u32,
    depth: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct VulkanInstanceFns {
    pub create_xlib_surface: vk_pfn_types::PFNvkCreateXlibSurfaceKHR,
    pub enumerate_physical_devices: vk_pfn_types::PFNvkEnumeratePhysicalDevices,
    pub _get_physical_device_properties2: vk_pfn_types::PFNvkGetPhysicalDeviceProperties2,
    pub get_physical_device_queue_family_properties:
        vk_pfn_types::PFNvkGetPhysicalDeviceQueueFamilyProperties,
    pub get_physical_device_surface_support_khr:
        vk_pfn_types::PFNvkGetPhysicalDeviceSurfaceSupportKHR,
}
