// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkMemoryMapFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkMemoryMapFlagBits.html)
#[derive(Debug, Clone, Copy)]
pub struct VkMemoryMapFlags {
    
}

#[doc(hidden)]
pub type RawVkMemoryMapFlags = u32;

impl VkWrappedType<RawVkMemoryMapFlags> for VkMemoryMapFlags {
    fn vk_to_raw(src: &VkMemoryMapFlags, dst: &mut RawVkMemoryMapFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkMemoryMapFlags> for RawVkMemoryMapFlags {
    fn vk_to_wrapped(src: &RawVkMemoryMapFlags) -> VkMemoryMapFlags {
        VkMemoryMapFlags {
            
        }
    }
}

impl Default for VkMemoryMapFlags {
    fn default() -> VkMemoryMapFlags {
        VkMemoryMapFlags {
            
        }
    }
}

impl VkMemoryMapFlags {
    
    pub fn none() -> VkMemoryMapFlags {
        VkMemoryMapFlags {
            
        }
    }
    
    pub fn all() -> VkMemoryMapFlags {
        VkMemoryMapFlags {
            
        }
    }
}

#[macro_export]
macro_rules! VkMemoryMapFlags {
    ( $( $x:ident ),* ) => {
        VkMemoryMapFlags {
            $($x: true,)*
            ..VkMemoryMapFlags::none()
        }
    }
}

impl VkMemoryMapFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}