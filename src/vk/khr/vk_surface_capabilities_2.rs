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
use vk::khr::vk_surface_capabilities::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawVkSurfaceCapabilities2 {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub surface_capabilities: RawVkSurfaceCapabilities,
}

#[derive(Debug, Clone)]
pub struct VkSurfaceCapabilities2 {
    pub surface_capabilities: VkSurfaceCapabilities,
}

impl VkRawType<VkSurfaceCapabilities2> for RawVkSurfaceCapabilities2 {
    fn vk_to_wrapped(src: &RawVkSurfaceCapabilities2) -> VkSurfaceCapabilities2 {
        VkSurfaceCapabilities2 {
            surface_capabilities: RawVkSurfaceCapabilities::vk_to_wrapped(&src.surface_capabilities),
        }
    }
}

impl VkWrappedType<RawVkSurfaceCapabilities2> for VkSurfaceCapabilities2 {
    fn vk_to_raw(src: &VkSurfaceCapabilities2, dst: &mut RawVkSurfaceCapabilities2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SurfaceCapabilities2Khr);
        dst.next = ptr::null();
        dst.surface_capabilities = vk_to_raw_value(&src.surface_capabilities);
    }
}

impl Default for VkSurfaceCapabilities2 {
    fn default() -> VkSurfaceCapabilities2 {
        VkSurfaceCapabilities2 {
            surface_capabilities: VkSurfaceCapabilities::default(),
        }
    }
}

impl VkSetup for VkSurfaceCapabilities2 {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.surface_capabilities, fn_table, instance, device);
    }
}

impl VkFree for RawVkSurfaceCapabilities2 {
    fn vk_free(&mut self) {
        RawVkSurfaceCapabilities::vk_free(&mut self.surface_capabilities);
    }
}