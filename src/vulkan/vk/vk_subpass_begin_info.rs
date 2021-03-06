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
use vulkan::vk::{VkSubpassContents,RawVkSubpassContents};

/// Wrapper for [VkSubpassBeginInfo](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassBeginInfo.html).
#[derive(Debug, Clone)]
pub struct VkSubpassBeginInfo {
    pub contents: VkSubpassContents,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSubpassBeginInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub contents: RawVkSubpassContents,
}

impl VkWrappedType<RawVkSubpassBeginInfo> for VkSubpassBeginInfo {
    fn vk_to_raw(src: &VkSubpassBeginInfo, dst: &mut RawVkSubpassBeginInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SubpassBeginInfo);
        dst.next = ptr::null_mut();
        dst.contents = vk_to_raw_value(&src.contents);
    }
}

impl VkRawType<VkSubpassBeginInfo> for RawVkSubpassBeginInfo {
    fn vk_to_wrapped(src: &RawVkSubpassBeginInfo) -> VkSubpassBeginInfo {
        VkSubpassBeginInfo {
            contents: RawVkSubpassContents::vk_to_wrapped(&src.contents),
        }
    }
}

impl Default for VkSubpassBeginInfo {
    fn default() -> VkSubpassBeginInfo {
        VkSubpassBeginInfo {
            contents: Default::default(),
        }
    }
}

impl VkSetup for VkSubpassBeginInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkSubpassBeginInfo {
    fn vk_free(&self) {
        
    }
}