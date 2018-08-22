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
use vk::khr::vk_surface::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawVkPhysicalDeviceSurfaceInfo2 {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub surface: RawVkSurface,
}

#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceSurfaceInfo2<'a> {
    pub surface: &'a VkSurface,
}

impl<'a> VkWrappedType<RawVkPhysicalDeviceSurfaceInfo2> for VkPhysicalDeviceSurfaceInfo2<'a> {
    fn vk_to_raw(src: &VkPhysicalDeviceSurfaceInfo2, dst: &mut RawVkPhysicalDeviceSurfaceInfo2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceSurfaceInfo2Khr);
        dst.next = ptr::null();
        dst.surface = vk_to_raw_value(src.surface);
    }
}

impl Default for VkPhysicalDeviceSurfaceInfo2<'static> {
    fn default() -> VkPhysicalDeviceSurfaceInfo2<'static> {
        VkPhysicalDeviceSurfaceInfo2 {
            surface: vk_null_ref(),
        }
    }
}

impl<'a> VkSetup for VkPhysicalDeviceSurfaceInfo2<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceSurfaceInfo2 {
    fn vk_free(&mut self) {
        
    }
}