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
use vulkan::nvx::{VkObjectEntryType,RawVkObjectEntryType};
use vulkan::nvx::{VkObjectEntryUsageFlags,RawVkObjectEntryUsageFlags};

/// Wrapper for [VkObjectTableCreateInfoNVX](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkObjectTableCreateInfoNVX.html).
#[derive(Debug, Clone)]
pub struct VkObjectTableCreateInfo<'a, 'b, 'c> {
    pub object_entry_types: &'a [VkObjectEntryType],
    pub object_entry_counts: &'b [usize],
    pub object_entry_usage_flags: &'c [VkObjectEntryUsageFlags],
    pub max_uniform_buffers_per_descriptor: usize,
    pub max_storage_buffers_per_descriptor: usize,
    pub max_storage_images_per_descriptor: usize,
    pub max_sampled_images_per_descriptor: usize,
    pub max_pipeline_layouts: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkObjectTableCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub object_count: u32,
    pub object_entry_types: *mut RawVkObjectEntryType,
    pub object_entry_counts: *mut u32,
    pub object_entry_usage_flags: *mut RawVkObjectEntryUsageFlags,
    pub max_uniform_buffers_per_descriptor: u32,
    pub max_storage_buffers_per_descriptor: u32,
    pub max_storage_images_per_descriptor: u32,
    pub max_sampled_images_per_descriptor: u32,
    pub max_pipeline_layouts: u32,
}

impl<'a, 'b, 'c> VkWrappedType<RawVkObjectTableCreateInfo> for VkObjectTableCreateInfo<'a, 'b, 'c> {
    fn vk_to_raw(src: &VkObjectTableCreateInfo, dst: &mut RawVkObjectTableCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ObjectTableCreateInfoNvx);
        dst.next = ptr::null();
        dst.object_count = cmp::max(cmp::max(src.object_entry_types.len(), src.object_entry_counts.len()), src.object_entry_usage_flags.len()) as u32;
        dst.object_entry_types = new_ptr_vk_array(src.object_entry_types);
        dst.object_entry_counts = new_ptr_vk_array(src.object_entry_counts);
        dst.object_entry_usage_flags = new_ptr_vk_array(src.object_entry_usage_flags);
        dst.max_uniform_buffers_per_descriptor = vk_to_raw_value(&src.max_uniform_buffers_per_descriptor);
        dst.max_storage_buffers_per_descriptor = vk_to_raw_value(&src.max_storage_buffers_per_descriptor);
        dst.max_storage_images_per_descriptor = vk_to_raw_value(&src.max_storage_images_per_descriptor);
        dst.max_sampled_images_per_descriptor = vk_to_raw_value(&src.max_sampled_images_per_descriptor);
        dst.max_pipeline_layouts = vk_to_raw_value(&src.max_pipeline_layouts);
    }
}

impl Default for VkObjectTableCreateInfo<'static, 'static, 'static> {
    fn default() -> VkObjectTableCreateInfo<'static, 'static, 'static> {
        VkObjectTableCreateInfo {
            object_entry_types: &[],
            object_entry_counts: &[],
            object_entry_usage_flags: &[],
            max_uniform_buffers_per_descriptor: 0,
            max_storage_buffers_per_descriptor: 0,
            max_storage_images_per_descriptor: 0,
            max_sampled_images_per_descriptor: 0,
            max_pipeline_layouts: 0,
        }
    }
}

impl<'a, 'b, 'c> VkSetup for VkObjectTableCreateInfo<'a, 'b, 'c> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkObjectTableCreateInfo {
    fn vk_free(&mut self) {
        free_ptr(self.object_entry_types);
        free_ptr(self.object_entry_counts);
        free_ptr(self.object_entry_usage_flags);
    }
}