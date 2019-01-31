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
use vulkan::vk::{VkPipelineDepthStencilStateCreateFlags,RawVkPipelineDepthStencilStateCreateFlags};
use vulkan::vk::{VkCompareOp,RawVkCompareOp};
use vulkan::vk::{VkStencilOpState,RawVkStencilOpState};

/// Wrapper for [VkPipelineDepthStencilStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineDepthStencilStateCreateInfo.html)
#[derive(Debug, Clone)]
pub struct VkPipelineDepthStencilStateCreateInfo {
    pub flags: VkPipelineDepthStencilStateCreateFlags,
    pub depth_test_enable: bool,
    pub depth_write_enable: bool,
    pub depth_compare_op: VkCompareOp,
    pub depth_bounds_test_enable: bool,
    pub stencil_test_enable: bool,
    pub front: VkStencilOpState,
    pub back: VkStencilOpState,
    pub min_depth_bounds: f32,
    pub max_depth_bounds: f32,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPipelineDepthStencilStateCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkPipelineDepthStencilStateCreateFlags,
    pub depth_test_enable: u32,
    pub depth_write_enable: u32,
    pub depth_compare_op: RawVkCompareOp,
    pub depth_bounds_test_enable: u32,
    pub stencil_test_enable: u32,
    pub front: RawVkStencilOpState,
    pub back: RawVkStencilOpState,
    pub min_depth_bounds: f32,
    pub max_depth_bounds: f32,
}

impl VkWrappedType<RawVkPipelineDepthStencilStateCreateInfo> for VkPipelineDepthStencilStateCreateInfo {
    fn vk_to_raw(src: &VkPipelineDepthStencilStateCreateInfo, dst: &mut RawVkPipelineDepthStencilStateCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineDepthStencilStateCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.depth_test_enable = vk_to_raw_value(&src.depth_test_enable);
        dst.depth_write_enable = vk_to_raw_value(&src.depth_write_enable);
        dst.depth_compare_op = vk_to_raw_value(&src.depth_compare_op);
        dst.depth_bounds_test_enable = vk_to_raw_value(&src.depth_bounds_test_enable);
        dst.stencil_test_enable = vk_to_raw_value(&src.stencil_test_enable);
        dst.front = vk_to_raw_value(&src.front);
        dst.back = vk_to_raw_value(&src.back);
        dst.min_depth_bounds = src.min_depth_bounds;
        dst.max_depth_bounds = src.max_depth_bounds;
    }
}

impl VkRawType<VkPipelineDepthStencilStateCreateInfo> for RawVkPipelineDepthStencilStateCreateInfo {
    fn vk_to_wrapped(src: &RawVkPipelineDepthStencilStateCreateInfo) -> VkPipelineDepthStencilStateCreateInfo {
        VkPipelineDepthStencilStateCreateInfo {
            flags: RawVkPipelineDepthStencilStateCreateFlags::vk_to_wrapped(&src.flags),
            depth_test_enable: u32::vk_to_wrapped(&src.depth_test_enable),
            depth_write_enable: u32::vk_to_wrapped(&src.depth_write_enable),
            depth_compare_op: RawVkCompareOp::vk_to_wrapped(&src.depth_compare_op),
            depth_bounds_test_enable: u32::vk_to_wrapped(&src.depth_bounds_test_enable),
            stencil_test_enable: u32::vk_to_wrapped(&src.stencil_test_enable),
            front: RawVkStencilOpState::vk_to_wrapped(&src.front),
            back: RawVkStencilOpState::vk_to_wrapped(&src.back),
            min_depth_bounds: src.min_depth_bounds,
            max_depth_bounds: src.max_depth_bounds,
        }
    }
}

impl Default for VkPipelineDepthStencilStateCreateInfo {
    fn default() -> VkPipelineDepthStencilStateCreateInfo {
        VkPipelineDepthStencilStateCreateInfo {
            flags: VkPipelineDepthStencilStateCreateFlags::default(),
            depth_test_enable: false,
            depth_write_enable: false,
            depth_compare_op: VkCompareOp::default(),
            depth_bounds_test_enable: false,
            stencil_test_enable: false,
            front: VkStencilOpState::default(),
            back: VkStencilOpState::default(),
            min_depth_bounds: 0.0,
            max_depth_bounds: 0.0,
        }
    }
}

impl VkSetup for VkPipelineDepthStencilStateCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.front, fn_table, instance, device);
        VkSetup::vk_setup(&mut self.back, fn_table, instance, device);
    }
}

impl VkFree for RawVkPipelineDepthStencilStateCreateInfo {
    fn vk_free(&mut self) {
        RawVkStencilOpState::vk_free(&mut self.front);
        RawVkStencilOpState::vk_free(&mut self.back);
    }
}