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

#[repr(C)]
pub struct RawVkProtectedSubmitInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub protected_submit: u32,
}

#[derive(Debug, Clone)]
pub struct VkProtectedSubmitInfo {
    pub protected_submit: bool,
}

impl VkRawType<VkProtectedSubmitInfo> for RawVkProtectedSubmitInfo {
    fn vk_to_wrapped(src: &RawVkProtectedSubmitInfo) -> VkProtectedSubmitInfo {
        VkProtectedSubmitInfo {
            protected_submit: u32::vk_to_wrapped(&src.protected_submit),
        }
    }
}

impl VkWrappedType<RawVkProtectedSubmitInfo> for VkProtectedSubmitInfo {
    fn vk_to_raw(src: &VkProtectedSubmitInfo, dst: &mut RawVkProtectedSubmitInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ProtectedSubmitInfo);
        dst.next = ptr::null();
        dst.protected_submit = vk_to_raw_value(&src.protected_submit);
    }
}

impl Default for VkProtectedSubmitInfo {
    fn default() -> VkProtectedSubmitInfo {
        VkProtectedSubmitInfo {
            protected_submit: false,
        }
    }
}

impl VkSetup for VkProtectedSubmitInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkProtectedSubmitInfo {
    fn vk_free(&mut self) {
        
    }
}