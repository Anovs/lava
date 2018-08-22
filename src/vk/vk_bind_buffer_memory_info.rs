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
use vk::vk_buffer::*;
use vk::vk_device_memory::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawVkBindBufferMemoryInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub buffer: RawVkBuffer,
    pub memory: RawVkDeviceMemory,
    pub memory_offset: u64,
}

#[derive(Debug, Clone)]
pub struct VkBindBufferMemoryInfo<'a, 'b> {
    pub buffer: &'a VkBuffer,
    pub memory: &'b VkDeviceMemory,
    pub memory_offset: usize,
}

impl<'a, 'b> VkWrappedType<RawVkBindBufferMemoryInfo> for VkBindBufferMemoryInfo<'a, 'b> {
    fn vk_to_raw(src: &VkBindBufferMemoryInfo, dst: &mut RawVkBindBufferMemoryInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::BindBufferMemoryInfo);
        dst.next = ptr::null();
        dst.buffer = vk_to_raw_value(src.buffer);
        dst.memory = vk_to_raw_value(src.memory);
        dst.memory_offset = vk_to_raw_value(&src.memory_offset);
    }
}

impl Default for VkBindBufferMemoryInfo<'static, 'static> {
    fn default() -> VkBindBufferMemoryInfo<'static, 'static> {
        VkBindBufferMemoryInfo {
            buffer: vk_null_ref(),
            memory: vk_null_ref(),
            memory_offset: 0,
        }
    }
}

impl<'a, 'b> VkSetup for VkBindBufferMemoryInfo<'a, 'b> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkBindBufferMemoryInfo {
    fn vk_free(&mut self) {
        
    }
}