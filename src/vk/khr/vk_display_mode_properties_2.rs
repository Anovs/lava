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
use vk::khr::vk_display_mode_properties::*;

#[repr(C)]
pub struct RawVkDisplayModeProperties2 {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub display_mode_properties: RawVkDisplayModeProperties,
}

#[derive(Debug, Clone)]
pub struct VkDisplayModeProperties2 {
    pub display_mode_properties: VkDisplayModeProperties,
}

impl VkRawType<VkDisplayModeProperties2> for RawVkDisplayModeProperties2 {
    fn vk_to_wrapped(src: &RawVkDisplayModeProperties2) -> VkDisplayModeProperties2 {
        VkDisplayModeProperties2 {
            display_mode_properties: RawVkDisplayModeProperties::vk_to_wrapped(&src.display_mode_properties),
        }
    }
}

impl VkSetup for VkDisplayModeProperties2 {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.display_mode_properties, fn_table, instance, device);
    }
}

impl VkFree for RawVkDisplayModeProperties2 {
    fn vk_free(&mut self) {
        RawVkDisplayModeProperties::vk_free(&mut self.display_mode_properties);
    }
}