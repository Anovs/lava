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

/// Wrapper for [VkPhysicalDeviceInlineUniformBlockPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceInlineUniformBlockPropertiesEXT.html)
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceInlineUniformBlockProperties {
    pub max_inline_uniform_block_size: usize,
    pub max_per_stage_descriptor_inline_uniform_blocks: usize,
    pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: usize,
    pub max_descriptor_set_inline_uniform_blocks: usize,
    pub max_descriptor_set_update_after_bind_inline_uniform_blocks: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceInlineUniformBlockProperties {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub max_inline_uniform_block_size: u32,
    pub max_per_stage_descriptor_inline_uniform_blocks: u32,
    pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
    pub max_descriptor_set_inline_uniform_blocks: u32,
    pub max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceInlineUniformBlockProperties> for VkPhysicalDeviceInlineUniformBlockProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceInlineUniformBlockProperties, dst: &mut RawVkPhysicalDeviceInlineUniformBlockProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceInlineUniformBlockPropertiesExt);
        dst.next = ptr::null();
        dst.max_inline_uniform_block_size = vk_to_raw_value(&src.max_inline_uniform_block_size);
        dst.max_per_stage_descriptor_inline_uniform_blocks = vk_to_raw_value(&src.max_per_stage_descriptor_inline_uniform_blocks);
        dst.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks = vk_to_raw_value(&src.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks);
        dst.max_descriptor_set_inline_uniform_blocks = vk_to_raw_value(&src.max_descriptor_set_inline_uniform_blocks);
        dst.max_descriptor_set_update_after_bind_inline_uniform_blocks = vk_to_raw_value(&src.max_descriptor_set_update_after_bind_inline_uniform_blocks);
    }
}

impl VkRawType<VkPhysicalDeviceInlineUniformBlockProperties> for RawVkPhysicalDeviceInlineUniformBlockProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceInlineUniformBlockProperties) -> VkPhysicalDeviceInlineUniformBlockProperties {
        VkPhysicalDeviceInlineUniformBlockProperties {
            max_inline_uniform_block_size: u32::vk_to_wrapped(&src.max_inline_uniform_block_size),
            max_per_stage_descriptor_inline_uniform_blocks: u32::vk_to_wrapped(&src.max_per_stage_descriptor_inline_uniform_blocks),
            max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32::vk_to_wrapped(&src.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks),
            max_descriptor_set_inline_uniform_blocks: u32::vk_to_wrapped(&src.max_descriptor_set_inline_uniform_blocks),
            max_descriptor_set_update_after_bind_inline_uniform_blocks: u32::vk_to_wrapped(&src.max_descriptor_set_update_after_bind_inline_uniform_blocks),
        }
    }
}

impl Default for VkPhysicalDeviceInlineUniformBlockProperties {
    fn default() -> VkPhysicalDeviceInlineUniformBlockProperties {
        VkPhysicalDeviceInlineUniformBlockProperties {
            max_inline_uniform_block_size: 0,
            max_per_stage_descriptor_inline_uniform_blocks: 0,
            max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: 0,
            max_descriptor_set_inline_uniform_blocks: 0,
            max_descriptor_set_update_after_bind_inline_uniform_blocks: 0,
        }
    }
}

impl VkSetup for VkPhysicalDeviceInlineUniformBlockProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceInlineUniformBlockProperties {
    fn vk_free(&mut self) {
        
    }
}