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
use vk::vk_format::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkImageViewASTCDecodeMode {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub decode_mode: RawVkFormat,
}

#[derive(Debug, Clone)]
pub struct VkImageViewASTCDecodeMode {
    pub decode_mode: VkFormat,
}

impl VkRawType<VkImageViewASTCDecodeMode> for RawVkImageViewASTCDecodeMode {
    fn vk_to_wrapped(src: &RawVkImageViewASTCDecodeMode) -> VkImageViewASTCDecodeMode {
        VkImageViewASTCDecodeMode {
            decode_mode: RawVkFormat::vk_to_wrapped(&src.decode_mode),
        }
    }
}

impl VkWrappedType<RawVkImageViewASTCDecodeMode> for VkImageViewASTCDecodeMode {
    fn vk_to_raw(src: &VkImageViewASTCDecodeMode, dst: &mut RawVkImageViewASTCDecodeMode) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ImageViewAstcDecodeModeExt);
        dst.next = ptr::null();
        dst.decode_mode = vk_to_raw_value(&src.decode_mode);
    }
}

impl Default for VkImageViewASTCDecodeMode {
    fn default() -> VkImageViewASTCDecodeMode {
        VkImageViewASTCDecodeMode {
            decode_mode: VkFormat::default(),
        }
    }
}

impl VkSetup for VkImageViewASTCDecodeMode {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkImageViewASTCDecodeMode {
    fn vk_free(&mut self) {
        
    }
}