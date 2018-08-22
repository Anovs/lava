// Generated by `scripts/generate_vk.js`

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
use vk::vk_instance_function_table::*;
use vk::vk_instance::*;
use vk::vk_device::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawVkRefreshCycleDuration {
    pub refresh_duration: u64,
}

#[derive(Debug, Clone)]
pub struct VkRefreshCycleDuration {
    pub refresh_duration: usize,
}

impl VkRawType<VkRefreshCycleDuration> for RawVkRefreshCycleDuration {
    fn vk_to_wrapped(src: &RawVkRefreshCycleDuration) -> VkRefreshCycleDuration {
        VkRefreshCycleDuration {
            refresh_duration: u64::vk_to_wrapped(&src.refresh_duration),
        }
    }
}

impl VkWrappedType<RawVkRefreshCycleDuration> for VkRefreshCycleDuration {
    fn vk_to_raw(src: &VkRefreshCycleDuration, dst: &mut RawVkRefreshCycleDuration) {
        dst.refresh_duration = vk_to_raw_value(&src.refresh_duration);
    }
}

impl Default for VkRefreshCycleDuration {
    fn default() -> VkRefreshCycleDuration {
        VkRefreshCycleDuration {
            refresh_duration: 0,
        }
    }
}

impl VkSetup for VkRefreshCycleDuration {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkRefreshCycleDuration {
    fn vk_free(&mut self) {
        
    }
}