// Generated by `scripts/generate_vk.js`

use std::os::raw::c_char;
use std::ops::Deref;
use std::ptr;
use std::cmp;
use std::mem;
use utils::c_bindings::*;
use utils::vk_convert::*;
use utils::vk_null::*;
use utils::vk_ptr::*;
use utils::vk_traits::*;
use vk::vk_instance_function_table::*;
use vk::vk_instance::*;
use vk::vk_device::*;
use vk::khr::vk_display_mode::*;
use vk::khr::vk_display_mode_parameters::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawVkDisplayModeProperties {
    pub display_mode: RawVkDisplayMode,
    pub parameters: RawVkDisplayModeParameters,
}

#[derive(Debug, Clone)]
pub struct VkDisplayModeProperties {
    pub display_mode: VkDisplayMode,
    pub parameters: VkDisplayModeParameters,
}

impl VkRawType<VkDisplayModeProperties> for RawVkDisplayModeProperties {
    fn vk_to_wrapped(src: &RawVkDisplayModeProperties) -> VkDisplayModeProperties {
        VkDisplayModeProperties {
            display_mode: RawVkDisplayMode::vk_to_wrapped(&src.display_mode),
            parameters: RawVkDisplayModeParameters::vk_to_wrapped(&src.parameters),
        }
    }
}

impl VkSetup for VkDisplayModeProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.display_mode, fn_table, instance, device);
        VkSetup::vk_setup(&mut self.parameters, fn_table, instance, device);
    }
}

impl VkFree for RawVkDisplayModeProperties {
    fn vk_free(&mut self) {
        RawVkDisplayModeParameters::vk_free(&mut self.parameters);
    }
}