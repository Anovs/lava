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
use vulkan::vk::{VkCommandPoolCreateFlags,RawVkCommandPoolCreateFlags};

/// Wrapper for [VkCommandPoolCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCommandPoolCreateInfo.html)
#[derive(Debug, Clone)]
pub struct VkCommandPoolCreateInfo {
    pub flags: VkCommandPoolCreateFlags,
    pub queue_family_index: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkCommandPoolCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkCommandPoolCreateFlags,
    pub queue_family_index: u32,
}

impl VkWrappedType<RawVkCommandPoolCreateInfo> for VkCommandPoolCreateInfo {
    fn vk_to_raw(src: &VkCommandPoolCreateInfo, dst: &mut RawVkCommandPoolCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::CommandPoolCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.queue_family_index = vk_to_raw_value(&src.queue_family_index);
    }
}

impl VkRawType<VkCommandPoolCreateInfo> for RawVkCommandPoolCreateInfo {
    fn vk_to_wrapped(src: &RawVkCommandPoolCreateInfo) -> VkCommandPoolCreateInfo {
        VkCommandPoolCreateInfo {
            flags: RawVkCommandPoolCreateFlags::vk_to_wrapped(&src.flags),
            queue_family_index: u32::vk_to_wrapped(&src.queue_family_index),
        }
    }
}

impl Default for VkCommandPoolCreateInfo {
    fn default() -> VkCommandPoolCreateInfo {
        VkCommandPoolCreateInfo {
            flags: VkCommandPoolCreateFlags::default(),
            queue_family_index: 0,
        }
    }
}

impl VkSetup for VkCommandPoolCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkCommandPoolCreateInfo {
    fn vk_free(&mut self) {
        
    }
}