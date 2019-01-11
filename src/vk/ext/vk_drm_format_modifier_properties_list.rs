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
use vk::ext::vk_drm_format_modifier_properties::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDrmFormatModifierPropertiesList {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub drm_format_modifier_count: u32,
    pub drm_format_modifier_properties: *mut RawVkDrmFormatModifierProperties,
}

#[derive(Debug, Clone)]
pub struct VkDrmFormatModifierPropertiesList<'a> {
    pub drm_format_modifier_properties: Option<&'a [VkDrmFormatModifierProperties]>,
}

impl<'a> VkWrappedType<RawVkDrmFormatModifierPropertiesList> for VkDrmFormatModifierPropertiesList<'a> {
    fn vk_to_raw(src: &VkDrmFormatModifierPropertiesList, dst: &mut RawVkDrmFormatModifierPropertiesList) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DrmFormatModifierPropertiesListExt);
        dst.next = ptr::null();
        dst.drm_format_modifier_count = get_array_option_len(src.drm_format_modifier_properties) as u32;
        dst.drm_format_modifier_properties = new_ptr_vk_array_checked(src.drm_format_modifier_properties);
    }
}

impl Default for VkDrmFormatModifierPropertiesList<'static> {
    fn default() -> VkDrmFormatModifierPropertiesList<'static> {
        VkDrmFormatModifierPropertiesList {
            drm_format_modifier_properties: None,
        }
    }
}

impl<'a> VkSetup for VkDrmFormatModifierPropertiesList<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDrmFormatModifierPropertiesList {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.drm_format_modifier_count as usize, self.drm_format_modifier_properties);
    }
}