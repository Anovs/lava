// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkMemoryAllocateFlags = u32;

#[derive(Debug, Clone, Copy)]
pub struct VkMemoryAllocateFlags {
    pub device_mask: bool,
}

impl VkRawType<VkMemoryAllocateFlags> for RawVkMemoryAllocateFlags {
    fn vk_to_wrapped(src: &RawVkMemoryAllocateFlags) -> VkMemoryAllocateFlags {
        VkMemoryAllocateFlags {
            device_mask: (src & 0x00000001) != 0,
        }
    }
}

impl VkWrappedType<RawVkMemoryAllocateFlags> for VkMemoryAllocateFlags {
    fn vk_to_raw(src: &VkMemoryAllocateFlags, dst: &mut RawVkMemoryAllocateFlags) {
        *dst = 0;
        if src.device_mask { *dst |= 0x00000001; }
    }
}

impl Default for VkMemoryAllocateFlags {
    fn default() -> VkMemoryAllocateFlags {
        VkMemoryAllocateFlags {
            device_mask: false,
        }
    }
}

impl VkMemoryAllocateFlags {
    
    pub fn none() -> VkMemoryAllocateFlags {
        VkMemoryAllocateFlags {
            device_mask: false,
        }
    }
    
    pub fn all() -> VkMemoryAllocateFlags {
        VkMemoryAllocateFlags {
            device_mask: true,
        }
    }
}