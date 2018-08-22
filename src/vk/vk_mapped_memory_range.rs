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
use vk::vk_device_memory::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawVkMappedMemoryRange {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub memory: RawVkDeviceMemory,
    pub offset: u64,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct VkMappedMemoryRange<'a> {
    pub memory: &'a VkDeviceMemory,
    pub offset: usize,
    pub size: usize,
}

impl<'a> VkWrappedType<RawVkMappedMemoryRange> for VkMappedMemoryRange<'a> {
    fn vk_to_raw(src: &VkMappedMemoryRange, dst: &mut RawVkMappedMemoryRange) {
        dst.s_type = vk_to_raw_value(&VkStructureType::MappedMemoryRange);
        dst.next = ptr::null();
        dst.memory = vk_to_raw_value(src.memory);
        dst.offset = vk_to_raw_value(&src.offset);
        dst.size = vk_to_raw_value(&src.size);
    }
}

impl Default for VkMappedMemoryRange<'static> {
    fn default() -> VkMappedMemoryRange<'static> {
        VkMappedMemoryRange {
            memory: vk_null_ref(),
            offset: 0,
            size: 0,
        }
    }
}

impl<'a> VkSetup for VkMappedMemoryRange<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkMappedMemoryRange {
    fn vk_free(&mut self) {
        
    }
}