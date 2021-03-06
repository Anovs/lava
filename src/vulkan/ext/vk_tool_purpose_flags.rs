// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkToolPurposeFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkToolPurposeFlagsEXT.html).
///
/// Use the macro `VkToolPurposeFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkToolPurposeFlags!(validation, profiling)
/// ```
/// ```
/// VkToolPurposeFlags {
///     validation: true,
///     profiling: true,
///     ..VkToolPurposeFlags::none()
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkToolPurposeFlags {
    pub validation: bool,
    pub profiling: bool,
    pub tracing: bool,
    pub additional_features: bool,
    pub modifying_features: bool,
    pub debug_reporting: bool,
    pub debug_markers: bool,
}

#[doc(hidden)]
pub type RawVkToolPurposeFlags = u32;

impl VkWrappedType<RawVkToolPurposeFlags> for VkToolPurposeFlags {
    fn vk_to_raw(src: &VkToolPurposeFlags, dst: &mut RawVkToolPurposeFlags) {
        *dst = 0;
        if src.validation { *dst |= 0x00000001; }
        if src.profiling { *dst |= 0x00000002; }
        if src.tracing { *dst |= 0x00000004; }
        if src.additional_features { *dst |= 0x00000008; }
        if src.modifying_features { *dst |= 0x00000010; }
        if src.debug_reporting { *dst |= 0x00000020; }
        if src.debug_markers { *dst |= 0x00000040; }
    }
}

impl VkRawType<VkToolPurposeFlags> for RawVkToolPurposeFlags {
    fn vk_to_wrapped(src: &RawVkToolPurposeFlags) -> VkToolPurposeFlags {
        VkToolPurposeFlags {
            validation: (src & 0x00000001) != 0,
            profiling: (src & 0x00000002) != 0,
            tracing: (src & 0x00000004) != 0,
            additional_features: (src & 0x00000008) != 0,
            modifying_features: (src & 0x00000010) != 0,
            debug_reporting: (src & 0x00000020) != 0,
            debug_markers: (src & 0x00000040) != 0,
        }
    }
}

impl Default for VkToolPurposeFlags {
    fn default() -> VkToolPurposeFlags {
        VkToolPurposeFlags {
            validation: false,
            profiling: false,
            tracing: false,
            additional_features: false,
            modifying_features: false,
            debug_reporting: false,
            debug_markers: false,
        }
    }
}

impl VkToolPurposeFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkToolPurposeFlags {
            validation: false,
            profiling: false,
            tracing: false,
            additional_features: false,
            modifying_features: false,
            debug_reporting: false,
            debug_markers: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkToolPurposeFlags {
            validation: true,
            profiling: true,
            tracing: true,
            additional_features: true,
            modifying_features: true,
            debug_reporting: true,
            debug_markers: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.validation { 0x00000001 } else { 0 }
        + if self.profiling { 0x00000002 } else { 0 }
        + if self.tracing { 0x00000004 } else { 0 }
        + if self.additional_features { 0x00000008 } else { 0 }
        + if self.modifying_features { 0x00000010 } else { 0 }
        + if self.debug_reporting { 0x00000020 } else { 0 }
        + if self.debug_markers { 0x00000040 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkToolPurposeFlags {
            validation: value & 0x00000001 > 0,
            profiling: value & 0x00000002 > 0,
            tracing: value & 0x00000004 > 0,
            additional_features: value & 0x00000008 > 0,
            modifying_features: value & 0x00000010 > 0,
            debug_reporting: value & 0x00000020 > 0,
            debug_markers: value & 0x00000040 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkToolPurposeFlags {
    ( $( $x:ident ),* ) => {
        VkToolPurposeFlags {
            $($x: true,)*
            ..VkToolPurposeFlags::none()
        }
    }
}