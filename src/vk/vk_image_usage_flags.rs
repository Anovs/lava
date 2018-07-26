// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkImageUsageFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkImageUsageFlags {
    transfer_src: bool,
    transfer_dst: bool,
    sampled: bool,
    storage: bool,
    color_attachment: bool,
    depth_stencil_attachment: bool,
    transient_attachment: bool,
    input_attachment: bool,
}

impl VkRawType<VkImageUsageFlags> for RawVkImageUsageFlags {
    
    fn vk_to_wrapped(src: &RawVkImageUsageFlags) -> VkImageUsageFlags {
        VkImageUsageFlags {
            transfer_src: (src & 0x00000001) != 0,
            transfer_dst: (src & 0x00000002) != 0,
            sampled: (src & 0x00000004) != 0,
            storage: (src & 0x00000008) != 0,
            color_attachment: (src & 0x00000010) != 0,
            depth_stencil_attachment: (src & 0x00000020) != 0,
            transient_attachment: (src & 0x00000040) != 0,
            input_attachment: (src & 0x00000080) != 0,
        }
    }
}

impl VkWrappedType<RawVkImageUsageFlags> for VkImageUsageFlags {
    
    fn vk_to_raw(src: &VkImageUsageFlags, dst: &mut RawVkImageUsageFlags) {
        *dst = 0;
        if src.transfer_src { *dst |= 0x00000001; }
        if src.transfer_dst { *dst |= 0x00000002; }
        if src.sampled { *dst |= 0x00000004; }
        if src.storage { *dst |= 0x00000008; }
        if src.color_attachment { *dst |= 0x00000010; }
        if src.depth_stencil_attachment { *dst |= 0x00000020; }
        if src.transient_attachment { *dst |= 0x00000040; }
        if src.input_attachment { *dst |= 0x00000080; }
    }
    
    fn vk_default() -> VkImageUsageFlags {
        VkImageUsageFlags {
            transfer_src: false,
            transfer_dst: false,
            sampled: false,
            storage: false,
            color_attachment: false,
            depth_stencil_attachment: false,
            transient_attachment: false,
            input_attachment: false,
        }
    }
}