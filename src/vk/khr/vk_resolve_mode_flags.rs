// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkResolveModeFlags = u32;

#[derive(Debug, Clone, Copy)]
pub struct VkResolveModeFlags {
    pub sample_zero: bool,
    pub average: bool,
    pub min: bool,
    pub max: bool,
}

impl VkRawType<VkResolveModeFlags> for RawVkResolveModeFlags {
    fn vk_to_wrapped(src: &RawVkResolveModeFlags) -> VkResolveModeFlags {
        VkResolveModeFlags {
            sample_zero: (src & 0x00000001) != 0,
            average: (src & 0x00000002) != 0,
            min: (src & 0x00000004) != 0,
            max: (src & 0x00000008) != 0,
        }
    }
}

impl VkWrappedType<RawVkResolveModeFlags> for VkResolveModeFlags {
    fn vk_to_raw(src: &VkResolveModeFlags, dst: &mut RawVkResolveModeFlags) {
        *dst = 0;
        if src.sample_zero { *dst |= 0x00000001; }
        if src.average { *dst |= 0x00000002; }
        if src.min { *dst |= 0x00000004; }
        if src.max { *dst |= 0x00000008; }
    }
}

impl Default for VkResolveModeFlags {
    fn default() -> VkResolveModeFlags {
        VkResolveModeFlags {
            sample_zero: false,
            average: false,
            min: false,
            max: false,
        }
    }
}

impl VkResolveModeFlags {
    
    pub fn none() -> VkResolveModeFlags {
        VkResolveModeFlags {
            sample_zero: false,
            average: false,
            min: false,
            max: false,
        }
    }
    
    pub fn all() -> VkResolveModeFlags {
        VkResolveModeFlags {
            sample_zero: true,
            average: true,
            min: true,
            max: true,
        }
    }
}

impl VkResolveModeFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.sample_zero { 0x00000001 } else { 0 }
        + if self.average { 0x00000002 } else { 0 }
        + if self.min { 0x00000004 } else { 0 }
        + if self.max { 0x00000008 } else { 0 }
    }
}