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
use vulkan::khr::{VkDisplayPlaneCapabilities,RawVkDisplayPlaneCapabilities};

/// Wrapper for [VkDisplayPlaneCapabilities2KHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDisplayPlaneCapabilities2KHR.html)
#[derive(Debug, Clone)]
pub struct VkDisplayPlaneCapabilities2 {
    pub capabilities: VkDisplayPlaneCapabilities,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDisplayPlaneCapabilities2 {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub capabilities: RawVkDisplayPlaneCapabilities,
}

impl VkWrappedType<RawVkDisplayPlaneCapabilities2> for VkDisplayPlaneCapabilities2 {
    fn vk_to_raw(src: &VkDisplayPlaneCapabilities2, dst: &mut RawVkDisplayPlaneCapabilities2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DisplayPlaneCapabilities2Khr);
        dst.next = ptr::null();
        dst.capabilities = vk_to_raw_value(&src.capabilities);
    }
}

impl VkRawType<VkDisplayPlaneCapabilities2> for RawVkDisplayPlaneCapabilities2 {
    fn vk_to_wrapped(src: &RawVkDisplayPlaneCapabilities2) -> VkDisplayPlaneCapabilities2 {
        VkDisplayPlaneCapabilities2 {
            capabilities: RawVkDisplayPlaneCapabilities::vk_to_wrapped(&src.capabilities),
        }
    }
}

impl Default for VkDisplayPlaneCapabilities2 {
    fn default() -> VkDisplayPlaneCapabilities2 {
        VkDisplayPlaneCapabilities2 {
            capabilities: VkDisplayPlaneCapabilities::default(),
        }
    }
}

impl VkSetup for VkDisplayPlaneCapabilities2 {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.capabilities, fn_table, instance, device);
    }
}

impl VkFree for RawVkDisplayPlaneCapabilities2 {
    fn vk_free(&mut self) {
        RawVkDisplayPlaneCapabilities::vk_free(&mut self.capabilities);
    }
}