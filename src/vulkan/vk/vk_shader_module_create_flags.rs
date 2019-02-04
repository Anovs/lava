// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkShaderModuleCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkShaderModuleCreateFlagBits.html).
///
/// Use the macro `VkShaderModuleCreateFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkShaderModuleCreateFlags!()
/// ```
/// ```
/// VkShaderModuleCreateFlags {
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct VkShaderModuleCreateFlags {
    
}

#[doc(hidden)]
pub type RawVkShaderModuleCreateFlags = u32;

impl VkWrappedType<RawVkShaderModuleCreateFlags> for VkShaderModuleCreateFlags {
    fn vk_to_raw(src: &VkShaderModuleCreateFlags, dst: &mut RawVkShaderModuleCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkShaderModuleCreateFlags> for RawVkShaderModuleCreateFlags {
    fn vk_to_wrapped(src: &RawVkShaderModuleCreateFlags) -> VkShaderModuleCreateFlags {
        VkShaderModuleCreateFlags {
            
        }
    }
}

impl Default for VkShaderModuleCreateFlags {
    fn default() -> VkShaderModuleCreateFlags {
        VkShaderModuleCreateFlags {
            
        }
    }
}

impl VkShaderModuleCreateFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkShaderModuleCreateFlags {
            
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkShaderModuleCreateFlags {
            
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkShaderModuleCreateFlags {
            
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkShaderModuleCreateFlags {
    ( $( $x:ident ),* ) => {
        VkShaderModuleCreateFlags {
            $($x: true,)*
            ..VkShaderModuleCreateFlags::none()
        }
    }
}