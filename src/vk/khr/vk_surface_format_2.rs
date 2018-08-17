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
use vk::khr::vk_surface_format::*;

#[repr(C)]
pub struct RawVkSurfaceFormat2 {
    s_type: RawVkStructureType,
    next: *const c_void,
    surface_format: RawVkSurfaceFormat,
}

#[derive(Debug, Clone)]
pub struct VkSurfaceFormat2 {
    pub surface_format: VkSurfaceFormat,
}

impl VkRawType<VkSurfaceFormat2> for RawVkSurfaceFormat2 {
    fn vk_to_wrapped(src: &RawVkSurfaceFormat2) -> VkSurfaceFormat2 {
        VkSurfaceFormat2 {
            surface_format: RawVkSurfaceFormat::vk_to_wrapped(&src.surface_format),
        }
    }
}

impl VkWrappedType<RawVkSurfaceFormat2> for VkSurfaceFormat2 {
    fn vk_to_raw(src: &VkSurfaceFormat2, dst: &mut RawVkSurfaceFormat2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SurfaceFormat2Khr);
        dst.next = ptr::null();
        dst.surface_format = vk_to_raw_value(&src.surface_format);
    }
}

impl Default for VkSurfaceFormat2 {
    fn default() -> VkSurfaceFormat2 {
        VkSurfaceFormat2 {
            surface_format: VkSurfaceFormat::default(),
        }
    }
}

impl VkSetup for VkSurfaceFormat2 {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.surface_format, fn_table, instance, device);
    }
}

impl VkFree for RawVkSurfaceFormat2 {
    fn vk_free(&mut self) {
        RawVkSurfaceFormat::vk_free(&mut self.surface_format);
    }
}