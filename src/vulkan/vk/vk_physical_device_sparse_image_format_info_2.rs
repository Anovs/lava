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
use vulkan::vk::{VkFormat,RawVkFormat};
use vulkan::vk::{VkImageType,RawVkImageType};
use vulkan::vk::{VkSampleCountFlags,RawVkSampleCountFlags};
use vulkan::vk::{VkImageUsageFlags,RawVkImageUsageFlags};
use vulkan::vk::{VkImageTiling,RawVkImageTiling};

/// Wrapper for [VkPhysicalDeviceSparseImageFormatInfo2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSparseImageFormatInfo2.html).
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceSparseImageFormatInfo2 {
    pub format: VkFormat,
    pub type_: VkImageType,
    pub samples: VkSampleCountFlags,
    pub usage: VkImageUsageFlags,
    pub tiling: VkImageTiling,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceSparseImageFormatInfo2 {
    pub s_type: RawVkStructureType,
    pub next: *mut c_void,
    pub format: RawVkFormat,
    pub type_: RawVkImageType,
    pub samples: RawVkSampleCountFlags,
    pub usage: RawVkImageUsageFlags,
    pub tiling: RawVkImageTiling,
}

impl VkWrappedType<RawVkPhysicalDeviceSparseImageFormatInfo2> for VkPhysicalDeviceSparseImageFormatInfo2 {
    fn vk_to_raw(src: &VkPhysicalDeviceSparseImageFormatInfo2, dst: &mut RawVkPhysicalDeviceSparseImageFormatInfo2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceSparseImageFormatInfo2);
        dst.next = ptr::null_mut();
        dst.format = vk_to_raw_value(&src.format);
        dst.type_ = vk_to_raw_value(&src.type_);
        dst.samples = vk_to_raw_value(&src.samples);
        dst.usage = vk_to_raw_value(&src.usage);
        dst.tiling = vk_to_raw_value(&src.tiling);
    }
}

impl VkRawType<VkPhysicalDeviceSparseImageFormatInfo2> for RawVkPhysicalDeviceSparseImageFormatInfo2 {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceSparseImageFormatInfo2) -> VkPhysicalDeviceSparseImageFormatInfo2 {
        VkPhysicalDeviceSparseImageFormatInfo2 {
            format: RawVkFormat::vk_to_wrapped(&src.format),
            type_: RawVkImageType::vk_to_wrapped(&src.type_),
            samples: RawVkSampleCountFlags::vk_to_wrapped(&src.samples),
            usage: RawVkImageUsageFlags::vk_to_wrapped(&src.usage),
            tiling: RawVkImageTiling::vk_to_wrapped(&src.tiling),
        }
    }
}

impl Default for VkPhysicalDeviceSparseImageFormatInfo2 {
    fn default() -> VkPhysicalDeviceSparseImageFormatInfo2 {
        VkPhysicalDeviceSparseImageFormatInfo2 {
            format: Default::default(),
            type_: Default::default(),
            samples: Default::default(),
            usage: Default::default(),
            tiling: Default::default(),
        }
    }
}

impl VkSetup for VkPhysicalDeviceSparseImageFormatInfo2 {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceSparseImageFormatInfo2 {
    fn vk_free(&self) {
        
    }
}