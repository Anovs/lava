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
use vulkan::vk::{VkBuffer,RawVkBuffer};
use vulkan::vk::{VkDeviceMemory,RawVkDeviceMemory};

/// Wrapper for [VkBindBufferMemoryInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindBufferMemoryInfo.html).
#[derive(Debug, Clone)]
pub struct VkBindBufferMemoryInfo {
    pub buffer: VkBuffer,
    pub memory: VkDeviceMemory,
    pub memory_offset: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkBindBufferMemoryInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub buffer: RawVkBuffer,
    pub memory: RawVkDeviceMemory,
    pub memory_offset: u64,
}

impl VkWrappedType<RawVkBindBufferMemoryInfo> for VkBindBufferMemoryInfo {
    fn vk_to_raw(src: &VkBindBufferMemoryInfo, dst: &mut RawVkBindBufferMemoryInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::BindBufferMemoryInfo);
        dst.next = ptr::null_mut();
        dst.buffer = vk_to_raw_value(&src.buffer);
        dst.memory = vk_to_raw_value(&src.memory);
        dst.memory_offset = vk_to_raw_value(&src.memory_offset);
    }
}

impl VkRawType<VkBindBufferMemoryInfo> for RawVkBindBufferMemoryInfo {
    fn vk_to_wrapped(src: &RawVkBindBufferMemoryInfo) -> VkBindBufferMemoryInfo {
        VkBindBufferMemoryInfo {
            buffer: RawVkBuffer::vk_to_wrapped(&src.buffer),
            memory: RawVkDeviceMemory::vk_to_wrapped(&src.memory),
            memory_offset: u64::vk_to_wrapped(&src.memory_offset),
        }
    }
}

impl Default for VkBindBufferMemoryInfo {
    fn default() -> VkBindBufferMemoryInfo {
        VkBindBufferMemoryInfo {
            buffer: Default::default(),
            memory: Default::default(),
            memory_offset: 0,
        }
    }
}

impl VkSetup for VkBindBufferMemoryInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.buffer, fn_table);
        VkSetup::vk_setup(&mut self.memory, fn_table);
    }
}

impl VkFree for RawVkBindBufferMemoryInfo {
    fn vk_free(&self) {
        
    }
}