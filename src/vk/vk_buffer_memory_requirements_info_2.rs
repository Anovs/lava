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

#[repr(C)]
pub struct RawVkBufferMemoryRequirementsInfo2 {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub buffer: RawVkBuffer,
}

#[derive(Debug, Clone)]
pub struct VkBufferMemoryRequirementsInfo2<'a> {
    pub buffer: &'a VkBuffer,
}

impl<'a> VkWrappedType<RawVkBufferMemoryRequirementsInfo2> for VkBufferMemoryRequirementsInfo2<'a> {
    fn vk_to_raw(src: &VkBufferMemoryRequirementsInfo2, dst: &mut RawVkBufferMemoryRequirementsInfo2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::BufferMemoryRequirementsInfo2);
        dst.next = ptr::null();
        dst.buffer = vk_to_raw_value(src.buffer);
    }
}

impl Default for VkBufferMemoryRequirementsInfo2<'static> {
    fn default() -> VkBufferMemoryRequirementsInfo2<'static> {
        VkBufferMemoryRequirementsInfo2 {
            buffer: vk_null_ref(),
        }
    }
}

impl<'a> VkSetup for VkBufferMemoryRequirementsInfo2<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkBufferMemoryRequirementsInfo2 {
    fn vk_free(&mut self) {
        
    }
}