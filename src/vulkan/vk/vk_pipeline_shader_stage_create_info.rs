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
use vulkan::vk::{VkPipelineShaderStageCreateFlags,RawVkPipelineShaderStageCreateFlags};
use vulkan::vk::{VkShaderStageFlags,RawVkShaderStageFlags};
use vulkan::vk::{VkShaderModule,RawVkShaderModule};
use vulkan::vk::{VkSpecializationInfo,RawVkSpecializationInfo};

/// Wrapper for [VkPipelineShaderStageCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineShaderStageCreateInfo.html).
#[derive(Debug, Clone)]
pub struct VkPipelineShaderStageCreateInfo<'a, 'b, 'c, 'd, 'e>
    where
        'd: 'c,
        'e: 'c,
{
    pub flags: VkPipelineShaderStageCreateFlags,
    pub stage: VkShaderStageFlags,
    pub module: &'a VkShaderModule,
    pub name: &'b str,
    pub specialization_info: Option<&'c VkSpecializationInfo<'d, 'e>>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPipelineShaderStageCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkPipelineShaderStageCreateFlags,
    pub stage: RawVkShaderStageFlags,
    pub module: RawVkShaderModule,
    pub name: *mut c_char,
    pub specialization_info: *mut RawVkSpecializationInfo,
}

impl<'a, 'b, 'c, 'd, 'e> VkWrappedType<RawVkPipelineShaderStageCreateInfo> for VkPipelineShaderStageCreateInfo<'a, 'b, 'c, 'd, 'e>
    where
        'd: 'c,
        'e: 'c,
{
    fn vk_to_raw(src: &VkPipelineShaderStageCreateInfo, dst: &mut RawVkPipelineShaderStageCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineShaderStageCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.stage = vk_to_raw_value(&src.stage);
        dst.module = vk_to_raw_value(src.module);
        dst.name = new_ptr_string(src.name);
        dst.specialization_info = new_ptr_vk_value_checked(src.specialization_info);
    }
}

impl Default for VkPipelineShaderStageCreateInfo<'static, 'static, 'static, 'static, 'static> {
    fn default() -> VkPipelineShaderStageCreateInfo<'static, 'static, 'static, 'static, 'static> {
        VkPipelineShaderStageCreateInfo {
            flags: VkPipelineShaderStageCreateFlags::default(),
            stage: VkShaderStageFlags::default(),
            module: vk_null_ref(),
            name: "",
            specialization_info: None,
        }
    }
}

impl<'a, 'b, 'c, 'd, 'e> VkSetup for VkPipelineShaderStageCreateInfo<'a, 'b, 'c, 'd, 'e>
    where
        'd: 'c,
        'e: 'c,
{
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPipelineShaderStageCreateInfo {
    fn vk_free(&mut self) {
        free_ptr(self.name);
        free_vk_ptr(self.specialization_info);
    }
}