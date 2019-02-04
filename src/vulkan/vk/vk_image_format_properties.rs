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
use vulkan::vk::{VkExtent3D,RawVkExtent3D};
use vulkan::vk::{VkSampleCountFlags,RawVkSampleCountFlags};

/// Wrapper for [VkImageFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkImageFormatProperties.html).
#[derive(Debug, Clone)]
pub struct VkImageFormatProperties {
    pub max_extent: VkExtent3D,
    pub max_mip_levels: usize,
    pub max_array_layers: usize,
    pub sample_counts: VkSampleCountFlags,
    pub max_resource_size: usize,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkImageFormatProperties {
    pub max_extent: RawVkExtent3D,
    pub max_mip_levels: u32,
    pub max_array_layers: u32,
    pub sample_counts: RawVkSampleCountFlags,
    pub max_resource_size: u64,
}

impl VkWrappedType<RawVkImageFormatProperties> for VkImageFormatProperties {
    fn vk_to_raw(src: &VkImageFormatProperties, dst: &mut RawVkImageFormatProperties) {
        dst.max_extent = vk_to_raw_value(&src.max_extent);
        dst.max_mip_levels = vk_to_raw_value(&src.max_mip_levels);
        dst.max_array_layers = vk_to_raw_value(&src.max_array_layers);
        dst.sample_counts = vk_to_raw_value(&src.sample_counts);
        dst.max_resource_size = vk_to_raw_value(&src.max_resource_size);
    }
}

impl VkRawType<VkImageFormatProperties> for RawVkImageFormatProperties {
    fn vk_to_wrapped(src: &RawVkImageFormatProperties) -> VkImageFormatProperties {
        VkImageFormatProperties {
            max_extent: RawVkExtent3D::vk_to_wrapped(&src.max_extent),
            max_mip_levels: u32::vk_to_wrapped(&src.max_mip_levels),
            max_array_layers: u32::vk_to_wrapped(&src.max_array_layers),
            sample_counts: RawVkSampleCountFlags::vk_to_wrapped(&src.sample_counts),
            max_resource_size: u64::vk_to_wrapped(&src.max_resource_size),
        }
    }
}

impl Default for VkImageFormatProperties {
    fn default() -> VkImageFormatProperties {
        VkImageFormatProperties {
            max_extent: VkExtent3D::default(),
            max_mip_levels: 0,
            max_array_layers: 0,
            sample_counts: VkSampleCountFlags::default(),
            max_resource_size: 0,
        }
    }
}

impl VkSetup for VkImageFormatProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.max_extent, fn_table, instance, device);
    }
}

impl VkFree for RawVkImageFormatProperties {
    fn vk_free(&mut self) {
        RawVkExtent3D::vk_free(&mut self.max_extent);
    }
}