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

/// Wrapper for [VkPhysicalDeviceScalarBlockLayoutFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceScalarBlockLayoutFeaturesEXT.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceScalarBlockLayoutFeatures {
    pub scalar_block_layout: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceScalarBlockLayoutFeatures {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub scalar_block_layout: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceScalarBlockLayoutFeatures> for VkPhysicalDeviceScalarBlockLayoutFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceScalarBlockLayoutFeatures, dst: &mut RawVkPhysicalDeviceScalarBlockLayoutFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceScalarBlockLayoutFeaturesExt);
        dst.next = ptr::null();
        dst.scalar_block_layout = vk_to_raw_value(&src.scalar_block_layout);
    }
}

impl VkRawType<VkPhysicalDeviceScalarBlockLayoutFeatures> for RawVkPhysicalDeviceScalarBlockLayoutFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceScalarBlockLayoutFeatures) -> VkPhysicalDeviceScalarBlockLayoutFeatures {
        VkPhysicalDeviceScalarBlockLayoutFeatures {
            scalar_block_layout: u32::vk_to_wrapped(&src.scalar_block_layout),
        }
    }
}

impl Default for VkPhysicalDeviceScalarBlockLayoutFeatures {
    fn default() -> VkPhysicalDeviceScalarBlockLayoutFeatures {
        VkPhysicalDeviceScalarBlockLayoutFeatures {
            scalar_block_layout: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceScalarBlockLayoutFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceScalarBlockLayoutFeatures {
    fn vk_free(&mut self) {
        
    }
}