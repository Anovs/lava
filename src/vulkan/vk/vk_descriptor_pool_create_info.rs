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
use vulkan::vk::{VkDescriptorPoolCreateFlags,RawVkDescriptorPoolCreateFlags};
use vulkan::vk::{VkDescriptorPoolSize,RawVkDescriptorPoolSize};

/// Wrapper for [VkDescriptorPoolCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDescriptorPoolCreateInfo.html).
#[derive(Debug, Clone)]
pub struct VkDescriptorPoolCreateInfo<'a> {
    pub flags: VkDescriptorPoolCreateFlags,
    pub max_sets: usize,
    pub pool_sizes: &'a [VkDescriptorPoolSize],
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDescriptorPoolCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkDescriptorPoolCreateFlags,
    pub max_sets: u32,
    pub pool_size_count: u32,
    pub pool_sizes: *mut RawVkDescriptorPoolSize,
}

impl<'a> VkWrappedType<RawVkDescriptorPoolCreateInfo> for VkDescriptorPoolCreateInfo<'a> {
    fn vk_to_raw(src: &VkDescriptorPoolCreateInfo, dst: &mut RawVkDescriptorPoolCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DescriptorPoolCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.max_sets = vk_to_raw_value(&src.max_sets);
        dst.pool_size_count = src.pool_sizes.len() as u32;
        dst.pool_sizes = new_ptr_vk_array(src.pool_sizes);
    }
}

impl Default for VkDescriptorPoolCreateInfo<'static> {
    fn default() -> VkDescriptorPoolCreateInfo<'static> {
        VkDescriptorPoolCreateInfo {
            flags: VkDescriptorPoolCreateFlags::default(),
            max_sets: 0,
            pool_sizes: &[],
        }
    }
}

impl<'a> VkSetup for VkDescriptorPoolCreateInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDescriptorPoolCreateInfo {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.pool_size_count as usize, self.pool_sizes);
    }
}