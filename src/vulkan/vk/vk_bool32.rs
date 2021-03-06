// Copied from `scripts/static/`

use utils::vk_traits::*;

impl VkRawType<bool> for u32 {
    fn vk_to_wrapped(value: &u32) -> bool {
        *value != 0
    }
}

impl VkWrappedType<u32> for bool {
    fn vk_to_raw(value: &bool, dst: &mut u32) {
        *dst = if *value { 1 } else { 0 }
    }
}