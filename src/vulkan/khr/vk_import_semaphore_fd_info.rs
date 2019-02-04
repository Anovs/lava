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
use vulkan::vk::{VkSemaphore,RawVkSemaphore};
use vulkan::vk::{VkSemaphoreImportFlags,RawVkSemaphoreImportFlags};
use vulkan::vk::{VkExternalSemaphoreHandleTypeFlags,RawVkExternalSemaphoreHandleTypeFlags};

/// Wrapper for [VkImportSemaphoreFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkImportSemaphoreFdInfoKHR.html)
#[derive(Debug, Clone)]
pub struct VkImportSemaphoreFdInfo<'a> {
    pub semaphore: &'a VkSemaphore,
    pub flags: VkSemaphoreImportFlags,
    pub handle_type: VkExternalSemaphoreHandleTypeFlags,
    pub fd: i32,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkImportSemaphoreFdInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub semaphore: RawVkSemaphore,
    pub flags: RawVkSemaphoreImportFlags,
    pub handle_type: RawVkExternalSemaphoreHandleTypeFlags,
    pub fd: i32,
}

impl<'a> VkWrappedType<RawVkImportSemaphoreFdInfo> for VkImportSemaphoreFdInfo<'a> {
    fn vk_to_raw(src: &VkImportSemaphoreFdInfo, dst: &mut RawVkImportSemaphoreFdInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ImportSemaphoreFdInfoKhr);
        dst.next = ptr::null();
        dst.semaphore = vk_to_raw_value(src.semaphore);
        dst.flags = vk_to_raw_value(&src.flags);
        dst.handle_type = vk_to_raw_value(&src.handle_type);
        dst.fd = src.fd;
    }
}

impl Default for VkImportSemaphoreFdInfo<'static> {
    fn default() -> VkImportSemaphoreFdInfo<'static> {
        VkImportSemaphoreFdInfo {
            semaphore: vk_null_ref(),
            flags: VkSemaphoreImportFlags::default(),
            handle_type: VkExternalSemaphoreHandleTypeFlags::default(),
            fd: 0,
        }
    }
}

impl<'a> VkSetup for VkImportSemaphoreFdInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkImportSemaphoreFdInfo {
    fn vk_free(&mut self) {
        
    }
}