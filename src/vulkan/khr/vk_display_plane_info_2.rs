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
use vulkan::khr::{VkDisplayMode,RawVkDisplayMode};

/// Wrapper for [VkDisplayPlaneInfo2KHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDisplayPlaneInfo2KHR.html).
#[derive(Debug, Clone)]
pub struct VkDisplayPlaneInfo2<'a> {
    pub mode: &'a VkDisplayMode,
    pub plane_index: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDisplayPlaneInfo2 {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub mode: RawVkDisplayMode,
    pub plane_index: u32,
}

impl<'a> VkWrappedType<RawVkDisplayPlaneInfo2> for VkDisplayPlaneInfo2<'a> {
    fn vk_to_raw(src: &VkDisplayPlaneInfo2, dst: &mut RawVkDisplayPlaneInfo2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DisplayPlaneInfo2Khr);
        dst.next = ptr::null();
        dst.mode = vk_to_raw_value(src.mode);
        dst.plane_index = vk_to_raw_value(&src.plane_index);
    }
}

impl Default for VkDisplayPlaneInfo2<'static> {
    fn default() -> VkDisplayPlaneInfo2<'static> {
        VkDisplayPlaneInfo2 {
            mode: vk_null_ref(),
            plane_index: 0,
        }
    }
}

impl<'a> VkSetup for VkDisplayPlaneInfo2<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDisplayPlaneInfo2 {
    fn vk_free(&mut self) {
        
    }
}