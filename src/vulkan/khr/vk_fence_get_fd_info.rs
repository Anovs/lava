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
use vulkan::vk::{VkFence,RawVkFence};
use vulkan::vk::{VkExternalFenceHandleTypeFlags,RawVkExternalFenceHandleTypeFlags};

/// Wrapper for [VkFenceGetFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceGetFdInfoKHR.html).
#[derive(Debug, Clone)]
pub struct VkFenceGetFdInfo {
    pub fence: VkFence,
    pub handle_type: VkExternalFenceHandleTypeFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkFenceGetFdInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub fence: RawVkFence,
    pub handle_type: RawVkExternalFenceHandleTypeFlags,
}

impl VkWrappedType<RawVkFenceGetFdInfo> for VkFenceGetFdInfo {
    fn vk_to_raw(src: &VkFenceGetFdInfo, dst: &mut RawVkFenceGetFdInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::FenceGetFdInfoKhr);
        dst.next = ptr::null_mut();
        dst.fence = vk_to_raw_value(&src.fence);
        dst.handle_type = vk_to_raw_value(&src.handle_type);
    }
}

impl VkRawType<VkFenceGetFdInfo> for RawVkFenceGetFdInfo {
    fn vk_to_wrapped(src: &RawVkFenceGetFdInfo) -> VkFenceGetFdInfo {
        VkFenceGetFdInfo {
            fence: RawVkFence::vk_to_wrapped(&src.fence),
            handle_type: RawVkExternalFenceHandleTypeFlags::vk_to_wrapped(&src.handle_type),
        }
    }
}

impl Default for VkFenceGetFdInfo {
    fn default() -> VkFenceGetFdInfo {
        VkFenceGetFdInfo {
            fence: Default::default(),
            handle_type: Default::default(),
        }
    }
}

impl VkSetup for VkFenceGetFdInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.fence, fn_table);
    }
}

impl VkFree for RawVkFenceGetFdInfo {
    fn vk_free(&self) {
        
    }
}