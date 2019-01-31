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
use vulkan::vk::{VkInputAttachmentAspectReference,RawVkInputAttachmentAspectReference};

/// Wrapper for [VkRenderPassInputAttachmentAspectCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkRenderPassInputAttachmentAspectCreateInfo.html)
#[derive(Debug, Clone)]
pub struct VkRenderPassInputAttachmentAspectCreateInfo<'a> {
    pub aspect_references: &'a [VkInputAttachmentAspectReference],
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkRenderPassInputAttachmentAspectCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub aspect_reference_count: u32,
    pub aspect_references: *mut RawVkInputAttachmentAspectReference,
}

impl<'a> VkWrappedType<RawVkRenderPassInputAttachmentAspectCreateInfo> for VkRenderPassInputAttachmentAspectCreateInfo<'a> {
    fn vk_to_raw(src: &VkRenderPassInputAttachmentAspectCreateInfo, dst: &mut RawVkRenderPassInputAttachmentAspectCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::RenderPassInputAttachmentAspectCreateInfo);
        dst.next = ptr::null();
        dst.aspect_reference_count = src.aspect_references.len() as u32;
        dst.aspect_references = new_ptr_vk_array(src.aspect_references);
    }
}

impl Default for VkRenderPassInputAttachmentAspectCreateInfo<'static> {
    fn default() -> VkRenderPassInputAttachmentAspectCreateInfo<'static> {
        VkRenderPassInputAttachmentAspectCreateInfo {
            aspect_references: &[],
        }
    }
}

impl<'a> VkSetup for VkRenderPassInputAttachmentAspectCreateInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkRenderPassInputAttachmentAspectCreateInfo {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.aspect_reference_count as usize, self.aspect_references);
    }
}