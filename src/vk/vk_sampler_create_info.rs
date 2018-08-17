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
use vk::vk_structure_type::*;
use vk::vk_sampler_create_flags::*;
use vk::vk_filter::*;
use vk::vk_sampler_mipmap_mode::*;
use vk::vk_sampler_address_mode::*;
use vk::vk_compare_op::*;
use vk::vk_border_color::*;

#[repr(C)]
pub struct RawVkSamplerCreateInfo {
    s_type: RawVkStructureType,
    next: *const c_void,
    flags: RawVkSamplerCreateFlags,
    mag_filter: RawVkFilter,
    min_filter: RawVkFilter,
    mipmap_mode: RawVkSamplerMipmapMode,
    address_mode_u: RawVkSamplerAddressMode,
    address_mode_v: RawVkSamplerAddressMode,
    address_mode_w: RawVkSamplerAddressMode,
    mip_lod_bias: f32,
    anisotropy_enable: u32,
    max_anisotropy: f32,
    compare_enable: u32,
    compare_op: RawVkCompareOp,
    min_lod: f32,
    max_lod: f32,
    border_color: RawVkBorderColor,
    unnormalized_coordinates: u32,
}

#[derive(Debug, Clone)]
pub struct VkSamplerCreateInfo {
    pub flags: VkSamplerCreateFlags,
    pub mag_filter: VkFilter,
    pub min_filter: VkFilter,
    pub mipmap_mode: VkSamplerMipmapMode,
    pub address_mode_u: VkSamplerAddressMode,
    pub address_mode_v: VkSamplerAddressMode,
    pub address_mode_w: VkSamplerAddressMode,
    pub mip_lod_bias: f32,
    pub anisotropy_enable: bool,
    pub max_anisotropy: f32,
    pub compare_enable: bool,
    pub compare_op: VkCompareOp,
    pub min_lod: f32,
    pub max_lod: f32,
    pub border_color: VkBorderColor,
    pub unnormalized_coordinates: bool,
}

impl VkRawType<VkSamplerCreateInfo> for RawVkSamplerCreateInfo {
    fn vk_to_wrapped(src: &RawVkSamplerCreateInfo) -> VkSamplerCreateInfo {
        VkSamplerCreateInfo {
            flags: RawVkSamplerCreateFlags::vk_to_wrapped(&src.flags),
            mag_filter: RawVkFilter::vk_to_wrapped(&src.mag_filter),
            min_filter: RawVkFilter::vk_to_wrapped(&src.min_filter),
            mipmap_mode: RawVkSamplerMipmapMode::vk_to_wrapped(&src.mipmap_mode),
            address_mode_u: RawVkSamplerAddressMode::vk_to_wrapped(&src.address_mode_u),
            address_mode_v: RawVkSamplerAddressMode::vk_to_wrapped(&src.address_mode_v),
            address_mode_w: RawVkSamplerAddressMode::vk_to_wrapped(&src.address_mode_w),
            mip_lod_bias: src.mip_lod_bias,
            anisotropy_enable: u32::vk_to_wrapped(&src.anisotropy_enable),
            max_anisotropy: src.max_anisotropy,
            compare_enable: u32::vk_to_wrapped(&src.compare_enable),
            compare_op: RawVkCompareOp::vk_to_wrapped(&src.compare_op),
            min_lod: src.min_lod,
            max_lod: src.max_lod,
            border_color: RawVkBorderColor::vk_to_wrapped(&src.border_color),
            unnormalized_coordinates: u32::vk_to_wrapped(&src.unnormalized_coordinates),
        }
    }
}

impl VkWrappedType<RawVkSamplerCreateInfo> for VkSamplerCreateInfo {
    fn vk_to_raw(src: &VkSamplerCreateInfo, dst: &mut RawVkSamplerCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SamplerCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.mag_filter = vk_to_raw_value(&src.mag_filter);
        dst.min_filter = vk_to_raw_value(&src.min_filter);
        dst.mipmap_mode = vk_to_raw_value(&src.mipmap_mode);
        dst.address_mode_u = vk_to_raw_value(&src.address_mode_u);
        dst.address_mode_v = vk_to_raw_value(&src.address_mode_v);
        dst.address_mode_w = vk_to_raw_value(&src.address_mode_w);
        dst.mip_lod_bias = src.mip_lod_bias;
        dst.anisotropy_enable = vk_to_raw_value(&src.anisotropy_enable);
        dst.max_anisotropy = src.max_anisotropy;
        dst.compare_enable = vk_to_raw_value(&src.compare_enable);
        dst.compare_op = vk_to_raw_value(&src.compare_op);
        dst.min_lod = src.min_lod;
        dst.max_lod = src.max_lod;
        dst.border_color = vk_to_raw_value(&src.border_color);
        dst.unnormalized_coordinates = vk_to_raw_value(&src.unnormalized_coordinates);
    }
}

impl Default for VkSamplerCreateInfo {
    fn default() -> VkSamplerCreateInfo {
        VkSamplerCreateInfo {
            flags: VkSamplerCreateFlags::default(),
            mag_filter: VkFilter::default(),
            min_filter: VkFilter::default(),
            mipmap_mode: VkSamplerMipmapMode::default(),
            address_mode_u: VkSamplerAddressMode::default(),
            address_mode_v: VkSamplerAddressMode::default(),
            address_mode_w: VkSamplerAddressMode::default(),
            mip_lod_bias: 0.0,
            anisotropy_enable: false,
            max_anisotropy: 0.0,
            compare_enable: false,
            compare_op: VkCompareOp::default(),
            min_lod: 0.0,
            max_lod: 0.0,
            border_color: VkBorderColor::default(),
            unnormalized_coordinates: false,
        }
    }
}

impl VkSetup for VkSamplerCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkSamplerCreateInfo {
    fn vk_free(&mut self) {
        
    }
}