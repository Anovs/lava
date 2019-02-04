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
use vulkan::vk::{VkStencilOp,RawVkStencilOp};
use vulkan::vk::{VkCompareOp,RawVkCompareOp};

/// Wrapper for [VkStencilOpState](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkStencilOpState.html).
#[derive(Debug, Clone)]
pub struct VkStencilOpState {
    pub fail_op: VkStencilOp,
    pub pass_op: VkStencilOp,
    pub depth_fail_op: VkStencilOp,
    pub compare_op: VkCompareOp,
    pub compare_mask: u32,
    pub write_mask: u32,
    pub reference: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkStencilOpState {
    pub fail_op: RawVkStencilOp,
    pub pass_op: RawVkStencilOp,
    pub depth_fail_op: RawVkStencilOp,
    pub compare_op: RawVkCompareOp,
    pub compare_mask: u32,
    pub write_mask: u32,
    pub reference: u32,
}

impl VkWrappedType<RawVkStencilOpState> for VkStencilOpState {
    fn vk_to_raw(src: &VkStencilOpState, dst: &mut RawVkStencilOpState) {
        dst.fail_op = vk_to_raw_value(&src.fail_op);
        dst.pass_op = vk_to_raw_value(&src.pass_op);
        dst.depth_fail_op = vk_to_raw_value(&src.depth_fail_op);
        dst.compare_op = vk_to_raw_value(&src.compare_op);
        dst.compare_mask = src.compare_mask;
        dst.write_mask = src.write_mask;
        dst.reference = vk_to_raw_value(&src.reference);
    }
}

impl VkRawType<VkStencilOpState> for RawVkStencilOpState {
    fn vk_to_wrapped(src: &RawVkStencilOpState) -> VkStencilOpState {
        VkStencilOpState {
            fail_op: RawVkStencilOp::vk_to_wrapped(&src.fail_op),
            pass_op: RawVkStencilOp::vk_to_wrapped(&src.pass_op),
            depth_fail_op: RawVkStencilOp::vk_to_wrapped(&src.depth_fail_op),
            compare_op: RawVkCompareOp::vk_to_wrapped(&src.compare_op),
            compare_mask: src.compare_mask,
            write_mask: src.write_mask,
            reference: u32::vk_to_wrapped(&src.reference),
        }
    }
}

impl Default for VkStencilOpState {
    fn default() -> VkStencilOpState {
        VkStencilOpState {
            fail_op: VkStencilOp::default(),
            pass_op: VkStencilOp::default(),
            depth_fail_op: VkStencilOp::default(),
            compare_op: VkCompareOp::default(),
            compare_mask: 0,
            write_mask: 0,
            reference: 0,
        }
    }
}

impl VkSetup for VkStencilOpState {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkStencilOpState {
    fn vk_free(&mut self) {
        
    }
}