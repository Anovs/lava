// Generated by `scripts/generate_vk.js`

use utils::vk_type::*;

pub type RawVkImageType = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkImageType {
    _1d = 0,
    _2d = 1,
    _3d = 2,
}

impl VkRawType<VkImageType> for RawVkImageType {
    
    fn vk_to_wrapped(src: &RawVkImageType) -> VkImageType {
        unsafe {
            *((src as *const i32) as *const VkImageType)
        }
    }
}

impl VkWrappedType<RawVkImageType> for VkImageType {
    
    fn vk_to_raw(src: &VkImageType, dst: &mut RawVkImageType) {
        *dst = *src as i32
    }
    
    fn vk_default() -> VkImageType {
        VkImageType::_1d
    }
}