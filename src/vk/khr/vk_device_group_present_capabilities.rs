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
use vk::khr::vk_device_group_present_mode_flags::*;

#[repr(C)]
pub struct RawVkDeviceGroupPresentCapabilities {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub present_mask: [u32; 32],
    pub modes: RawVkDeviceGroupPresentModeFlags,
}

#[derive(Debug, Clone)]
pub struct VkDeviceGroupPresentCapabilities {
    pub present_mask: [u32; 32],
    pub modes: VkDeviceGroupPresentModeFlags,
}

impl VkRawType<VkDeviceGroupPresentCapabilities> for RawVkDeviceGroupPresentCapabilities {
    fn vk_to_wrapped(src: &RawVkDeviceGroupPresentCapabilities) -> VkDeviceGroupPresentCapabilities {
        VkDeviceGroupPresentCapabilities {
            present_mask: unsafe { let mut dst_array : [u32; 32] = mem::uninitialized(); to_array(&src.present_mask, &mut dst_array); dst_array },
            modes: RawVkDeviceGroupPresentModeFlags::vk_to_wrapped(&src.modes),
        }
    }
}

impl VkWrappedType<RawVkDeviceGroupPresentCapabilities> for VkDeviceGroupPresentCapabilities {
    fn vk_to_raw(src: &VkDeviceGroupPresentCapabilities, dst: &mut RawVkDeviceGroupPresentCapabilities) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DeviceGroupPresentCapabilitiesKhr);
        dst.next = ptr::null();
        dst.present_mask = unsafe { let mut dst_array : [u32; 32] = mem::uninitialized(); to_array(&src.present_mask, &mut dst_array); dst_array };
        dst.modes = vk_to_raw_value(&src.modes);
    }
}

impl Default for VkDeviceGroupPresentCapabilities {
    fn default() -> VkDeviceGroupPresentCapabilities {
        VkDeviceGroupPresentCapabilities {
            present_mask: [0; 32],
            modes: VkDeviceGroupPresentModeFlags::default(),
        }
    }
}

impl VkSetup for VkDeviceGroupPresentCapabilities {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDeviceGroupPresentCapabilities {
    fn vk_free(&mut self) {
        
    }
}