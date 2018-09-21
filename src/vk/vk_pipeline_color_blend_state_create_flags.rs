// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkPipelineColorBlendStateCreateFlags = u32;

#[derive(Debug, Clone, Copy)]
pub struct VkPipelineColorBlendStateCreateFlags {
    
}

impl VkRawType<VkPipelineColorBlendStateCreateFlags> for RawVkPipelineColorBlendStateCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineColorBlendStateCreateFlags) -> VkPipelineColorBlendStateCreateFlags {
        VkPipelineColorBlendStateCreateFlags {
            
        }
    }
}

impl VkWrappedType<RawVkPipelineColorBlendStateCreateFlags> for VkPipelineColorBlendStateCreateFlags {
    fn vk_to_raw(src: &VkPipelineColorBlendStateCreateFlags, dst: &mut RawVkPipelineColorBlendStateCreateFlags) {
        *dst = 0;
    }
}

impl Default for VkPipelineColorBlendStateCreateFlags {
    fn default() -> VkPipelineColorBlendStateCreateFlags {
        VkPipelineColorBlendStateCreateFlags {
            
        }
    }
}

impl VkPipelineColorBlendStateCreateFlags {
    
    pub fn none() -> VkPipelineColorBlendStateCreateFlags {
        VkPipelineColorBlendStateCreateFlags {
            
        }
    }
    
    pub fn all() -> VkPipelineColorBlendStateCreateFlags {
        VkPipelineColorBlendStateCreateFlags {
            
        }
    }
}

impl VkPipelineColorBlendStateCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}