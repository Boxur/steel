#![allow(unused)]
use super::{vk_pfn_types, vk_structure_types};
use crate::graphics::vulkan::raw::{self};

#[repr(C)]
#[derive(Debug)]
pub struct VkInstanceCreateInfo {
    s_type: vk_structure_types::VkStructureType,
    p_next: *const std::ffi::c_void,
    flags: u32,
    p_application_info: *const VkApplicationInfo,
    enabled_layer_count: u32,
    pp_enabled_layer_names: *const *const std::ffi::c_char,
    enabled_extension_count: u32,
    pp_enabled_extension_names: *const *const std::ffi::c_char,
}

impl VkInstanceCreateInfo {
    pub fn builder() -> Self {
        Self {
            s_type: raw::vk_structure_types::VkStructureType::InstanceCreateInfo,
            p_next: core::ptr::null(),
            flags: 0,
            p_application_info: core::ptr::null(),
            enabled_layer_count: 0,
            pp_enabled_layer_names: core::ptr::null(),
            enabled_extension_count: 0,
            pp_enabled_extension_names: core::ptr::null(),
        }
    }

    pub fn p_next(mut self, next: *const std::ffi::c_void) -> Self {
        self.p_next = next;
        self
    }

    pub fn flags(mut self, flags: u32) -> Self {
        self.flags = flags;
        self
    }

    pub fn application_info<'a>(mut self, application_info: &'a VkApplicationInfo) -> Self {
        self.p_application_info = application_info;
        self
    }

    pub fn enabled_extensions<'a>(mut self, extensions: &'a [*const i8]) -> Self {
        self.enabled_extension_count = extensions.len() as u32;
        self.pp_enabled_extension_names = extensions.as_ptr();
        self
    }

    pub fn enabled_layers<'a>(mut self, layers: &'a [*const i8]) -> Self {
        self.enabled_layer_count = layers.len() as u32;
        self.pp_enabled_layer_names = layers.as_ptr();
        self
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct VkApplicationInfo {
    s_type: vk_structure_types::VkStructureType,
    p_next: *const std::ffi::c_void,
    p_application_name: *const u8,
    application_version: u32,
    p_engine_name: *const u8,
    engine_version: u32,
    api_version: u32,
}

impl VkApplicationInfo {
    pub fn builder() -> Self {
        Self {
            s_type: raw::vk_structure_types::VkStructureType::ApplicationInfo,
            p_next: core::ptr::null(),
            p_application_name: b"\0".as_ptr(),
            application_version: 0,
            p_engine_name: b"\0".as_ptr(),
            engine_version: 0,
            api_version: raw::VK_API_VERSION_1_3,
        }
    }

    pub fn p_next(mut self, next: *const std::ffi::c_void) -> Self {
        self.p_next = next;
        self
    }

    pub fn application_name(mut self, name: &[u8]) -> Self {
        self.p_application_name = name.as_ptr();
        self
    }

    pub fn application_version(mut self, version: u32) -> Self {
        self.application_version = version;
        self
    }

    pub fn engine_name(mut self, name: &[u8]) -> Self {
        self.p_engine_name = name.as_ptr();
        self
    }

    pub fn engine_version(mut self, version: u32) -> Self {
        self.engine_version = version;
        self
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct VkXlibSurfaceCreateInfoKHR {
    s_type: vk_structure_types::VkStructureType,
    p_next: *const std::ffi::c_void,
    flags: u32,
    dpy: *mut super::XDisplay,
    window: super::XWindow,
}

impl VkXlibSurfaceCreateInfoKHR {
    pub fn builder(display: *mut super::XDisplay, window: super::XWindow) -> Self {
        Self {
            s_type: raw::vk_structure_types::VkStructureType::XlibSurfaceCreateInfoKHR,
            p_next: core::ptr::null(),
            flags: 0,
            dpy: display,
            window: window,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct VkPhysicalDeviceProperties2 {
    pub s_type: vk_structure_types::VkStructureType,
    pub p_next: *const std::ffi::c_void,
    pub properties: VkPhysicalDeviceProperties,
}

impl Default for VkPhysicalDeviceProperties2 {
    fn default() -> Self {
        Self {
            s_type: raw::vk_structure_types::VkStructureType::PhysicalDeviceProperties2,
            p_next: core::ptr::null(),
            properties: raw::vk_structures::VkPhysicalDeviceProperties::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct VkPhysicalDeviceProperties {
    api_version: u32,
    driver_version: u32,
    vendor_id: u32,
    device_id: u32,
    device_type: vk_structure_types::VkPhysicalDeviceType,
    device_name: [u8; 256_usize],
    pipeline_cache_uuid: [u8; 16_usize],
    limits: VkPhysicalDeviceLimits,
    sparse_properties: VkPhysicalDeviceSparseProperties,
}

impl Default for VkPhysicalDeviceProperties {
    fn default() -> Self {
        Self {
            api_version: 0,
            driver_version: 0,
            vendor_id: 0,
            device_id: 0,
            device_type: raw::vk_structure_types::VkPhysicalDeviceType::default(),
            device_name: [0; 256],
            pipeline_cache_uuid: [0; 16],
            limits: raw::vk_structures::VkPhysicalDeviceLimits::default(),
            sparse_properties: raw::vk_structures::VkPhysicalDeviceSparseProperties::default(),
        }
    }
}

impl VkPhysicalDeviceProperties {
    pub fn get_device_name(&self) -> [u8; 256_usize] {
        self.device_name
    }

    pub fn get_device_type(&self) -> vk_structure_types::VkPhysicalDeviceType {
        self.device_type
    }
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
    pub buffer_image_granularity: super::VkDeviceSize,
    pub sparse_address_space_size: super::VkDeviceSize,
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
    pub min_texel_buffer_offset_alignment: super::VkDeviceSize,
    pub min_uniform_buffer_offset_alignment: super::VkDeviceSize,
    pub min_storage_buffer_offset_alignment: super::VkDeviceSize,
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
    pub framebuffer_color_sample_counts: super::VkSampleCountFlags,
    pub framebuffer_depth_sample_counts: super::VkSampleCountFlags,
    pub framebuffer_stencil_sample_counts: super::VkSampleCountFlags,
    pub framebuffer_no_attachments_sample_counts: super::VkSampleCountFlags,
    pub max_color_attachments: u32,
    pub sampled_image_color_sample_counts: super::VkSampleCountFlags,
    pub sampled_image_integer_sample_counts: super::VkSampleCountFlags,
    pub sampled_image_depth_sample_counts: super::VkSampleCountFlags,
    pub sampled_image_stencil_sample_counts: super::VkSampleCountFlags,
    pub storage_image_sample_counts: super::VkSampleCountFlags,
    pub max_sample_mask_words: u32,
    pub timestamp_compute_and_graphics: super::VkBool32,
    pub timestamp_period: f32,
    pub max_clip_distances: u32,
    pub max_cull_distances: u32,
    pub max_combined_clip_and_cull_distances: u32,
    pub discrete_queue_priorities: u32,
    pub point_size_range: [f32; 2],
    pub line_width_range: [f32; 2],
    pub point_size_granularity: f32,
    pub line_width_granularity: f32,
    pub strict_lines: super::VkBool32,
    pub standard_sample_locations: super::VkBool32,
    pub optimal_buffer_copy_offset_alignment: super::VkDeviceSize,
    pub optimal_buffer_copy_row_pitch_alignment: super::VkDeviceSize,
    pub non_coherent_atom_size: super::VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct VkPhysicalDeviceSparseProperties {
    pub residency_standard_2d_block_shape: super::VkBool32,
    pub residency_standard_2d_multisample_block_shape: super::VkBool32,
    pub residency_standard_3d_block_shape: super::VkBool32,
    pub residency_aligned_mip_size: super::VkBool32,
    pub residency_non_resident_strict: super::VkBool32,
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

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct VkDeviceCreateInfo {
    s_type: vk_structure_types::VkStructureType,
    p_next: *const std::ffi::c_void,
    flags: u32,
    queue_create_info_count: u32,
    p_queue_create_infos: *const VkDeviceQueueCreateInfo,
    enabled_layer_count: u32,
    pp_enabled_layer_names: *const *const i8,
    enabled_extension_count: u32,
    pp_enabled_extension_names: *const *const i8,
    p_enabled_features: *const VkPhysicalDeviceFeatures,
}

impl VkDeviceCreateInfo {
    pub fn builder() -> Self {
        Self {
            s_type: vk_structure_types::VkStructureType::DeviceCreateInfo,
            p_next: core::ptr::null(),
            flags: 0,
            queue_create_info_count: 0,
            p_queue_create_infos: core::ptr::null(),
            enabled_layer_count: 0,
            pp_enabled_layer_names: core::ptr::null_mut(),
            enabled_extension_count: 0,
            pp_enabled_extension_names: core::ptr::null_mut(),
            p_enabled_features: core::ptr::null(),
        }
    }

    pub fn p_next(mut self, next: *const std::ffi::c_void) -> Self {
        self.p_next = next;
        self
    }

    pub fn queue_create_info(
        mut self,
        queue_create_info: &Vec<VkDeviceQueueCreateInfo>,
        count: u32,
    ) -> Self {
        self.queue_create_info_count = count;
        self.p_queue_create_infos = queue_create_info.as_ptr();
        self
    }

    pub fn enabled_extensions(mut self, extensions: &Vec<*const i8>) -> Self {
        self.enabled_extension_count = extensions.len() as u32;
        self.pp_enabled_extension_names = extensions.as_ptr();
        self
    }
}

#[cfg(test)]
mod vk_device_create_info_tests {
    use super::*;
    #[test]
    fn builder() {
        let device_create_info = VkDeviceCreateInfo::builder();
        assert_eq!(
            device_create_info.s_type,
            vk_structure_types::VkStructureType::DeviceCreateInfo
        );
        assert_eq!(device_create_info.p_next, core::ptr::null());
        assert_eq!(device_create_info.flags, 0);
        assert_eq!(device_create_info.queue_create_info_count, 0);
        assert_eq!(device_create_info.p_queue_create_infos, core::ptr::null());
        assert_eq!(device_create_info.enabled_extension_count, 0);
        assert_eq!(
            device_create_info.pp_enabled_extension_names,
            core::ptr::null_mut()
        );
        assert_eq!(device_create_info.enabled_layer_count, 0);
        assert_eq!(
            device_create_info.pp_enabled_layer_names,
            core::ptr::null_mut()
        );
    }

    #[test]
    fn p_next() {
        let sth = 2;
        let device_create_info =
            VkDeviceCreateInfo::builder().p_next(&raw const sth as *const std::ffi::c_void);
        assert_eq!(
            device_create_info.p_next,
            &raw const sth as *const std::ffi::c_void
        );
    }

    #[test]
    fn queue_create_info() {
        let mut queue_create_info: Vec<VkDeviceQueueCreateInfo> = vec![];
        let priorities = vec![1.0_f32];
        queue_create_info.push(VkDeviceQueueCreateInfo::builder(0, &priorities, 1));
        queue_create_info.push(VkDeviceQueueCreateInfo::builder(1, &priorities, 1));
        let device_create_info =
            VkDeviceCreateInfo::builder().queue_create_info(&queue_create_info, 2);
        assert_eq!(device_create_info.queue_create_info_count, 2);
        assert_eq!(
            device_create_info.p_queue_create_infos,
            queue_create_info.as_ptr()
        );
        unsafe {
            assert_eq!(
                (std::slice::from_raw_parts(device_create_info.p_queue_create_infos.cast_mut(), 2))
                    [0],
                queue_create_info[0]
            );
            assert_eq!(
                (std::slice::from_raw_parts(device_create_info.p_queue_create_infos.cast_mut(), 2))
                    [1],
                queue_create_info[1]
            );
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct VkDeviceQueueCreateInfo {
    s_type: vk_structure_types::VkStructureType,
    p_next: *const std::ffi::c_void,
    flags: vk_structure_types::VkDeviceQueueCreateFlags,
    queue_family_index: u32,
    queue_count: u32,
    p_queue_priorities: *const f32,
}

impl VkDeviceQueueCreateInfo {
    pub fn builder(queue_family_index: u32, queue_priorities: &Vec<f32>, count: u32) -> Self {
        Self {
            s_type: vk_structure_types::VkStructureType::DeviceQueueCreateInfo,
            p_next: core::ptr::null(),
            flags: vk_structure_types::VkDeviceQueueCreateFlagBits::None.into(),
            queue_family_index,
            queue_count: count,
            p_queue_priorities: queue_priorities.as_ptr(),
        }
    }
}

#[cfg(test)]
mod vk_device_queue_create_info_tests {
    use super::*;
    #[test]
    fn builder() {
        let priorities = vec![1.0_f32];
        let device_queue_create_info = VkDeviceQueueCreateInfo::builder(0, &priorities, 1);
        assert_eq!(
            device_queue_create_info.s_type,
            vk_structure_types::VkStructureType::DeviceQueueCreateInfo
        );
        assert_eq!(device_queue_create_info.p_next, core::ptr::null());
        assert_eq!(device_queue_create_info.queue_count, 1);
        assert_eq!(
            device_queue_create_info.flags,
            vk_structure_types::VkDeviceQueueCreateFlagBits::None.into()
        );
        unsafe {
            assert_eq!(
                std::slice::from_raw_parts(device_queue_create_info.p_queue_priorities, 1)[0],
                1.0_f32
            );
        }
        assert!(device_queue_create_info.queue_count > 0);
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct VkPhysicalDeviceFeatures {
    robust_buffer_access: super::VkBool32,
    full_draw_index_uint32: super::VkBool32,
    image_cube_array: super::VkBool32,
    independent_blend: super::VkBool32,
    geometry_shader: super::VkBool32,
    tessellation_shader: super::VkBool32,
    sample_rate_shading: super::VkBool32,
    dual_src_blend: super::VkBool32,
    logic_op: super::VkBool32,
    multi_draw_indirect: super::VkBool32,
    draw_indirect_first_instance: super::VkBool32,
    depth_clamp: super::VkBool32,
    depth_bias_clamp: super::VkBool32,
    fill_mode_non_solid: super::VkBool32,
    depth_bounds: super::VkBool32,
    wide_lines: super::VkBool32,
    large_points: super::VkBool32,
    alpha_to_one: super::VkBool32,
    multi_viewport: super::VkBool32,
    sampler_anisotropy: super::VkBool32,
    texture_compression_etc2: super::VkBool32,
    texture_compression_astc_ldr: super::VkBool32,
    texture_compression_bc: super::VkBool32,
    occlusion_query_precise: super::VkBool32,
    pipeline_statistics_query: super::VkBool32,
    vertex_pipeline_stores_and_atomics: super::VkBool32,
    fragment_stores_and_atomics: super::VkBool32,
    shader_tessellation_and_geometry_point_size: super::VkBool32,
    shader_image_gather_extended: super::VkBool32,
    shader_storage_image_extended_formats: super::VkBool32,
    shader_storage_image_multisample: super::VkBool32,
    shader_storage_image_read_without_format: super::VkBool32,
    shader_storage_image_write_without_format: super::VkBool32,
    shader_uniform_buffer_array_dynamic_indexing: super::VkBool32,
    shader_sampled_image_array_dynamic_indexing: super::VkBool32,
    shader_storage_buffer_array_dynamic_indexing: super::VkBool32,
    shader_storage_image_array_dynamic_indexing: super::VkBool32,
    shader_clip_distance: super::VkBool32,
    shader_cull_distance: super::VkBool32,
    shader_float64: super::VkBool32,
    shader_int64: super::VkBool32,
    shader_int16: super::VkBool32,
    shader_resource_residency: super::VkBool32,
    shader_resource_min_lod: super::VkBool32,
    sparse_binding: super::VkBool32,
    sparse_residency_buffer: super::VkBool32,
    sparse_residency_image_2d: super::VkBool32,
    sparse_residency_image_3d: super::VkBool32,
    sparse_residency_2_samples: super::VkBool32,
    sparse_residency_4_samples: super::VkBool32,
    sparse_residency_8_samples: super::VkBool32,
    sparse_residency1_6_samples: super::VkBool32,
    sparse_residency_aliased: super::VkBool32,
    variable_multisample_rate: super::VkBool32,
    inherited_queries: super::VkBool32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct VulkanInstanceFns {
    pub create_xlib_surface: Option<vk_pfn_types::PFNvkCreateXlibSurfaceKHR>,
    pub get_physical_device_surface_support_khr:
        Option<vk_pfn_types::PFNvkGetPhysicalDeviceSurfaceSupportKHR>,
}
