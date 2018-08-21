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
use vk::vk_pipeline_layout_create_flags::*;
use vk::vk_descriptor_set_layout::*;
use vk::vk_push_constant_range::*;

#[repr(C)]
pub struct RawVkPipelineLayoutCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkPipelineLayoutCreateFlags,
    pub set_layout_count: u32,
    pub set_layouts: *mut RawVkDescriptorSetLayout,
    pub push_constant_range_count: u32,
    pub push_constant_ranges: *mut RawVkPushConstantRange,
}

#[derive(Debug, Clone)]
pub struct VkPipelineLayoutCreateInfo<'a, 'b, 'c>
    where
        'b: 'a,
{
    pub flags: VkPipelineLayoutCreateFlags,
    pub set_layouts: &'a [&'b VkDescriptorSetLayout],
    pub push_constant_ranges: &'c [VkPushConstantRange],
}

impl<'a, 'b, 'c> VkWrappedType<RawVkPipelineLayoutCreateInfo> for VkPipelineLayoutCreateInfo<'a, 'b, 'c>
    where
        'b: 'a,
{
    fn vk_to_raw(src: &VkPipelineLayoutCreateInfo, dst: &mut RawVkPipelineLayoutCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineLayoutCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.set_layout_count = src.set_layouts.len() as u32;
        dst.set_layouts = new_ptr_vk_array_from_ref(src.set_layouts);
        dst.push_constant_range_count = src.push_constant_ranges.len() as u32;
        dst.push_constant_ranges = new_ptr_vk_array(src.push_constant_ranges);
    }
}

impl Default for VkPipelineLayoutCreateInfo<'static, 'static, 'static> {
    fn default() -> VkPipelineLayoutCreateInfo<'static, 'static, 'static> {
        VkPipelineLayoutCreateInfo {
            flags: VkPipelineLayoutCreateFlags::default(),
            set_layouts: &[],
            push_constant_ranges: &[],
        }
    }
}

impl<'a, 'b, 'c> VkSetup for VkPipelineLayoutCreateInfo<'a, 'b, 'c>
    where
        'b: 'a,
{
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPipelineLayoutCreateInfo {
    fn vk_free(&mut self) {
        free_ptr(self.set_layouts);
        free_vk_ptr_array(self.push_constant_range_count as usize, self.push_constant_ranges);
    }
}