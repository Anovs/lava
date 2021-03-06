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

/// Wrapper for [VkPhysicalDeviceIDProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceIDProperties.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceIDProperties {
    pub device_uuid: [u8; 16],
    pub driver_uuid: [u8; 16],
    pub device_luid: [u8; 8],
    pub device_node_mask: u32,
    pub device_luidvalid: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceIDProperties {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub device_uuid: [u8; 16],
    pub driver_uuid: [u8; 16],
    pub device_luid: [u8; 8],
    pub device_node_mask: u32,
    pub device_luidvalid: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceIDProperties> for VkPhysicalDeviceIDProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceIDProperties, dst: &mut RawVkPhysicalDeviceIDProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceIdProperties);
        dst.next = ptr::null_mut();
        dst.device_uuid = unsafe { let mut dst_array : [u8; 16] = mem::MaybeUninit::uninit().assume_init(); to_array(&src.device_uuid, &mut dst_array); dst_array };
        dst.driver_uuid = unsafe { let mut dst_array : [u8; 16] = mem::MaybeUninit::uninit().assume_init(); to_array(&src.driver_uuid, &mut dst_array); dst_array };
        dst.device_luid = unsafe { let mut dst_array : [u8; 8] = mem::MaybeUninit::uninit().assume_init(); to_array(&src.device_luid, &mut dst_array); dst_array };
        dst.device_node_mask = src.device_node_mask;
        dst.device_luidvalid = vk_to_raw_value(&src.device_luidvalid);
    }
}

impl VkRawType<VkPhysicalDeviceIDProperties> for RawVkPhysicalDeviceIDProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceIDProperties) -> VkPhysicalDeviceIDProperties {
        VkPhysicalDeviceIDProperties {
            device_uuid: unsafe { let mut dst_array : [u8; 16] = mem::MaybeUninit::uninit().assume_init(); to_array(&src.device_uuid, &mut dst_array); dst_array },
            driver_uuid: unsafe { let mut dst_array : [u8; 16] = mem::MaybeUninit::uninit().assume_init(); to_array(&src.driver_uuid, &mut dst_array); dst_array },
            device_luid: unsafe { let mut dst_array : [u8; 8] = mem::MaybeUninit::uninit().assume_init(); to_array(&src.device_luid, &mut dst_array); dst_array },
            device_node_mask: src.device_node_mask,
            device_luidvalid: u32::vk_to_wrapped(&src.device_luidvalid),
        }
    }
}

impl Default for VkPhysicalDeviceIDProperties {
    fn default() -> VkPhysicalDeviceIDProperties {
        VkPhysicalDeviceIDProperties {
            device_uuid: [0; 16],
            driver_uuid: [0; 16],
            device_luid: [0; 8],
            device_node_mask: 0,
            device_luidvalid: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceIDProperties {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceIDProperties {
    fn vk_free(&self) {
        
    }
}