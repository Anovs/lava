// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkCommandPoolTrimFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCommandPoolTrimFlagBits.html)
///
/// Use the macro `VkCommandPoolTrimFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkCommandPoolTrimFlags!()
/// ```
/// ```
/// VkCommandPoolTrimFlags {
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct VkCommandPoolTrimFlags {
    
}

#[doc(hidden)]
pub type RawVkCommandPoolTrimFlags = u32;

impl VkWrappedType<RawVkCommandPoolTrimFlags> for VkCommandPoolTrimFlags {
    fn vk_to_raw(src: &VkCommandPoolTrimFlags, dst: &mut RawVkCommandPoolTrimFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkCommandPoolTrimFlags> for RawVkCommandPoolTrimFlags {
    fn vk_to_wrapped(src: &RawVkCommandPoolTrimFlags) -> VkCommandPoolTrimFlags {
        VkCommandPoolTrimFlags {
            
        }
    }
}

impl Default for VkCommandPoolTrimFlags {
    fn default() -> VkCommandPoolTrimFlags {
        VkCommandPoolTrimFlags {
            
        }
    }
}

impl VkCommandPoolTrimFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkCommandPoolTrimFlags {
            
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkCommandPoolTrimFlags {
            
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkCommandPoolTrimFlags {
            
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkCommandPoolTrimFlags {
    ( $( $x:ident ),* ) => {
        VkCommandPoolTrimFlags {
            $($x: true,)*
            ..VkCommandPoolTrimFlags::none()
        }
    }
}