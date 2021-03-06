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

/// Wrapper for [VkDedicatedAllocationBufferCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDedicatedAllocationBufferCreateInfoNV.html).
#[derive(Debug, Clone)]
pub struct VkDedicatedAllocationBufferCreateInfo {
    pub dedicated_allocation: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDedicatedAllocationBufferCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub dedicated_allocation: u32,
}

impl VkWrappedType<RawVkDedicatedAllocationBufferCreateInfo> for VkDedicatedAllocationBufferCreateInfo {
    fn vk_to_raw(src: &VkDedicatedAllocationBufferCreateInfo, dst: &mut RawVkDedicatedAllocationBufferCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DedicatedAllocationBufferCreateInfoNv);
        dst.next = ptr::null_mut();
        dst.dedicated_allocation = vk_to_raw_value(&src.dedicated_allocation);
    }
}

impl VkRawType<VkDedicatedAllocationBufferCreateInfo> for RawVkDedicatedAllocationBufferCreateInfo {
    fn vk_to_wrapped(src: &RawVkDedicatedAllocationBufferCreateInfo) -> VkDedicatedAllocationBufferCreateInfo {
        VkDedicatedAllocationBufferCreateInfo {
            dedicated_allocation: u32::vk_to_wrapped(&src.dedicated_allocation),
        }
    }
}

impl Default for VkDedicatedAllocationBufferCreateInfo {
    fn default() -> VkDedicatedAllocationBufferCreateInfo {
        VkDedicatedAllocationBufferCreateInfo {
            dedicated_allocation: false,
        }
    }
}

impl VkSetup for VkDedicatedAllocationBufferCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkDedicatedAllocationBufferCreateInfo {
    fn vk_free(&self) {
        
    }
}