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
use vulkan::vk::{VkImageSubresource,RawVkImageSubresource};
use vulkan::vk::{VkOffset3D,RawVkOffset3D};
use vulkan::vk::{VkExtent3D,RawVkExtent3D};
use vulkan::vk::{VkDeviceMemory,RawVkDeviceMemory};
use vulkan::vk::{VkSparseMemoryBindFlags,RawVkSparseMemoryBindFlags};

/// Wrapper for [VkSparseImageMemoryBind](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSparseImageMemoryBind.html).
#[derive(Debug, Clone)]
pub struct VkSparseImageMemoryBind<'a> {
    pub subresource: VkImageSubresource,
    pub offset: VkOffset3D,
    pub extent: VkExtent3D,
    pub memory: Option<&'a VkDeviceMemory>,
    pub memory_offset: usize,
    pub flags: VkSparseMemoryBindFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSparseImageMemoryBind {
    pub subresource: RawVkImageSubresource,
    pub offset: RawVkOffset3D,
    pub extent: RawVkExtent3D,
    pub memory: RawVkDeviceMemory,
    pub memory_offset: u64,
    pub flags: RawVkSparseMemoryBindFlags,
}

impl<'a> VkWrappedType<RawVkSparseImageMemoryBind> for VkSparseImageMemoryBind<'a> {
    fn vk_to_raw(src: &VkSparseImageMemoryBind, dst: &mut RawVkSparseImageMemoryBind) {
        dst.subresource = vk_to_raw_value(&src.subresource);
        dst.offset = vk_to_raw_value(&src.offset);
        dst.extent = vk_to_raw_value(&src.extent);
        dst.memory = if src.memory.is_some() { vk_to_raw_value(src.memory.unwrap()) } else { 0 };
        dst.memory_offset = vk_to_raw_value(&src.memory_offset);
        dst.flags = vk_to_raw_value(&src.flags);
    }
}

impl Default for VkSparseImageMemoryBind<'static> {
    fn default() -> VkSparseImageMemoryBind<'static> {
        VkSparseImageMemoryBind {
            subresource: VkImageSubresource::default(),
            offset: VkOffset3D::default(),
            extent: VkExtent3D::default(),
            memory: None,
            memory_offset: 0,
            flags: VkSparseMemoryBindFlags::default(),
        }
    }
}

impl<'a> VkSetup for VkSparseImageMemoryBind<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.subresource, fn_table, instance, device);
        VkSetup::vk_setup(&mut self.offset, fn_table, instance, device);
        VkSetup::vk_setup(&mut self.extent, fn_table, instance, device);
    }
}

impl VkFree for RawVkSparseImageMemoryBind {
    fn vk_free(&mut self) {
        RawVkImageSubresource::vk_free(&mut self.subresource);
        RawVkOffset3D::vk_free(&mut self.offset);
        RawVkExtent3D::vk_free(&mut self.extent);
    }
}