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
use vulkan::vk::{VkAccessFlags,RawVkAccessFlags};
use vulkan::vk::{VkImageLayout,RawVkImageLayout};
use vulkan::vk::{VkImage,RawVkImage};
use vulkan::vk::{VkImageSubresourceRange,RawVkImageSubresourceRange};

/// Wrapper for [VkImageMemoryBarrier](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageMemoryBarrier.html).
#[derive(Debug, Clone)]
pub struct VkImageMemoryBarrier {
    pub src_access_mask: VkAccessFlags,
    pub dst_access_mask: VkAccessFlags,
    pub old_layout: VkImageLayout,
    pub new_layout: VkImageLayout,
    pub src_queue_family_index: usize,
    pub dst_queue_family_index: usize,
    pub image: VkImage,
    pub subresource_range: VkImageSubresourceRange,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkImageMemoryBarrier {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub src_access_mask: RawVkAccessFlags,
    pub dst_access_mask: RawVkAccessFlags,
    pub old_layout: RawVkImageLayout,
    pub new_layout: RawVkImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: RawVkImage,
    pub subresource_range: RawVkImageSubresourceRange,
}

impl VkWrappedType<RawVkImageMemoryBarrier> for VkImageMemoryBarrier {
    fn vk_to_raw(src: &VkImageMemoryBarrier, dst: &mut RawVkImageMemoryBarrier) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ImageMemoryBarrier);
        dst.next = ptr::null_mut();
        dst.src_access_mask = vk_to_raw_value(&src.src_access_mask);
        dst.dst_access_mask = vk_to_raw_value(&src.dst_access_mask);
        dst.old_layout = vk_to_raw_value(&src.old_layout);
        dst.new_layout = vk_to_raw_value(&src.new_layout);
        dst.src_queue_family_index = vk_to_raw_value(&src.src_queue_family_index);
        dst.dst_queue_family_index = vk_to_raw_value(&src.dst_queue_family_index);
        dst.image = vk_to_raw_value(&src.image);
        dst.subresource_range = vk_to_raw_value(&src.subresource_range);
    }
}

impl VkRawType<VkImageMemoryBarrier> for RawVkImageMemoryBarrier {
    fn vk_to_wrapped(src: &RawVkImageMemoryBarrier) -> VkImageMemoryBarrier {
        VkImageMemoryBarrier {
            src_access_mask: RawVkAccessFlags::vk_to_wrapped(&src.src_access_mask),
            dst_access_mask: RawVkAccessFlags::vk_to_wrapped(&src.dst_access_mask),
            old_layout: RawVkImageLayout::vk_to_wrapped(&src.old_layout),
            new_layout: RawVkImageLayout::vk_to_wrapped(&src.new_layout),
            src_queue_family_index: u32::vk_to_wrapped(&src.src_queue_family_index),
            dst_queue_family_index: u32::vk_to_wrapped(&src.dst_queue_family_index),
            image: RawVkImage::vk_to_wrapped(&src.image),
            subresource_range: RawVkImageSubresourceRange::vk_to_wrapped(&src.subresource_range),
        }
    }
}

impl Default for VkImageMemoryBarrier {
    fn default() -> VkImageMemoryBarrier {
        VkImageMemoryBarrier {
            src_access_mask: Default::default(),
            dst_access_mask: Default::default(),
            old_layout: Default::default(),
            new_layout: Default::default(),
            src_queue_family_index: 0,
            dst_queue_family_index: 0,
            image: Default::default(),
            subresource_range: Default::default(),
        }
    }
}

impl VkSetup for VkImageMemoryBarrier {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.image, fn_table);
        VkSetup::vk_setup(&mut self.subresource_range, fn_table);
    }
}

impl VkFree for RawVkImageMemoryBarrier {
    fn vk_free(&self) {
        
    }
}