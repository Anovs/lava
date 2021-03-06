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

/// Wrapper for [VkPhysicalDeviceLineRasterizationPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceLineRasterizationPropertiesEXT.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceLineRasterizationProperties {
    pub line_sub_pixel_precision_bits: u32,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceLineRasterizationProperties {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub line_sub_pixel_precision_bits: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceLineRasterizationProperties> for VkPhysicalDeviceLineRasterizationProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceLineRasterizationProperties, dst: &mut RawVkPhysicalDeviceLineRasterizationProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceLineRasterizationPropertiesExt);
        dst.next = ptr::null_mut();
        dst.line_sub_pixel_precision_bits = src.line_sub_pixel_precision_bits;
    }
}

impl VkRawType<VkPhysicalDeviceLineRasterizationProperties> for RawVkPhysicalDeviceLineRasterizationProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceLineRasterizationProperties) -> VkPhysicalDeviceLineRasterizationProperties {
        VkPhysicalDeviceLineRasterizationProperties {
            line_sub_pixel_precision_bits: src.line_sub_pixel_precision_bits,
        }
    }
}

impl Default for VkPhysicalDeviceLineRasterizationProperties {
    fn default() -> VkPhysicalDeviceLineRasterizationProperties {
        VkPhysicalDeviceLineRasterizationProperties {
            line_sub_pixel_precision_bits: 0,
        }
    }
}

impl VkSetup for VkPhysicalDeviceLineRasterizationProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceLineRasterizationProperties {
    fn vk_free(&self) {
        
    }
}