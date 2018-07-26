// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkRenderPassCreateFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkRenderPassCreateFlags {
    
}

impl VkRawType<VkRenderPassCreateFlags> for RawVkRenderPassCreateFlags {
    
    fn vk_to_wrapped(src: &RawVkRenderPassCreateFlags) -> VkRenderPassCreateFlags {
        VkRenderPassCreateFlags {
            
        }
    }
}

impl VkWrappedType<RawVkRenderPassCreateFlags> for VkRenderPassCreateFlags {
    
    fn vk_to_raw(src: &VkRenderPassCreateFlags, dst: &mut RawVkRenderPassCreateFlags) {
        *dst = 0;
    }
    
    fn vk_default() -> VkRenderPassCreateFlags {
        VkRenderPassCreateFlags {
            
        }
    }
}