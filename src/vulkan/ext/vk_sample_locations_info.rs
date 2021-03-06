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
use vulkan::vk::{VkStructureType,RawVkStructureType};
use vulkan::vk::{VkSampleCountFlags,RawVkSampleCountFlags};
use vulkan::vk::{VkExtent2D,RawVkExtent2D};
use vulkan::ext::{VkSampleLocation,RawVkSampleLocation};

/// Wrapper for [VkSampleLocationsInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSampleLocationsInfoEXT.html).
#[derive(Debug, Clone)]
pub struct VkSampleLocationsInfo {
    pub sample_locations_per_pixel: VkSampleCountFlags,
    pub sample_location_grid_size: VkExtent2D,
    pub sample_locations: Vec<VkSampleLocation>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkSampleLocationsInfo {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub sample_locations_per_pixel: RawVkSampleCountFlags,
    pub sample_location_grid_size: RawVkExtent2D,
    pub sample_locations_count: u32,
    pub sample_locations: *mut RawVkSampleLocation,
}

impl VkWrappedType<RawVkSampleLocationsInfo> for VkSampleLocationsInfo {
    fn vk_to_raw(src: &VkSampleLocationsInfo, dst: &mut RawVkSampleLocationsInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SampleLocationsInfoExt);
        dst.next = ptr::null_mut();
        dst.sample_locations_per_pixel = vk_to_raw_value(&src.sample_locations_per_pixel);
        dst.sample_location_grid_size = vk_to_raw_value(&src.sample_location_grid_size);
        dst.sample_locations_count = src.sample_locations.len() as u32;
        dst.sample_locations = new_ptr_vk_array(&src.sample_locations);
    }
}

impl VkRawType<VkSampleLocationsInfo> for RawVkSampleLocationsInfo {
    fn vk_to_wrapped(src: &RawVkSampleLocationsInfo) -> VkSampleLocationsInfo {
        VkSampleLocationsInfo {
            sample_locations_per_pixel: RawVkSampleCountFlags::vk_to_wrapped(&src.sample_locations_per_pixel),
            sample_location_grid_size: RawVkExtent2D::vk_to_wrapped(&src.sample_location_grid_size),
            sample_locations: new_vk_array(src.sample_locations_count, src.sample_locations),
        }
    }
}

impl Default for VkSampleLocationsInfo {
    fn default() -> VkSampleLocationsInfo {
        VkSampleLocationsInfo {
            sample_locations_per_pixel: Default::default(),
            sample_location_grid_size: Default::default(),
            sample_locations: Vec::new(),
        }
    }
}

impl VkSetup for VkSampleLocationsInfo {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        VkSetup::vk_setup(&mut self.sample_location_grid_size, fn_table);
    }
}

impl VkFree for RawVkSampleLocationsInfo {
    fn vk_free(&self) {
        free_vk_ptr_array(self.sample_locations_count as usize, self.sample_locations);
    }
}