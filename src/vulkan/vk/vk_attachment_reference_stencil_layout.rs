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
use vulkan::vk::{VkImageLayout,RawVkImageLayout};

/// Wrapper for [VkAttachmentReferenceStencilLayout](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentReferenceStencilLayout.html).
#[derive(Debug, Clone)]
pub struct VkAttachmentReferenceStencilLayout {
    pub stencil_layout: VkImageLayout,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkAttachmentReferenceStencilLayout {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub stencil_layout: RawVkImageLayout,
}

impl VkWrappedType<RawVkAttachmentReferenceStencilLayout> for VkAttachmentReferenceStencilLayout {
    fn vk_to_raw(src: &VkAttachmentReferenceStencilLayout, dst: &mut RawVkAttachmentReferenceStencilLayout) {
        dst.s_type = vk_to_raw_value(&VkStructureType::AttachmentReferenceStencilLayout);
        dst.next = ptr::null_mut();
        dst.stencil_layout = vk_to_raw_value(&src.stencil_layout);
    }
}

impl VkRawType<VkAttachmentReferenceStencilLayout> for RawVkAttachmentReferenceStencilLayout {
    fn vk_to_wrapped(src: &RawVkAttachmentReferenceStencilLayout) -> VkAttachmentReferenceStencilLayout {
        VkAttachmentReferenceStencilLayout {
            stencil_layout: RawVkImageLayout::vk_to_wrapped(&src.stencil_layout),
        }
    }
}

impl Default for VkAttachmentReferenceStencilLayout {
    fn default() -> VkAttachmentReferenceStencilLayout {
        VkAttachmentReferenceStencilLayout {
            stencil_layout: Default::default(),
        }
    }
}

impl VkSetup for VkAttachmentReferenceStencilLayout {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkAttachmentReferenceStencilLayout {
    fn vk_free(&self) {
        
    }
}