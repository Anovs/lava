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
use vk::vk_pipeline_bind_point::*;
use vk::nvx::vk_indirect_commands_layout_usage_flags::*;
use vk::nvx::vk_indirect_commands_layout_token::*;

#[repr(C)]
pub struct RawVkIndirectCommandsLayoutCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub pipeline_bind_point: RawVkPipelineBindPoint,
    pub flags: RawVkIndirectCommandsLayoutUsageFlags,
    pub token_count: u32,
    pub tokens: *mut RawVkIndirectCommandsLayoutToken,
}

#[derive(Debug, Clone)]
pub struct VkIndirectCommandsLayoutCreateInfo<'a> {
    pub pipeline_bind_point: VkPipelineBindPoint,
    pub flags: VkIndirectCommandsLayoutUsageFlags,
    pub tokens: &'a [VkIndirectCommandsLayoutToken],
}

impl<'a> VkWrappedType<RawVkIndirectCommandsLayoutCreateInfo> for VkIndirectCommandsLayoutCreateInfo<'a> {
    fn vk_to_raw(src: &VkIndirectCommandsLayoutCreateInfo, dst: &mut RawVkIndirectCommandsLayoutCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::IndirectCommandsLayoutCreateInfoNvx);
        dst.next = ptr::null();
        dst.pipeline_bind_point = vk_to_raw_value(&src.pipeline_bind_point);
        dst.flags = vk_to_raw_value(&src.flags);
        dst.token_count = src.tokens.len() as u32;
        dst.tokens = new_ptr_vk_array(src.tokens);
    }
}

impl Default for VkIndirectCommandsLayoutCreateInfo<'static> {
    fn default() -> VkIndirectCommandsLayoutCreateInfo<'static> {
        VkIndirectCommandsLayoutCreateInfo {
            pipeline_bind_point: VkPipelineBindPoint::default(),
            flags: VkIndirectCommandsLayoutUsageFlags::default(),
            tokens: &[],
        }
    }
}

impl<'a> VkSetup for VkIndirectCommandsLayoutCreateInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkIndirectCommandsLayoutCreateInfo {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.token_count as usize, self.tokens);
    }
}