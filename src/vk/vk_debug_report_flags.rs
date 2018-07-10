// Generated by `scripts/generate_vk.js`

use vk::*;
use std::os::raw::c_char;

pub type RawVkDebugReportFlags = u32;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct VkDebugReportFlags {
    pub information: bool,
    pub warning: bool,
    pub performance_warning: bool,
    pub error: bool,
    pub debug: bool,
}

impl VkFlags for VkDebugReportFlags {
    
    fn none() -> Self {
        Self {
            information: false,
            warning: false,
            performance_warning: false,
            error: false,
            debug: false,
        }
    }
    
    fn all() -> Self {
        Self {
            information: true,
            warning: true,
            performance_warning: true,
            error: true,
            debug: true,
        }
    }
}

impl VkFrom<VkDebugReportFlags> for RawVkDebugReportFlags {
    
    fn vk_from(value: &VkDebugReportFlags) -> Self { {
            0
            + (if value.information { 0x00000001 } else { 0 })
            + (if value.warning { 0x00000002 } else { 0 })
            + (if value.performance_warning { 0x00000004 } else { 0 })
            + (if value.error { 0x00000008 } else { 0 })
            + (if value.debug { 0x00000010 } else { 0 })
        }
    }
}

impl VkFrom<RawVkDebugReportFlags> for VkDebugReportFlags {
    
    fn vk_from(value: &RawVkDebugReportFlags) -> Self {
        Self {
            information: (value & 0x00000001) != 0,
            warning: (value & 0x00000002) != 0,
            performance_warning: (value & 0x00000004) != 0,
            error: (value & 0x00000008) != 0,
            debug: (value & 0x00000010) != 0,
        }
    }
}