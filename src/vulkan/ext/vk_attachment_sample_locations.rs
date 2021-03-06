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
use vulkan::ext::{VkSampleLocationsInfo,RawVkSampleLocationsInfo};

/// Wrapper for [VkAttachmentSampleLocationsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentSampleLocationsEXT.html).
#[derive(Debug, Clone)]
pub struct VkAttachmentSampleLocations {
    pub attachment_index: usize,
    pub sample_locations_info: VkSampleLocationsInfo,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkAttachmentSampleLocations {
    pub attachment_index: u32,
    pub sample_locations_info: RawVkSampleLocationsInfo,
}

impl VkWrappedType<RawVkAttachmentSampleLocations> for VkAttachmentSampleLocations {
    fn vk_to_raw(src: &VkAttachmentSampleLocations, dst: &mut RawVkAttachmentSampleLocations) {
        dst.attachment_index = vk_to_raw_value(&src.attachment_index);
        dst.sample_locations_info = vk_to_raw_value(&src.sample_locations_info);
    }
}

impl VkRawType<VkAttachmentSampleLocations> for RawVkAttachmentSampleLocations {
    fn vk_to_wrapped(src: &RawVkAttachmentSampleLocations) -> VkAttachmentSampleLocations {
        VkAttachmentSampleLocations {
            attachment_index: u32::vk_to_wrapped(&src.attachment_index),
            sample_locations_info: RawVkSampleLocationsInfo::vk_to_wrapped(&src.sample_locations_info),
        }
    }
}

impl Default for VkAttachmentSampleLocations {
    fn default() -> VkAttachmentSampleLocations {
        VkAttachmentSampleLocations {
            attachment_index: 0,
            sample_locations_info: Default::default(),
        }
    }
}

impl VkSetup for VkAttachmentSampleLocations {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.sample_locations_info, fn_table);
    }
}

impl VkFree for RawVkAttachmentSampleLocations {
    fn vk_free(&self) {
        
    }
}