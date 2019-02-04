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
use vulkan::ext::{VkDisplayPowerState,RawVkDisplayPowerState};

/// Wrapper for [VkDisplayPowerInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDisplayPowerInfoEXT.html).
#[derive(Debug, Clone)]
pub struct VkDisplayPowerInfo {
    pub power_state: VkDisplayPowerState,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDisplayPowerInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub power_state: RawVkDisplayPowerState,
}

impl VkWrappedType<RawVkDisplayPowerInfo> for VkDisplayPowerInfo {
    fn vk_to_raw(src: &VkDisplayPowerInfo, dst: &mut RawVkDisplayPowerInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DisplayPowerInfoExt);
        dst.next = ptr::null();
        dst.power_state = vk_to_raw_value(&src.power_state);
    }
}

impl VkRawType<VkDisplayPowerInfo> for RawVkDisplayPowerInfo {
    fn vk_to_wrapped(src: &RawVkDisplayPowerInfo) -> VkDisplayPowerInfo {
        VkDisplayPowerInfo {
            power_state: RawVkDisplayPowerState::vk_to_wrapped(&src.power_state),
        }
    }
}

impl Default for VkDisplayPowerInfo {
    fn default() -> VkDisplayPowerInfo {
        VkDisplayPowerInfo {
            power_state: VkDisplayPowerState::default(),
        }
    }
}

impl VkSetup for VkDisplayPowerInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDisplayPowerInfo {
    fn vk_free(&mut self) {
        
    }
}