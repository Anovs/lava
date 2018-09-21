// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkConditionalRenderingFlags = u32;

#[derive(Debug, Clone, Copy)]
pub struct VkConditionalRenderingFlags {
    pub inverted: bool,
}

impl VkRawType<VkConditionalRenderingFlags> for RawVkConditionalRenderingFlags {
    fn vk_to_wrapped(src: &RawVkConditionalRenderingFlags) -> VkConditionalRenderingFlags {
        VkConditionalRenderingFlags {
            inverted: (src & 0x00000001) != 0,
        }
    }
}

impl VkWrappedType<RawVkConditionalRenderingFlags> for VkConditionalRenderingFlags {
    fn vk_to_raw(src: &VkConditionalRenderingFlags, dst: &mut RawVkConditionalRenderingFlags) {
        *dst = 0;
        if src.inverted { *dst |= 0x00000001; }
    }
}

impl Default for VkConditionalRenderingFlags {
    fn default() -> VkConditionalRenderingFlags {
        VkConditionalRenderingFlags {
            inverted: false,
        }
    }
}

impl VkConditionalRenderingFlags {
    
    pub fn none() -> VkConditionalRenderingFlags {
        VkConditionalRenderingFlags {
            inverted: false,
        }
    }
    
    pub fn all() -> VkConditionalRenderingFlags {
        VkConditionalRenderingFlags {
            inverted: true,
        }
    }
}

impl VkConditionalRenderingFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.inverted { 0x00000001 } else { 0 }
    }
}