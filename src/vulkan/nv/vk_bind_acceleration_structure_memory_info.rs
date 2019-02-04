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
use vulkan::nv::{VkAccelerationStructure,RawVkAccelerationStructure};
use vulkan::vk::{VkDeviceMemory,RawVkDeviceMemory};

/// Wrapper for [VkBindAccelerationStructureMemoryInfoNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkBindAccelerationStructureMemoryInfoNV.html).
#[derive(Debug, Clone)]
pub struct VkBindAccelerationStructureMemoryInfo<'a, 'b, 'c> {
    pub acceleration_structure: &'a VkAccelerationStructure,
    pub memory: &'b VkDeviceMemory,
    pub memory_offset: usize,
    pub device_indices: &'c [usize],
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkBindAccelerationStructureMemoryInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub acceleration_structure: RawVkAccelerationStructure,
    pub memory: RawVkDeviceMemory,
    pub memory_offset: u64,
    pub device_index_count: u32,
    pub device_indices: *mut u32,
}

impl<'a, 'b, 'c> VkWrappedType<RawVkBindAccelerationStructureMemoryInfo> for VkBindAccelerationStructureMemoryInfo<'a, 'b, 'c> {
    fn vk_to_raw(src: &VkBindAccelerationStructureMemoryInfo, dst: &mut RawVkBindAccelerationStructureMemoryInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::BindAccelerationStructureMemoryInfoNv);
        dst.next = ptr::null();
        dst.acceleration_structure = vk_to_raw_value(src.acceleration_structure);
        dst.memory = vk_to_raw_value(src.memory);
        dst.memory_offset = vk_to_raw_value(&src.memory_offset);
        dst.device_index_count = src.device_indices.len() as u32;
        dst.device_indices = new_ptr_vk_array(src.device_indices);
    }
}

impl Default for VkBindAccelerationStructureMemoryInfo<'static, 'static, 'static> {
    fn default() -> VkBindAccelerationStructureMemoryInfo<'static, 'static, 'static> {
        VkBindAccelerationStructureMemoryInfo {
            acceleration_structure: vk_null_ref(),
            memory: vk_null_ref(),
            memory_offset: 0,
            device_indices: &[],
        }
    }
}

impl<'a, 'b, 'c> VkSetup for VkBindAccelerationStructureMemoryInfo<'a, 'b, 'c> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkBindAccelerationStructureMemoryInfo {
    fn vk_free(&mut self) {
        free_ptr(self.device_indices);
    }
}