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
use vulkan::nv::{VkRayTracingShaderGroupType,RawVkRayTracingShaderGroupType};

/// Wrapper for [VkRayTracingShaderGroupCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkRayTracingShaderGroupCreateInfoNV.html)
#[derive(Debug, Clone)]
pub struct VkRayTracingShaderGroupCreateInfo {
    pub type_: VkRayTracingShaderGroupType,
    pub general_shader: usize,
    pub closest_hit_shader: usize,
    pub any_hit_shader: usize,
    pub intersection_shader: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkRayTracingShaderGroupCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub type_: RawVkRayTracingShaderGroupType,
    pub general_shader: u32,
    pub closest_hit_shader: u32,
    pub any_hit_shader: u32,
    pub intersection_shader: u32,
}

impl VkWrappedType<RawVkRayTracingShaderGroupCreateInfo> for VkRayTracingShaderGroupCreateInfo {
    fn vk_to_raw(src: &VkRayTracingShaderGroupCreateInfo, dst: &mut RawVkRayTracingShaderGroupCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::RayTracingShaderGroupCreateInfoNv);
        dst.next = ptr::null();
        dst.type_ = vk_to_raw_value(&src.type_);
        dst.general_shader = vk_to_raw_value(&src.general_shader);
        dst.closest_hit_shader = vk_to_raw_value(&src.closest_hit_shader);
        dst.any_hit_shader = vk_to_raw_value(&src.any_hit_shader);
        dst.intersection_shader = vk_to_raw_value(&src.intersection_shader);
    }
}

impl VkRawType<VkRayTracingShaderGroupCreateInfo> for RawVkRayTracingShaderGroupCreateInfo {
    fn vk_to_wrapped(src: &RawVkRayTracingShaderGroupCreateInfo) -> VkRayTracingShaderGroupCreateInfo {
        VkRayTracingShaderGroupCreateInfo {
            type_: RawVkRayTracingShaderGroupType::vk_to_wrapped(&src.type_),
            general_shader: u32::vk_to_wrapped(&src.general_shader),
            closest_hit_shader: u32::vk_to_wrapped(&src.closest_hit_shader),
            any_hit_shader: u32::vk_to_wrapped(&src.any_hit_shader),
            intersection_shader: u32::vk_to_wrapped(&src.intersection_shader),
        }
    }
}

impl Default for VkRayTracingShaderGroupCreateInfo {
    fn default() -> VkRayTracingShaderGroupCreateInfo {
        VkRayTracingShaderGroupCreateInfo {
            type_: VkRayTracingShaderGroupType::default(),
            general_shader: 0,
            closest_hit_shader: 0,
            any_hit_shader: 0,
            intersection_shader: 0,
        }
    }
}

impl VkSetup for VkRayTracingShaderGroupCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkRayTracingShaderGroupCreateInfo {
    fn vk_free(&mut self) {
        
    }
}