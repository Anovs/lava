// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkObjectEntryType = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkObjectEntryType {
    DescriptorSet = 0,
    Pipeline = 1,
    IndexBuffer = 2,
    VertexBuffer = 3,
    PushConstant = 4,
}

impl VkRawType<VkObjectEntryType> for RawVkObjectEntryType {
    
    fn vk_to_wrapped(src: &RawVkObjectEntryType) -> VkObjectEntryType {
        unsafe {
            *((src as *const i32) as *const VkObjectEntryType)
        }
    }
}

impl VkWrappedType<RawVkObjectEntryType> for VkObjectEntryType {
    
    fn vk_to_raw(src: &VkObjectEntryType, dst: &mut RawVkObjectEntryType) {
        *dst = *src as i32
    }
    
    fn vk_default() -> VkObjectEntryType {
        VkObjectEntryType::DescriptorSet
    }
}