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
use vulkan::nvx::{VkIndirectCommandsTokenType,RawVkIndirectCommandsTokenType};

/// Wrapper for [VkIndirectCommandsLayoutTokenNVX](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkIndirectCommandsLayoutTokenNVX.html).
#[derive(Debug, Clone)]
pub struct VkIndirectCommandsLayoutToken {
    pub token_type: VkIndirectCommandsTokenType,
    pub binding_unit: usize,
    pub dynamic_count: usize,
    pub divisor: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkIndirectCommandsLayoutToken {
    pub token_type: RawVkIndirectCommandsTokenType,
    pub binding_unit: u32,
    pub dynamic_count: u32,
    pub divisor: u32,
}

impl VkWrappedType<RawVkIndirectCommandsLayoutToken> for VkIndirectCommandsLayoutToken {
    fn vk_to_raw(src: &VkIndirectCommandsLayoutToken, dst: &mut RawVkIndirectCommandsLayoutToken) {
        dst.token_type = vk_to_raw_value(&src.token_type);
        dst.binding_unit = vk_to_raw_value(&src.binding_unit);
        dst.dynamic_count = vk_to_raw_value(&src.dynamic_count);
        dst.divisor = vk_to_raw_value(&src.divisor);
    }
}

impl VkRawType<VkIndirectCommandsLayoutToken> for RawVkIndirectCommandsLayoutToken {
    fn vk_to_wrapped(src: &RawVkIndirectCommandsLayoutToken) -> VkIndirectCommandsLayoutToken {
        VkIndirectCommandsLayoutToken {
            token_type: RawVkIndirectCommandsTokenType::vk_to_wrapped(&src.token_type),
            binding_unit: u32::vk_to_wrapped(&src.binding_unit),
            dynamic_count: u32::vk_to_wrapped(&src.dynamic_count),
            divisor: u32::vk_to_wrapped(&src.divisor),
        }
    }
}

impl Default for VkIndirectCommandsLayoutToken {
    fn default() -> VkIndirectCommandsLayoutToken {
        VkIndirectCommandsLayoutToken {
            token_type: VkIndirectCommandsTokenType::default(),
            binding_unit: 0,
            dynamic_count: 0,
            divisor: 0,
        }
    }
}

impl VkSetup for VkIndirectCommandsLayoutToken {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkIndirectCommandsLayoutToken {
    fn vk_free(&mut self) {
        
    }
}