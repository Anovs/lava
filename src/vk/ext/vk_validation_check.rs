// Generated by `scripts/generate_vk.js`

use utils::vk_type::VkType;

pub type RawVkValidationCheck = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkValidationCheck {
    All = 0,
    Shaders = 1,
}

impl VkType<RawVkValidationCheck> for VkValidationCheck {
    
    fn vk_to_raw(src: &VkValidationCheck, dst: &mut RawVkValidationCheck) {
        *dst = *src as i32
    }
    
    fn vk_from_raw(src: &RawVkValidationCheck) -> VkValidationCheck {
        unsafe {
            *((src as *const i32) as *const VkValidationCheck)
        }
    }
    
    fn vk_default() -> VkValidationCheck {
        VkValidationCheck::All
    }
}