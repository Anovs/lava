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

#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceVulkanMemoryModelFeatures {
    pub vulkan_memory_model: bool,
    pub vulkan_memory_model_device_scope: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceVulkanMemoryModelFeatures {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub vulkan_memory_model: u32,
    pub vulkan_memory_model_device_scope: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceVulkanMemoryModelFeatures> for VkPhysicalDeviceVulkanMemoryModelFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceVulkanMemoryModelFeatures, dst: &mut RawVkPhysicalDeviceVulkanMemoryModelFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceVulkanMemoryModelFeaturesKhr);
        dst.next = ptr::null();
        dst.vulkan_memory_model = vk_to_raw_value(&src.vulkan_memory_model);
        dst.vulkan_memory_model_device_scope = vk_to_raw_value(&src.vulkan_memory_model_device_scope);
    }
}

impl VkRawType<VkPhysicalDeviceVulkanMemoryModelFeatures> for RawVkPhysicalDeviceVulkanMemoryModelFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceVulkanMemoryModelFeatures) -> VkPhysicalDeviceVulkanMemoryModelFeatures {
        VkPhysicalDeviceVulkanMemoryModelFeatures {
            vulkan_memory_model: u32::vk_to_wrapped(&src.vulkan_memory_model),
            vulkan_memory_model_device_scope: u32::vk_to_wrapped(&src.vulkan_memory_model_device_scope),
        }
    }
}

impl Default for VkPhysicalDeviceVulkanMemoryModelFeatures {
    fn default() -> VkPhysicalDeviceVulkanMemoryModelFeatures {
        VkPhysicalDeviceVulkanMemoryModelFeatures {
            vulkan_memory_model: false,
            vulkan_memory_model_device_scope: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceVulkanMemoryModelFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceVulkanMemoryModelFeatures {
    fn vk_free(&mut self) {
        
    }
}