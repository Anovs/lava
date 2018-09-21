// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkInstanceCreateFlags = u32;

#[derive(Debug, Clone, Copy)]
pub struct VkInstanceCreateFlags {
    
}

impl VkRawType<VkInstanceCreateFlags> for RawVkInstanceCreateFlags {
    fn vk_to_wrapped(src: &RawVkInstanceCreateFlags) -> VkInstanceCreateFlags {
        VkInstanceCreateFlags {
            
        }
    }
}

impl VkWrappedType<RawVkInstanceCreateFlags> for VkInstanceCreateFlags {
    fn vk_to_raw(src: &VkInstanceCreateFlags, dst: &mut RawVkInstanceCreateFlags) {
        *dst = 0;
    }
}

impl Default for VkInstanceCreateFlags {
    fn default() -> VkInstanceCreateFlags {
        VkInstanceCreateFlags {
            
        }
    }
}

impl VkInstanceCreateFlags {
    
    pub fn none() -> VkInstanceCreateFlags {
        VkInstanceCreateFlags {
            
        }
    }
    
    pub fn all() -> VkInstanceCreateFlags {
        VkInstanceCreateFlags {
            
        }
    }
}

impl VkInstanceCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}