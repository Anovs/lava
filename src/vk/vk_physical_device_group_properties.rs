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
use vk::vk_physical_device::*;

#[repr(C)]
pub struct RawVkPhysicalDeviceGroupProperties {
    s_type: RawVkStructureType,
    next: *const c_void,
    physical_device_count: u32,
    physical_devices: [RawVkPhysicalDevice; 32],
    subset_allocation: u32,
}

#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceGroupProperties {
    pub physical_devices: Vec<VkPhysicalDevice>,
    pub subset_allocation: bool,
}

impl VkRawType<VkPhysicalDeviceGroupProperties> for RawVkPhysicalDeviceGroupProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceGroupProperties) -> VkPhysicalDeviceGroupProperties {
        VkPhysicalDeviceGroupProperties {
            physical_devices: new_vk_array(src.physical_device_count, src.physical_devices.as_ptr()),
            subset_allocation: u32::vk_to_wrapped(&src.subset_allocation),
        }
    }
}

impl VkWrappedType<RawVkPhysicalDeviceGroupProperties> for VkPhysicalDeviceGroupProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceGroupProperties, dst: &mut RawVkPhysicalDeviceGroupProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceGroupProperties);
        dst.next = ptr::null();
        dst.physical_device_count = src.physical_devices.len() as u32;
        dst.physical_devices = unsafe { let mut dst_array : [RawVkPhysicalDevice; 32] = mem::uninitialized(); vk_to_raw_array(&src.physical_devices, &mut dst_array); dst_array };
        dst.subset_allocation = vk_to_raw_value(&src.subset_allocation);
    }
}

impl Default for VkPhysicalDeviceGroupProperties {
    fn default() -> VkPhysicalDeviceGroupProperties {
        VkPhysicalDeviceGroupProperties {
            physical_devices: Vec::new(),
            subset_allocation: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceGroupProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceGroupProperties {
    fn vk_free(&mut self) {
        
    }
}