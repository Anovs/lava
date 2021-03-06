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

/// Wrapper for [VkPerformanceMarkerInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceMarkerInfoINTEL.html).
#[derive(Debug, Clone)]
pub struct VkPerformanceMarkerInfo {
    pub marker: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPerformanceMarkerInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub marker: u64,
}

impl VkWrappedType<RawVkPerformanceMarkerInfo> for VkPerformanceMarkerInfo {
    fn vk_to_raw(src: &VkPerformanceMarkerInfo, dst: &mut RawVkPerformanceMarkerInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PerformanceMarkerInfoIntel);
        dst.next = ptr::null_mut();
        dst.marker = vk_to_raw_value(&src.marker);
    }
}

impl VkRawType<VkPerformanceMarkerInfo> for RawVkPerformanceMarkerInfo {
    fn vk_to_wrapped(src: &RawVkPerformanceMarkerInfo) -> VkPerformanceMarkerInfo {
        VkPerformanceMarkerInfo {
            marker: u64::vk_to_wrapped(&src.marker),
        }
    }
}

impl Default for VkPerformanceMarkerInfo {
    fn default() -> VkPerformanceMarkerInfo {
        VkPerformanceMarkerInfo {
            marker: 0,
        }
    }
}

impl VkSetup for VkPerformanceMarkerInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPerformanceMarkerInfo {
    fn vk_free(&self) {
        
    }
}