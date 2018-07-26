// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkSubpassDescriptionFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkSubpassDescriptionFlags {
    per_view_attributes_nvx: bool,
    per_view_position_x_only_nvx: bool,
}

impl VkRawType<VkSubpassDescriptionFlags> for RawVkSubpassDescriptionFlags {
    
    fn vk_to_wrapped(src: &RawVkSubpassDescriptionFlags) -> VkSubpassDescriptionFlags {
        VkSubpassDescriptionFlags {
            per_view_attributes_nvx: (src & 0x00000001) != 0,
            per_view_position_x_only_nvx: (src & 0x00000002) != 0,
        }
    }
}

impl VkWrappedType<RawVkSubpassDescriptionFlags> for VkSubpassDescriptionFlags {
    
    fn vk_to_raw(src: &VkSubpassDescriptionFlags, dst: &mut RawVkSubpassDescriptionFlags) {
        *dst = 0;
        if src.per_view_attributes_nvx { *dst |= 0x00000001; }
        if src.per_view_position_x_only_nvx { *dst |= 0x00000002; }
    }
    
    fn vk_default() -> VkSubpassDescriptionFlags {
        VkSubpassDescriptionFlags {
            per_view_attributes_nvx: false,
            per_view_position_x_only_nvx: false,
        }
    }
}