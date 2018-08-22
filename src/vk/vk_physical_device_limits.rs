// Generated by `scripts/generate_vk.js`

use std::os::raw::c_char;
use std::ops::Deref;
use std::ptr;
use std::cmp;
use std::mem;
use utils::c_bindings::*;
use utils::vk_convert::*;
use utils::vk_null::*;
use utils::vk_ptr::*;
use utils::vk_traits::*;
use vk::vk_instance_function_table::*;
use vk::vk_instance::*;
use vk::vk_device::*;
use vk::vk_sample_count_flags::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawVkPhysicalDeviceLimits {
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
    pub buffer_image_granularity: u64,
    pub sparse_address_space_size: u64,
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
    pub min_memory_map_alignment: usize,
    pub min_texel_buffer_offset_alignment: u64,
    pub min_uniform_buffer_offset_alignment: u64,
    pub min_storage_buffer_offset_alignment: u64,
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
    pub framebuffer_color_sample_counts: RawVkSampleCountFlags,
    pub framebuffer_depth_sample_counts: RawVkSampleCountFlags,
    pub framebuffer_stencil_sample_counts: RawVkSampleCountFlags,
    pub framebuffer_no_attachments_sample_counts: RawVkSampleCountFlags,
    pub max_color_attachments: u32,
    pub sampled_image_color_sample_counts: RawVkSampleCountFlags,
    pub sampled_image_integer_sample_counts: RawVkSampleCountFlags,
    pub sampled_image_depth_sample_counts: RawVkSampleCountFlags,
    pub sampled_image_stencil_sample_counts: RawVkSampleCountFlags,
    pub storage_image_sample_counts: RawVkSampleCountFlags,
    pub max_sample_mask_words: u32,
    pub timestamp_compute_and_graphics: u32,
    pub timestamp_period: f32,
    pub max_clip_distances: u32,
    pub max_cull_distances: u32,
    pub max_combined_clip_and_cull_distances: u32,
    pub discrete_queue_priorities: u32,
    pub point_size_range: [f32; 2],
    pub line_width_range: [f32; 2],
    pub point_size_granularity: f32,
    pub line_width_granularity: f32,
    pub strict_lines: u32,
    pub standard_sample_locations: u32,
    pub optimal_buffer_copy_offset_alignment: u64,
    pub optimal_buffer_copy_row_pitch_alignment: u64,
    pub non_coherent_atom_size: u64,
}

#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceLimits {
    pub max_image_dimension_1d: usize,
    pub max_image_dimension_2d: usize,
    pub max_image_dimension_3d: usize,
    pub max_image_dimension_cube: usize,
    pub max_image_array_layers: usize,
    pub max_texel_buffer_elements: usize,
    pub max_uniform_buffer_range: usize,
    pub max_storage_buffer_range: usize,
    pub max_push_constants_size: usize,
    pub max_memory_allocation_count: usize,
    pub max_sampler_allocation_count: usize,
    pub buffer_image_granularity: usize,
    pub sparse_address_space_size: usize,
    pub max_bound_descriptor_sets: usize,
    pub max_per_stage_descriptor_samplers: usize,
    pub max_per_stage_descriptor_uniform_buffers: usize,
    pub max_per_stage_descriptor_storage_buffers: usize,
    pub max_per_stage_descriptor_sampled_images: usize,
    pub max_per_stage_descriptor_storage_images: usize,
    pub max_per_stage_descriptor_input_attachments: usize,
    pub max_per_stage_resources: usize,
    pub max_descriptor_set_samplers: usize,
    pub max_descriptor_set_uniform_buffers: usize,
    pub max_descriptor_set_uniform_buffers_dynamic: usize,
    pub max_descriptor_set_storage_buffers: usize,
    pub max_descriptor_set_storage_buffers_dynamic: usize,
    pub max_descriptor_set_sampled_images: usize,
    pub max_descriptor_set_storage_images: usize,
    pub max_descriptor_set_input_attachments: usize,
    pub max_vertex_input_attributes: usize,
    pub max_vertex_input_bindings: usize,
    pub max_vertex_input_attribute_offset: usize,
    pub max_vertex_input_binding_stride: usize,
    pub max_vertex_output_components: usize,
    pub max_tessellation_generation_level: usize,
    pub max_tessellation_patch_size: usize,
    pub max_tessellation_control_per_vertex_input_components: usize,
    pub max_tessellation_control_per_vertex_output_components: usize,
    pub max_tessellation_control_per_patch_output_components: usize,
    pub max_tessellation_control_total_output_components: usize,
    pub max_tessellation_evaluation_input_components: usize,
    pub max_tessellation_evaluation_output_components: usize,
    pub max_geometry_shader_invocations: usize,
    pub max_geometry_input_components: usize,
    pub max_geometry_output_components: usize,
    pub max_geometry_output_vertices: usize,
    pub max_geometry_total_output_components: usize,
    pub max_fragment_input_components: usize,
    pub max_fragment_output_attachments: usize,
    pub max_fragment_dual_src_attachments: usize,
    pub max_fragment_combined_output_resources: usize,
    pub max_compute_shared_memory_size: usize,
    pub max_compute_work_group_count: [usize; 3],
    pub max_compute_work_group_invocations: usize,
    pub max_compute_work_group_size: [usize; 3],
    pub sub_pixel_precision_bits: usize,
    pub sub_texel_precision_bits: usize,
    pub mipmap_precision_bits: usize,
    pub max_draw_indexed_index_value: usize,
    pub max_draw_indirect_count: usize,
    pub max_sampler_lod_bias: f32,
    pub max_sampler_anisotropy: f32,
    pub max_viewports: usize,
    pub max_viewport_dimensions: [usize; 2],
    pub viewport_bounds_range: [f32; 2],
    pub viewport_sub_pixel_bits: usize,
    pub min_memory_map_alignment: usize,
    pub min_texel_buffer_offset_alignment: usize,
    pub min_uniform_buffer_offset_alignment: usize,
    pub min_storage_buffer_offset_alignment: usize,
    pub min_texel_offset: isize,
    pub max_texel_offset: usize,
    pub min_texel_gather_offset: isize,
    pub max_texel_gather_offset: usize,
    pub min_interpolation_offset: f32,
    pub max_interpolation_offset: f32,
    pub sub_pixel_interpolation_offset_bits: usize,
    pub max_framebuffer_width: usize,
    pub max_framebuffer_height: usize,
    pub max_framebuffer_layers: usize,
    pub framebuffer_color_sample_counts: VkSampleCountFlags,
    pub framebuffer_depth_sample_counts: VkSampleCountFlags,
    pub framebuffer_stencil_sample_counts: VkSampleCountFlags,
    pub framebuffer_no_attachments_sample_counts: VkSampleCountFlags,
    pub max_color_attachments: usize,
    pub sampled_image_color_sample_counts: VkSampleCountFlags,
    pub sampled_image_integer_sample_counts: VkSampleCountFlags,
    pub sampled_image_depth_sample_counts: VkSampleCountFlags,
    pub sampled_image_stencil_sample_counts: VkSampleCountFlags,
    pub storage_image_sample_counts: VkSampleCountFlags,
    pub max_sample_mask_words: u32,
    pub timestamp_compute_and_graphics: bool,
    pub timestamp_period: f32,
    pub max_clip_distances: usize,
    pub max_cull_distances: usize,
    pub max_combined_clip_and_cull_distances: usize,
    pub discrete_queue_priorities: usize,
    pub point_size_range: [f32; 2],
    pub line_width_range: [f32; 2],
    pub point_size_granularity: f32,
    pub line_width_granularity: f32,
    pub strict_lines: bool,
    pub standard_sample_locations: bool,
    pub optimal_buffer_copy_offset_alignment: usize,
    pub optimal_buffer_copy_row_pitch_alignment: usize,
    pub non_coherent_atom_size: usize,
}

impl VkRawType<VkPhysicalDeviceLimits> for RawVkPhysicalDeviceLimits {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceLimits) -> VkPhysicalDeviceLimits {
        VkPhysicalDeviceLimits {
            max_image_dimension_1d: u32::vk_to_wrapped(&src.max_image_dimension_1d),
            max_image_dimension_2d: u32::vk_to_wrapped(&src.max_image_dimension_2d),
            max_image_dimension_3d: u32::vk_to_wrapped(&src.max_image_dimension_3d),
            max_image_dimension_cube: u32::vk_to_wrapped(&src.max_image_dimension_cube),
            max_image_array_layers: u32::vk_to_wrapped(&src.max_image_array_layers),
            max_texel_buffer_elements: u32::vk_to_wrapped(&src.max_texel_buffer_elements),
            max_uniform_buffer_range: u32::vk_to_wrapped(&src.max_uniform_buffer_range),
            max_storage_buffer_range: u32::vk_to_wrapped(&src.max_storage_buffer_range),
            max_push_constants_size: u32::vk_to_wrapped(&src.max_push_constants_size),
            max_memory_allocation_count: u32::vk_to_wrapped(&src.max_memory_allocation_count),
            max_sampler_allocation_count: u32::vk_to_wrapped(&src.max_sampler_allocation_count),
            buffer_image_granularity: u64::vk_to_wrapped(&src.buffer_image_granularity),
            sparse_address_space_size: u64::vk_to_wrapped(&src.sparse_address_space_size),
            max_bound_descriptor_sets: u32::vk_to_wrapped(&src.max_bound_descriptor_sets),
            max_per_stage_descriptor_samplers: u32::vk_to_wrapped(&src.max_per_stage_descriptor_samplers),
            max_per_stage_descriptor_uniform_buffers: u32::vk_to_wrapped(&src.max_per_stage_descriptor_uniform_buffers),
            max_per_stage_descriptor_storage_buffers: u32::vk_to_wrapped(&src.max_per_stage_descriptor_storage_buffers),
            max_per_stage_descriptor_sampled_images: u32::vk_to_wrapped(&src.max_per_stage_descriptor_sampled_images),
            max_per_stage_descriptor_storage_images: u32::vk_to_wrapped(&src.max_per_stage_descriptor_storage_images),
            max_per_stage_descriptor_input_attachments: u32::vk_to_wrapped(&src.max_per_stage_descriptor_input_attachments),
            max_per_stage_resources: u32::vk_to_wrapped(&src.max_per_stage_resources),
            max_descriptor_set_samplers: u32::vk_to_wrapped(&src.max_descriptor_set_samplers),
            max_descriptor_set_uniform_buffers: u32::vk_to_wrapped(&src.max_descriptor_set_uniform_buffers),
            max_descriptor_set_uniform_buffers_dynamic: u32::vk_to_wrapped(&src.max_descriptor_set_uniform_buffers_dynamic),
            max_descriptor_set_storage_buffers: u32::vk_to_wrapped(&src.max_descriptor_set_storage_buffers),
            max_descriptor_set_storage_buffers_dynamic: u32::vk_to_wrapped(&src.max_descriptor_set_storage_buffers_dynamic),
            max_descriptor_set_sampled_images: u32::vk_to_wrapped(&src.max_descriptor_set_sampled_images),
            max_descriptor_set_storage_images: u32::vk_to_wrapped(&src.max_descriptor_set_storage_images),
            max_descriptor_set_input_attachments: u32::vk_to_wrapped(&src.max_descriptor_set_input_attachments),
            max_vertex_input_attributes: u32::vk_to_wrapped(&src.max_vertex_input_attributes),
            max_vertex_input_bindings: u32::vk_to_wrapped(&src.max_vertex_input_bindings),
            max_vertex_input_attribute_offset: u32::vk_to_wrapped(&src.max_vertex_input_attribute_offset),
            max_vertex_input_binding_stride: u32::vk_to_wrapped(&src.max_vertex_input_binding_stride),
            max_vertex_output_components: u32::vk_to_wrapped(&src.max_vertex_output_components),
            max_tessellation_generation_level: u32::vk_to_wrapped(&src.max_tessellation_generation_level),
            max_tessellation_patch_size: u32::vk_to_wrapped(&src.max_tessellation_patch_size),
            max_tessellation_control_per_vertex_input_components: u32::vk_to_wrapped(&src.max_tessellation_control_per_vertex_input_components),
            max_tessellation_control_per_vertex_output_components: u32::vk_to_wrapped(&src.max_tessellation_control_per_vertex_output_components),
            max_tessellation_control_per_patch_output_components: u32::vk_to_wrapped(&src.max_tessellation_control_per_patch_output_components),
            max_tessellation_control_total_output_components: u32::vk_to_wrapped(&src.max_tessellation_control_total_output_components),
            max_tessellation_evaluation_input_components: u32::vk_to_wrapped(&src.max_tessellation_evaluation_input_components),
            max_tessellation_evaluation_output_components: u32::vk_to_wrapped(&src.max_tessellation_evaluation_output_components),
            max_geometry_shader_invocations: u32::vk_to_wrapped(&src.max_geometry_shader_invocations),
            max_geometry_input_components: u32::vk_to_wrapped(&src.max_geometry_input_components),
            max_geometry_output_components: u32::vk_to_wrapped(&src.max_geometry_output_components),
            max_geometry_output_vertices: u32::vk_to_wrapped(&src.max_geometry_output_vertices),
            max_geometry_total_output_components: u32::vk_to_wrapped(&src.max_geometry_total_output_components),
            max_fragment_input_components: u32::vk_to_wrapped(&src.max_fragment_input_components),
            max_fragment_output_attachments: u32::vk_to_wrapped(&src.max_fragment_output_attachments),
            max_fragment_dual_src_attachments: u32::vk_to_wrapped(&src.max_fragment_dual_src_attachments),
            max_fragment_combined_output_resources: u32::vk_to_wrapped(&src.max_fragment_combined_output_resources),
            max_compute_shared_memory_size: u32::vk_to_wrapped(&src.max_compute_shared_memory_size),
            max_compute_work_group_count: unsafe { let mut dst_array : [usize; 3] = mem::uninitialized(); vk_to_wrapped_array(&src.max_compute_work_group_count, &mut dst_array); dst_array },
            max_compute_work_group_invocations: u32::vk_to_wrapped(&src.max_compute_work_group_invocations),
            max_compute_work_group_size: unsafe { let mut dst_array : [usize; 3] = mem::uninitialized(); vk_to_wrapped_array(&src.max_compute_work_group_size, &mut dst_array); dst_array },
            sub_pixel_precision_bits: u32::vk_to_wrapped(&src.sub_pixel_precision_bits),
            sub_texel_precision_bits: u32::vk_to_wrapped(&src.sub_texel_precision_bits),
            mipmap_precision_bits: u32::vk_to_wrapped(&src.mipmap_precision_bits),
            max_draw_indexed_index_value: u32::vk_to_wrapped(&src.max_draw_indexed_index_value),
            max_draw_indirect_count: u32::vk_to_wrapped(&src.max_draw_indirect_count),
            max_sampler_lod_bias: src.max_sampler_lod_bias,
            max_sampler_anisotropy: src.max_sampler_anisotropy,
            max_viewports: u32::vk_to_wrapped(&src.max_viewports),
            max_viewport_dimensions: unsafe { let mut dst_array : [usize; 2] = mem::uninitialized(); vk_to_wrapped_array(&src.max_viewport_dimensions, &mut dst_array); dst_array },
            viewport_bounds_range: unsafe { let mut dst_array : [f32; 2] = mem::uninitialized(); to_array(&src.viewport_bounds_range, &mut dst_array); dst_array },
            viewport_sub_pixel_bits: u32::vk_to_wrapped(&src.viewport_sub_pixel_bits),
            min_memory_map_alignment: src.min_memory_map_alignment,
            min_texel_buffer_offset_alignment: u64::vk_to_wrapped(&src.min_texel_buffer_offset_alignment),
            min_uniform_buffer_offset_alignment: u64::vk_to_wrapped(&src.min_uniform_buffer_offset_alignment),
            min_storage_buffer_offset_alignment: u64::vk_to_wrapped(&src.min_storage_buffer_offset_alignment),
            min_texel_offset: i32::vk_to_wrapped(&src.min_texel_offset),
            max_texel_offset: u32::vk_to_wrapped(&src.max_texel_offset),
            min_texel_gather_offset: i32::vk_to_wrapped(&src.min_texel_gather_offset),
            max_texel_gather_offset: u32::vk_to_wrapped(&src.max_texel_gather_offset),
            min_interpolation_offset: src.min_interpolation_offset,
            max_interpolation_offset: src.max_interpolation_offset,
            sub_pixel_interpolation_offset_bits: u32::vk_to_wrapped(&src.sub_pixel_interpolation_offset_bits),
            max_framebuffer_width: u32::vk_to_wrapped(&src.max_framebuffer_width),
            max_framebuffer_height: u32::vk_to_wrapped(&src.max_framebuffer_height),
            max_framebuffer_layers: u32::vk_to_wrapped(&src.max_framebuffer_layers),
            framebuffer_color_sample_counts: RawVkSampleCountFlags::vk_to_wrapped(&src.framebuffer_color_sample_counts),
            framebuffer_depth_sample_counts: RawVkSampleCountFlags::vk_to_wrapped(&src.framebuffer_depth_sample_counts),
            framebuffer_stencil_sample_counts: RawVkSampleCountFlags::vk_to_wrapped(&src.framebuffer_stencil_sample_counts),
            framebuffer_no_attachments_sample_counts: RawVkSampleCountFlags::vk_to_wrapped(&src.framebuffer_no_attachments_sample_counts),
            max_color_attachments: u32::vk_to_wrapped(&src.max_color_attachments),
            sampled_image_color_sample_counts: RawVkSampleCountFlags::vk_to_wrapped(&src.sampled_image_color_sample_counts),
            sampled_image_integer_sample_counts: RawVkSampleCountFlags::vk_to_wrapped(&src.sampled_image_integer_sample_counts),
            sampled_image_depth_sample_counts: RawVkSampleCountFlags::vk_to_wrapped(&src.sampled_image_depth_sample_counts),
            sampled_image_stencil_sample_counts: RawVkSampleCountFlags::vk_to_wrapped(&src.sampled_image_stencil_sample_counts),
            storage_image_sample_counts: RawVkSampleCountFlags::vk_to_wrapped(&src.storage_image_sample_counts),
            max_sample_mask_words: src.max_sample_mask_words,
            timestamp_compute_and_graphics: u32::vk_to_wrapped(&src.timestamp_compute_and_graphics),
            timestamp_period: src.timestamp_period,
            max_clip_distances: u32::vk_to_wrapped(&src.max_clip_distances),
            max_cull_distances: u32::vk_to_wrapped(&src.max_cull_distances),
            max_combined_clip_and_cull_distances: u32::vk_to_wrapped(&src.max_combined_clip_and_cull_distances),
            discrete_queue_priorities: u32::vk_to_wrapped(&src.discrete_queue_priorities),
            point_size_range: unsafe { let mut dst_array : [f32; 2] = mem::uninitialized(); to_array(&src.point_size_range, &mut dst_array); dst_array },
            line_width_range: unsafe { let mut dst_array : [f32; 2] = mem::uninitialized(); to_array(&src.line_width_range, &mut dst_array); dst_array },
            point_size_granularity: src.point_size_granularity,
            line_width_granularity: src.line_width_granularity,
            strict_lines: u32::vk_to_wrapped(&src.strict_lines),
            standard_sample_locations: u32::vk_to_wrapped(&src.standard_sample_locations),
            optimal_buffer_copy_offset_alignment: u64::vk_to_wrapped(&src.optimal_buffer_copy_offset_alignment),
            optimal_buffer_copy_row_pitch_alignment: u64::vk_to_wrapped(&src.optimal_buffer_copy_row_pitch_alignment),
            non_coherent_atom_size: u64::vk_to_wrapped(&src.non_coherent_atom_size),
        }
    }
}

impl VkWrappedType<RawVkPhysicalDeviceLimits> for VkPhysicalDeviceLimits {
    fn vk_to_raw(src: &VkPhysicalDeviceLimits, dst: &mut RawVkPhysicalDeviceLimits) {
        dst.max_image_dimension_1d = vk_to_raw_value(&src.max_image_dimension_1d);
        dst.max_image_dimension_2d = vk_to_raw_value(&src.max_image_dimension_2d);
        dst.max_image_dimension_3d = vk_to_raw_value(&src.max_image_dimension_3d);
        dst.max_image_dimension_cube = vk_to_raw_value(&src.max_image_dimension_cube);
        dst.max_image_array_layers = vk_to_raw_value(&src.max_image_array_layers);
        dst.max_texel_buffer_elements = vk_to_raw_value(&src.max_texel_buffer_elements);
        dst.max_uniform_buffer_range = vk_to_raw_value(&src.max_uniform_buffer_range);
        dst.max_storage_buffer_range = vk_to_raw_value(&src.max_storage_buffer_range);
        dst.max_push_constants_size = vk_to_raw_value(&src.max_push_constants_size);
        dst.max_memory_allocation_count = vk_to_raw_value(&src.max_memory_allocation_count);
        dst.max_sampler_allocation_count = vk_to_raw_value(&src.max_sampler_allocation_count);
        dst.buffer_image_granularity = vk_to_raw_value(&src.buffer_image_granularity);
        dst.sparse_address_space_size = vk_to_raw_value(&src.sparse_address_space_size);
        dst.max_bound_descriptor_sets = vk_to_raw_value(&src.max_bound_descriptor_sets);
        dst.max_per_stage_descriptor_samplers = vk_to_raw_value(&src.max_per_stage_descriptor_samplers);
        dst.max_per_stage_descriptor_uniform_buffers = vk_to_raw_value(&src.max_per_stage_descriptor_uniform_buffers);
        dst.max_per_stage_descriptor_storage_buffers = vk_to_raw_value(&src.max_per_stage_descriptor_storage_buffers);
        dst.max_per_stage_descriptor_sampled_images = vk_to_raw_value(&src.max_per_stage_descriptor_sampled_images);
        dst.max_per_stage_descriptor_storage_images = vk_to_raw_value(&src.max_per_stage_descriptor_storage_images);
        dst.max_per_stage_descriptor_input_attachments = vk_to_raw_value(&src.max_per_stage_descriptor_input_attachments);
        dst.max_per_stage_resources = vk_to_raw_value(&src.max_per_stage_resources);
        dst.max_descriptor_set_samplers = vk_to_raw_value(&src.max_descriptor_set_samplers);
        dst.max_descriptor_set_uniform_buffers = vk_to_raw_value(&src.max_descriptor_set_uniform_buffers);
        dst.max_descriptor_set_uniform_buffers_dynamic = vk_to_raw_value(&src.max_descriptor_set_uniform_buffers_dynamic);
        dst.max_descriptor_set_storage_buffers = vk_to_raw_value(&src.max_descriptor_set_storage_buffers);
        dst.max_descriptor_set_storage_buffers_dynamic = vk_to_raw_value(&src.max_descriptor_set_storage_buffers_dynamic);
        dst.max_descriptor_set_sampled_images = vk_to_raw_value(&src.max_descriptor_set_sampled_images);
        dst.max_descriptor_set_storage_images = vk_to_raw_value(&src.max_descriptor_set_storage_images);
        dst.max_descriptor_set_input_attachments = vk_to_raw_value(&src.max_descriptor_set_input_attachments);
        dst.max_vertex_input_attributes = vk_to_raw_value(&src.max_vertex_input_attributes);
        dst.max_vertex_input_bindings = vk_to_raw_value(&src.max_vertex_input_bindings);
        dst.max_vertex_input_attribute_offset = vk_to_raw_value(&src.max_vertex_input_attribute_offset);
        dst.max_vertex_input_binding_stride = vk_to_raw_value(&src.max_vertex_input_binding_stride);
        dst.max_vertex_output_components = vk_to_raw_value(&src.max_vertex_output_components);
        dst.max_tessellation_generation_level = vk_to_raw_value(&src.max_tessellation_generation_level);
        dst.max_tessellation_patch_size = vk_to_raw_value(&src.max_tessellation_patch_size);
        dst.max_tessellation_control_per_vertex_input_components = vk_to_raw_value(&src.max_tessellation_control_per_vertex_input_components);
        dst.max_tessellation_control_per_vertex_output_components = vk_to_raw_value(&src.max_tessellation_control_per_vertex_output_components);
        dst.max_tessellation_control_per_patch_output_components = vk_to_raw_value(&src.max_tessellation_control_per_patch_output_components);
        dst.max_tessellation_control_total_output_components = vk_to_raw_value(&src.max_tessellation_control_total_output_components);
        dst.max_tessellation_evaluation_input_components = vk_to_raw_value(&src.max_tessellation_evaluation_input_components);
        dst.max_tessellation_evaluation_output_components = vk_to_raw_value(&src.max_tessellation_evaluation_output_components);
        dst.max_geometry_shader_invocations = vk_to_raw_value(&src.max_geometry_shader_invocations);
        dst.max_geometry_input_components = vk_to_raw_value(&src.max_geometry_input_components);
        dst.max_geometry_output_components = vk_to_raw_value(&src.max_geometry_output_components);
        dst.max_geometry_output_vertices = vk_to_raw_value(&src.max_geometry_output_vertices);
        dst.max_geometry_total_output_components = vk_to_raw_value(&src.max_geometry_total_output_components);
        dst.max_fragment_input_components = vk_to_raw_value(&src.max_fragment_input_components);
        dst.max_fragment_output_attachments = vk_to_raw_value(&src.max_fragment_output_attachments);
        dst.max_fragment_dual_src_attachments = vk_to_raw_value(&src.max_fragment_dual_src_attachments);
        dst.max_fragment_combined_output_resources = vk_to_raw_value(&src.max_fragment_combined_output_resources);
        dst.max_compute_shared_memory_size = vk_to_raw_value(&src.max_compute_shared_memory_size);
        dst.max_compute_work_group_count = unsafe { let mut dst_array : [u32; 3] = mem::uninitialized(); vk_to_raw_array(&src.max_compute_work_group_count, &mut dst_array); dst_array };
        dst.max_compute_work_group_invocations = vk_to_raw_value(&src.max_compute_work_group_invocations);
        dst.max_compute_work_group_size = unsafe { let mut dst_array : [u32; 3] = mem::uninitialized(); vk_to_raw_array(&src.max_compute_work_group_size, &mut dst_array); dst_array };
        dst.sub_pixel_precision_bits = vk_to_raw_value(&src.sub_pixel_precision_bits);
        dst.sub_texel_precision_bits = vk_to_raw_value(&src.sub_texel_precision_bits);
        dst.mipmap_precision_bits = vk_to_raw_value(&src.mipmap_precision_bits);
        dst.max_draw_indexed_index_value = vk_to_raw_value(&src.max_draw_indexed_index_value);
        dst.max_draw_indirect_count = vk_to_raw_value(&src.max_draw_indirect_count);
        dst.max_sampler_lod_bias = src.max_sampler_lod_bias;
        dst.max_sampler_anisotropy = src.max_sampler_anisotropy;
        dst.max_viewports = vk_to_raw_value(&src.max_viewports);
        dst.max_viewport_dimensions = unsafe { let mut dst_array : [u32; 2] = mem::uninitialized(); vk_to_raw_array(&src.max_viewport_dimensions, &mut dst_array); dst_array };
        dst.viewport_bounds_range = unsafe { let mut dst_array : [f32; 2] = mem::uninitialized(); to_array(&src.viewport_bounds_range, &mut dst_array); dst_array };
        dst.viewport_sub_pixel_bits = vk_to_raw_value(&src.viewport_sub_pixel_bits);
        dst.min_memory_map_alignment = src.min_memory_map_alignment;
        dst.min_texel_buffer_offset_alignment = vk_to_raw_value(&src.min_texel_buffer_offset_alignment);
        dst.min_uniform_buffer_offset_alignment = vk_to_raw_value(&src.min_uniform_buffer_offset_alignment);
        dst.min_storage_buffer_offset_alignment = vk_to_raw_value(&src.min_storage_buffer_offset_alignment);
        dst.min_texel_offset = vk_to_raw_value(&src.min_texel_offset);
        dst.max_texel_offset = vk_to_raw_value(&src.max_texel_offset);
        dst.min_texel_gather_offset = vk_to_raw_value(&src.min_texel_gather_offset);
        dst.max_texel_gather_offset = vk_to_raw_value(&src.max_texel_gather_offset);
        dst.min_interpolation_offset = src.min_interpolation_offset;
        dst.max_interpolation_offset = src.max_interpolation_offset;
        dst.sub_pixel_interpolation_offset_bits = vk_to_raw_value(&src.sub_pixel_interpolation_offset_bits);
        dst.max_framebuffer_width = vk_to_raw_value(&src.max_framebuffer_width);
        dst.max_framebuffer_height = vk_to_raw_value(&src.max_framebuffer_height);
        dst.max_framebuffer_layers = vk_to_raw_value(&src.max_framebuffer_layers);
        dst.framebuffer_color_sample_counts = vk_to_raw_value(&src.framebuffer_color_sample_counts);
        dst.framebuffer_depth_sample_counts = vk_to_raw_value(&src.framebuffer_depth_sample_counts);
        dst.framebuffer_stencil_sample_counts = vk_to_raw_value(&src.framebuffer_stencil_sample_counts);
        dst.framebuffer_no_attachments_sample_counts = vk_to_raw_value(&src.framebuffer_no_attachments_sample_counts);
        dst.max_color_attachments = vk_to_raw_value(&src.max_color_attachments);
        dst.sampled_image_color_sample_counts = vk_to_raw_value(&src.sampled_image_color_sample_counts);
        dst.sampled_image_integer_sample_counts = vk_to_raw_value(&src.sampled_image_integer_sample_counts);
        dst.sampled_image_depth_sample_counts = vk_to_raw_value(&src.sampled_image_depth_sample_counts);
        dst.sampled_image_stencil_sample_counts = vk_to_raw_value(&src.sampled_image_stencil_sample_counts);
        dst.storage_image_sample_counts = vk_to_raw_value(&src.storage_image_sample_counts);
        dst.max_sample_mask_words = src.max_sample_mask_words;
        dst.timestamp_compute_and_graphics = vk_to_raw_value(&src.timestamp_compute_and_graphics);
        dst.timestamp_period = src.timestamp_period;
        dst.max_clip_distances = vk_to_raw_value(&src.max_clip_distances);
        dst.max_cull_distances = vk_to_raw_value(&src.max_cull_distances);
        dst.max_combined_clip_and_cull_distances = vk_to_raw_value(&src.max_combined_clip_and_cull_distances);
        dst.discrete_queue_priorities = vk_to_raw_value(&src.discrete_queue_priorities);
        dst.point_size_range = unsafe { let mut dst_array : [f32; 2] = mem::uninitialized(); to_array(&src.point_size_range, &mut dst_array); dst_array };
        dst.line_width_range = unsafe { let mut dst_array : [f32; 2] = mem::uninitialized(); to_array(&src.line_width_range, &mut dst_array); dst_array };
        dst.point_size_granularity = src.point_size_granularity;
        dst.line_width_granularity = src.line_width_granularity;
        dst.strict_lines = vk_to_raw_value(&src.strict_lines);
        dst.standard_sample_locations = vk_to_raw_value(&src.standard_sample_locations);
        dst.optimal_buffer_copy_offset_alignment = vk_to_raw_value(&src.optimal_buffer_copy_offset_alignment);
        dst.optimal_buffer_copy_row_pitch_alignment = vk_to_raw_value(&src.optimal_buffer_copy_row_pitch_alignment);
        dst.non_coherent_atom_size = vk_to_raw_value(&src.non_coherent_atom_size);
    }
}

impl Default for VkPhysicalDeviceLimits {
    fn default() -> VkPhysicalDeviceLimits {
        VkPhysicalDeviceLimits {
            max_image_dimension_1d: 0,
            max_image_dimension_2d: 0,
            max_image_dimension_3d: 0,
            max_image_dimension_cube: 0,
            max_image_array_layers: 0,
            max_texel_buffer_elements: 0,
            max_uniform_buffer_range: 0,
            max_storage_buffer_range: 0,
            max_push_constants_size: 0,
            max_memory_allocation_count: 0,
            max_sampler_allocation_count: 0,
            buffer_image_granularity: 0,
            sparse_address_space_size: 0,
            max_bound_descriptor_sets: 0,
            max_per_stage_descriptor_samplers: 0,
            max_per_stage_descriptor_uniform_buffers: 0,
            max_per_stage_descriptor_storage_buffers: 0,
            max_per_stage_descriptor_sampled_images: 0,
            max_per_stage_descriptor_storage_images: 0,
            max_per_stage_descriptor_input_attachments: 0,
            max_per_stage_resources: 0,
            max_descriptor_set_samplers: 0,
            max_descriptor_set_uniform_buffers: 0,
            max_descriptor_set_uniform_buffers_dynamic: 0,
            max_descriptor_set_storage_buffers: 0,
            max_descriptor_set_storage_buffers_dynamic: 0,
            max_descriptor_set_sampled_images: 0,
            max_descriptor_set_storage_images: 0,
            max_descriptor_set_input_attachments: 0,
            max_vertex_input_attributes: 0,
            max_vertex_input_bindings: 0,
            max_vertex_input_attribute_offset: 0,
            max_vertex_input_binding_stride: 0,
            max_vertex_output_components: 0,
            max_tessellation_generation_level: 0,
            max_tessellation_patch_size: 0,
            max_tessellation_control_per_vertex_input_components: 0,
            max_tessellation_control_per_vertex_output_components: 0,
            max_tessellation_control_per_patch_output_components: 0,
            max_tessellation_control_total_output_components: 0,
            max_tessellation_evaluation_input_components: 0,
            max_tessellation_evaluation_output_components: 0,
            max_geometry_shader_invocations: 0,
            max_geometry_input_components: 0,
            max_geometry_output_components: 0,
            max_geometry_output_vertices: 0,
            max_geometry_total_output_components: 0,
            max_fragment_input_components: 0,
            max_fragment_output_attachments: 0,
            max_fragment_dual_src_attachments: 0,
            max_fragment_combined_output_resources: 0,
            max_compute_shared_memory_size: 0,
            max_compute_work_group_count: unsafe { let mut dst_array : [usize; 3] = mem::uninitialized(); fill_vk_array(&mut dst_array); dst_array },
            max_compute_work_group_invocations: 0,
            max_compute_work_group_size: unsafe { let mut dst_array : [usize; 3] = mem::uninitialized(); fill_vk_array(&mut dst_array); dst_array },
            sub_pixel_precision_bits: 0,
            sub_texel_precision_bits: 0,
            mipmap_precision_bits: 0,
            max_draw_indexed_index_value: 0,
            max_draw_indirect_count: 0,
            max_sampler_lod_bias: 0.0,
            max_sampler_anisotropy: 0.0,
            max_viewports: 0,
            max_viewport_dimensions: unsafe { let mut dst_array : [usize; 2] = mem::uninitialized(); fill_vk_array(&mut dst_array); dst_array },
            viewport_bounds_range: [0.0; 2],
            viewport_sub_pixel_bits: 0,
            min_memory_map_alignment: 0,
            min_texel_buffer_offset_alignment: 0,
            min_uniform_buffer_offset_alignment: 0,
            min_storage_buffer_offset_alignment: 0,
            min_texel_offset: 0,
            max_texel_offset: 0,
            min_texel_gather_offset: 0,
            max_texel_gather_offset: 0,
            min_interpolation_offset: 0.0,
            max_interpolation_offset: 0.0,
            sub_pixel_interpolation_offset_bits: 0,
            max_framebuffer_width: 0,
            max_framebuffer_height: 0,
            max_framebuffer_layers: 0,
            framebuffer_color_sample_counts: VkSampleCountFlags::default(),
            framebuffer_depth_sample_counts: VkSampleCountFlags::default(),
            framebuffer_stencil_sample_counts: VkSampleCountFlags::default(),
            framebuffer_no_attachments_sample_counts: VkSampleCountFlags::default(),
            max_color_attachments: 0,
            sampled_image_color_sample_counts: VkSampleCountFlags::default(),
            sampled_image_integer_sample_counts: VkSampleCountFlags::default(),
            sampled_image_depth_sample_counts: VkSampleCountFlags::default(),
            sampled_image_stencil_sample_counts: VkSampleCountFlags::default(),
            storage_image_sample_counts: VkSampleCountFlags::default(),
            max_sample_mask_words: 0,
            timestamp_compute_and_graphics: false,
            timestamp_period: 0.0,
            max_clip_distances: 0,
            max_cull_distances: 0,
            max_combined_clip_and_cull_distances: 0,
            discrete_queue_priorities: 0,
            point_size_range: [0.0; 2],
            line_width_range: [0.0; 2],
            point_size_granularity: 0.0,
            line_width_granularity: 0.0,
            strict_lines: false,
            standard_sample_locations: false,
            optimal_buffer_copy_offset_alignment: 0,
            optimal_buffer_copy_row_pitch_alignment: 0,
            non_coherent_atom_size: 0,
        }
    }
}

impl VkSetup for VkPhysicalDeviceLimits {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceLimits {
    fn vk_free(&mut self) {
        
    }
}