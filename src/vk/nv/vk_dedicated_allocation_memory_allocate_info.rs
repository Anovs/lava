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
use vk::vk_image::*;
use vk::vk_buffer::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawVkDedicatedAllocationMemoryAllocateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub image: RawVkImage,
    pub buffer: RawVkBuffer,
}

#[derive(Debug, Clone)]
pub struct VkDedicatedAllocationMemoryAllocateInfo<'a, 'b> {
    pub image: Option<&'a VkImage>,
    pub buffer: Option<&'b VkBuffer>,
}

impl<'a, 'b> VkWrappedType<RawVkDedicatedAllocationMemoryAllocateInfo> for VkDedicatedAllocationMemoryAllocateInfo<'a, 'b> {
    fn vk_to_raw(src: &VkDedicatedAllocationMemoryAllocateInfo, dst: &mut RawVkDedicatedAllocationMemoryAllocateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DedicatedAllocationMemoryAllocateInfoNv);
        dst.next = ptr::null();
        dst.image = if src.image.is_some() { vk_to_raw_value(src.image.unwrap()) } else { 0 };
        dst.buffer = if src.buffer.is_some() { vk_to_raw_value(src.buffer.unwrap()) } else { 0 };
    }
}

impl Default for VkDedicatedAllocationMemoryAllocateInfo<'static, 'static> {
    fn default() -> VkDedicatedAllocationMemoryAllocateInfo<'static, 'static> {
        VkDedicatedAllocationMemoryAllocateInfo {
            image: None,
            buffer: None,
        }
    }
}

impl<'a, 'b> VkSetup for VkDedicatedAllocationMemoryAllocateInfo<'a, 'b> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDedicatedAllocationMemoryAllocateInfo {
    fn vk_free(&mut self) {
        
    }
}