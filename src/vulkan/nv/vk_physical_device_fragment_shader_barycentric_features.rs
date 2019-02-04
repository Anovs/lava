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

/// Wrapper for [VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceFragmentShaderBarycentricFeatures {
    pub fragment_shader_barycentric: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceFragmentShaderBarycentricFeatures {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub fragment_shader_barycentric: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceFragmentShaderBarycentricFeatures> for VkPhysicalDeviceFragmentShaderBarycentricFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceFragmentShaderBarycentricFeatures, dst: &mut RawVkPhysicalDeviceFragmentShaderBarycentricFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceFragmentShaderBarycentricFeaturesNv);
        dst.next = ptr::null();
        dst.fragment_shader_barycentric = vk_to_raw_value(&src.fragment_shader_barycentric);
    }
}

impl VkRawType<VkPhysicalDeviceFragmentShaderBarycentricFeatures> for RawVkPhysicalDeviceFragmentShaderBarycentricFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceFragmentShaderBarycentricFeatures) -> VkPhysicalDeviceFragmentShaderBarycentricFeatures {
        VkPhysicalDeviceFragmentShaderBarycentricFeatures {
            fragment_shader_barycentric: u32::vk_to_wrapped(&src.fragment_shader_barycentric),
        }
    }
}

impl Default for VkPhysicalDeviceFragmentShaderBarycentricFeatures {
    fn default() -> VkPhysicalDeviceFragmentShaderBarycentricFeatures {
        VkPhysicalDeviceFragmentShaderBarycentricFeatures {
            fragment_shader_barycentric: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceFragmentShaderBarycentricFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceFragmentShaderBarycentricFeatures {
    fn vk_free(&mut self) {
        
    }
}