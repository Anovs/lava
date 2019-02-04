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
use vulkan::khr::{VkDeviceGroupPresentModeFlags,RawVkDeviceGroupPresentModeFlags};

/// Wrapper for [VkDeviceGroupPresentInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDeviceGroupPresentInfoKHR.html)
#[derive(Debug, Clone)]
pub struct VkDeviceGroupPresentInfo<'a> {
    pub device_masks: &'a [u32],
    pub mode: VkDeviceGroupPresentModeFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDeviceGroupPresentInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub swapchain_count: u32,
    pub device_masks: *const u32,
    pub mode: RawVkDeviceGroupPresentModeFlags,
}

impl<'a> VkWrappedType<RawVkDeviceGroupPresentInfo> for VkDeviceGroupPresentInfo<'a> {
    fn vk_to_raw(src: &VkDeviceGroupPresentInfo, dst: &mut RawVkDeviceGroupPresentInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DeviceGroupPresentInfoKhr);
        dst.next = ptr::null();
        dst.swapchain_count = src.device_masks.len() as u32;
        dst.device_masks = src.device_masks.as_ptr();
        dst.mode = vk_to_raw_value(&src.mode);
    }
}

impl Default for VkDeviceGroupPresentInfo<'static> {
    fn default() -> VkDeviceGroupPresentInfo<'static> {
        VkDeviceGroupPresentInfo {
            device_masks: &[],
            mode: VkDeviceGroupPresentModeFlags::default(),
        }
    }
}

impl<'a> VkSetup for VkDeviceGroupPresentInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDeviceGroupPresentInfo {
    fn vk_free(&mut self) {
        
    }
}