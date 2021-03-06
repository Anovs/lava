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

/// Wrapper for [VkPerformanceQuerySubmitInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceQuerySubmitInfoKHR.html).
#[derive(Debug, Clone)]
pub struct VkPerformanceQuerySubmitInfo {
    pub counter_pass_index: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPerformanceQuerySubmitInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub counter_pass_index: u32,
}

impl VkWrappedType<RawVkPerformanceQuerySubmitInfo> for VkPerformanceQuerySubmitInfo {
    fn vk_to_raw(src: &VkPerformanceQuerySubmitInfo, dst: &mut RawVkPerformanceQuerySubmitInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PerformanceQuerySubmitInfoKhr);
        dst.next = ptr::null_mut();
        dst.counter_pass_index = vk_to_raw_value(&src.counter_pass_index);
    }
}

impl VkRawType<VkPerformanceQuerySubmitInfo> for RawVkPerformanceQuerySubmitInfo {
    fn vk_to_wrapped(src: &RawVkPerformanceQuerySubmitInfo) -> VkPerformanceQuerySubmitInfo {
        VkPerformanceQuerySubmitInfo {
            counter_pass_index: u32::vk_to_wrapped(&src.counter_pass_index),
        }
    }
}

impl Default for VkPerformanceQuerySubmitInfo {
    fn default() -> VkPerformanceQuerySubmitInfo {
        VkPerformanceQuerySubmitInfo {
            counter_pass_index: 0,
        }
    }
}

impl VkSetup for VkPerformanceQuerySubmitInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPerformanceQuerySubmitInfo {
    fn vk_free(&self) {
        
    }
}