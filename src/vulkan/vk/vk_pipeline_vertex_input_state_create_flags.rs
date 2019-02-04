// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPipelineVertexInputStateCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineVertexInputStateCreateFlagBits.html)
///
/// Use the macro `VkPipelineVertexInputStateCreateFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkPipelineVertexInputStateCreateFlags!()
/// ```
/// ```
/// VkPipelineVertexInputStateCreateFlags {
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineVertexInputStateCreateFlags {
    
}

#[doc(hidden)]
pub type RawVkPipelineVertexInputStateCreateFlags = u32;

impl VkWrappedType<RawVkPipelineVertexInputStateCreateFlags> for VkPipelineVertexInputStateCreateFlags {
    fn vk_to_raw(src: &VkPipelineVertexInputStateCreateFlags, dst: &mut RawVkPipelineVertexInputStateCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkPipelineVertexInputStateCreateFlags> for RawVkPipelineVertexInputStateCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineVertexInputStateCreateFlags) -> VkPipelineVertexInputStateCreateFlags {
        VkPipelineVertexInputStateCreateFlags {
            
        }
    }
}

impl Default for VkPipelineVertexInputStateCreateFlags {
    fn default() -> VkPipelineVertexInputStateCreateFlags {
        VkPipelineVertexInputStateCreateFlags {
            
        }
    }
}

impl VkPipelineVertexInputStateCreateFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkPipelineVertexInputStateCreateFlags {
            
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkPipelineVertexInputStateCreateFlags {
            
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkPipelineVertexInputStateCreateFlags {
            
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkPipelineVertexInputStateCreateFlags {
    ( $( $x:ident ),* ) => {
        VkPipelineVertexInputStateCreateFlags {
            $($x: true,)*
            ..VkPipelineVertexInputStateCreateFlags::none()
        }
    }
}