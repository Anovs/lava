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
use vk::nvx::vk_object_table::*;
use vk::nvx::vk_indirect_commands_layout::*;
use vk::nvx::vk_indirect_commands_token::*;
use vk::vk_command_buffer::*;
use vk::vk_buffer::*;

#[repr(C)]
pub struct RawVkCmdProcessCommandsInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub object_table: RawVkObjectTable,
    pub indirect_commands_layout: RawVkIndirectCommandsLayout,
    pub indirect_commands_token_count: u32,
    pub indirect_commands_tokens: *mut RawVkIndirectCommandsToken,
    pub max_sequences_count: u32,
    pub target_command_buffer: RawVkCommandBuffer,
    pub sequences_count_buffer: RawVkBuffer,
    pub sequences_count_offset: u64,
    pub sequences_index_buffer: RawVkBuffer,
    pub sequences_index_offset: u64,
}

#[derive(Debug, Clone)]
pub struct VkCmdProcessCommandsInfo<'a, 'b, 'c, 'd, 'e, 'f, 'g>
    where
        'd: 'c,
{
    pub object_table: &'a VkObjectTable,
    pub indirect_commands_layout: &'b VkIndirectCommandsLayout,
    pub indirect_commands_tokens: &'c [VkIndirectCommandsToken<'d>],
    pub max_sequences_count: usize,
    pub target_command_buffer: Option<&'e VkCommandBuffer>,
    pub sequences_count_buffer: Option<&'f VkBuffer>,
    pub sequences_count_offset: usize,
    pub sequences_index_buffer: Option<&'g VkBuffer>,
    pub sequences_index_offset: usize,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> VkWrappedType<RawVkCmdProcessCommandsInfo> for VkCmdProcessCommandsInfo<'a, 'b, 'c, 'd, 'e, 'f, 'g>
    where
        'd: 'c,
{
    fn vk_to_raw(src: &VkCmdProcessCommandsInfo, dst: &mut RawVkCmdProcessCommandsInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::CmdProcessCommandsInfoNvx);
        dst.next = ptr::null();
        dst.object_table = vk_to_raw_value(src.object_table);
        dst.indirect_commands_layout = vk_to_raw_value(src.indirect_commands_layout);
        dst.indirect_commands_token_count = src.indirect_commands_tokens.len() as u32;
        dst.indirect_commands_tokens = new_ptr_vk_array(src.indirect_commands_tokens);
        dst.max_sequences_count = vk_to_raw_value(&src.max_sequences_count);
        dst.target_command_buffer = if src.target_command_buffer.is_some() { vk_to_raw_value(src.target_command_buffer.unwrap()) } else { 0 };
        dst.sequences_count_buffer = if src.sequences_count_buffer.is_some() { vk_to_raw_value(src.sequences_count_buffer.unwrap()) } else { 0 };
        dst.sequences_count_offset = vk_to_raw_value(&src.sequences_count_offset);
        dst.sequences_index_buffer = if src.sequences_index_buffer.is_some() { vk_to_raw_value(src.sequences_index_buffer.unwrap()) } else { 0 };
        dst.sequences_index_offset = vk_to_raw_value(&src.sequences_index_offset);
    }
}

impl Default for VkCmdProcessCommandsInfo<'static, 'static, 'static, 'static, 'static, 'static, 'static> {
    fn default() -> VkCmdProcessCommandsInfo<'static, 'static, 'static, 'static, 'static, 'static, 'static> {
        VkCmdProcessCommandsInfo {
            object_table: vk_null_ref(),
            indirect_commands_layout: vk_null_ref(),
            indirect_commands_tokens: &[],
            max_sequences_count: 0,
            target_command_buffer: None,
            sequences_count_buffer: None,
            sequences_count_offset: 0,
            sequences_index_buffer: None,
            sequences_index_offset: 0,
        }
    }
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> VkSetup for VkCmdProcessCommandsInfo<'a, 'b, 'c, 'd, 'e, 'f, 'g>
    where
        'd: 'c,
{
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkCmdProcessCommandsInfo {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.indirect_commands_token_count as usize, self.indirect_commands_tokens);
    }
}