// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkEventCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkEventCreateFlagBits.html)
#[derive(Debug, Clone, Copy)]
pub struct VkEventCreateFlags {
    
}

#[doc(hidden)]
pub type RawVkEventCreateFlags = u32;

impl VkWrappedType<RawVkEventCreateFlags> for VkEventCreateFlags {
    fn vk_to_raw(src: &VkEventCreateFlags, dst: &mut RawVkEventCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkEventCreateFlags> for RawVkEventCreateFlags {
    fn vk_to_wrapped(src: &RawVkEventCreateFlags) -> VkEventCreateFlags {
        VkEventCreateFlags {
            
        }
    }
}

impl Default for VkEventCreateFlags {
    fn default() -> VkEventCreateFlags {
        VkEventCreateFlags {
            
        }
    }
}

impl VkEventCreateFlags {
    
    pub fn none() -> VkEventCreateFlags {
        VkEventCreateFlags {
            
        }
    }
    
    pub fn all() -> VkEventCreateFlags {
        VkEventCreateFlags {
            
        }
    }
}

#[macro_export]
macro_rules! VkEventCreateFlags {
    ( $( $x:ident ),* ) => {
        VkEventCreateFlags {
            $($x: true,)*
            ..VkEventCreateFlags::none()
        }
    }
}

impl VkEventCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
    }
}