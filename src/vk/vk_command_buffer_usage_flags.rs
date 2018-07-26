// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkCommandBufferUsageFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkCommandBufferUsageFlags {
    one_time_submit: bool,
    render_pass_continue: bool,
    simultaneous_use: bool,
}

impl VkRawType<VkCommandBufferUsageFlags> for RawVkCommandBufferUsageFlags {
    
    fn vk_to_wrapped(src: &RawVkCommandBufferUsageFlags) -> VkCommandBufferUsageFlags {
        VkCommandBufferUsageFlags {
            one_time_submit: (src & 0x00000001) != 0,
            render_pass_continue: (src & 0x00000002) != 0,
            simultaneous_use: (src & 0x00000004) != 0,
        }
    }
}

impl VkWrappedType<RawVkCommandBufferUsageFlags> for VkCommandBufferUsageFlags {
    
    fn vk_to_raw(src: &VkCommandBufferUsageFlags, dst: &mut RawVkCommandBufferUsageFlags) {
        *dst = 0;
        if src.one_time_submit { *dst |= 0x00000001; }
        if src.render_pass_continue { *dst |= 0x00000002; }
        if src.simultaneous_use { *dst |= 0x00000004; }
    }
    
    fn vk_default() -> VkCommandBufferUsageFlags {
        VkCommandBufferUsageFlags {
            one_time_submit: false,
            render_pass_continue: false,
            simultaneous_use: false,
        }
    }
}