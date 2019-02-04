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
use vulkan::vk::{VkPipelineVertexInputStateCreateFlags,RawVkPipelineVertexInputStateCreateFlags};
use vulkan::vk::{VkVertexInputBindingDescription,RawVkVertexInputBindingDescription};
use vulkan::vk::{VkVertexInputAttributeDescription,RawVkVertexInputAttributeDescription};

/// Wrapper for [VkPipelineVertexInputStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineVertexInputStateCreateInfo.html)
#[derive(Debug, Clone)]
pub struct VkPipelineVertexInputStateCreateInfo<'a, 'b> {
    pub flags: VkPipelineVertexInputStateCreateFlags,
    pub vertex_binding_descriptions: &'a [VkVertexInputBindingDescription],
    pub vertex_attribute_descriptions: &'b [VkVertexInputAttributeDescription],
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPipelineVertexInputStateCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkPipelineVertexInputStateCreateFlags,
    pub vertex_binding_description_count: u32,
    pub vertex_binding_descriptions: *mut RawVkVertexInputBindingDescription,
    pub vertex_attribute_description_count: u32,
    pub vertex_attribute_descriptions: *mut RawVkVertexInputAttributeDescription,
}

impl<'a, 'b> VkWrappedType<RawVkPipelineVertexInputStateCreateInfo> for VkPipelineVertexInputStateCreateInfo<'a, 'b> {
    fn vk_to_raw(src: &VkPipelineVertexInputStateCreateInfo, dst: &mut RawVkPipelineVertexInputStateCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineVertexInputStateCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.vertex_binding_description_count = src.vertex_binding_descriptions.len() as u32;
        dst.vertex_binding_descriptions = new_ptr_vk_array(src.vertex_binding_descriptions);
        dst.vertex_attribute_description_count = src.vertex_attribute_descriptions.len() as u32;
        dst.vertex_attribute_descriptions = new_ptr_vk_array(src.vertex_attribute_descriptions);
    }
}

impl Default for VkPipelineVertexInputStateCreateInfo<'static, 'static> {
    fn default() -> VkPipelineVertexInputStateCreateInfo<'static, 'static> {
        VkPipelineVertexInputStateCreateInfo {
            flags: VkPipelineVertexInputStateCreateFlags::default(),
            vertex_binding_descriptions: &[],
            vertex_attribute_descriptions: &[],
        }
    }
}

impl<'a, 'b> VkSetup for VkPipelineVertexInputStateCreateInfo<'a, 'b> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPipelineVertexInputStateCreateInfo {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.vertex_binding_description_count as usize, self.vertex_binding_descriptions);
        free_vk_ptr_array(self.vertex_attribute_description_count as usize, self.vertex_attribute_descriptions);
    }
}