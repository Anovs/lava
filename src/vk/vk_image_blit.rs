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
use vk::vk_image_subresource_layers::*;
use vk::vk_offset_3d::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawVkImageBlit {
    pub src_subresource: RawVkImageSubresourceLayers,
    pub src_offsets: [RawVkOffset3D; 2],
    pub dst_subresource: RawVkImageSubresourceLayers,
    pub dst_offsets: [RawVkOffset3D; 2],
}

#[derive(Debug, Clone)]
pub struct VkImageBlit {
    pub src_subresource: VkImageSubresourceLayers,
    pub src_offsets: [VkOffset3D; 2],
    pub dst_subresource: VkImageSubresourceLayers,
    pub dst_offsets: [VkOffset3D; 2],
}

impl VkRawType<VkImageBlit> for RawVkImageBlit {
    fn vk_to_wrapped(src: &RawVkImageBlit) -> VkImageBlit {
        VkImageBlit {
            src_subresource: RawVkImageSubresourceLayers::vk_to_wrapped(&src.src_subresource),
            src_offsets: unsafe { let mut dst_array : [VkOffset3D; 2] = mem::uninitialized(); vk_to_wrapped_array(&src.src_offsets, &mut dst_array); dst_array },
            dst_subresource: RawVkImageSubresourceLayers::vk_to_wrapped(&src.dst_subresource),
            dst_offsets: unsafe { let mut dst_array : [VkOffset3D; 2] = mem::uninitialized(); vk_to_wrapped_array(&src.dst_offsets, &mut dst_array); dst_array },
        }
    }
}

impl VkWrappedType<RawVkImageBlit> for VkImageBlit {
    fn vk_to_raw(src: &VkImageBlit, dst: &mut RawVkImageBlit) {
        dst.src_subresource = vk_to_raw_value(&src.src_subresource);
        dst.src_offsets = unsafe { let mut dst_array : [RawVkOffset3D; 2] = mem::uninitialized(); vk_to_raw_array(&src.src_offsets, &mut dst_array); dst_array };
        dst.dst_subresource = vk_to_raw_value(&src.dst_subresource);
        dst.dst_offsets = unsafe { let mut dst_array : [RawVkOffset3D; 2] = mem::uninitialized(); vk_to_raw_array(&src.dst_offsets, &mut dst_array); dst_array };
    }
}

impl Default for VkImageBlit {
    fn default() -> VkImageBlit {
        VkImageBlit {
            src_subresource: VkImageSubresourceLayers::default(),
            src_offsets: unsafe { let mut dst_array : [VkOffset3D; 2] = mem::uninitialized(); fill_vk_array(&mut dst_array); dst_array },
            dst_subresource: VkImageSubresourceLayers::default(),
            dst_offsets: unsafe { let mut dst_array : [VkOffset3D; 2] = mem::uninitialized(); fill_vk_array(&mut dst_array); dst_array },
        }
    }
}

impl VkSetup for VkImageBlit {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.src_subresource, fn_table, instance, device);
        VkSetup::vk_setup(&mut self.dst_subresource, fn_table, instance, device);
    }
}

impl VkFree for RawVkImageBlit {
    fn vk_free(&mut self) {
        RawVkImageSubresourceLayers::vk_free(&mut self.src_subresource);
        for elt in self.src_offsets.iter_mut() { RawVkOffset3D::vk_free(elt); };
        RawVkImageSubresourceLayers::vk_free(&mut self.dst_subresource);
        for elt in self.dst_offsets.iter_mut() { RawVkOffset3D::vk_free(elt); };
    }
}