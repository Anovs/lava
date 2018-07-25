// Generated by `scripts/generate_vk.js`

use utils::vk_type::VkType;

pub type RawVkImageTiling = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkImageTiling {
    Optimal = 0,
    Linear = 1,
}

impl VkType<RawVkImageTiling> for VkImageTiling {
    
    fn vk_to_raw(src: &VkImageTiling, dst: &mut RawVkImageTiling) {
        *dst = *src as i32
    }
    
    fn vk_from_raw(src: &RawVkImageTiling) -> VkImageTiling {
        unsafe {
            *((src as *const i32) as *const VkImageTiling)
        }
    }
    
    fn vk_default() -> VkImageTiling {
        VkImageTiling::Optimal
    }
}