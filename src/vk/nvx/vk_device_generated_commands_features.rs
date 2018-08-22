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
pub struct RawVkDeviceGeneratedCommandsFeatures {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub compute_binding_point_support: u32,
}

#[derive(Debug, Clone)]
pub struct VkDeviceGeneratedCommandsFeatures {
    pub compute_binding_point_support: bool,
}

impl VkRawType<VkDeviceGeneratedCommandsFeatures> for RawVkDeviceGeneratedCommandsFeatures {
    fn vk_to_wrapped(src: &RawVkDeviceGeneratedCommandsFeatures) -> VkDeviceGeneratedCommandsFeatures {
        VkDeviceGeneratedCommandsFeatures {
            compute_binding_point_support: u32::vk_to_wrapped(&src.compute_binding_point_support),
        }
    }
}

impl VkWrappedType<RawVkDeviceGeneratedCommandsFeatures> for VkDeviceGeneratedCommandsFeatures {
    fn vk_to_raw(src: &VkDeviceGeneratedCommandsFeatures, dst: &mut RawVkDeviceGeneratedCommandsFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DeviceGeneratedCommandsFeaturesNvx);
        dst.next = ptr::null();
        dst.compute_binding_point_support = vk_to_raw_value(&src.compute_binding_point_support);
    }
}

impl Default for VkDeviceGeneratedCommandsFeatures {
    fn default() -> VkDeviceGeneratedCommandsFeatures {
        VkDeviceGeneratedCommandsFeatures {
            compute_binding_point_support: false,
        }
    }
}

impl VkSetup for VkDeviceGeneratedCommandsFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDeviceGeneratedCommandsFeatures {
    fn vk_free(&mut self) {
        
    }
}