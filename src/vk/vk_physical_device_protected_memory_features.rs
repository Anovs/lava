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
pub struct RawVkPhysicalDeviceProtectedMemoryFeatures {
    s_type: RawVkStructureType,
    next: *const c_void,
    protected_memory: u32,
}

#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceProtectedMemoryFeatures {
    pub protected_memory: bool,
}

impl VkRawType<VkPhysicalDeviceProtectedMemoryFeatures> for RawVkPhysicalDeviceProtectedMemoryFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceProtectedMemoryFeatures) -> VkPhysicalDeviceProtectedMemoryFeatures {
        VkPhysicalDeviceProtectedMemoryFeatures {
            protected_memory: u32::vk_to_wrapped(&src.protected_memory),
        }
    }
}

impl VkWrappedType<RawVkPhysicalDeviceProtectedMemoryFeatures> for VkPhysicalDeviceProtectedMemoryFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceProtectedMemoryFeatures, dst: &mut RawVkPhysicalDeviceProtectedMemoryFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceProtectedMemoryFeatures);
        dst.next = ptr::null();
        dst.protected_memory = vk_to_raw_value(&src.protected_memory);
    }
}

impl Default for VkPhysicalDeviceProtectedMemoryFeatures {
    fn default() -> VkPhysicalDeviceProtectedMemoryFeatures {
        VkPhysicalDeviceProtectedMemoryFeatures {
            protected_memory: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceProtectedMemoryFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceProtectedMemoryFeatures {
    fn vk_free(&mut self) {
        
    }
}