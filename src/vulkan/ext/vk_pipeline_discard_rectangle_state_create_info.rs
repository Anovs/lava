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
use vulkan::ext::{VkPipelineDiscardRectangleStateCreateFlags,RawVkPipelineDiscardRectangleStateCreateFlags};
use vulkan::ext::{VkDiscardRectangleMode,RawVkDiscardRectangleMode};
use vulkan::vk::{VkRect2D,RawVkRect2D};

/// Wrapper for [VkPipelineDiscardRectangleStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDiscardRectangleStateCreateInfoEXT.html).
#[derive(Debug, Clone)]
pub struct VkPipelineDiscardRectangleStateCreateInfo {
    pub flags: VkPipelineDiscardRectangleStateCreateFlags,
    pub discard_rectangle_mode: VkDiscardRectangleMode,
    pub discard_rectangles: Option<Vec<VkRect2D>>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPipelineDiscardRectangleStateCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub flags: RawVkPipelineDiscardRectangleStateCreateFlags,
    pub discard_rectangle_mode: RawVkDiscardRectangleMode,
    pub discard_rectangle_count: u32,
    pub discard_rectangles: *mut RawVkRect2D,
}

impl VkWrappedType<RawVkPipelineDiscardRectangleStateCreateInfo> for VkPipelineDiscardRectangleStateCreateInfo {
    fn vk_to_raw(src: &VkPipelineDiscardRectangleStateCreateInfo, dst: &mut RawVkPipelineDiscardRectangleStateCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineDiscardRectangleStateCreateInfoExt);
        dst.next = ptr::null_mut();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.discard_rectangle_mode = vk_to_raw_value(&src.discard_rectangle_mode);
        dst.discard_rectangle_count = get_array_option_len(&src.discard_rectangles) as u32;
        dst.discard_rectangles = new_ptr_vk_array_checked(&src.discard_rectangles);
    }
}

impl VkRawType<VkPipelineDiscardRectangleStateCreateInfo> for RawVkPipelineDiscardRectangleStateCreateInfo {
    fn vk_to_wrapped(src: &RawVkPipelineDiscardRectangleStateCreateInfo) -> VkPipelineDiscardRectangleStateCreateInfo {
        VkPipelineDiscardRectangleStateCreateInfo {
            flags: RawVkPipelineDiscardRectangleStateCreateFlags::vk_to_wrapped(&src.flags),
            discard_rectangle_mode: RawVkDiscardRectangleMode::vk_to_wrapped(&src.discard_rectangle_mode),
            discard_rectangles: new_vk_array_checked(src.discard_rectangle_count, src.discard_rectangles),
        }
    }
}

impl Default for VkPipelineDiscardRectangleStateCreateInfo {
    fn default() -> VkPipelineDiscardRectangleStateCreateInfo {
        VkPipelineDiscardRectangleStateCreateInfo {
            flags: Default::default(),
            discard_rectangle_mode: Default::default(),
            discard_rectangles: None,
        }
    }
}

impl VkSetup for VkPipelineDiscardRectangleStateCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPipelineDiscardRectangleStateCreateInfo {
    fn vk_free(&self) {
        free_vk_ptr_array(self.discard_rectangle_count as usize, self.discard_rectangles);
    }
}