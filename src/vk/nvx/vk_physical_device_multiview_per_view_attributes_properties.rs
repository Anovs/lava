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

#[repr(C)]
#[derive(Debug)]
pub struct RawVkPhysicalDeviceMultiviewPerViewAttributesProperties {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub per_view_position_all_components: u32,
}

#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceMultiviewPerViewAttributesProperties {
    pub per_view_position_all_components: bool,
}

impl VkRawType<VkPhysicalDeviceMultiviewPerViewAttributesProperties> for RawVkPhysicalDeviceMultiviewPerViewAttributesProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceMultiviewPerViewAttributesProperties) -> VkPhysicalDeviceMultiviewPerViewAttributesProperties {
        VkPhysicalDeviceMultiviewPerViewAttributesProperties {
            per_view_position_all_components: u32::vk_to_wrapped(&src.per_view_position_all_components),
        }
    }
}

impl VkWrappedType<RawVkPhysicalDeviceMultiviewPerViewAttributesProperties> for VkPhysicalDeviceMultiviewPerViewAttributesProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceMultiviewPerViewAttributesProperties, dst: &mut RawVkPhysicalDeviceMultiviewPerViewAttributesProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceMultiviewPerViewAttributesPropertiesNvx);
        dst.next = ptr::null();
        dst.per_view_position_all_components = vk_to_raw_value(&src.per_view_position_all_components);
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