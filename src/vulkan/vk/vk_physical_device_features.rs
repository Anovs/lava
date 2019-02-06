// Generated by `scripts/generate.js`

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
use vulkan::vk::*;

/// Wrapper for [VkPhysicalDeviceFeatures](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceFeatures.html).
///
/// Use the macro `VkPhysicalDeviceFeatures!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkPhysicalDeviceFeatures!(robust_buffer_access, full_draw_index_uint_32)
/// ```
/// ```
/// VkPhysicalDeviceFeatures {
///     robust_buffer_access: true,
///     full_draw_index_uint_32: true,
///     ..VkPhysicalDeviceFeatures::none()
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceFeatures {
    pub robust_buffer_access: bool,
    pub full_draw_index_uint_32: bool,
    pub image_cube_array: bool,
    pub independent_blend: bool,
    pub geometry_shader: bool,
    pub tessellation_shader: bool,
    pub sample_rate_shading: bool,
    pub dual_src_blend: bool,
    pub logic_op: bool,
    pub multi_draw_indirect: bool,
    pub draw_indirect_first_instance: bool,
    pub depth_clamp: bool,
    pub depth_bias_clamp: bool,
    pub fill_mode_non_solid: bool,
    pub depth_bounds: bool,
    pub wide_lines: bool,
    pub large_points: bool,
    pub alpha_to_one: bool,
    pub multi_viewport: bool,
    pub sampler_anisotropy: bool,
    pub texture_compression_etc_2: bool,
    pub texture_compression_astc_ldr: bool,
    pub texture_compression_bc: bool,
    pub occlusion_query_precise: bool,
    pub pipeline_statistics_query: bool,
    pub vertex_pipeline_stores_and_atomics: bool,
    pub fragment_stores_and_atomics: bool,
    pub shader_tessellation_and_geometry_point_size: bool,
    pub shader_image_gather_extended: bool,
    pub shader_storage_image_extended_formats: bool,
    pub shader_storage_image_multisample: bool,
    pub shader_storage_image_read_without_format: bool,
    pub shader_storage_image_write_without_format: bool,
    pub shader_uniform_buffer_array_dynamic_indexing: bool,
    pub shader_sampled_image_array_dynamic_indexing: bool,
    pub shader_storage_buffer_array_dynamic_indexing: bool,
    pub shader_storage_image_array_dynamic_indexing: bool,
    pub shader_clip_distance: bool,
    pub shader_cull_distance: bool,
    pub shader_float_64: bool,
    pub shader_int_64: bool,
    pub shader_int_16: bool,
    pub shader_resource_residency: bool,
    pub shader_resource_min_lod: bool,
    pub sparse_binding: bool,
    pub sparse_residency_buffer: bool,
    pub sparse_residency_image_2d: bool,
    pub sparse_residency_image_3d: bool,
    pub sparse_residency_2_samples: bool,
    pub sparse_residency_4_samples: bool,
    pub sparse_residency_8_samples: bool,
    pub sparse_residency_16_samples: bool,
    pub sparse_residency_aliased: bool,
    pub variable_multisample_rate: bool,
    pub inherited_queries: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceFeatures {
    pub robust_buffer_access: u32,
    pub full_draw_index_uint_32: u32,
    pub image_cube_array: u32,
    pub independent_blend: u32,
    pub geometry_shader: u32,
    pub tessellation_shader: u32,
    pub sample_rate_shading: u32,
    pub dual_src_blend: u32,
    pub logic_op: u32,
    pub multi_draw_indirect: u32,
    pub draw_indirect_first_instance: u32,
    pub depth_clamp: u32,
    pub depth_bias_clamp: u32,
    pub fill_mode_non_solid: u32,
    pub depth_bounds: u32,
    pub wide_lines: u32,
    pub large_points: u32,
    pub alpha_to_one: u32,
    pub multi_viewport: u32,
    pub sampler_anisotropy: u32,
    pub texture_compression_etc_2: u32,
    pub texture_compression_astc_ldr: u32,
    pub texture_compression_bc: u32,
    pub occlusion_query_precise: u32,
    pub pipeline_statistics_query: u32,
    pub vertex_pipeline_stores_and_atomics: u32,
    pub fragment_stores_and_atomics: u32,
    pub shader_tessellation_and_geometry_point_size: u32,
    pub shader_image_gather_extended: u32,
    pub shader_storage_image_extended_formats: u32,
    pub shader_storage_image_multisample: u32,
    pub shader_storage_image_read_without_format: u32,
    pub shader_storage_image_write_without_format: u32,
    pub shader_uniform_buffer_array_dynamic_indexing: u32,
    pub shader_sampled_image_array_dynamic_indexing: u32,
    pub shader_storage_buffer_array_dynamic_indexing: u32,
    pub shader_storage_image_array_dynamic_indexing: u32,
    pub shader_clip_distance: u32,
    pub shader_cull_distance: u32,
    pub shader_float_64: u32,
    pub shader_int_64: u32,
    pub shader_int_16: u32,
    pub shader_resource_residency: u32,
    pub shader_resource_min_lod: u32,
    pub sparse_binding: u32,
    pub sparse_residency_buffer: u32,
    pub sparse_residency_image_2d: u32,
    pub sparse_residency_image_3d: u32,
    pub sparse_residency_2_samples: u32,
    pub sparse_residency_4_samples: u32,
    pub sparse_residency_8_samples: u32,
    pub sparse_residency_16_samples: u32,
    pub sparse_residency_aliased: u32,
    pub variable_multisample_rate: u32,
    pub inherited_queries: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceFeatures> for VkPhysicalDeviceFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceFeatures, dst: &mut RawVkPhysicalDeviceFeatures) {
        dst.robust_buffer_access = vk_to_raw_value(&src.robust_buffer_access);
        dst.full_draw_index_uint_32 = vk_to_raw_value(&src.full_draw_index_uint_32);
        dst.image_cube_array = vk_to_raw_value(&src.image_cube_array);
        dst.independent_blend = vk_to_raw_value(&src.independent_blend);
        dst.geometry_shader = vk_to_raw_value(&src.geometry_shader);
        dst.tessellation_shader = vk_to_raw_value(&src.tessellation_shader);
        dst.sample_rate_shading = vk_to_raw_value(&src.sample_rate_shading);
        dst.dual_src_blend = vk_to_raw_value(&src.dual_src_blend);
        dst.logic_op = vk_to_raw_value(&src.logic_op);
        dst.multi_draw_indirect = vk_to_raw_value(&src.multi_draw_indirect);
        dst.draw_indirect_first_instance = vk_to_raw_value(&src.draw_indirect_first_instance);
        dst.depth_clamp = vk_to_raw_value(&src.depth_clamp);
        dst.depth_bias_clamp = vk_to_raw_value(&src.depth_bias_clamp);
        dst.fill_mode_non_solid = vk_to_raw_value(&src.fill_mode_non_solid);
        dst.depth_bounds = vk_to_raw_value(&src.depth_bounds);
        dst.wide_lines = vk_to_raw_value(&src.wide_lines);
        dst.large_points = vk_to_raw_value(&src.large_points);
        dst.alpha_to_one = vk_to_raw_value(&src.alpha_to_one);
        dst.multi_viewport = vk_to_raw_value(&src.multi_viewport);
        dst.sampler_anisotropy = vk_to_raw_value(&src.sampler_anisotropy);
        dst.texture_compression_etc_2 = vk_to_raw_value(&src.texture_compression_etc_2);
        dst.texture_compression_astc_ldr = vk_to_raw_value(&src.texture_compression_astc_ldr);
        dst.texture_compression_bc = vk_to_raw_value(&src.texture_compression_bc);
        dst.occlusion_query_precise = vk_to_raw_value(&src.occlusion_query_precise);
        dst.pipeline_statistics_query = vk_to_raw_value(&src.pipeline_statistics_query);
        dst.vertex_pipeline_stores_and_atomics = vk_to_raw_value(&src.vertex_pipeline_stores_and_atomics);
        dst.fragment_stores_and_atomics = vk_to_raw_value(&src.fragment_stores_and_atomics);
        dst.shader_tessellation_and_geometry_point_size = vk_to_raw_value(&src.shader_tessellation_and_geometry_point_size);
        dst.shader_image_gather_extended = vk_to_raw_value(&src.shader_image_gather_extended);
        dst.shader_storage_image_extended_formats = vk_to_raw_value(&src.shader_storage_image_extended_formats);
        dst.shader_storage_image_multisample = vk_to_raw_value(&src.shader_storage_image_multisample);
        dst.shader_storage_image_read_without_format = vk_to_raw_value(&src.shader_storage_image_read_without_format);
        dst.shader_storage_image_write_without_format = vk_to_raw_value(&src.shader_storage_image_write_without_format);
        dst.shader_uniform_buffer_array_dynamic_indexing = vk_to_raw_value(&src.shader_uniform_buffer_array_dynamic_indexing);
        dst.shader_sampled_image_array_dynamic_indexing = vk_to_raw_value(&src.shader_sampled_image_array_dynamic_indexing);
        dst.shader_storage_buffer_array_dynamic_indexing = vk_to_raw_value(&src.shader_storage_buffer_array_dynamic_indexing);
        dst.shader_storage_image_array_dynamic_indexing = vk_to_raw_value(&src.shader_storage_image_array_dynamic_indexing);
        dst.shader_clip_distance = vk_to_raw_value(&src.shader_clip_distance);
        dst.shader_cull_distance = vk_to_raw_value(&src.shader_cull_distance);
        dst.shader_float_64 = vk_to_raw_value(&src.shader_float_64);
        dst.shader_int_64 = vk_to_raw_value(&src.shader_int_64);
        dst.shader_int_16 = vk_to_raw_value(&src.shader_int_16);
        dst.shader_resource_residency = vk_to_raw_value(&src.shader_resource_residency);
        dst.shader_resource_min_lod = vk_to_raw_value(&src.shader_resource_min_lod);
        dst.sparse_binding = vk_to_raw_value(&src.sparse_binding);
        dst.sparse_residency_buffer = vk_to_raw_value(&src.sparse_residency_buffer);
        dst.sparse_residency_image_2d = vk_to_raw_value(&src.sparse_residency_image_2d);
        dst.sparse_residency_image_3d = vk_to_raw_value(&src.sparse_residency_image_3d);
        dst.sparse_residency_2_samples = vk_to_raw_value(&src.sparse_residency_2_samples);
        dst.sparse_residency_4_samples = vk_to_raw_value(&src.sparse_residency_4_samples);
        dst.sparse_residency_8_samples = vk_to_raw_value(&src.sparse_residency_8_samples);
        dst.sparse_residency_16_samples = vk_to_raw_value(&src.sparse_residency_16_samples);
        dst.sparse_residency_aliased = vk_to_raw_value(&src.sparse_residency_aliased);
        dst.variable_multisample_rate = vk_to_raw_value(&src.variable_multisample_rate);
        dst.inherited_queries = vk_to_raw_value(&src.inherited_queries);
    }
}

impl VkRawType<VkPhysicalDeviceFeatures> for RawVkPhysicalDeviceFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceFeatures) -> VkPhysicalDeviceFeatures {
        VkPhysicalDeviceFeatures {
            robust_buffer_access: u32::vk_to_wrapped(&src.robust_buffer_access),
            full_draw_index_uint_32: u32::vk_to_wrapped(&src.full_draw_index_uint_32),
            image_cube_array: u32::vk_to_wrapped(&src.image_cube_array),
            independent_blend: u32::vk_to_wrapped(&src.independent_blend),
            geometry_shader: u32::vk_to_wrapped(&src.geometry_shader),
            tessellation_shader: u32::vk_to_wrapped(&src.tessellation_shader),
            sample_rate_shading: u32::vk_to_wrapped(&src.sample_rate_shading),
            dual_src_blend: u32::vk_to_wrapped(&src.dual_src_blend),
            logic_op: u32::vk_to_wrapped(&src.logic_op),
            multi_draw_indirect: u32::vk_to_wrapped(&src.multi_draw_indirect),
            draw_indirect_first_instance: u32::vk_to_wrapped(&src.draw_indirect_first_instance),
            depth_clamp: u32::vk_to_wrapped(&src.depth_clamp),
            depth_bias_clamp: u32::vk_to_wrapped(&src.depth_bias_clamp),
            fill_mode_non_solid: u32::vk_to_wrapped(&src.fill_mode_non_solid),
            depth_bounds: u32::vk_to_wrapped(&src.depth_bounds),
            wide_lines: u32::vk_to_wrapped(&src.wide_lines),
            large_points: u32::vk_to_wrapped(&src.large_points),
            alpha_to_one: u32::vk_to_wrapped(&src.alpha_to_one),
            multi_viewport: u32::vk_to_wrapped(&src.multi_viewport),
            sampler_anisotropy: u32::vk_to_wrapped(&src.sampler_anisotropy),
            texture_compression_etc_2: u32::vk_to_wrapped(&src.texture_compression_etc_2),
            texture_compression_astc_ldr: u32::vk_to_wrapped(&src.texture_compression_astc_ldr),
            texture_compression_bc: u32::vk_to_wrapped(&src.texture_compression_bc),
            occlusion_query_precise: u32::vk_to_wrapped(&src.occlusion_query_precise),
            pipeline_statistics_query: u32::vk_to_wrapped(&src.pipeline_statistics_query),
            vertex_pipeline_stores_and_atomics: u32::vk_to_wrapped(&src.vertex_pipeline_stores_and_atomics),
            fragment_stores_and_atomics: u32::vk_to_wrapped(&src.fragment_stores_and_atomics),
            shader_tessellation_and_geometry_point_size: u32::vk_to_wrapped(&src.shader_tessellation_and_geometry_point_size),
            shader_image_gather_extended: u32::vk_to_wrapped(&src.shader_image_gather_extended),
            shader_storage_image_extended_formats: u32::vk_to_wrapped(&src.shader_storage_image_extended_formats),
            shader_storage_image_multisample: u32::vk_to_wrapped(&src.shader_storage_image_multisample),
            shader_storage_image_read_without_format: u32::vk_to_wrapped(&src.shader_storage_image_read_without_format),
            shader_storage_image_write_without_format: u32::vk_to_wrapped(&src.shader_storage_image_write_without_format),
            shader_uniform_buffer_array_dynamic_indexing: u32::vk_to_wrapped(&src.shader_uniform_buffer_array_dynamic_indexing),
            shader_sampled_image_array_dynamic_indexing: u32::vk_to_wrapped(&src.shader_sampled_image_array_dynamic_indexing),
            shader_storage_buffer_array_dynamic_indexing: u32::vk_to_wrapped(&src.shader_storage_buffer_array_dynamic_indexing),
            shader_storage_image_array_dynamic_indexing: u32::vk_to_wrapped(&src.shader_storage_image_array_dynamic_indexing),
            shader_clip_distance: u32::vk_to_wrapped(&src.shader_clip_distance),
            shader_cull_distance: u32::vk_to_wrapped(&src.shader_cull_distance),
            shader_float_64: u32::vk_to_wrapped(&src.shader_float_64),
            shader_int_64: u32::vk_to_wrapped(&src.shader_int_64),
            shader_int_16: u32::vk_to_wrapped(&src.shader_int_16),
            shader_resource_residency: u32::vk_to_wrapped(&src.shader_resource_residency),
            shader_resource_min_lod: u32::vk_to_wrapped(&src.shader_resource_min_lod),
            sparse_binding: u32::vk_to_wrapped(&src.sparse_binding),
            sparse_residency_buffer: u32::vk_to_wrapped(&src.sparse_residency_buffer),
            sparse_residency_image_2d: u32::vk_to_wrapped(&src.sparse_residency_image_2d),
            sparse_residency_image_3d: u32::vk_to_wrapped(&src.sparse_residency_image_3d),
            sparse_residency_2_samples: u32::vk_to_wrapped(&src.sparse_residency_2_samples),
            sparse_residency_4_samples: u32::vk_to_wrapped(&src.sparse_residency_4_samples),
            sparse_residency_8_samples: u32::vk_to_wrapped(&src.sparse_residency_8_samples),
            sparse_residency_16_samples: u32::vk_to_wrapped(&src.sparse_residency_16_samples),
            sparse_residency_aliased: u32::vk_to_wrapped(&src.sparse_residency_aliased),
            variable_multisample_rate: u32::vk_to_wrapped(&src.variable_multisample_rate),
            inherited_queries: u32::vk_to_wrapped(&src.inherited_queries),
        }
    }
}

impl Default for VkPhysicalDeviceFeatures {
    fn default() -> VkPhysicalDeviceFeatures {
        VkPhysicalDeviceFeatures {
            robust_buffer_access: false,
            full_draw_index_uint_32: false,
            image_cube_array: false,
            independent_blend: false,
            geometry_shader: false,
            tessellation_shader: false,
            sample_rate_shading: false,
            dual_src_blend: false,
            logic_op: false,
            multi_draw_indirect: false,
            draw_indirect_first_instance: false,
            depth_clamp: false,
            depth_bias_clamp: false,
            fill_mode_non_solid: false,
            depth_bounds: false,
            wide_lines: false,
            large_points: false,
            alpha_to_one: false,
            multi_viewport: false,
            sampler_anisotropy: false,
            texture_compression_etc_2: false,
            texture_compression_astc_ldr: false,
            texture_compression_bc: false,
            occlusion_query_precise: false,
            pipeline_statistics_query: false,
            vertex_pipeline_stores_and_atomics: false,
            fragment_stores_and_atomics: false,
            shader_tessellation_and_geometry_point_size: false,
            shader_image_gather_extended: false,
            shader_storage_image_extended_formats: false,
            shader_storage_image_multisample: false,
            shader_storage_image_read_without_format: false,
            shader_storage_image_write_without_format: false,
            shader_uniform_buffer_array_dynamic_indexing: false,
            shader_sampled_image_array_dynamic_indexing: false,
            shader_storage_buffer_array_dynamic_indexing: false,
            shader_storage_image_array_dynamic_indexing: false,
            shader_clip_distance: false,
            shader_cull_distance: false,
            shader_float_64: false,
            shader_int_64: false,
            shader_int_16: false,
            shader_resource_residency: false,
            shader_resource_min_lod: false,
            sparse_binding: false,
            sparse_residency_buffer: false,
            sparse_residency_image_2d: false,
            sparse_residency_image_3d: false,
            sparse_residency_2_samples: false,
            sparse_residency_4_samples: false,
            sparse_residency_8_samples: false,
            sparse_residency_16_samples: false,
            sparse_residency_aliased: false,
            variable_multisample_rate: false,
            inherited_queries: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceFeatures {
    fn vk_free(&self) {
        
    }
}

impl VkPhysicalDeviceFeatures {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkPhysicalDeviceFeatures {
            robust_buffer_access: false,
            full_draw_index_uint_32: false,
            image_cube_array: false,
            independent_blend: false,
            geometry_shader: false,
            tessellation_shader: false,
            sample_rate_shading: false,
            dual_src_blend: false,
            logic_op: false,
            multi_draw_indirect: false,
            draw_indirect_first_instance: false,
            depth_clamp: false,
            depth_bias_clamp: false,
            fill_mode_non_solid: false,
            depth_bounds: false,
            wide_lines: false,
            large_points: false,
            alpha_to_one: false,
            multi_viewport: false,
            sampler_anisotropy: false,
            texture_compression_etc_2: false,
            texture_compression_astc_ldr: false,
            texture_compression_bc: false,
            occlusion_query_precise: false,
            pipeline_statistics_query: false,
            vertex_pipeline_stores_and_atomics: false,
            fragment_stores_and_atomics: false,
            shader_tessellation_and_geometry_point_size: false,
            shader_image_gather_extended: false,
            shader_storage_image_extended_formats: false,
            shader_storage_image_multisample: false,
            shader_storage_image_read_without_format: false,
            shader_storage_image_write_without_format: false,
            shader_uniform_buffer_array_dynamic_indexing: false,
            shader_sampled_image_array_dynamic_indexing: false,
            shader_storage_buffer_array_dynamic_indexing: false,
            shader_storage_image_array_dynamic_indexing: false,
            shader_clip_distance: false,
            shader_cull_distance: false,
            shader_float_64: false,
            shader_int_64: false,
            shader_int_16: false,
            shader_resource_residency: false,
            shader_resource_min_lod: false,
            sparse_binding: false,
            sparse_residency_buffer: false,
            sparse_residency_image_2d: false,
            sparse_residency_image_3d: false,
            sparse_residency_2_samples: false,
            sparse_residency_4_samples: false,
            sparse_residency_8_samples: false,
            sparse_residency_16_samples: false,
            sparse_residency_aliased: false,
            variable_multisample_rate: false,
            inherited_queries: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkPhysicalDeviceFeatures {
            robust_buffer_access: true,
            full_draw_index_uint_32: true,
            image_cube_array: true,
            independent_blend: true,
            geometry_shader: true,
            tessellation_shader: true,
            sample_rate_shading: true,
            dual_src_blend: true,
            logic_op: true,
            multi_draw_indirect: true,
            draw_indirect_first_instance: true,
            depth_clamp: true,
            depth_bias_clamp: true,
            fill_mode_non_solid: true,
            depth_bounds: true,
            wide_lines: true,
            large_points: true,
            alpha_to_one: true,
            multi_viewport: true,
            sampler_anisotropy: true,
            texture_compression_etc_2: true,
            texture_compression_astc_ldr: true,
            texture_compression_bc: true,
            occlusion_query_precise: true,
            pipeline_statistics_query: true,
            vertex_pipeline_stores_and_atomics: true,
            fragment_stores_and_atomics: true,
            shader_tessellation_and_geometry_point_size: true,
            shader_image_gather_extended: true,
            shader_storage_image_extended_formats: true,
            shader_storage_image_multisample: true,
            shader_storage_image_read_without_format: true,
            shader_storage_image_write_without_format: true,
            shader_uniform_buffer_array_dynamic_indexing: true,
            shader_sampled_image_array_dynamic_indexing: true,
            shader_storage_buffer_array_dynamic_indexing: true,
            shader_storage_image_array_dynamic_indexing: true,
            shader_clip_distance: true,
            shader_cull_distance: true,
            shader_float_64: true,
            shader_int_64: true,
            shader_int_16: true,
            shader_resource_residency: true,
            shader_resource_min_lod: true,
            sparse_binding: true,
            sparse_residency_buffer: true,
            sparse_residency_image_2d: true,
            sparse_residency_image_3d: true,
            sparse_residency_2_samples: true,
            sparse_residency_4_samples: true,
            sparse_residency_8_samples: true,
            sparse_residency_16_samples: true,
            sparse_residency_aliased: true,
            variable_multisample_rate: true,
            inherited_queries: true,
        }
    }
    
    pub fn to_u64(&self) -> u64 {
        0
        + if self.robust_buffer_access { 1 } else { 0 }
        + if self.full_draw_index_uint_32 { 2 } else { 0 }
        + if self.image_cube_array { 4 } else { 0 }
        + if self.independent_blend { 8 } else { 0 }
        + if self.geometry_shader { 16 } else { 0 }
        + if self.tessellation_shader { 32 } else { 0 }
        + if self.sample_rate_shading { 64 } else { 0 }
        + if self.dual_src_blend { 128 } else { 0 }
        + if self.logic_op { 256 } else { 0 }
        + if self.multi_draw_indirect { 512 } else { 0 }
        + if self.draw_indirect_first_instance { 1024 } else { 0 }
        + if self.depth_clamp { 2048 } else { 0 }
        + if self.depth_bias_clamp { 4096 } else { 0 }
        + if self.fill_mode_non_solid { 8192 } else { 0 }
        + if self.depth_bounds { 16384 } else { 0 }
        + if self.wide_lines { 32768 } else { 0 }
        + if self.large_points { 65536 } else { 0 }
        + if self.alpha_to_one { 131072 } else { 0 }
        + if self.multi_viewport { 262144 } else { 0 }
        + if self.sampler_anisotropy { 524288 } else { 0 }
        + if self.texture_compression_etc_2 { 1048576 } else { 0 }
        + if self.texture_compression_astc_ldr { 2097152 } else { 0 }
        + if self.texture_compression_bc { 4194304 } else { 0 }
        + if self.occlusion_query_precise { 8388608 } else { 0 }
        + if self.pipeline_statistics_query { 16777216 } else { 0 }
        + if self.vertex_pipeline_stores_and_atomics { 33554432 } else { 0 }
        + if self.fragment_stores_and_atomics { 67108864 } else { 0 }
        + if self.shader_tessellation_and_geometry_point_size { 134217728 } else { 0 }
        + if self.shader_image_gather_extended { 268435456 } else { 0 }
        + if self.shader_storage_image_extended_formats { 536870912 } else { 0 }
        + if self.shader_storage_image_multisample { 1073741824 } else { 0 }
        + if self.shader_storage_image_read_without_format { 2147483648 } else { 0 }
        + if self.shader_storage_image_write_without_format { 4294967296 } else { 0 }
        + if self.shader_uniform_buffer_array_dynamic_indexing { 8589934592 } else { 0 }
        + if self.shader_sampled_image_array_dynamic_indexing { 17179869184 } else { 0 }
        + if self.shader_storage_buffer_array_dynamic_indexing { 34359738368 } else { 0 }
        + if self.shader_storage_image_array_dynamic_indexing { 68719476736 } else { 0 }
        + if self.shader_clip_distance { 137438953472 } else { 0 }
        + if self.shader_cull_distance { 274877906944 } else { 0 }
        + if self.shader_float_64 { 549755813888 } else { 0 }
        + if self.shader_int_64 { 1099511627776 } else { 0 }
        + if self.shader_int_16 { 2199023255552 } else { 0 }
        + if self.shader_resource_residency { 4398046511104 } else { 0 }
        + if self.shader_resource_min_lod { 8796093022208 } else { 0 }
        + if self.sparse_binding { 17592186044416 } else { 0 }
        + if self.sparse_residency_buffer { 35184372088832 } else { 0 }
        + if self.sparse_residency_image_2d { 70368744177664 } else { 0 }
        + if self.sparse_residency_image_3d { 140737488355328 } else { 0 }
        + if self.sparse_residency_2_samples { 281474976710656 } else { 0 }
        + if self.sparse_residency_4_samples { 562949953421312 } else { 0 }
        + if self.sparse_residency_8_samples { 1125899906842624 } else { 0 }
        + if self.sparse_residency_16_samples { 2251799813685248 } else { 0 }
        + if self.sparse_residency_aliased { 4503599627370496 } else { 0 }
        + if self.variable_multisample_rate { 9007199254740992 } else { 0 }
        + if self.inherited_queries { 18014398509481984 } else { 0 }
    }
    
    pub fn from_u64(value: u64) -> Self {
        VkPhysicalDeviceFeatures {
            robust_buffer_access: value & 1 > 0,
            full_draw_index_uint_32: value & 2 > 0,
            image_cube_array: value & 4 > 0,
            independent_blend: value & 8 > 0,
            geometry_shader: value & 16 > 0,
            tessellation_shader: value & 32 > 0,
            sample_rate_shading: value & 64 > 0,
            dual_src_blend: value & 128 > 0,
            logic_op: value & 256 > 0,
            multi_draw_indirect: value & 512 > 0,
            draw_indirect_first_instance: value & 1024 > 0,
            depth_clamp: value & 2048 > 0,
            depth_bias_clamp: value & 4096 > 0,
            fill_mode_non_solid: value & 8192 > 0,
            depth_bounds: value & 16384 > 0,
            wide_lines: value & 32768 > 0,
            large_points: value & 65536 > 0,
            alpha_to_one: value & 131072 > 0,
            multi_viewport: value & 262144 > 0,
            sampler_anisotropy: value & 524288 > 0,
            texture_compression_etc_2: value & 1048576 > 0,
            texture_compression_astc_ldr: value & 2097152 > 0,
            texture_compression_bc: value & 4194304 > 0,
            occlusion_query_precise: value & 8388608 > 0,
            pipeline_statistics_query: value & 16777216 > 0,
            vertex_pipeline_stores_and_atomics: value & 33554432 > 0,
            fragment_stores_and_atomics: value & 67108864 > 0,
            shader_tessellation_and_geometry_point_size: value & 134217728 > 0,
            shader_image_gather_extended: value & 268435456 > 0,
            shader_storage_image_extended_formats: value & 536870912 > 0,
            shader_storage_image_multisample: value & 1073741824 > 0,
            shader_storage_image_read_without_format: value & 2147483648 > 0,
            shader_storage_image_write_without_format: value & 4294967296 > 0,
            shader_uniform_buffer_array_dynamic_indexing: value & 8589934592 > 0,
            shader_sampled_image_array_dynamic_indexing: value & 17179869184 > 0,
            shader_storage_buffer_array_dynamic_indexing: value & 34359738368 > 0,
            shader_storage_image_array_dynamic_indexing: value & 68719476736 > 0,
            shader_clip_distance: value & 137438953472 > 0,
            shader_cull_distance: value & 274877906944 > 0,
            shader_float_64: value & 549755813888 > 0,
            shader_int_64: value & 1099511627776 > 0,
            shader_int_16: value & 2199023255552 > 0,
            shader_resource_residency: value & 4398046511104 > 0,
            shader_resource_min_lod: value & 8796093022208 > 0,
            sparse_binding: value & 17592186044416 > 0,
            sparse_residency_buffer: value & 35184372088832 > 0,
            sparse_residency_image_2d: value & 70368744177664 > 0,
            sparse_residency_image_3d: value & 140737488355328 > 0,
            sparse_residency_2_samples: value & 281474976710656 > 0,
            sparse_residency_4_samples: value & 562949953421312 > 0,
            sparse_residency_8_samples: value & 1125899906842624 > 0,
            sparse_residency_16_samples: value & 2251799813685248 > 0,
            sparse_residency_aliased: value & 4503599627370496 > 0,
            variable_multisample_rate: value & 9007199254740992 > 0,
            inherited_queries: value & 18014398509481984 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkPhysicalDeviceFeatures {
    ( $( $x:ident ),* ) => {
        VkPhysicalDeviceFeatures {
            $($x: true,)*
            ..VkPhysicalDeviceFeatures::none()
        }
    }
}