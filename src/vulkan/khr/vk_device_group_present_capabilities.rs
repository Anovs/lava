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
use vulkan::khr::{VkDeviceGroupPresentModeFlags,RawVkDeviceGroupPresentModeFlags};

/// Wrapper for [VkDeviceGroupPresentCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceGroupPresentCapabilitiesKHR.html).
#[derive(Debug, Clone)]
pub struct VkDeviceGroupPresentCapabilities {
    pub present_mask: [u32; 32],
    pub modes: VkDeviceGroupPresentModeFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDeviceGroupPresentCapabilities {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub present_mask: [u32; 32],
    pub modes: RawVkDeviceGroupPresentModeFlags,
}

impl VkWrappedType<RawVkDeviceGroupPresentCapabilities> for VkDeviceGroupPresentCapabilities {
    fn vk_to_raw(src: &VkDeviceGroupPresentCapabilities, dst: &mut RawVkDeviceGroupPresentCapabilities) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DeviceGroupPresentCapabilitiesKhr);
        dst.next = ptr::null_mut();
        dst.present_mask = unsafe { let mut dst_array : [u32; 32] = mem::MaybeUninit::uninit().assume_init(); to_array(&src.present_mask, &mut dst_array); dst_array };
        dst.modes = vk_to_raw_value(&src.modes);
    }
}

impl VkRawType<VkDeviceGroupPresentCapabilities> for RawVkDeviceGroupPresentCapabilities {
    fn vk_to_wrapped(src: &RawVkDeviceGroupPresentCapabilities) -> VkDeviceGroupPresentCapabilities {
        VkDeviceGroupPresentCapabilities {
            present_mask: unsafe { let mut dst_array : [u32; 32] = mem::MaybeUninit::uninit().assume_init(); to_array(&src.present_mask, &mut dst_array); dst_array },
            modes: RawVkDeviceGroupPresentModeFlags::vk_to_wrapped(&src.modes),
        }
    }
}

impl Default for VkDeviceGroupPresentCapabilities {
    fn default() -> VkDeviceGroupPresentCapabilities {
        VkDeviceGroupPresentCapabilities {
            present_mask: [0; 32],
            modes: Default::default(),
        }
    }
}

impl VkSetup for VkDeviceGroupPresentCapabilities {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkDeviceGroupPresentCapabilities {
    fn vk_free(&self) {
        
    }
}