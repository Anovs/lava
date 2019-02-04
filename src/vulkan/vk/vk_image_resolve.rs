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
use vulkan::vk::{VkImageSubresourceLayers,RawVkImageSubresourceLayers};
use vulkan::vk::{VkOffset3D,RawVkOffset3D};
use vulkan::vk::{VkExtent3D,RawVkExtent3D};

/// Wrapper for [VkImageResolve](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkImageResolve.html)
#[derive(Debug, Clone)]
pub struct VkImageResolve {
    pub src_subresource: VkImageSubresourceLayers,
    pub src_offset: VkOffset3D,
    pub dst_subresource: VkImageSubresourceLayers,
    pub dst_offset: VkOffset3D,
    pub extent: VkExtent3D,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkImageResolve {
    pub src_subresource: RawVkImageSubresourceLayers,
    pub src_offset: RawVkOffset3D,
    pub dst_subresource: RawVkImageSubresourceLayers,
    pub dst_offset: RawVkOffset3D,
    pub extent: RawVkExtent3D,
}

impl VkWrappedType<RawVkImageResolve> for VkImageResolve {
    fn vk_to_raw(src: &VkImageResolve, dst: &mut RawVkImageResolve) {
        dst.src_subresource = vk_to_raw_value(&src.src_subresource);
        dst.src_offset = vk_to_raw_value(&src.src_offset);
        dst.dst_subresource = vk_to_raw_value(&src.dst_subresource);
        dst.dst_offset = vk_to_raw_value(&src.dst_offset);
        dst.extent = vk_to_raw_value(&src.extent);
    }
}

impl VkRawType<VkImageResolve> for RawVkImageResolve {
    fn vk_to_wrapped(src: &RawVkImageResolve) -> VkImageResolve {
        VkImageResolve {
            src_subresource: RawVkImageSubresourceLayers::vk_to_wrapped(&src.src_subresource),
            src_offset: RawVkOffset3D::vk_to_wrapped(&src.src_offset),
            dst_subresource: RawVkImageSubresourceLayers::vk_to_wrapped(&src.dst_subresource),
            dst_offset: RawVkOffset3D::vk_to_wrapped(&src.dst_offset),
            extent: RawVkExtent3D::vk_to_wrapped(&src.extent),
        }
    }
}

impl Default for VkImageResolve {
    fn default() -> VkImageResolve {
        VkImageResolve {
            src_subresource: VkImageSubresourceLayers::default(),
            src_offset: VkOffset3D::default(),
            dst_subresource: VkImageSubresourceLayers::default(),
            dst_offset: VkOffset3D::default(),
            extent: VkExtent3D::default(),
        }
    }
}

impl VkSetup for VkImageResolve {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.src_subresource, fn_table, instance, device);
        VkSetup::vk_setup(&mut self.src_offset, fn_table, instance, device);
        VkSetup::vk_setup(&mut self.dst_subresource, fn_table, instance, device);
        VkSetup::vk_setup(&mut self.dst_offset, fn_table, instance, device);
        VkSetup::vk_setup(&mut self.extent, fn_table, instance, device);
    }
}

impl VkFree for RawVkImageResolve {
    fn vk_free(&mut self) {
        RawVkImageSubresourceLayers::vk_free(&mut self.src_subresource);
        RawVkOffset3D::vk_free(&mut self.src_offset);
        RawVkImageSubresourceLayers::vk_free(&mut self.dst_subresource);
        RawVkOffset3D::vk_free(&mut self.dst_offset);
        RawVkExtent3D::vk_free(&mut self.extent);
    }
}