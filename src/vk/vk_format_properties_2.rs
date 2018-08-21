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
use vk::vk_format_properties::*;

#[repr(C)]
pub struct RawVkFormatProperties2 {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub format_properties: RawVkFormatProperties,
}

#[derive(Debug, Clone)]
pub struct VkFormatProperties2 {
    pub format_properties: VkFormatProperties,
}

impl VkRawType<VkFormatProperties2> for RawVkFormatProperties2 {
    fn vk_to_wrapped(src: &RawVkFormatProperties2) -> VkFormatProperties2 {
        VkFormatProperties2 {
            format_properties: RawVkFormatProperties::vk_to_wrapped(&src.format_properties),
        }
    }
}

impl VkWrappedType<RawVkFormatProperties2> for VkFormatProperties2 {
    fn vk_to_raw(src: &VkFormatProperties2, dst: &mut RawVkFormatProperties2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::FormatProperties2);
        dst.next = ptr::null();
        dst.format_properties = vk_to_raw_value(&src.format_properties);
    }
}

impl Default for VkFormatProperties2 {
    fn default() -> VkFormatProperties2 {
        VkFormatProperties2 {
            format_properties: VkFormatProperties::default(),
        }
    }
}

impl VkSetup for VkFormatProperties2 {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.format_properties, fn_table, instance, device);
    }
}

impl VkFree for RawVkFormatProperties2 {
    fn vk_free(&mut self) {
        RawVkFormatProperties::vk_free(&mut self.format_properties);
    }
}