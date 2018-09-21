// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkCommandPoolResetFlags = u32;

#[derive(Debug, Clone, Copy)]
pub struct VkCommandPoolResetFlags {
    pub release_resources: bool,
}

impl VkRawType<VkCommandPoolResetFlags> for RawVkCommandPoolResetFlags {
    fn vk_to_wrapped(src: &RawVkCommandPoolResetFlags) -> VkCommandPoolResetFlags {
        VkCommandPoolResetFlags {
            release_resources: (src & 0x00000001) != 0,
        }
    }
}

impl VkWrappedType<RawVkCommandPoolResetFlags> for VkCommandPoolResetFlags {
    fn vk_to_raw(src: &VkCommandPoolResetFlags, dst: &mut RawVkCommandPoolResetFlags) {
        *dst = 0;
        if src.release_resources { *dst |= 0x00000001; }
    }
}

impl Default for VkCommandPoolResetFlags {
    fn default() -> VkCommandPoolResetFlags {
        VkCommandPoolResetFlags {
            release_resources: false,
        }
    }
}

impl VkCommandPoolResetFlags {
    
    pub fn none() -> VkCommandPoolResetFlags {
        VkCommandPoolResetFlags {
            release_resources: false,
        }
    }
    
    pub fn all() -> VkCommandPoolResetFlags {
        VkCommandPoolResetFlags {
            release_resources: true,
        }
    }
}

impl VkCommandPoolResetFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.release_resources { 0x00000001 } else { 0 }
    }
}