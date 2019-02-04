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

/// Wrapper for [VkSubpassEndInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSubpassEndInfoKHR.html).
#[derive(Debug, Clone)]
pub struct VkSubpassEndInfo {
    
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSubpassEndInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
}

impl VkWrappedType<RawVkSubpassEndInfo> for VkSubpassEndInfo {
    fn vk_to_raw(src: &VkSubpassEndInfo, dst: &mut RawVkSubpassEndInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SubpassEndInfoKhr);
        dst.next = ptr::null();
    }
}

impl VkRawType<VkSubpassEndInfo> for RawVkSubpassEndInfo {
    fn vk_to_wrapped(src: &RawVkSubpassEndInfo) -> VkSubpassEndInfo {
        VkSubpassEndInfo {
            
        }
    }
}

impl Default for VkSubpassEndInfo {
    fn default() -> VkSubpassEndInfo {
        VkSubpassEndInfo {
            
        }
    }
}

impl VkSetup for VkSubpassEndInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkSubpassEndInfo {
    fn vk_free(&mut self) {
        
    }
}