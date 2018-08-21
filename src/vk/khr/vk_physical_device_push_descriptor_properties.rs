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
pub struct RawVkPhysicalDevicePushDescriptorProperties {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub max_push_descriptors: u32,
}

#[derive(Debug, Clone)]
pub struct VkPhysicalDevicePushDescriptorProperties {
    pub max_push_descriptors: usize,
}

impl VkRawType<VkPhysicalDevicePushDescriptorProperties> for RawVkPhysicalDevicePushDescriptorProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDevicePushDescriptorProperties) -> VkPhysicalDevicePushDescriptorProperties {
        VkPhysicalDevicePushDescriptorProperties {
            max_push_descriptors: u32::vk_to_wrapped(&src.max_push_descriptors),
        }
    }
}

impl VkWrappedType<RawVkPhysicalDevicePushDescriptorProperties> for VkPhysicalDevicePushDescriptorProperties {
    fn vk_to_raw(src: &VkPhysicalDevicePushDescriptorProperties, dst: &mut RawVkPhysicalDevicePushDescriptorProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDevicePushDescriptorPropertiesKhr);
        dst.next = ptr::null();
        dst.max_push_descriptors = vk_to_raw_value(&src.max_push_descriptors);
    }
}

impl Default for VkPhysicalDevicePushDescriptorProperties {
    fn default() -> VkPhysicalDevicePushDescriptorProperties {
        VkPhysicalDevicePushDescriptorProperties {
            max_push_descriptors: 0,
        }
    }
}

impl VkSetup for VkPhysicalDevicePushDescriptorProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDevicePushDescriptorProperties {
    fn vk_free(&mut self) {
        
    }
}