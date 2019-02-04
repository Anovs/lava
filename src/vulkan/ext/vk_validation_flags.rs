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
use vulkan::ext::{VkValidationCheck,RawVkValidationCheck};

/// Wrapper for [VkValidationFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkValidationFlagBitsEXT.html).
#[derive(Debug, Clone)]
pub struct VkValidationFlags<'a> {
    pub disabled_validation_checks: &'a [VkValidationCheck],
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkValidationFlags {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub disabled_validation_check_count: u32,
    pub disabled_validation_checks: *mut RawVkValidationCheck,
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