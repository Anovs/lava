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

/// Wrapper for [VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX.html)
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceMultiviewPerViewAttributesProperties {
    pub per_view_position_all_components: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceMultiviewPerViewAttributesProperties {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub per_view_position_all_components: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceMultiviewPerViewAttributesProperties> for VkPhysicalDeviceMultiviewPerViewAttributesProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceMultiviewPerViewAttributesProperties, dst: &mut RawVkPhysicalDeviceMultiviewPerViewAttributesProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceMultiviewPerViewAttributesPropertiesNvx);
        dst.next = ptr::null();
        dst.per_view_position_all_components = vk_to_raw_value(&src.per_view_position_all_components);
    }
}

impl VkRawType<VkPhysicalDeviceMultiviewPerViewAttributesProperties> for RawVkPhysicalDeviceMultiviewPerViewAttributesProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceMultiviewPerViewAttributesProperties) -> VkPhysicalDeviceMultiviewPerViewAttributesProperties {
        VkPhysicalDeviceMultiviewPerViewAttributesProperties {
            per_view_position_all_components: u32::vk_to_wrapped(&src.per_view_position_all_components),
        }
    }
}

impl Default for VkPhysicalDeviceMultiviewPerViewAttributesProperties {
    fn default() -> VkPhysicalDeviceMultiviewPerViewAttributesProperties {
        VkPhysicalDeviceMultiviewPerViewAttributesProperties {
            per_view_position_all_components: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceMultiviewPerViewAttributesProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceMultiviewPerViewAttributesProperties {
    fn vk_free(&mut self) {
        
    }
}