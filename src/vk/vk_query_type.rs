// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkQueryType = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkQueryType {
    Occlusion = 0,
    PipelineStatistics = 1,
    Timestamp = 2,
}

impl VkRawType<VkQueryType> for RawVkQueryType {
    fn vk_to_wrapped(src: &RawVkQueryType) -> VkQueryType {
        unsafe {
            *((src as *const i32) as *const VkQueryType)
        }
    }
}

impl VkWrappedType<RawVkQueryType> for VkQueryType {
    fn vk_to_raw(src: &VkQueryType, dst: &mut RawVkQueryType) {
        *dst = *src as i32
    }
}

impl Default for VkQueryType {
    fn default() -> VkQueryType {
        VkQueryType::Occlusion
    }
}