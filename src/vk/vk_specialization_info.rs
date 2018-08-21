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
use vk::vk_specialization_map_entry::*;

#[repr(C)]
pub struct RawVkSpecializationInfo {
    pub map_entry_count: u32,
    pub map_entries: *mut RawVkSpecializationMapEntry,
    pub data_size: usize,
    pub data: *const c_void,
}

#[derive(Debug, Clone)]
pub struct VkSpecializationInfo<'a, 'b> {
    pub map_entries: &'a [VkSpecializationMapEntry],
    pub data_size: usize,
    pub data: &'b c_void,
}

impl<'a, 'b> VkWrappedType<RawVkSpecializationInfo> for VkSpecializationInfo<'a, 'b> {
    fn vk_to_raw(src: &VkSpecializationInfo, dst: &mut RawVkSpecializationInfo) {
        dst.map_entry_count = src.map_entries.len() as u32;
        dst.map_entries = new_ptr_vk_array(src.map_entries);
        dst.data_size = src.data_size;
        dst.data = src.data as *const c_void;
    }
}

impl Default for VkSpecializationInfo<'static, 'static> {
    fn default() -> VkSpecializationInfo<'static, 'static> {
        VkSpecializationInfo {
            map_entries: &[],
            data_size: 0,
            data: &0,
        }
    }
}

impl<'a, 'b> VkSetup for VkSpecializationInfo<'a, 'b> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkSpecializationInfo {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.map_entry_count as usize, self.map_entries);
    }
}