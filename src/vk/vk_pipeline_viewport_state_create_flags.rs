// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkPipelineViewportStateCreateFlags = u32;

#[derive(Debug, Clone, Copy)]
pub struct VkPipelineViewportStateCreateFlags {
    
}

impl VkRawType<VkPipelineViewportStateCreateFlags> for RawVkPipelineViewportStateCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineViewportStateCreateFlags) -> VkPipelineViewportStateCreateFlags {
        VkPipelineViewportStateCreateFlags {
            
        }
    }
}

impl VkWrappedType<RawVkPipelineViewportStateCreateFlags> for VkPipelineViewportStateCreateFlags {
    fn vk_to_raw(src: &VkPipelineViewportStateCreateFlags, dst: &mut RawVkPipelineViewportStateCreateFlags) {
        *dst = 0;
    }
}

impl Default for VkPipelineViewportStateCreateFlags {
    fn default() -> VkPipelineViewportStateCreateFlags {
        VkPipelineViewportStateCreateFlags {
            
        }
    }
}

impl VkPipelineViewportStateCreateFlags {
    
    pub fn none() -> VkPipelineViewportStateCreateFlags {
        VkPipelineViewportStateCreateFlags {
            
        }
    }
    
    pub fn all() -> VkPipelineViewportStateCreateFlags {
        VkPipelineViewportStateCreateFlags {
            
        }
    }
}

impl VkPipelineViewportStateCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}