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
use vulkan::vk::{VkExternalMemoryHandleTypeFlags,RawVkExternalMemoryHandleTypeFlags};

/// Wrapper for [VkExportMemoryAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkExportMemoryAllocateInfo.html).
#[derive(Debug, Clone)]
pub struct VkExportMemoryAllocateInfo {
    pub handle_types: VkExternalMemoryHandleTypeFlags,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkExportMemoryAllocateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub handle_types: RawVkExternalMemoryHandleTypeFlags,
}

impl VkWrappedType<RawVkExportMemoryAllocateInfo> for VkExportMemoryAllocateInfo {
    fn vk_to_raw(src: &VkExportMemoryAllocateInfo, dst: &mut RawVkExportMemoryAllocateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ExportMemoryAllocateInfo);
        dst.next = ptr::null();
        dst.handle_types = vk_to_raw_value(&src.handle_types);
    }
}

impl VkRawType<VkExportMemoryAllocateInfo> for RawVkExportMemoryAllocateInfo {
    fn vk_to_wrapped(src: &RawVkExportMemoryAllocateInfo) -> VkExportMemoryAllocateInfo {
        VkExportMemoryAllocateInfo {
            handle_types: RawVkExternalMemoryHandleTypeFlags::vk_to_wrapped(&src.handle_types),
        }
    }
}

impl Default for VkExportMemoryAllocateInfo {
    fn default() -> VkExportMemoryAllocateInfo {
        VkExportMemoryAllocateInfo {
            handle_types: VkExternalMemoryHandleTypeFlags::default(),
        }
    }
}

impl VkSetup for VkExportMemoryAllocateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkExportMemoryAllocateInfo {
    fn vk_free(&mut self) {
        
    }
}