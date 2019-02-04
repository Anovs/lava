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

/// Wrapper for [VkPhysicalDevice16BitStorageFeatures](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDevice16BitStorageFeatures.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDevice16BitStorageFeatures {
    pub storage_buffer_16_bit_access: bool,
    pub uniform_and_storage_buffer_16_bit_access: bool,
    pub storage_push_constant_16: bool,
    pub storage_input_output_16: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDevice16BitStorageFeatures {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub storage_buffer_16_bit_access: u32,
    pub uniform_and_storage_buffer_16_bit_access: u32,
    pub storage_push_constant_16: u32,
    pub storage_input_output_16: u32,
}

impl VkWrappedType<RawVkPhysicalDevice16BitStorageFeatures> for VkPhysicalDevice16BitStorageFeatures {
    fn vk_to_raw(src: &VkPhysicalDevice16BitStorageFeatures, dst: &mut RawVkPhysicalDevice16BitStorageFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDevice16bitStorageFeatures);
        dst.next = ptr::null();
        dst.storage_buffer_16_bit_access = vk_to_raw_value(&src.storage_buffer_16_bit_access);
        dst.uniform_and_storage_buffer_16_bit_access = vk_to_raw_value(&src.uniform_and_storage_buffer_16_bit_access);
        dst.storage_push_constant_16 = vk_to_raw_value(&src.storage_push_constant_16);
        dst.storage_input_output_16 = vk_to_raw_value(&src.storage_input_output_16);
    }
}

impl VkRawType<VkPhysicalDevice16BitStorageFeatures> for RawVkPhysicalDevice16BitStorageFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDevice16BitStorageFeatures) -> VkPhysicalDevice16BitStorageFeatures {
        VkPhysicalDevice16BitStorageFeatures {
            storage_buffer_16_bit_access: u32::vk_to_wrapped(&src.storage_buffer_16_bit_access),
            uniform_and_storage_buffer_16_bit_access: u32::vk_to_wrapped(&src.uniform_and_storage_buffer_16_bit_access),
            storage_push_constant_16: u32::vk_to_wrapped(&src.storage_push_constant_16),
            storage_input_output_16: u32::vk_to_wrapped(&src.storage_input_output_16),
        }
    }
}

impl Default for VkPhysicalDevice16BitStorageFeatures {
    fn default() -> VkPhysicalDevice16BitStorageFeatures {
        VkPhysicalDevice16BitStorageFeatures {
            storage_buffer_16_bit_access: false,
            uniform_and_storage_buffer_16_bit_access: false,
            storage_push_constant_16: false,
            storage_input_output_16: false,
        }
    }
}

impl VkSetup for VkPhysicalDevice16BitStorageFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDevice16BitStorageFeatures {
    fn vk_free(&mut self) {
        
    }
}