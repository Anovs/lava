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
use vk::ext::vk_validation_check::*;

#[repr(C)]
pub struct RawVkValidationFlags {
    s_type: RawVkStructureType,
    next: *const c_void,
    disabled_validation_check_count: u32,
    disabled_validation_checks: *mut RawVkValidationCheck,
}

#[derive(Debug, Clone)]
pub struct VkValidationFlags<'a> {
    pub disabled_validation_checks: &'a [VkValidationCheck],
}

impl<'a> VkWrappedType<RawVkValidationFlags> for VkValidationFlags<'a> {
    fn vk_to_raw(src: &VkValidationFlags, dst: &mut RawVkValidationFlags) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ValidationFlagsExt);
        dst.next = ptr::null();
        dst.disabled_validation_check_count = src.disabled_validation_checks.len() as u32;
        dst.disabled_validation_checks = new_ptr_vk_array(src.disabled_validation_checks);
    }
}

impl Default for VkValidationFlags<'static> {
    fn default() -> VkValidationFlags<'static> {
        VkValidationFlags {
            disabled_validation_checks: &[],
        }
    }
}

impl<'a> VkSetup for VkValidationFlags<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkValidationFlags {
    fn vk_free(&mut self) {
        free_ptr(self.disabled_validation_checks);
    }
}