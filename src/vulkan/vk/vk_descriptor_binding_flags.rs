// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkDescriptorBindingFlags](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorBindingFlags.html).
///
/// Use the macro `VkDescriptorBindingFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkDescriptorBindingFlags!(update_after_bind, update_unused_while_pending)
/// ```
/// ```
/// VkDescriptorBindingFlags {
///     update_after_bind: true,
///     update_unused_while_pending: true,
///     ..VkDescriptorBindingFlags::none()
/// }
/// ```
#[derive(Debug, Clone)]
pub struct VkDescriptorBindingFlags {
    pub update_after_bind: bool,
    pub update_unused_while_pending: bool,
    pub partially_bound: bool,
    pub variable_descriptor_count: bool,
}

#[doc(hidden)]
pub type RawVkDescriptorBindingFlags = u32;

impl VkWrappedType<RawVkDescriptorBindingFlags> for VkDescriptorBindingFlags {
    fn vk_to_raw(src: &VkDescriptorBindingFlags, dst: &mut RawVkDescriptorBindingFlags) {
        *dst = 0;
        if src.update_after_bind { *dst |= 0x00000001; }
        if src.update_unused_while_pending { *dst |= 0x00000002; }
        if src.partially_bound { *dst |= 0x00000004; }
        if src.variable_descriptor_count { *dst |= 0x00000008; }
    }
}

impl VkRawType<VkDescriptorBindingFlags> for RawVkDescriptorBindingFlags {
    fn vk_to_wrapped(src: &RawVkDescriptorBindingFlags) -> VkDescriptorBindingFlags {
        VkDescriptorBindingFlags {
            update_after_bind: (src & 0x00000001) != 0,
            update_unused_while_pending: (src & 0x00000002) != 0,
            partially_bound: (src & 0x00000004) != 0,
            variable_descriptor_count: (src & 0x00000008) != 0,
        }
    }
}

impl Default for VkDescriptorBindingFlags {
    fn default() -> VkDescriptorBindingFlags {
        VkDescriptorBindingFlags {
            update_after_bind: false,
            update_unused_while_pending: false,
            partially_bound: false,
            variable_descriptor_count: false,
        }
    }
}

impl VkDescriptorBindingFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkDescriptorBindingFlags {
            update_after_bind: false,
            update_unused_while_pending: false,
            partially_bound: false,
            variable_descriptor_count: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkDescriptorBindingFlags {
            update_after_bind: true,
            update_unused_while_pending: true,
            partially_bound: true,
            variable_descriptor_count: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.update_after_bind { 0x00000001 } else { 0 }
        + if self.update_unused_while_pending { 0x00000002 } else { 0 }
        + if self.partially_bound { 0x00000004 } else { 0 }
        + if self.variable_descriptor_count { 0x00000008 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkDescriptorBindingFlags {
            update_after_bind: value & 0x00000001 > 0,
            update_unused_while_pending: value & 0x00000002 > 0,
            partially_bound: value & 0x00000004 > 0,
            variable_descriptor_count: value & 0x00000008 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkDescriptorBindingFlags {
    ( $( $x:ident ),* ) => {
        VkDescriptorBindingFlags {
            $($x: true,)*
            ..VkDescriptorBindingFlags::none()
        }
    }
}