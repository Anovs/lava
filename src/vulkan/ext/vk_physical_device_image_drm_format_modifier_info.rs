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
use vulkan::vk::{VkSharingMode,RawVkSharingMode};

/// Wrapper for [VkPhysicalDeviceImageDrmFormatModifierInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceImageDrmFormatModifierInfoEXT.html)
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceImageDrmFormatModifierInfo<'a> {
    pub drm_format_modifier: usize,
    pub sharing_mode: VkSharingMode,
    pub queue_family_indices: &'a [usize],
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceImageDrmFormatModifierInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub drm_format_modifier: u64,
    pub sharing_mode: RawVkSharingMode,
    pub queue_family_index_count: u32,
    pub queue_family_indices: *mut u32,
}

impl<'a> VkWrappedType<RawVkPhysicalDeviceImageDrmFormatModifierInfo> for VkPhysicalDeviceImageDrmFormatModifierInfo<'a> {
    fn vk_to_raw(src: &VkPhysicalDeviceImageDrmFormatModifierInfo, dst: &mut RawVkPhysicalDeviceImageDrmFormatModifierInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceImageDrmFormatModifierInfoExt);
        dst.next = ptr::null();
        dst.drm_format_modifier = vk_to_raw_value(&src.drm_format_modifier);
        dst.sharing_mode = vk_to_raw_value(&src.sharing_mode);
        dst.queue_family_index_count = src.queue_family_indices.len() as u32;
        dst.queue_family_indices = new_ptr_vk_array(src.queue_family_indices);
    }
}

impl Default for VkPhysicalDeviceImageDrmFormatModifierInfo<'static> {
    fn default() -> VkPhysicalDeviceImageDrmFormatModifierInfo<'static> {
        VkPhysicalDeviceImageDrmFormatModifierInfo {
            drm_format_modifier: 0,
            sharing_mode: VkSharingMode::default(),
            queue_family_indices: &[],
        }
    }
}

impl<'a> VkSetup for VkPhysicalDeviceImageDrmFormatModifierInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceImageDrmFormatModifierInfo {
    fn vk_free(&mut self) {
        free_ptr(self.queue_family_indices);
    }
}