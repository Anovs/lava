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

/// Wrapper for [VkPhysicalDevice8BitStorageFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDevice8BitStorageFeaturesKHR.html)
#[derive(Debug, Clone)]
pub struct VkPhysicalDevice8BitStorageFeatures {
    pub storage_buffer_8_bit_access: bool,
    pub uniform_and_storage_buffer_8_bit_access: bool,
    pub storage_push_constant_8: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDevice8BitStorageFeatures {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub storage_buffer_8_bit_access: u32,
    pub uniform_and_storage_buffer_8_bit_access: u32,
    pub storage_push_constant_8: u32,
}

impl VkWrappedType<RawVkPhysicalDevice8BitStorageFeatures> for VkPhysicalDevice8BitStorageFeatures {
    fn vk_to_raw(src: &VkPhysicalDevice8BitStorageFeatures, dst: &mut RawVkPhysicalDevice8BitStorageFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDevice8bitStorageFeaturesKhr);
        dst.next = ptr::null();
        dst.storage_buffer_8_bit_access = vk_to_raw_value(&src.storage_buffer_8_bit_access);
        dst.uniform_and_storage_buffer_8_bit_access = vk_to_raw_value(&src.uniform_and_storage_buffer_8_bit_access);
        dst.storage_push_constant_8 = vk_to_raw_value(&src.storage_push_constant_8);
    }
}

impl VkRawType<VkPhysicalDevice8BitStorageFeatures> for RawVkPhysicalDevice8BitStorageFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDevice8BitStorageFeatures) -> VkPhysicalDevice8BitStorageFeatures {
        VkPhysicalDevice8BitStorageFeatures {
            storage_buffer_8_bit_access: u32::vk_to_wrapped(&src.storage_buffer_8_bit_access),
            uniform_and_storage_buffer_8_bit_access: u32::vk_to_wrapped(&src.uniform_and_storage_buffer_8_bit_access),
            storage_push_constant_8: u32::vk_to_wrapped(&src.storage_push_constant_8),
        }
    }
}

impl Default for VkPhysicalDevice8BitStorageFeatures {
    fn default() -> VkPhysicalDevice8BitStorageFeatures {
        VkPhysicalDevice8BitStorageFeatures {
            storage_buffer_8_bit_access: false,
            uniform_and_storage_buffer_8_bit_access: false,
            storage_push_constant_8: false,
        }
    }
}

impl VkSetup for VkPhysicalDevice8BitStorageFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDevice8BitStorageFeatures {
    fn vk_free(&mut self) {
        
    }
}