// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkPipelineCacheCreateFlags = u32;

#[derive(Debug, Clone, Copy)]
pub struct VkPipelineCacheCreateFlags {
    
}

impl VkRawType<VkPipelineCacheCreateFlags> for RawVkPipelineCacheCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineCacheCreateFlags) -> VkPipelineCacheCreateFlags {
        VkPipelineCacheCreateFlags {
            
        }
    }
}

impl VkWrappedType<RawVkPipelineCacheCreateFlags> for VkPipelineCacheCreateFlags {
    fn vk_to_raw(src: &VkPipelineCacheCreateFlags, dst: &mut RawVkPipelineCacheCreateFlags) {
        *dst = 0;
    }
}

impl Default for VkPipelineCacheCreateFlags {
    fn default() -> VkPipelineCacheCreateFlags {
        VkPipelineCacheCreateFlags {
            
        }
    }
}

impl VkPipelineCacheCreateFlags {
    
    pub fn none() -> VkPipelineCacheCreateFlags {
        VkPipelineCacheCreateFlags {
            
        }
    }
    
    pub fn all() -> VkPipelineCacheCreateFlags {
        VkPipelineCacheCreateFlags {
            
        }
    }
}

impl VkPipelineCacheCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}