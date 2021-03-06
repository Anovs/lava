// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkDisplaySurfaceCreateFlagsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplaySurfaceCreateFlagsKHR.html).
///
/// Use the macro `VkDisplaySurfaceCreateFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkDisplaySurfaceCreateFlags!()
/// ```
/// ```
/// VkDisplaySurfaceCreateFlags {
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkDisplaySurfaceCreateFlags {
    
}

#[doc(hidden)]
pub type RawVkDisplaySurfaceCreateFlags = u32;

impl VkWrappedType<RawVkDisplaySurfaceCreateFlags> for VkDisplaySurfaceCreateFlags {
    fn vk_to_raw(src: &VkDisplaySurfaceCreateFlags, dst: &mut RawVkDisplaySurfaceCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkDisplaySurfaceCreateFlags> for RawVkDisplaySurfaceCreateFlags {
    fn vk_to_wrapped(src: &RawVkDisplaySurfaceCreateFlags) -> VkDisplaySurfaceCreateFlags {
        VkDisplaySurfaceCreateFlags {
            
        }
    }
}

impl Default for VkDisplaySurfaceCreateFlags {
    fn default() -> VkDisplaySurfaceCreateFlags {
        VkDisplaySurfaceCreateFlags {
            
        }
    }
}

impl VkDisplaySurfaceCreateFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkDisplaySurfaceCreateFlags {
            
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkDisplaySurfaceCreateFlags {
            
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkDisplaySurfaceCreateFlags {
            
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkDisplaySurfaceCreateFlags {
    ( $( $x:ident ),* ) => {
        VkDisplaySurfaceCreateFlags {
            $($x: true,)*
            ..VkDisplaySurfaceCreateFlags::none()
        }
    }
}