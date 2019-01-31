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
use vulkan::vk::{VkImageAspectFlags,RawVkImageAspectFlags};
use vulkan::vk::{VkClearValue,RawVkClearValue};

/// Wrapper for [VkClearAttachment](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkClearAttachment.html)
#[derive(Debug, Clone)]
pub struct VkClearAttachment {
    pub aspect_mask: VkImageAspectFlags,
    pub color_attachment: usize,
    pub clear_value: VkClearValue,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkClearAttachment {
    pub aspect_mask: RawVkImageAspectFlags,
    pub color_attachment: u32,
    pub clear_value: RawVkClearValue,
}

impl VkWrappedType<RawVkClearAttachment> for VkClearAttachment {
    fn vk_to_raw(src: &VkClearAttachment, dst: &mut RawVkClearAttachment) {
        dst.aspect_mask = vk_to_raw_value(&src.aspect_mask);
        dst.color_attachment = vk_to_raw_value(&src.color_attachment);
        dst.clear_value = vk_to_raw_value(&src.clear_value);
    }
}

impl Default for VkClearAttachment {
    fn default() -> VkClearAttachment {
        VkClearAttachment {
            aspect_mask: VkImageAspectFlags::default(),
            color_attachment: 0,
            clear_value: VkClearValue::default(),
        }
    }
}

impl VkSetup for VkClearAttachment {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkClearAttachment {
    fn vk_free(&mut self) {
        
    }
}