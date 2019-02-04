// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPipelineCoverageModulationStateCreateFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineCoverageModulationStateCreateFlagBitsNV.html)
///
/// Use the macro `VkPipelineCoverageModulationStateCreateFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkPipelineCoverageModulationStateCreateFlags!()
/// ```
/// ```
/// VkPipelineCoverageModulationStateCreateFlags {
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineCoverageModulationStateCreateFlags {
    
}

#[doc(hidden)]
pub type RawVkPipelineCoverageModulationStateCreateFlags = u32;

impl VkWrappedType<RawVkPipelineCoverageModulationStateCreateFlags> for VkPipelineCoverageModulationStateCreateFlags {
    fn vk_to_raw(src: &VkPipelineCoverageModulationStateCreateFlags, dst: &mut RawVkPipelineCoverageModulationStateCreateFlags) {
        *dst = 0;
    }
}

impl VkRawType<VkPipelineCoverageModulationStateCreateFlags> for RawVkPipelineCoverageModulationStateCreateFlags {
    fn vk_to_wrapped(src: &RawVkPipelineCoverageModulationStateCreateFlags) -> VkPipelineCoverageModulationStateCreateFlags {
        VkPipelineCoverageModulationStateCreateFlags {
            
        }
    }
}

impl Default for VkPipelineCoverageModulationStateCreateFlags {
    fn default() -> VkPipelineCoverageModulationStateCreateFlags {
        VkPipelineCoverageModulationStateCreateFlags {
            
        }
    }
}

impl VkPipelineCoverageModulationStateCreateFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkPipelineCoverageModulationStateCreateFlags {
            
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkPipelineCoverageModulationStateCreateFlags {
            
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkPipelineCoverageModulationStateCreateFlags {
            
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkPipelineCoverageModulationStateCreateFlags {
    ( $( $x:ident ),* ) => {
        VkPipelineCoverageModulationStateCreateFlags {
            $($x: true,)*
            ..VkPipelineCoverageModulationStateCreateFlags::none()
        }
    }
}