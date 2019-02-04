// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkDebugUtilsMessengerCallbackDataFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDebugUtilsMessengerCallbackDataFlagBitsEXT.html).
///
/// Use the macro `VkDebugUtilsMessengerCallbackDataFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkDebugUtilsMessengerCallbackDataFlags!()
/// ```
/// ```
/// VkDebugUtilsMessengerCallbackDataFlags {
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct VkDebugUtilsMessengerCallbackDataFlags {
    
}

#[doc(hidden)]
pub type RawVkDebugUtilsMessengerCallbackDataFlags = u32;

impl VkWrappedType<RawVkDebugUtilsMessengerCallbackDataFlags> for VkDebugUtilsMessengerCallbackDataFlags {
    fn vk_to_raw(src: &VkDebugUtilsMessengerCallbackDataFlags, dst: &mut RawVkDebugUtilsMessengerCallbackDataFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkDebugUtilsMessengerCallbackDataFlags> for RawVkDebugUtilsMessengerCallbackDataFlags {
    fn vk_to_wrapped(src: &RawVkDebugUtilsMessengerCallbackDataFlags) -> VkDebugUtilsMessengerCallbackDataFlags {
        VkDebugUtilsMessengerCallbackDataFlags {
            
        }
    }
}

impl Default for VkDebugUtilsMessengerCallbackDataFlags {
    fn default() -> VkDebugUtilsMessengerCallbackDataFlags {
        VkDebugUtilsMessengerCallbackDataFlags {
            
        }
    }
}

impl VkDebugUtilsMessengerCallbackDataFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkDebugUtilsMessengerCallbackDataFlags {
            
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkDebugUtilsMessengerCallbackDataFlags {
            
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkDebugUtilsMessengerCallbackDataFlags {
            
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkDebugUtilsMessengerCallbackDataFlags {
    ( $( $x:ident ),* ) => {
        VkDebugUtilsMessengerCallbackDataFlags {
            $($x: true,)*
            ..VkDebugUtilsMessengerCallbackDataFlags::none()
        }
    }
}