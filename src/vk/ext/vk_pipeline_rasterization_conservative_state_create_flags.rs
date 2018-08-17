// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkPipelineRasterizationConservativeStateCreateFlags = u32;

#[derive(Debug, Clone, Copy)]
pub struct VkPipelineRasterizationConservativeStateCreateFlags {
    
}

impl VkRawType<VkPipelineRasterizationConservativeStateCreateFlags> for RawVkPipelineRasterizationConservativeStateCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineRasterizationConservativeStateCreateFlags) -> VkPipelineRasterizationConservativeStateCreateFlags {
        VkPipelineRasterizationConservativeStateCreateFlags {
            
        }
    }
}

impl VkWrappedType<RawVkPipelineRasterizationConservativeStateCreateFlags> for VkPipelineRasterizationConservativeStateCreateFlags {
    fn vk_to_raw(src: &VkPipelineRasterizationConservativeStateCreateFlags, dst: &mut RawVkPipelineRasterizationConservativeStateCreateFlags) {
        *dst = 0;
    }
}

impl Default for VkPipelineRasterizationConservativeStateCreateFlags {
    fn default() -> VkPipelineRasterizationConservativeStateCreateFlags {
        VkPipelineRasterizationConservativeStateCreateFlags {
            
        }
    }
}

impl VkPipelineRasterizationConservativeStateCreateFlags {
    
    pub fn none() -> VkPipelineRasterizationConservativeStateCreateFlags {
        VkPipelineRasterizationConservativeStateCreateFlags {
            
        }
    }
    
    pub fn all() -> VkPipelineRasterizationConservativeStateCreateFlags {
        VkPipelineRasterizationConservativeStateCreateFlags {
            
        }
    }
}