// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkSamplerReductionMode = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkSamplerReductionMode {
    WeightedAverage = 0,
    Min = 1,
    Max = 2,
}

impl VkRawType<VkSamplerReductionMode> for RawVkSamplerReductionMode {
    
    fn vk_to_wrapped(src: &RawVkSamplerReductionMode) -> VkSamplerReductionMode {
        unsafe {
            *((src as *const i32) as *const VkSamplerReductionMode)
        }
    }
}

impl VkWrappedType<RawVkSamplerReductionMode> for VkSamplerReductionMode {
    
    fn vk_to_raw(src: &VkSamplerReductionMode, dst: &mut RawVkSamplerReductionMode) {
        *dst = *src as i32
    }
    
    fn vk_default() -> VkSamplerReductionMode {
        VkSamplerReductionMode::WeightedAverage
    }
}