// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkPipelineDiscardRectangleStateCreateFlags = u32;

#[derive(Debug, Copy, Clone)]
pub struct VkPipelineDiscardRectangleStateCreateFlags {
    
}

impl VkRawType<VkPipelineDiscardRectangleStateCreateFlags> for RawVkPipelineDiscardRectangleStateCreateFlags {
    
    fn vk_to_wrapped(src: &RawVkPipelineDiscardRectangleStateCreateFlags) -> VkPipelineDiscardRectangleStateCreateFlags {
        VkPipelineDiscardRectangleStateCreateFlags {
            
        }
    }
}

impl VkWrappedType<RawVkPipelineDiscardRectangleStateCreateFlags> for VkPipelineDiscardRectangleStateCreateFlags {
    
    fn vk_to_raw(src: &VkPipelineDiscardRectangleStateCreateFlags, dst: &mut RawVkPipelineDiscardRectangleStateCreateFlags) {
        *dst = 0;
    }
    
    fn vk_default() -> VkPipelineDiscardRectangleStateCreateFlags {
        VkPipelineDiscardRectangleStateCreateFlags {
            
        }
    }
}