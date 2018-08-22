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
pub struct RawVkExtent2D {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct VkExtent2D {
    pub width: u32,
    pub height: u32,
}

impl VkRawType<VkExtent2D> for RawVkExtent2D {
    fn vk_to_wrapped(src: &RawVkExtent2D) -> VkExtent2D {
        VkExtent2D {
            width: src.width,
            height: src.height,
        }
    }
}

impl VkWrappedType<RawVkExtent2D> for VkExtent2D {
    fn vk_to_raw(src: &VkExtent2D, dst: &mut RawVkExtent2D) {
        dst.width = src.width;
        dst.height = src.height;
    }
}

impl Default for VkExtent2D {
    fn default() -> VkExtent2D {
        VkExtent2D {
            width: 0,
            height: 0,
        }
    }
}

impl VkSetup for VkExtent2D {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkExtent2D {
    fn vk_free(&mut self) {
        
    }
}