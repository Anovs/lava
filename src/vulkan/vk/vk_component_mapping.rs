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
use vulkan::vk::{VkComponentSwizzle,RawVkComponentSwizzle};

/// Wrapper for [VkComponentMapping](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkComponentMapping.html).
#[derive(Debug, Clone)]
pub struct VkComponentMapping {
    pub r: VkComponentSwizzle,
    pub g: VkComponentSwizzle,
    pub b: VkComponentSwizzle,
    pub a: VkComponentSwizzle,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkComponentMapping {
    pub r: RawVkComponentSwizzle,
    pub g: RawVkComponentSwizzle,
    pub b: RawVkComponentSwizzle,
    pub a: RawVkComponentSwizzle,
}

impl VkWrappedType<RawVkComponentMapping> for VkComponentMapping {
    fn vk_to_raw(src: &VkComponentMapping, dst: &mut RawVkComponentMapping) {
        dst.r = vk_to_raw_value(&src.r);
        dst.g = vk_to_raw_value(&src.g);
        dst.b = vk_to_raw_value(&src.b);
        dst.a = vk_to_raw_value(&src.a);
    }
}

impl VkRawType<VkComponentMapping> for RawVkComponentMapping {
    fn vk_to_wrapped(src: &RawVkComponentMapping) -> VkComponentMapping {
        VkComponentMapping {
            r: RawVkComponentSwizzle::vk_to_wrapped(&src.r),
            g: RawVkComponentSwizzle::vk_to_wrapped(&src.g),
            b: RawVkComponentSwizzle::vk_to_wrapped(&src.b),
            a: RawVkComponentSwizzle::vk_to_wrapped(&src.a),
        }
    }
}

impl Default for VkComponentMapping {
    fn default() -> VkComponentMapping {
        VkComponentMapping {
            r: VkComponentSwizzle::default(),
            g: VkComponentSwizzle::default(),
            b: VkComponentSwizzle::default(),
            a: VkComponentSwizzle::default(),
        }
    }
}

impl VkSetup for VkComponentMapping {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkComponentMapping {
    fn vk_free(&mut self) {
        
    }
}