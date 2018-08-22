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
pub struct RawVkSampleLocation {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct VkSampleLocation {
    pub x: f32,
    pub y: f32,
}

impl VkRawType<VkSampleLocation> for RawVkSampleLocation {
    fn vk_to_wrapped(src: &RawVkSampleLocation) -> VkSampleLocation {
        VkSampleLocation {
            x: src.x,
            y: src.y,
        }
    }
}

impl VkWrappedType<RawVkSampleLocation> for VkSampleLocation {
    fn vk_to_raw(src: &VkSampleLocation, dst: &mut RawVkSampleLocation) {
        dst.x = src.x;
        dst.y = src.y;
    }
}

impl Default for VkSampleLocation {
    fn default() -> VkSampleLocation {
        VkSampleLocation {
            x: 0.0,
            y: 0.0,
        }
    }
}

impl VkSetup for VkSampleLocation {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkSampleLocation {
    fn vk_free(&mut self) {
        
    }
}