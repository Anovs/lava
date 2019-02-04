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

/// Wrapper for [VkPhysicalDeviceMemoryBudgetPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceMemoryBudgetPropertiesEXT.html)
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceMemoryBudgetProperties {
    pub heap_budget: [usize; 16],
    pub heap_usage: [usize; 16],
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceMemoryBudgetProperties {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub heap_budget: [u64; 16],
    pub heap_usage: [u64; 16],
}

impl VkWrappedType<RawVkPhysicalDeviceMemoryBudgetProperties> for VkPhysicalDeviceMemoryBudgetProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceMemoryBudgetProperties, dst: &mut RawVkPhysicalDeviceMemoryBudgetProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceMemoryBudgetPropertiesExt);
        dst.next = ptr::null();
        dst.heap_budget = unsafe { let mut dst_array : [u64; 16] = mem::uninitialized(); vk_to_raw_array(&src.heap_budget, &mut dst_array); dst_array };
        dst.heap_usage = unsafe { let mut dst_array : [u64; 16] = mem::uninitialized(); vk_to_raw_array(&src.heap_usage, &mut dst_array); dst_array };
    }
}

impl VkRawType<VkPhysicalDeviceMemoryBudgetProperties> for RawVkPhysicalDeviceMemoryBudgetProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceMemoryBudgetProperties) -> VkPhysicalDeviceMemoryBudgetProperties {
        VkPhysicalDeviceMemoryBudgetProperties {
            heap_budget: unsafe { let mut dst_array : [usize; 16] = mem::uninitialized(); vk_to_wrapped_array(&src.heap_budget, &mut dst_array); dst_array },
            heap_usage: unsafe { let mut dst_array : [usize; 16] = mem::uninitialized(); vk_to_wrapped_array(&src.heap_usage, &mut dst_array); dst_array },
        }
    }
}

impl Default for VkPhysicalDeviceMemoryBudgetProperties {
    fn default() -> VkPhysicalDeviceMemoryBudgetProperties {
        VkPhysicalDeviceMemoryBudgetProperties {
            heap_budget: unsafe { let mut dst_array : [usize; 16] = mem::uninitialized(); fill_vk_array(&mut dst_array); dst_array },
            heap_usage: unsafe { let mut dst_array : [usize; 16] = mem::uninitialized(); fill_vk_array(&mut dst_array); dst_array },
        }
    }
}

impl VkSetup for VkPhysicalDeviceMemoryBudgetProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceMemoryBudgetProperties {
    fn vk_free(&mut self) {
        
    }
}