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
use vk::nv::vk_pipeline_coverage_to_color_state_create_flags::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawVkPipelineCoverageToColorStateCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkPipelineCoverageToColorStateCreateFlags,
    pub coverage_to_color_enable: u32,
    pub coverage_to_color_location: u32,
}

#[derive(Debug, Clone)]
pub struct VkPipelineCoverageToColorStateCreateInfo {
    pub flags: VkPipelineCoverageToColorStateCreateFlags,
    pub coverage_to_color_enable: bool,
    pub coverage_to_color_location: usize,
}

impl VkRawType<VkPipelineCoverageToColorStateCreateInfo> for RawVkPipelineCoverageToColorStateCreateInfo {
    fn vk_to_wrapped(src: &RawVkPipelineCoverageToColorStateCreateInfo) -> VkPipelineCoverageToColorStateCreateInfo {
        VkPipelineCoverageToColorStateCreateInfo {
            flags: RawVkPipelineCoverageToColorStateCreateFlags::vk_to_wrapped(&src.flags),
            coverage_to_color_enable: u32::vk_to_wrapped(&src.coverage_to_color_enable),
            coverage_to_color_location: u32::vk_to_wrapped(&src.coverage_to_color_location),
        }
    }
}

impl VkWrappedType<RawVkPipelineCoverageToColorStateCreateInfo> for VkPipelineCoverageToColorStateCreateInfo {
    fn vk_to_raw(src: &VkPipelineCoverageToColorStateCreateInfo, dst: &mut RawVkPipelineCoverageToColorStateCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineCoverageToColorStateCreateInfoNv);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.coverage_to_color_enable = vk_to_raw_value(&src.coverage_to_color_enable);
        dst.coverage_to_color_location = vk_to_raw_value(&src.coverage_to_color_location);
    }
}

impl Default for VkPipelineCoverageToColorStateCreateInfo {
    fn default() -> VkPipelineCoverageToColorStateCreateInfo {
        VkPipelineCoverageToColorStateCreateInfo {
            flags: VkPipelineCoverageToColorStateCreateFlags::default(),
            coverage_to_color_enable: false,
            coverage_to_color_location: 0,
        }
    }
}

impl VkSetup for VkPipelineCoverageToColorStateCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPipelineCoverageToColorStateCreateInfo {
    fn vk_free(&mut self) {
        
    }
}