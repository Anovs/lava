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
use vulkan::vk::{VkRect2D,RawVkRect2D};

/// Wrapper for [VkDeviceGroupRenderPassBeginInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDeviceGroupRenderPassBeginInfo.html)
#[derive(Debug, Clone)]
pub struct VkDeviceGroupRenderPassBeginInfo<'a> {
    pub device_mask: u32,
    pub device_render_areas: &'a [VkRect2D],
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDeviceGroupRenderPassBeginInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub device_mask: u32,
    pub device_render_area_count: u32,
    pub device_render_areas: *mut RawVkRect2D,
}

impl<'a> VkWrappedType<RawVkDeviceGroupRenderPassBeginInfo> for VkDeviceGroupRenderPassBeginInfo<'a> {
    fn vk_to_raw(src: &VkDeviceGroupRenderPassBeginInfo, dst: &mut RawVkDeviceGroupRenderPassBeginInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DeviceGroupRenderPassBeginInfo);
        dst.next = ptr::null();
        dst.device_mask = src.device_mask;
        dst.device_render_area_count = src.device_render_areas.len() as u32;
        dst.device_render_areas = new_ptr_vk_array(src.device_render_areas);
    }
}

impl Default for VkDeviceGroupRenderPassBeginInfo<'static> {
    fn default() -> VkDeviceGroupRenderPassBeginInfo<'static> {
        VkDeviceGroupRenderPassBeginInfo {
            device_mask: 0,
            device_render_areas: &[],
        }
    }
}

impl<'a> VkSetup for VkDeviceGroupRenderPassBeginInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDeviceGroupRenderPassBeginInfo {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.device_render_area_count as usize, self.device_render_areas);
    }
}