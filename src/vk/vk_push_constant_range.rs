// Generated by `scripts/generate_vk.js`

use vk::*;
use std::os::raw::c_char;
use std::ptr::null;
use libc::*;

#[repr(C)]
pub struct RawVkPushConstantRange {
    stage_flags: RawVkShaderStageFlags,
    offset: u32,
    size: u32,
}

#[derive(Debug)]
pub struct VkPushConstantRange {
    pub stage_flags: VkShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

impl VkFrom<VkPushConstantRange> for RawVkPushConstantRange {
    
    fn vk_from(value: &VkPushConstantRange) -> Self {
        Self {
            stage_flags: VkFrom::vk_from(&value.stage_flags),
            offset: value.offset,
            size: value.size,
        }
    }
}

impl VkFrom<RawVkPushConstantRange> for VkPushConstantRange {
    
    fn vk_from(value: &RawVkPushConstantRange) -> Self {
        Self {
            stage_flags: VkFrom::vk_from(&value.stage_flags),
            offset: value.offset,
            size: value.size,
        }
    }
}