// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkShaderStageFlags = u32;

#[derive(Debug, Clone, Copy)]
pub struct VkShaderStageFlags {
    pub vertex: bool,
    pub tessellation_control: bool,
    pub tessellation_evaluation: bool,
    pub geometry: bool,
    pub fragment: bool,
    pub compute: bool,
    pub all_graphics: bool,
}

impl VkRawType<VkShaderStageFlags> for RawVkShaderStageFlags {
    fn vk_to_wrapped(src: &RawVkShaderStageFlags) -> VkShaderStageFlags {
        VkShaderStageFlags {
            vertex: (src & 0x00000001) != 0,
            tessellation_control: (src & 0x00000002) != 0,
            tessellation_evaluation: (src & 0x00000004) != 0,
            geometry: (src & 0x00000008) != 0,
            fragment: (src & 0x00000010) != 0,
            compute: (src & 0x00000020) != 0,
            all_graphics: (src & 0x0000001F) != 0,
        }
    }
}

impl VkWrappedType<RawVkShaderStageFlags> for VkShaderStageFlags {
    fn vk_to_raw(src: &VkShaderStageFlags, dst: &mut RawVkShaderStageFlags) {
        *dst = 0;
        if src.vertex { *dst |= 0x00000001; }
        if src.tessellation_control { *dst |= 0x00000002; }
        if src.tessellation_evaluation { *dst |= 0x00000004; }
        if src.geometry { *dst |= 0x00000008; }
        if src.fragment { *dst |= 0x00000010; }
        if src.compute { *dst |= 0x00000020; }
        if src.all_graphics { *dst |= 0x0000001F; }
    }
}

impl Default for VkShaderStageFlags {
    fn default() -> VkShaderStageFlags {
        VkShaderStageFlags {
            vertex: false,
            tessellation_control: false,
            tessellation_evaluation: false,
            geometry: false,
            fragment: false,
            compute: false,
            all_graphics: false,
        }
    }
}

impl VkShaderStageFlags {
    
    pub fn none() -> VkShaderStageFlags {
        VkShaderStageFlags {
            vertex: false,
            tessellation_control: false,
            tessellation_evaluation: false,
            geometry: false,
            fragment: false,
            compute: false,
            all_graphics: false,
        }
    }
    
    pub fn all() -> VkShaderStageFlags {
        VkShaderStageFlags {
            vertex: true,
            tessellation_control: true,
            tessellation_evaluation: true,
            geometry: true,
            fragment: true,
            compute: true,
            all_graphics: true,
        }
    }
}