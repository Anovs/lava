// Generated by `scripts/generate_vk.js`

use vk::*;
use std::os::raw::c_char;
use std::vec::Vec;
use std::ptr::null;
use libc::c_void;
use glfw::*;

pub type RawVkDebugReportCallbackEXT = RawVkHandle;

#[derive(Debug)]
pub struct VkDebugReportCallbackEXT {
    _handle: RawVkDebugReportCallbackEXT,
}

impl VkDebugReportCallbackEXT {
    
    pub fn handle(&self) -> RawVkDebugReportCallbackEXT {
        self._handle
    }
}

impl VkFrom<VkDebugReportCallbackEXT> for RawVkDebugReportCallbackEXT {
    
    fn vk_from(value: &VkDebugReportCallbackEXT) -> Self {
        value._handle
    }
}

impl VkFrom<RawVkDebugReportCallbackEXT> for VkDebugReportCallbackEXT {
    
    fn vk_from(value: &RawVkDebugReportCallbackEXT) -> Self {
        Self {
            _handle: *value,
        }
    }
}