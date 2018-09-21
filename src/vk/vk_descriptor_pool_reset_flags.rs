// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkDescriptorPoolResetFlags = u32;

#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorPoolResetFlags {
    
}

impl VkRawType<VkDescriptorPoolResetFlags> for RawVkDescriptorPoolResetFlags {
    fn vk_to_wrapped(src: &RawVkDescriptorPoolResetFlags) -> VkDescriptorPoolResetFlags {
        VkDescriptorPoolResetFlags {
            
        }
    }
}

impl VkWrappedType<RawVkDescriptorPoolResetFlags> for VkDescriptorPoolResetFlags {
    fn vk_to_raw(src: &VkDescriptorPoolResetFlags, dst: &mut RawVkDescriptorPoolResetFlags) {
        *dst = 0;
    }
}

impl Default for VkDescriptorPoolResetFlags {
    fn default() -> VkDescriptorPoolResetFlags {
        VkDescriptorPoolResetFlags {
            
        }
    }
}

impl VkDescriptorPoolResetFlags {
    
    pub fn none() -> VkDescriptorPoolResetFlags {
        VkDescriptorPoolResetFlags {
            
        }
    }
    
    pub fn all() -> VkDescriptorPoolResetFlags {
        VkDescriptorPoolResetFlags {
            
        }
    }
}

impl VkDescriptorPoolResetFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}