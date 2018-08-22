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
pub struct RawVkPastPresentationTiming {
    pub present_id: u32,
    pub desired_present_time: u64,
    pub actual_present_time: u64,
    pub earliest_present_time: u64,
    pub present_margin: u64,
}

#[derive(Debug, Clone)]
pub struct VkPastPresentationTiming {
    pub present_id: usize,
    pub desired_present_time: usize,
    pub actual_present_time: usize,
    pub earliest_present_time: usize,
    pub present_margin: usize,
}

impl VkRawType<VkPastPresentationTiming> for RawVkPastPresentationTiming {
    fn vk_to_wrapped(src: &RawVkPastPresentationTiming) -> VkPastPresentationTiming {
        VkPastPresentationTiming {
            present_id: u32::vk_to_wrapped(&src.present_id),
            desired_present_time: u64::vk_to_wrapped(&src.desired_present_time),
            actual_present_time: u64::vk_to_wrapped(&src.actual_present_time),
            earliest_present_time: u64::vk_to_wrapped(&src.earliest_present_time),
            present_margin: u64::vk_to_wrapped(&src.present_margin),
        }
    }
}

impl VkWrappedType<RawVkPastPresentationTiming> for VkPastPresentationTiming {
    fn vk_to_raw(src: &VkPastPresentationTiming, dst: &mut RawVkPastPresentationTiming) {
        dst.present_id = vk_to_raw_value(&src.present_id);
        dst.desired_present_time = vk_to_raw_value(&src.desired_present_time);
        dst.actual_present_time = vk_to_raw_value(&src.actual_present_time);
        dst.earliest_present_time = vk_to_raw_value(&src.earliest_present_time);
        dst.present_margin = vk_to_raw_value(&src.present_margin);
    }
}

impl Default for VkPastPresentationTiming {
    fn default() -> VkPastPresentationTiming {
        VkPastPresentationTiming {
            present_id: 0,
            desired_present_time: 0,
            actual_present_time: 0,
            earliest_present_time: 0,
            present_margin: 0,
        }
    }
}

impl VkSetup for VkPastPresentationTiming {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPastPresentationTiming {
    fn vk_free(&mut self) {
        
    }
}