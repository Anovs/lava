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

/// Wrapper for [VkPhysicalDeviceSamplerYcbcrConversionFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSamplerYcbcrConversionFeatures.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeatures {
    pub sampler_ycbcr_conversion: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceSamplerYcbcrConversionFeatures {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub sampler_ycbcr_conversion: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceSamplerYcbcrConversionFeatures> for VkPhysicalDeviceSamplerYcbcrConversionFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceSamplerYcbcrConversionFeatures, dst: &mut RawVkPhysicalDeviceSamplerYcbcrConversionFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceSamplerYcbcrConversionFeatures);
        dst.next = ptr::null_mut();
        dst.sampler_ycbcr_conversion = vk_to_raw_value(&src.sampler_ycbcr_conversion);
    }
}

impl VkRawType<VkPhysicalDeviceSamplerYcbcrConversionFeatures> for RawVkPhysicalDeviceSamplerYcbcrConversionFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceSamplerYcbcrConversionFeatures) -> VkPhysicalDeviceSamplerYcbcrConversionFeatures {
        VkPhysicalDeviceSamplerYcbcrConversionFeatures {
            sampler_ycbcr_conversion: u32::vk_to_wrapped(&src.sampler_ycbcr_conversion),
        }
    }
}

impl Default for VkPhysicalDeviceSamplerYcbcrConversionFeatures {
    fn default() -> VkPhysicalDeviceSamplerYcbcrConversionFeatures {
        VkPhysicalDeviceSamplerYcbcrConversionFeatures {
            sampler_ycbcr_conversion: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceSamplerYcbcrConversionFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceSamplerYcbcrConversionFeatures {
    fn vk_free(&self) {
        
    }
}