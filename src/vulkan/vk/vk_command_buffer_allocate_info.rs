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
use vulkan::vk::{VkCommandPool,RawVkCommandPool};
use vulkan::vk::{VkCommandBufferLevel,RawVkCommandBufferLevel};

/// Wrapper for [VkCommandBufferAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCommandBufferAllocateInfo.html)
#[derive(Debug, Clone)]
pub struct VkCommandBufferAllocateInfo<'a> {
    pub command_pool: &'a VkCommandPool,
    pub level: VkCommandBufferLevel,
    pub command_buffer_count: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkCommandBufferAllocateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub command_pool: RawVkCommandPool,
    pub level: RawVkCommandBufferLevel,
    pub command_buffer_count: u32,
}

impl<'a> VkWrappedType<RawVkCommandBufferAllocateInfo> for VkCommandBufferAllocateInfo<'a> {
    fn vk_to_raw(src: &VkCommandBufferAllocateInfo, dst: &mut RawVkCommandBufferAllocateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::CommandBufferAllocateInfo);
        dst.next = ptr::null();
        dst.command_pool = vk_to_raw_value(src.command_pool);
        dst.level = vk_to_raw_value(&src.level);
        dst.command_buffer_count = vk_to_raw_value(&src.command_buffer_count);
    }
}

impl Default for VkCommandBufferAllocateInfo<'static> {
    fn default() -> VkCommandBufferAllocateInfo<'static> {
        VkCommandBufferAllocateInfo {
            command_pool: vk_null_ref(),
            level: VkCommandBufferLevel::default(),
            command_buffer_count: 0,
        }
    }
}

impl<'a> VkSetup for VkCommandBufferAllocateInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkCommandBufferAllocateInfo {
    fn vk_free(&mut self) {
        
    }
}