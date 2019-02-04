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
use vulkan::vk::{VkStructureType,RawVkStructureType};

/// Wrapper for [VkPhysicalDeviceRayTracingPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceRayTracingPropertiesNV.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceRayTracingProperties {
    pub shader_group_handle_size: usize,
    pub max_recursion_depth: usize,
    pub max_shader_group_stride: usize,
    pub shader_group_base_alignment: usize,
    pub max_geometry_count: usize,
    pub max_instance_count: usize,
    pub max_triangle_count: usize,
    pub max_descriptor_set_acceleration_structures: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceRayTracingProperties {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub shader_group_handle_size: u32,
    pub max_recursion_depth: u32,
    pub max_shader_group_stride: u32,
    pub shader_group_base_alignment: u32,
    pub max_geometry_count: u64,
    pub max_instance_count: u64,
    pub max_triangle_count: u64,
    pub max_descriptor_set_acceleration_structures: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceRayTracingProperties> for VkPhysicalDeviceRayTracingProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceRayTracingProperties, dst: &mut RawVkPhysicalDeviceRayTracingProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceRayTracingPropertiesNv);
        dst.next = ptr::null();
        dst.shader_group_handle_size = vk_to_raw_value(&src.shader_group_handle_size);
        dst.max_recursion_depth = vk_to_raw_value(&src.max_recursion_depth);
        dst.max_shader_group_stride = vk_to_raw_value(&src.max_shader_group_stride);
        dst.shader_group_base_alignment = vk_to_raw_value(&src.shader_group_base_alignment);
        dst.max_geometry_count = vk_to_raw_value(&src.max_geometry_count);
        dst.max_instance_count = vk_to_raw_value(&src.max_instance_count);
        dst.max_triangle_count = vk_to_raw_value(&src.max_triangle_count);
        dst.max_descriptor_set_acceleration_structures = vk_to_raw_value(&src.max_descriptor_set_acceleration_structures);
    }
}

impl VkRawType<VkPhysicalDeviceRayTracingProperties> for RawVkPhysicalDeviceRayTracingProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceRayTracingProperties) -> VkPhysicalDeviceRayTracingProperties {
        VkPhysicalDeviceRayTracingProperties {
            shader_group_handle_size: u32::vk_to_wrapped(&src.shader_group_handle_size),
            max_recursion_depth: u32::vk_to_wrapped(&src.max_recursion_depth),
            max_shader_group_stride: u32::vk_to_wrapped(&src.max_shader_group_stride),
            shader_group_base_alignment: u32::vk_to_wrapped(&src.shader_group_base_alignment),
            max_geometry_count: u64::vk_to_wrapped(&src.max_geometry_count),
            max_instance_count: u64::vk_to_wrapped(&src.max_instance_count),
            max_triangle_count: u64::vk_to_wrapped(&src.max_triangle_count),
            max_descriptor_set_acceleration_structures: u32::vk_to_wrapped(&src.max_descriptor_set_acceleration_structures),
        }
    }
}

impl Default for VkPhysicalDeviceRayTracingProperties {
    fn default() -> VkPhysicalDeviceRayTracingProperties {
        VkPhysicalDeviceRayTracingProperties {
            shader_group_handle_size: 0,
            max_recursion_depth: 0,
            max_shader_group_stride: 0,
            shader_group_base_alignment: 0,
            max_geometry_count: 0,
            max_instance_count: 0,
            max_triangle_count: 0,
            max_descriptor_set_acceleration_structures: 0,
        }
    }
}

impl VkSetup for VkPhysicalDeviceRayTracingProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceRayTracingProperties {
    fn vk_free(&mut self) {
        
    }
}