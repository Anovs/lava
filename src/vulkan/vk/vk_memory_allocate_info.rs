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

/// Wrapper for [VkMemoryAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkMemoryAllocateInfo.html)
#[derive(Debug, Clone)]
pub struct VkMemoryAllocateInfo {
    pub allocation_size: usize,
    pub memory_type_index: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkMemoryAllocateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub allocation_size: u64,
    pub memory_type_index: u32,
}

impl VkWrappedType<RawVkMemoryAllocateInfo> for VkMemoryAllocateInfo {
    fn vk_to_raw(src: &VkMemoryAllocateInfo, dst: &mut RawVkMemoryAllocateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::MemoryAllocateInfo);
        dst.next = ptr::null();
        dst.allocation_size = vk_to_raw_value(&src.allocation_size);
        dst.memory_type_index = vk_to_raw_value(&src.memory_type_index);
    }
}

impl VkRawType<VkMemoryAllocateInfo> for RawVkMemoryAllocateInfo {
    fn vk_to_wrapped(src: &RawVkMemoryAllocateInfo) -> VkMemoryAllocateInfo {
        VkMemoryAllocateInfo {
            allocation_size: u64::vk_to_wrapped(&src.allocation_size),
            memory_type_index: u32::vk_to_wrapped(&src.memory_type_index),
        }
    }
}

impl Default for VkMemoryAllocateInfo {
    fn default() -> VkMemoryAllocateInfo {
        VkMemoryAllocateInfo {
            allocation_size: 0,
            memory_type_index: 0,
        }
    }
}

impl VkSetup for VkMemoryAllocateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkMemoryAllocateInfo {
    fn vk_free(&mut self) {
        
    }
}