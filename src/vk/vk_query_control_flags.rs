// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkQueryControlFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkQueryControlFlags {
    precise: bool,
}

impl VkRawType<VkQueryControlFlags> for RawVkQueryControlFlags {
    
    fn vk_to_wrapped(src: &RawVkQueryControlFlags) -> VkQueryControlFlags {
        VkQueryControlFlags {
            precise: (src & 0x00000001) != 0,
        }
    }
}

impl VkWrappedType<RawVkQueryControlFlags> for VkQueryControlFlags {
    
    fn vk_to_raw(src: &VkQueryControlFlags, dst: &mut RawVkQueryControlFlags) {
        *dst = 0;
        if src.precise { *dst |= 0x00000001; }
    }
    
    fn vk_default() -> VkQueryControlFlags {
        VkQueryControlFlags {
            precise: false,
        }
    }
}