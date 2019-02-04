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
use vulkan::nvx::{VkObjectTable,RawVkObjectTable};
use vulkan::nvx::{VkIndirectCommandsLayout,RawVkIndirectCommandsLayout};

/// Wrapper for [VkCmdReserveSpaceForCommandsInfoNVX](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCmdReserveSpaceForCommandsInfoNVX.html)
#[derive(Debug, Clone)]
pub struct VkCmdReserveSpaceForCommandsInfo<'a, 'b> {
    pub object_table: &'a VkObjectTable,
    pub indirect_commands_layout: &'b VkIndirectCommandsLayout,
    pub max_sequences_count: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkCmdReserveSpaceForCommandsInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub object_table: RawVkObjectTable,
    pub indirect_commands_layout: RawVkIndirectCommandsLayout,
    pub max_sequences_count: u32,
}

impl<'a, 'b> VkWrappedType<RawVkCmdReserveSpaceForCommandsInfo> for VkCmdReserveSpaceForCommandsInfo<'a, 'b> {
    fn vk_to_raw(src: &VkCmdReserveSpaceForCommandsInfo, dst: &mut RawVkCmdReserveSpaceForCommandsInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::CmdReserveSpaceForCommandsInfoNvx);
        dst.next = ptr::null();
        dst.object_table = vk_to_raw_value(src.object_table);
        dst.indirect_commands_layout = vk_to_raw_value(src.indirect_commands_layout);
        dst.max_sequences_count = vk_to_raw_value(&src.max_sequences_count);
    }
}

impl Default for VkCmdReserveSpaceForCommandsInfo<'static, 'static> {
    fn default() -> VkCmdReserveSpaceForCommandsInfo<'static, 'static> {
        VkCmdReserveSpaceForCommandsInfo {
            object_table: vk_null_ref(),
            indirect_commands_layout: vk_null_ref(),
            max_sequences_count: 0,
        }
    }
}

impl<'a, 'b> VkSetup for VkCmdReserveSpaceForCommandsInfo<'a, 'b> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkCmdReserveSpaceForCommandsInfo {
    fn vk_free(&mut self) {
        
    }
}