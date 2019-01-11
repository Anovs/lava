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
use vk::khr::vk_resolve_mode_flags::*;
use vk::khr::vk_attachment_reference_2::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSubpassDescriptionDepthStencilResolve {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub depth_resolve_mode: RawVkResolveModeFlags,
    pub stencil_resolve_mode: RawVkResolveModeFlags,
    pub depth_stencil_resolve_attachment: *mut RawVkAttachmentReference2,
}

#[derive(Debug, Clone)]
pub struct VkSubpassDescriptionDepthStencilResolve<'a> {
    pub depth_resolve_mode: VkResolveModeFlags,
    pub stencil_resolve_mode: VkResolveModeFlags,
    pub depth_stencil_resolve_attachment: Option<&'a VkAttachmentReference2>,
}

impl<'a> VkWrappedType<RawVkSubpassDescriptionDepthStencilResolve> for VkSubpassDescriptionDepthStencilResolve<'a> {
    fn vk_to_raw(src: &VkSubpassDescriptionDepthStencilResolve, dst: &mut RawVkSubpassDescriptionDepthStencilResolve) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SubpassDescriptionDepthStencilResolveKhr);
        dst.next = ptr::null();
        dst.depth_resolve_mode = vk_to_raw_value(&src.depth_resolve_mode);
        dst.stencil_resolve_mode = vk_to_raw_value(&src.stencil_resolve_mode);
        dst.depth_stencil_resolve_attachment = new_ptr_vk_value_checked(src.depth_stencil_resolve_attachment);
    }
}

impl Default for VkSubpassDescriptionDepthStencilResolve<'static> {
    fn default() -> VkSubpassDescriptionDepthStencilResolve<'static> {
        VkSubpassDescriptionDepthStencilResolve {
            depth_resolve_mode: VkResolveModeFlags::default(),
            stencil_resolve_mode: VkResolveModeFlags::default(),
            depth_stencil_resolve_attachment: None,
        }
    }
}

impl<'a> VkSetup for VkSubpassDescriptionDepthStencilResolve<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkSubpassDescriptionDepthStencilResolve {
    fn vk_free(&mut self) {
        free_vk_ptr(self.depth_stencil_resolve_attachment);
    }
}