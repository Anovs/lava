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
use vulkan::vk::{VkDescriptorUpdateTemplateCreateFlags,RawVkDescriptorUpdateTemplateCreateFlags};
use vulkan::vk::{VkDescriptorUpdateTemplateEntry,RawVkDescriptorUpdateTemplateEntry};
use vulkan::vk::{VkDescriptorUpdateTemplateType,RawVkDescriptorUpdateTemplateType};
use vulkan::vk::{VkDescriptorSetLayout,RawVkDescriptorSetLayout};
use vulkan::vk::{VkPipelineBindPoint,RawVkPipelineBindPoint};
use vulkan::vk::{VkPipelineLayout,RawVkPipelineLayout};

/// Wrapper for [VkDescriptorUpdateTemplateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDescriptorUpdateTemplateCreateInfo.html).
#[derive(Debug, Clone)]
pub struct VkDescriptorUpdateTemplateCreateInfo<'a, 'b, 'c> {
    pub flags: VkDescriptorUpdateTemplateCreateFlags,
    pub descriptor_update_entries: &'a [VkDescriptorUpdateTemplateEntry],
    pub template_type: VkDescriptorUpdateTemplateType,
    pub descriptor_set_layout: Option<&'b VkDescriptorSetLayout>,
    pub pipeline_bind_point: VkPipelineBindPoint,
    pub pipeline_layout: &'c VkPipelineLayout,
    pub set: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDescriptorUpdateTemplateCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkDescriptorUpdateTemplateCreateFlags,
    pub descriptor_update_entry_count: u32,
    pub descriptor_update_entries: *mut RawVkDescriptorUpdateTemplateEntry,
    pub template_type: RawVkDescriptorUpdateTemplateType,
    pub descriptor_set_layout: RawVkDescriptorSetLayout,
    pub pipeline_bind_point: RawVkPipelineBindPoint,
    pub pipeline_layout: RawVkPipelineLayout,
    pub set: u32,
}

impl<'a, 'b, 'c> VkWrappedType<RawVkDescriptorUpdateTemplateCreateInfo> for VkDescriptorUpdateTemplateCreateInfo<'a, 'b, 'c> {
    fn vk_to_raw(src: &VkDescriptorUpdateTemplateCreateInfo, dst: &mut RawVkDescriptorUpdateTemplateCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DescriptorUpdateTemplateCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.descriptor_update_entry_count = src.descriptor_update_entries.len() as u32;
        dst.descriptor_update_entries = new_ptr_vk_array(src.descriptor_update_entries);
        dst.template_type = vk_to_raw_value(&src.template_type);
        dst.descriptor_set_layout = if src.descriptor_set_layout.is_some() { vk_to_raw_value(src.descriptor_set_layout.unwrap()) } else { 0 };
        dst.pipeline_bind_point = vk_to_raw_value(&src.pipeline_bind_point);
        dst.pipeline_layout = vk_to_raw_value(src.pipeline_layout);
        dst.set = vk_to_raw_value(&src.set);
    }
}

impl Default for VkDescriptorUpdateTemplateCreateInfo<'static, 'static, 'static> {
    fn default() -> VkDescriptorUpdateTemplateCreateInfo<'static, 'static, 'static> {
        VkDescriptorUpdateTemplateCreateInfo {
            flags: VkDescriptorUpdateTemplateCreateFlags::default(),
            descriptor_update_entries: &[],
            template_type: VkDescriptorUpdateTemplateType::default(),
            descriptor_set_layout: None,
            pipeline_bind_point: VkPipelineBindPoint::default(),
            pipeline_layout: vk_null_ref(),
            set: 0,
        }
    }
}

impl<'a, 'b, 'c> VkSetup for VkDescriptorUpdateTemplateCreateInfo<'a, 'b, 'c> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDescriptorUpdateTemplateCreateInfo {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.descriptor_update_entry_count as usize, self.descriptor_update_entries);
    }
}