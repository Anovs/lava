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
use vulkan::vk::{VkImage,RawVkImage};
use vulkan::vk::{VkBuffer,RawVkBuffer};

/// Wrapper for [VkDedicatedAllocationMemoryAllocateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDedicatedAllocationMemoryAllocateInfoNV.html).
#[derive(Debug, Clone)]
pub struct VkDedicatedAllocationMemoryAllocateInfo<'a, 'b> {
    pub image: Option<&'a VkImage>,
    pub buffer: Option<&'b VkBuffer>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDedicatedAllocationMemoryAllocateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub image: RawVkImage,
    pub buffer: RawVkBuffer,
}

impl<'a, 'b> VkWrappedType<RawVkDedicatedAllocationMemoryAllocateInfo> for VkDedicatedAllocationMemoryAllocateInfo<'a, 'b> {
    fn vk_to_raw(src: &VkDedicatedAllocationMemoryAllocateInfo, dst: &mut RawVkDedicatedAllocationMemoryAllocateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DedicatedAllocationMemoryAllocateInfoNv);
        dst.next = ptr::null();
        dst.image = if src.image.is_some() { vk_to_raw_value(src.image.unwrap()) } else { 0 };
        dst.buffer = if src.buffer.is_some() { vk_to_raw_value(src.buffer.unwrap()) } else { 0 };
    }
}

impl Default for VkDedicatedAllocationMemoryAllocateInfo<'static, 'static> {
    fn default() -> VkDedicatedAllocationMemoryAllocateInfo<'static, 'static> {
        VkDedicatedAllocationMemoryAllocateInfo {
            image: None,
            buffer: None,
        }
    }
}

impl<'a, 'b> VkSetup for VkDedicatedAllocationMemoryAllocateInfo<'a, 'b> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDedicatedAllocationMemoryAllocateInfo {
    fn vk_free(&mut self) {
        
    }
}