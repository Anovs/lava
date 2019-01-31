// Generated by `scripts/generate.js`

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
use vulkan::vk::*;
use vulkan::khr::{VkDisplay,RawVkDisplay};
use vulkan::vk::{VkExtent2D,RawVkExtent2D};
use vulkan::khr::{VkSurfaceTransformFlags,RawVkSurfaceTransformFlags};

/// Wrapper for [VkDisplayPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDisplayPropertiesKHR.html)
#[derive(Debug, Clone)]
pub struct VkDisplayProperties {
    pub display: VkDisplay,
    pub display_name: Option<String>,
    pub physical_dimensions: VkExtent2D,
    pub physical_resolution: VkExtent2D,
    pub supported_transforms: VkSurfaceTransformFlags,
    pub plane_reorder_possible: bool,
    pub persistent_content: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDisplayProperties {
    pub display: RawVkDisplay,
    pub display_name: *const c_char,
    pub physical_dimensions: RawVkExtent2D,
    pub physical_resolution: RawVkExtent2D,
    pub supported_transforms: RawVkSurfaceTransformFlags,
    pub plane_reorder_possible: u32,
    pub persistent_content: u32,
}

impl VkRawType<VkDisplayProperties> for RawVkDisplayProperties {
    fn vk_to_wrapped(src: &RawVkDisplayProperties) -> VkDisplayProperties {
        VkDisplayProperties {
            display: RawVkDisplay::vk_to_wrapped(&src.display),
            display_name: new_string_checked(src.display_name),
            physical_dimensions: RawVkExtent2D::vk_to_wrapped(&src.physical_dimensions),
            physical_resolution: RawVkExtent2D::vk_to_wrapped(&src.physical_resolution),
            supported_transforms: RawVkSurfaceTransformFlags::vk_to_wrapped(&src.supported_transforms),
            plane_reorder_possible: u32::vk_to_wrapped(&src.plane_reorder_possible),
            persistent_content: u32::vk_to_wrapped(&src.persistent_content),
        }
    }
}

impl VkSetup for VkDisplayProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.display, fn_table, instance, device);
        VkSetup::vk_setup(&mut self.physical_dimensions, fn_table, instance, device);
        VkSetup::vk_setup(&mut self.physical_resolution, fn_table, instance, device);
    }
}

impl VkFree for RawVkDisplayProperties {
    fn vk_free(&mut self) {
        RawVkExtent2D::vk_free(&mut self.physical_dimensions);
        RawVkExtent2D::vk_free(&mut self.physical_resolution);
    }
}