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
use vulkan::khr::{VkDisplayPlaneAlphaFlags,RawVkDisplayPlaneAlphaFlags};
use vulkan::vk::{VkOffset2D,RawVkOffset2D};
use vulkan::vk::{VkExtent2D,RawVkExtent2D};

/// Wrapper for [VkDisplayPlaneCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDisplayPlaneCapabilitiesKHR.html).
#[derive(Debug, Clone)]
pub struct VkDisplayPlaneCapabilities {
    pub supported_alpha: VkDisplayPlaneAlphaFlags,
    pub min_src_position: VkOffset2D,
    pub max_src_position: VkOffset2D,
    pub min_src_extent: VkExtent2D,
    pub max_src_extent: VkExtent2D,
    pub min_dst_position: VkOffset2D,
    pub max_dst_position: VkOffset2D,
    pub min_dst_extent: VkExtent2D,
    pub max_dst_extent: VkExtent2D,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDisplayPlaneCapabilities {
    pub supported_alpha: RawVkDisplayPlaneAlphaFlags,
    pub min_src_position: RawVkOffset2D,
    pub max_src_position: RawVkOffset2D,
    pub min_src_extent: RawVkExtent2D,
    pub max_src_extent: RawVkExtent2D,
    pub min_dst_position: RawVkOffset2D,
    pub max_dst_position: RawVkOffset2D,
    pub min_dst_extent: RawVkExtent2D,
    pub max_dst_extent: RawVkExtent2D,
}

impl VkWrappedType<RawVkDisplayPlaneCapabilities> for VkDisplayPlaneCapabilities {
    fn vk_to_raw(src: &VkDisplayPlaneCapabilities, dst: &mut RawVkDisplayPlaneCapabilities) {
        dst.supported_alpha = vk_to_raw_value(&src.supported_alpha);
        dst.min_src_position = vk_to_raw_value(&src.min_src_position);
        dst.max_src_position = vk_to_raw_value(&src.max_src_position);
        dst.min_src_extent = vk_to_raw_value(&src.min_src_extent);
        dst.max_src_extent = vk_to_raw_value(&src.max_src_extent);
        dst.min_dst_position = vk_to_raw_value(&src.min_dst_position);
        dst.max_dst_position = vk_to_raw_value(&src.max_dst_position);
        dst.min_dst_extent = vk_to_raw_value(&src.min_dst_extent);
        dst.max_dst_extent = vk_to_raw_value(&src.max_dst_extent);
    }
}

impl VkRawType<VkDisplayPlaneCapabilities> for RawVkDisplayPlaneCapabilities {
    fn vk_to_wrapped(src: &RawVkDisplayPlaneCapabilities) -> VkDisplayPlaneCapabilities {
        VkDisplayPlaneCapabilities {
            supported_alpha: RawVkDisplayPlaneAlphaFlags::vk_to_wrapped(&src.supported_alpha),
            min_src_position: RawVkOffset2D::vk_to_wrapped(&src.min_src_position),
            max_src_position: RawVkOffset2D::vk_to_wrapped(&src.max_src_position),
            min_src_extent: RawVkExtent2D::vk_to_wrapped(&src.min_src_extent),
            max_src_extent: RawVkExtent2D::vk_to_wrapped(&src.max_src_extent),
            min_dst_position: RawVkOffset2D::vk_to_wrapped(&src.min_dst_position),
            max_dst_position: RawVkOffset2D::vk_to_wrapped(&src.max_dst_position),
            min_dst_extent: RawVkExtent2D::vk_to_wrapped(&src.min_dst_extent),
            max_dst_extent: RawVkExtent2D::vk_to_wrapped(&src.max_dst_extent),
        }
    }
}

impl Default for VkDisplayPlaneCapabilities {
    fn default() -> VkDisplayPlaneCapabilities {
        VkDisplayPlaneCapabilities {
            supported_alpha: VkDisplayPlaneAlphaFlags::default(),
            min_src_position: VkOffset2D::default(),
            max_src_position: VkOffset2D::default(),
            min_src_extent: VkExtent2D::default(),
            max_src_extent: VkExtent2D::default(),
            min_dst_position: VkOffset2D::default(),
            max_dst_position: VkOffset2D::default(),
            min_dst_extent: VkExtent2D::default(),
            max_dst_extent: VkExtent2D::default(),
        }
    }
}

impl VkSetup for VkDisplayPlaneCapabilities {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.min_src_position, fn_table, instance, device);
        VkSetup::vk_setup(&mut self.max_src_position, fn_table, instance, device);
        VkSetup::vk_setup(&mut self.min_src_extent, fn_table, instance, device);
        VkSetup::vk_setup(&mut self.max_src_extent, fn_table, instance, device);
        VkSetup::vk_setup(&mut self.min_dst_position, fn_table, instance, device);
        VkSetup::vk_setup(&mut self.max_dst_position, fn_table, instance, device);
        VkSetup::vk_setup(&mut self.min_dst_extent, fn_table, instance, device);
        VkSetup::vk_setup(&mut self.max_dst_extent, fn_table, instance, device);
    }
}

impl VkFree for RawVkDisplayPlaneCapabilities {
    fn vk_free(&mut self) {
        RawVkOffset2D::vk_free(&mut self.min_src_position);
        RawVkOffset2D::vk_free(&mut self.max_src_position);
        RawVkExtent2D::vk_free(&mut self.min_src_extent);
        RawVkExtent2D::vk_free(&mut self.max_src_extent);
        RawVkOffset2D::vk_free(&mut self.min_dst_position);
        RawVkOffset2D::vk_free(&mut self.max_dst_position);
        RawVkExtent2D::vk_free(&mut self.min_dst_extent);
        RawVkExtent2D::vk_free(&mut self.max_dst_extent);
    }
}