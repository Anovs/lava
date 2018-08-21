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
use vk::vk_descriptor_set::*;

#[repr(C)]
pub struct RawVkCopyDescriptorSet {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub src_set: RawVkDescriptorSet,
    pub src_binding: u32,
    pub src_array_element: u32,
    pub dst_set: RawVkDescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
}

#[derive(Debug, Clone)]
pub struct VkCopyDescriptorSet<'a, 'b> {
    pub src_set: &'a VkDescriptorSet,
    pub src_binding: usize,
    pub src_array_element: usize,
    pub dst_set: &'b VkDescriptorSet,
    pub dst_binding: usize,
    pub dst_array_element: usize,
    pub descriptor_count: usize,
}

impl<'a, 'b> VkWrappedType<RawVkCopyDescriptorSet> for VkCopyDescriptorSet<'a, 'b> {
    fn vk_to_raw(src: &VkCopyDescriptorSet, dst: &mut RawVkCopyDescriptorSet) {
        dst.s_type = vk_to_raw_value(&VkStructureType::CopyDescriptorSet);
        dst.next = ptr::null();
        dst.src_set = vk_to_raw_value(src.src_set);
        dst.src_binding = vk_to_raw_value(&src.src_binding);
        dst.src_array_element = vk_to_raw_value(&src.src_array_element);
        dst.dst_set = vk_to_raw_value(src.dst_set);
        dst.dst_binding = vk_to_raw_value(&src.dst_binding);
        dst.dst_array_element = vk_to_raw_value(&src.dst_array_element);
        dst.descriptor_count = vk_to_raw_value(&src.descriptor_count);
    }
}

impl Default for VkCopyDescriptorSet<'static, 'static> {
    fn default() -> VkCopyDescriptorSet<'static, 'static> {
        VkCopyDescriptorSet {
            src_set: vk_null_ref(),
            src_binding: 0,
            src_array_element: 0,
            dst_set: vk_null_ref(),
            dst_binding: 0,
            dst_array_element: 0,
            descriptor_count: 0,
        }
    }
}

impl<'a, 'b> VkSetup for VkCopyDescriptorSet<'a, 'b> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkCopyDescriptorSet {
    fn vk_free(&mut self) {
        
    }
}