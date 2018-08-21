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

#[repr(C)]
pub struct RawVkDescriptorSetVariableDescriptorCountAllocateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub descriptor_set_count: u32,
    pub descriptor_counts: *mut u32,
}

#[derive(Debug, Clone)]
pub struct VkDescriptorSetVariableDescriptorCountAllocateInfo<'a> {
    pub descriptor_counts: &'a [usize],
}

impl<'a> VkWrappedType<RawVkDescriptorSetVariableDescriptorCountAllocateInfo> for VkDescriptorSetVariableDescriptorCountAllocateInfo<'a> {
    fn vk_to_raw(src: &VkDescriptorSetVariableDescriptorCountAllocateInfo, dst: &mut RawVkDescriptorSetVariableDescriptorCountAllocateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DescriptorSetVariableDescriptorCountAllocateInfoExt);
        dst.next = ptr::null();
        dst.descriptor_set_count = src.descriptor_counts.len() as u32;
        dst.descriptor_counts = new_ptr_vk_array(src.descriptor_counts);
    }
}

impl Default for VkDescriptorSetVariableDescriptorCountAllocateInfo<'static> {
    fn default() -> VkDescriptorSetVariableDescriptorCountAllocateInfo<'static> {
        VkDescriptorSetVariableDescriptorCountAllocateInfo {
            descriptor_counts: &[],
        }
    }
}

impl<'a> VkSetup for VkDescriptorSetVariableDescriptorCountAllocateInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDescriptorSetVariableDescriptorCountAllocateInfo {
    fn vk_free(&mut self) {
        free_ptr(self.descriptor_counts);
    }
}