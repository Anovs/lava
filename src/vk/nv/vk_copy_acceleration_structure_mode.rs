// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkCopyAccelerationStructureMode = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkCopyAccelerationStructureMode {
    Clone = 0,
    Compact = 1,
}

impl VkRawType<VkCopyAccelerationStructureMode> for RawVkCopyAccelerationStructureMode {
    fn vk_to_wrapped(src: &RawVkCopyAccelerationStructureMode) -> VkCopyAccelerationStructureMode {
        unsafe {
            *((src as *const i32) as *const VkCopyAccelerationStructureMode)
        }
    }
}

impl VkWrappedType<RawVkCopyAccelerationStructureMode> for VkCopyAccelerationStructureMode {
    fn vk_to_raw(src: &VkCopyAccelerationStructureMode, dst: &mut RawVkCopyAccelerationStructureMode) {
        *dst = *src as i32
    }
}

impl Default for VkCopyAccelerationStructureMode {
    fn default() -> VkCopyAccelerationStructureMode {
        VkCopyAccelerationStructureMode::Clone
    }
}