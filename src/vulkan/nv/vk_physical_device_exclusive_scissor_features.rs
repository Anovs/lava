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

/// Wrapper for [VkPhysicalDeviceExclusiveScissorFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExclusiveScissorFeaturesNV.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceExclusiveScissorFeatures {
    pub exclusive_scissor: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceExclusiveScissorFeatures {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub exclusive_scissor: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceExclusiveScissorFeatures> for VkPhysicalDeviceExclusiveScissorFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceExclusiveScissorFeatures, dst: &mut RawVkPhysicalDeviceExclusiveScissorFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceExclusiveScissorFeaturesNv);
        dst.next = ptr::null_mut();
        dst.exclusive_scissor = vk_to_raw_value(&src.exclusive_scissor);
    }
}

impl VkRawType<VkPhysicalDeviceExclusiveScissorFeatures> for RawVkPhysicalDeviceExclusiveScissorFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceExclusiveScissorFeatures) -> VkPhysicalDeviceExclusiveScissorFeatures {
        VkPhysicalDeviceExclusiveScissorFeatures {
            exclusive_scissor: u32::vk_to_wrapped(&src.exclusive_scissor),
        }
    }
}

impl Default for VkPhysicalDeviceExclusiveScissorFeatures {
    fn default() -> VkPhysicalDeviceExclusiveScissorFeatures {
        VkPhysicalDeviceExclusiveScissorFeatures {
            exclusive_scissor: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceExclusiveScissorFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceExclusiveScissorFeatures {
    fn vk_free(&self) {
        
    }
}