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
use vk::ext::vk_blend_overlap::*;

#[repr(C)]
pub struct RawVkPipelineColorBlendAdvancedStateCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub src_premultiplied: u32,
    pub dst_premultiplied: u32,
    pub blend_overlap: RawVkBlendOverlap,
}

#[derive(Debug, Clone)]
pub struct VkPipelineColorBlendAdvancedStateCreateInfo {
    pub src_premultiplied: bool,
    pub dst_premultiplied: bool,
    pub blend_overlap: VkBlendOverlap,
}

impl VkRawType<VkPipelineColorBlendAdvancedStateCreateInfo> for RawVkPipelineColorBlendAdvancedStateCreateInfo {
    fn vk_to_wrapped(src: &RawVkPipelineColorBlendAdvancedStateCreateInfo) -> VkPipelineColorBlendAdvancedStateCreateInfo {
        VkPipelineColorBlendAdvancedStateCreateInfo {
            src_premultiplied: u32::vk_to_wrapped(&src.src_premultiplied),
            dst_premultiplied: u32::vk_to_wrapped(&src.dst_premultiplied),
            blend_overlap: RawVkBlendOverlap::vk_to_wrapped(&src.blend_overlap),
        }
    }
}

impl VkWrappedType<RawVkPipelineColorBlendAdvancedStateCreateInfo> for VkPipelineColorBlendAdvancedStateCreateInfo {
    fn vk_to_raw(src: &VkPipelineColorBlendAdvancedStateCreateInfo, dst: &mut RawVkPipelineColorBlendAdvancedStateCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineColorBlendAdvancedStateCreateInfoExt);
        dst.next = ptr::null();
        dst.src_premultiplied = vk_to_raw_value(&src.src_premultiplied);
        dst.dst_premultiplied = vk_to_raw_value(&src.dst_premultiplied);
        dst.blend_overlap = vk_to_raw_value(&src.blend_overlap);
    }
}

impl Default for VkPipelineColorBlendAdvancedStateCreateInfo {
    fn default() -> VkPipelineColorBlendAdvancedStateCreateInfo {
        VkPipelineColorBlendAdvancedStateCreateInfo {
            src_premultiplied: false,
            dst_premultiplied: false,
            blend_overlap: VkBlendOverlap::default(),
        }
    }
}

impl VkSetup for VkPipelineColorBlendAdvancedStateCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPipelineColorBlendAdvancedStateCreateInfo {
    fn vk_free(&mut self) {
        
    }
}