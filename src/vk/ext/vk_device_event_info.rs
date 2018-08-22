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
use vk::vk_structure_type::*;
use vk::ext::vk_device_event_type::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawVkDeviceEventInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub device_event: RawVkDeviceEventType,
}

#[derive(Debug, Clone)]
pub struct VkDeviceEventInfo {
    pub device_event: VkDeviceEventType,
}

impl VkRawType<VkDeviceEventInfo> for RawVkDeviceEventInfo {
    fn vk_to_wrapped(src: &RawVkDeviceEventInfo) -> VkDeviceEventInfo {
        VkDeviceEventInfo {
            device_event: RawVkDeviceEventType::vk_to_wrapped(&src.device_event),
        }
    }
}

impl VkWrappedType<RawVkDeviceEventInfo> for VkDeviceEventInfo {
    fn vk_to_raw(src: &VkDeviceEventInfo, dst: &mut RawVkDeviceEventInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DeviceEventInfoExt);
        dst.next = ptr::null();
        dst.device_event = vk_to_raw_value(&src.device_event);
    }
}

impl Default for VkDeviceEventInfo {
    fn default() -> VkDeviceEventInfo {
        VkDeviceEventInfo {
            device_event: VkDeviceEventType::default(),
        }
    }
}

impl VkSetup for VkDeviceEventInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDeviceEventInfo {
    fn vk_free(&mut self) {
        
    }
}