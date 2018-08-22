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
use vk::vk_offset_2d::*;
use vk::vk_extent_2d::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawVkRectLayer {
    pub offset: RawVkOffset2D,
    pub extent: RawVkExtent2D,
    pub layer: u32,
}

#[derive(Debug, Clone)]
pub struct VkRectLayer {
    pub offset: VkOffset2D,
    pub extent: VkExtent2D,
    pub layer: usize,
}

impl VkRawType<VkRectLayer> for RawVkRectLayer {
    fn vk_to_wrapped(src: &RawVkRectLayer) -> VkRectLayer {
        VkRectLayer {
            offset: RawVkOffset2D::vk_to_wrapped(&src.offset),
            extent: RawVkExtent2D::vk_to_wrapped(&src.extent),
            layer: u32::vk_to_wrapped(&src.layer),
        }
    }
}

impl VkWrappedType<RawVkRectLayer> for VkRectLayer {
    fn vk_to_raw(src: &VkRectLayer, dst: &mut RawVkRectLayer) {
        dst.offset = vk_to_raw_value(&src.offset);
        dst.extent = vk_to_raw_value(&src.extent);
        dst.layer = vk_to_raw_value(&src.layer);
    }
}

impl Default for VkRectLayer {
    fn default() -> VkRectLayer {
        VkRectLayer {
            offset: VkOffset2D::default(),
            extent: VkExtent2D::default(),
            layer: 0,
        }
    }
}

impl VkSetup for VkRectLayer {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.offset, fn_table, instance, device);
        VkSetup::vk_setup(&mut self.extent, fn_table, instance, device);
    }
}

impl VkFree for RawVkRectLayer {
    fn vk_free(&mut self) {
        RawVkOffset2D::vk_free(&mut self.offset);
        RawVkExtent2D::vk_free(&mut self.extent);
    }
}