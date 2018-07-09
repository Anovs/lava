// Generated by `scripts/generate_vk.js`

use vk::*;
use std::os::raw::c_char;

pub type RawVkImageAspectFlags = u32;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct VkImageAspectFlags {
    pub color: bool,
    pub depth: bool,
    pub stencil: bool,
    pub metadata: bool,
    pub plane_0: bool,
    pub plane_1: bool,
    pub plane_2: bool,
}

impl VkFlags for VkImageAspectFlags {
    
    fn none() -> Self {
        Self {
            color: false,
            depth: false,
            stencil: false,
            metadata: false,
            plane_0: false,
            plane_1: false,
            plane_2: false,
        }
    }
    
    fn all() -> Self {
        Self {
            color: true,
            depth: true,
            stencil: true,
            metadata: true,
            plane_0: true,
            plane_1: true,
            plane_2: true,
        }
    }
}

impl VkFrom<VkImageAspectFlags> for RawVkImageAspectFlags {
    
    fn vk_from(value: &VkImageAspectFlags) -> Self { {
            0
            + (if value.color { 0x00000001 } else { 0 })
            + (if value.depth { 0x00000002 } else { 0 })
            + (if value.stencil { 0x00000004 } else { 0 })
            + (if value.metadata { 0x00000008 } else { 0 })
            + (if value.plane_0 { 0x00000010 } else { 0 })
            + (if value.plane_1 { 0x00000020 } else { 0 })
            + (if value.plane_2 { 0x00000040 } else { 0 })
        }
    }
}

impl VkFrom<RawVkImageAspectFlags> for VkImageAspectFlags {
    
    fn vk_from(value: &RawVkImageAspectFlags) -> Self {
        Self {
            color: (value & 0x00000001) != 0,
            depth: (value & 0x00000002) != 0,
            stencil: (value & 0x00000004) != 0,
            metadata: (value & 0x00000008) != 0,
            plane_0: (value & 0x00000010) != 0,
            plane_1: (value & 0x00000020) != 0,
            plane_2: (value & 0x00000040) != 0,
        }
    }
}