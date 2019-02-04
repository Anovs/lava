// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkImageAspectFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkImageAspectFlagBits.html)
///
/// Use the macro `VkImageAspectFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkImageAspectFlags!(color, depth)
/// ```
/// ```
/// VkImageAspectFlags {
///     color: true,
///     depth: true,
///     ..VkImageAspectFlags::none()
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct VkImageAspectFlags {
    pub color: bool,
    pub depth: bool,
    pub stencil: bool,
    pub metadata: bool,
    pub plane_0: bool,
    pub plane_1: bool,
    pub plane_2: bool,
    pub memory_plane_0_ext: bool,
    pub memory_plane_1_ext: bool,
    pub memory_plane_2_ext: bool,
    pub memory_plane_3_ext: bool,
}

#[doc(hidden)]
pub type RawVkImageAspectFlags = u32;

impl VkWrappedType<RawVkImageAspectFlags> for VkImageAspectFlags {
    fn vk_to_raw(src: &VkImageAspectFlags, dst: &mut RawVkImageAspectFlags) {
        *dst = 0;
        if src.color { *dst |= 0x00000001; }
        if src.depth { *dst |= 0x00000002; }
        if src.stencil { *dst |= 0x00000004; }
        if src.metadata { *dst |= 0x00000008; }
        if src.plane_0 { *dst |= 0x00000010; }
        if src.plane_1 { *dst |= 0x00000020; }
        if src.plane_2 { *dst |= 0x00000040; }
        if src.memory_plane_0_ext { *dst |= 0x00000080; }
        if src.memory_plane_1_ext { *dst |= 0x00000100; }
        if src.memory_plane_2_ext { *dst |= 0x00000200; }
        if src.memory_plane_3_ext { *dst |= 0x00000400; }
    }
}

impl VkRawType<VkImageAspectFlags> for RawVkImageAspectFlags {
    fn vk_to_wrapped(src: &RawVkImageAspectFlags) -> VkImageAspectFlags {
        VkImageAspectFlags {
            color: (src & 0x00000001) != 0,
            depth: (src & 0x00000002) != 0,
            stencil: (src & 0x00000004) != 0,
            metadata: (src & 0x00000008) != 0,
            plane_0: (src & 0x00000010) != 0,
            plane_1: (src & 0x00000020) != 0,
            plane_2: (src & 0x00000040) != 0,
            memory_plane_0_ext: (src & 0x00000080) != 0,
            memory_plane_1_ext: (src & 0x00000100) != 0,
            memory_plane_2_ext: (src & 0x00000200) != 0,
            memory_plane_3_ext: (src & 0x00000400) != 0,
        }
    }
}

impl Default for VkImageAspectFlags {
    fn default() -> VkImageAspectFlags {
        VkImageAspectFlags {
            color: false,
            depth: false,
            stencil: false,
            metadata: false,
            plane_0: false,
            plane_1: false,
            plane_2: false,
            memory_plane_0_ext: false,
            memory_plane_1_ext: false,
            memory_plane_2_ext: false,
            memory_plane_3_ext: false,
        }
    }
}

impl VkImageAspectFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkImageAspectFlags {
            color: false,
            depth: false,
            stencil: false,
            metadata: false,
            plane_0: false,
            plane_1: false,
            plane_2: false,
            memory_plane_0_ext: false,
            memory_plane_1_ext: false,
            memory_plane_2_ext: false,
            memory_plane_3_ext: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkImageAspectFlags {
            color: true,
            depth: true,
            stencil: true,
            metadata: true,
            plane_0: true,
            plane_1: true,
            plane_2: true,
            memory_plane_0_ext: true,
            memory_plane_1_ext: true,
            memory_plane_2_ext: true,
            memory_plane_3_ext: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.color { 0x00000001 } else { 0 }
        + if self.depth { 0x00000002 } else { 0 }
        + if self.stencil { 0x00000004 } else { 0 }
        + if self.metadata { 0x00000008 } else { 0 }
        + if self.plane_0 { 0x00000010 } else { 0 }
        + if self.plane_1 { 0x00000020 } else { 0 }
        + if self.plane_2 { 0x00000040 } else { 0 }
        + if self.memory_plane_0_ext { 0x00000080 } else { 0 }
        + if self.memory_plane_1_ext { 0x00000100 } else { 0 }
        + if self.memory_plane_2_ext { 0x00000200 } else { 0 }
        + if self.memory_plane_3_ext { 0x00000400 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkImageAspectFlags {
            color: value & 0x00000001 > 0,
            depth: value & 0x00000002 > 0,
            stencil: value & 0x00000004 > 0,
            metadata: value & 0x00000008 > 0,
            plane_0: value & 0x00000010 > 0,
            plane_1: value & 0x00000020 > 0,
            plane_2: value & 0x00000040 > 0,
            memory_plane_0_ext: value & 0x00000080 > 0,
            memory_plane_1_ext: value & 0x00000100 > 0,
            memory_plane_2_ext: value & 0x00000200 > 0,
            memory_plane_3_ext: value & 0x00000400 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkImageAspectFlags {
    ( $( $x:ident ),* ) => {
        VkImageAspectFlags {
            $($x: true,)*
            ..VkImageAspectFlags::none()
        }
    }
}