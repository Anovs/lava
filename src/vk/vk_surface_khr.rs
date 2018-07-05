// Generated by `scripts/generate_vk.js`

use vk::*;
use std::os::raw::c_char;
use std::vec::Vec;
use std::ptr::null;
use libc::c_void;

pub type RawVkSurfaceKHR = RawVkHandle;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct VkSurfaceKHR {
    _handle: RawVkSurfaceKHR,
}

impl VkSurfaceKHR {
    
    pub fn handle(&self) -> RawVkSurfaceKHR {
        self._handle
    }
}

impl VkFrom<VkSurfaceKHR> for RawVkSurfaceKHR {
    
    fn vk_from(value: &VkSurfaceKHR) -> Self {
        value._handle
    }
}

impl VkFrom<RawVkSurfaceKHR> for VkSurfaceKHR {
    
    fn vk_from(value: &RawVkSurfaceKHR) -> Self {
        Self {
            _handle: *value,
        }
    }
}

extern {
}