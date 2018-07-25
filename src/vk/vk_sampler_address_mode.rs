// Generated by `scripts/generate_vk.js`

use utils::vk_type::VkType;

pub type RawVkSamplerAddressMode = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkSamplerAddressMode {
    Repeat = 0,
    MirroredRepeat = 1,
    ClampToEdge = 2,
    ClampToBorder = 3,
    MirrorClampToEdge = 4,
}

impl VkType<RawVkSamplerAddressMode> for VkSamplerAddressMode {
    
    fn vk_to_raw(src: &VkSamplerAddressMode, dst: &mut RawVkSamplerAddressMode) {
        *dst = *src as i32
    }
    
    fn vk_from_raw(src: &RawVkSamplerAddressMode) -> VkSamplerAddressMode {
        unsafe {
            *((src as *const i32) as *const VkSamplerAddressMode)
        }
    }
    
    fn vk_default() -> VkSamplerAddressMode {
        VkSamplerAddressMode::Repeat
    }
}