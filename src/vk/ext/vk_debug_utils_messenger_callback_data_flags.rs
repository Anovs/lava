// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkDebugUtilsMessengerCallbackDataFlags = u32;

#[derive(Debug, Clone, Copy)]
pub struct VkDebugUtilsMessengerCallbackDataFlags {
    
}

impl VkRawType<VkDebugUtilsMessengerCallbackDataFlags> for RawVkDebugUtilsMessengerCallbackDataFlags {
    fn vk_to_wrapped(src: &RawVkDebugUtilsMessengerCallbackDataFlags) -> VkDebugUtilsMessengerCallbackDataFlags {
        VkDebugUtilsMessengerCallbackDataFlags {
            
        }
    }
}

impl VkWrappedType<RawVkDebugUtilsMessengerCallbackDataFlags> for VkDebugUtilsMessengerCallbackDataFlags {
    fn vk_to_raw(src: &VkDebugUtilsMessengerCallbackDataFlags, dst: &mut RawVkDebugUtilsMessengerCallbackDataFlags) {
        *dst = 0;
    }
}

impl Default for VkDebugUtilsMessengerCallbackDataFlags {
    fn default() -> VkDebugUtilsMessengerCallbackDataFlags {
        VkDebugUtilsMessengerCallbackDataFlags {
            
        }
    }
}

impl VkDebugUtilsMessengerCallbackDataFlags {
    
    pub fn none() -> VkDebugUtilsMessengerCallbackDataFlags {
        VkDebugUtilsMessengerCallbackDataFlags {
            
        }
    }
    
    pub fn all() -> VkDebugUtilsMessengerCallbackDataFlags {
        VkDebugUtilsMessengerCallbackDataFlags {
            
        }
    }
}