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

/// Wrapper for [VkDeviceGeneratedCommandsLimitsNVX](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDeviceGeneratedCommandsLimitsNVX.html).
#[derive(Debug, Clone)]
pub struct VkDeviceGeneratedCommandsLimits {
    pub max_indirect_commands_layout_token_count: usize,
    pub max_object_entry_counts: usize,
    pub min_sequence_count_buffer_offset_alignment: usize,
    pub min_sequence_index_buffer_offset_alignment: usize,
    pub min_commands_token_buffer_offset_alignment: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDeviceGeneratedCommandsLimits {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub max_indirect_commands_layout_token_count: u32,
    pub max_object_entry_counts: u32,
    pub min_sequence_count_buffer_offset_alignment: u32,
    pub min_sequence_index_buffer_offset_alignment: u32,
    pub min_commands_token_buffer_offset_alignment: u32,
}

impl VkWrappedType<RawVkDeviceGeneratedCommandsLimits> for VkDeviceGeneratedCommandsLimits {
    fn vk_to_raw(src: &VkDeviceGeneratedCommandsLimits, dst: &mut RawVkDeviceGeneratedCommandsLimits) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DeviceGeneratedCommandsLimitsNvx);
        dst.next = ptr::null();
        dst.max_indirect_commands_layout_token_count = vk_to_raw_value(&src.max_indirect_commands_layout_token_count);
        dst.max_object_entry_counts = vk_to_raw_value(&src.max_object_entry_counts);
        dst.min_sequence_count_buffer_offset_alignment = vk_to_raw_value(&src.min_sequence_count_buffer_offset_alignment);
        dst.min_sequence_index_buffer_offset_alignment = vk_to_raw_value(&src.min_sequence_index_buffer_offset_alignment);
        dst.min_commands_token_buffer_offset_alignment = vk_to_raw_value(&src.min_commands_token_buffer_offset_alignment);
    }
}

impl VkRawType<VkDeviceGeneratedCommandsLimits> for RawVkDeviceGeneratedCommandsLimits {
    fn vk_to_wrapped(src: &RawVkDeviceGeneratedCommandsLimits) -> VkDeviceGeneratedCommandsLimits {
        VkDeviceGeneratedCommandsLimits {
            max_indirect_commands_layout_token_count: u32::vk_to_wrapped(&src.max_indirect_commands_layout_token_count),
            max_object_entry_counts: u32::vk_to_wrapped(&src.max_object_entry_counts),
            min_sequence_count_buffer_offset_alignment: u32::vk_to_wrapped(&src.min_sequence_count_buffer_offset_alignment),
            min_sequence_index_buffer_offset_alignment: u32::vk_to_wrapped(&src.min_sequence_index_buffer_offset_alignment),
            min_commands_token_buffer_offset_alignment: u32::vk_to_wrapped(&src.min_commands_token_buffer_offset_alignment),
        }
    }
}

impl Default for VkDeviceGeneratedCommandsLimits {
    fn default() -> VkDeviceGeneratedCommandsLimits {
        VkDeviceGeneratedCommandsLimits {
            max_indirect_commands_layout_token_count: 0,
            max_object_entry_counts: 0,
            min_sequence_count_buffer_offset_alignment: 0,
            min_sequence_index_buffer_offset_alignment: 0,
            min_commands_token_buffer_offset_alignment: 0,
        }
    }
}

impl VkSetup for VkDeviceGeneratedCommandsLimits {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDeviceGeneratedCommandsLimits {
    fn vk_free(&mut self) {
        
    }
}