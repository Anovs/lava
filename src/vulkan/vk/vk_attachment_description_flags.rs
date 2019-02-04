// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkAttachmentDescriptionFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkAttachmentDescriptionFlagBits.html).
///
/// Use the macro `VkAttachmentDescriptionFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkAttachmentDescriptionFlags!(may_alias)
/// ```
/// ```
/// VkAttachmentDescriptionFlags {
///     may_alias: true,
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct VkAttachmentDescriptionFlags {
    pub may_alias: bool,
}

#[doc(hidden)]
pub type RawVkAttachmentDescriptionFlags = u32;

impl VkWrappedType<RawVkAttachmentDescriptionFlags> for VkAttachmentDescriptionFlags {
    fn vk_to_raw(src: &VkAttachmentDescriptionFlags, dst: &mut RawVkAttachmentDescriptionFlags) {
        *dst = 0;
        if src.may_alias { *dst |= 0x00000001; }
    }
}

impl VkRawType<VkAttachmentDescriptionFlags> for RawVkAttachmentDescriptionFlags {
    fn vk_to_wrapped(src: &RawVkAttachmentDescriptionFlags) -> VkAttachmentDescriptionFlags {
        VkAttachmentDescriptionFlags {
            may_alias: (src & 0x00000001) != 0,
        }
    }
}

impl Default for VkAttachmentDescriptionFlags {
    fn default() -> VkAttachmentDescriptionFlags {
        VkAttachmentDescriptionFlags {
            may_alias: false,
        }
    }
}

impl VkAttachmentDescriptionFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkAttachmentDescriptionFlags {
            may_alias: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkAttachmentDescriptionFlags {
            may_alias: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.may_alias { 0x00000001 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkAttachmentDescriptionFlags {
            may_alias: value & 0x00000001 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkAttachmentDescriptionFlags {
    ( $( $x:ident ),* ) => {
        VkAttachmentDescriptionFlags {
            $($x: true,)*
            ..VkAttachmentDescriptionFlags::none()
        }
    }
}