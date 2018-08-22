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
use vk::vk_buffer_view_create_flags::*;
use vk::vk_buffer::*;
use vk::vk_format::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawVkBufferViewCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkBufferViewCreateFlags,
    pub buffer: RawVkBuffer,
    pub format: RawVkFormat,
    pub offset: u64,
    pub range: u64,
}

#[derive(Debug, Clone)]
pub struct VkBufferViewCreateInfo<'a> {
    pub flags: VkBufferViewCreateFlags,
    pub buffer: &'a VkBuffer,
    pub format: VkFormat,
    pub offset: usize,
    pub range: usize,
}

impl<'a> VkWrappedType<RawVkBufferViewCreateInfo> for VkBufferViewCreateInfo<'a> {
    fn vk_to_raw(src: &VkBufferViewCreateInfo, dst: &mut RawVkBufferViewCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::BufferViewCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.buffer = vk_to_raw_value(src.buffer);
        dst.format = vk_to_raw_value(&src.format);
        dst.offset = vk_to_raw_value(&src.offset);
        dst.range = vk_to_raw_value(&src.range);
    }
}

impl Default for VkBufferViewCreateInfo<'static> {
    fn default() -> VkBufferViewCreateInfo<'static> {
        VkBufferViewCreateInfo {
            flags: VkBufferViewCreateFlags::default(),
            buffer: vk_null_ref(),
            format: VkFormat::default(),
            offset: 0,
            range: 0,
        }
    }
}

impl<'a> VkSetup for VkBufferViewCreateInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkBufferViewCreateInfo {
    fn vk_free(&mut self) {
        
    }
}