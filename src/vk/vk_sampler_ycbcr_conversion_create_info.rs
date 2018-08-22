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
use vk::vk_format::*;
use vk::vk_sampler_ycbcr_model_conversion::*;
use vk::vk_sampler_ycbcr_range::*;
use vk::vk_component_mapping::*;
use vk::vk_chroma_location::*;
use vk::vk_filter::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawVkSamplerYcbcrConversionCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub format: RawVkFormat,
    pub ycbcr_model: RawVkSamplerYcbcrModelConversion,
    pub ycbcr_range: RawVkSamplerYcbcrRange,
    pub components: RawVkComponentMapping,
    pub x_chroma_offset: RawVkChromaLocation,
    pub y_chroma_offset: RawVkChromaLocation,
    pub chroma_filter: RawVkFilter,
    pub force_explicit_reconstruction: u32,
}

#[derive(Debug, Clone)]
pub struct VkSamplerYcbcrConversionCreateInfo {
    pub format: VkFormat,
    pub ycbcr_model: VkSamplerYcbcrModelConversion,
    pub ycbcr_range: VkSamplerYcbcrRange,
    pub components: VkComponentMapping,
    pub x_chroma_offset: VkChromaLocation,
    pub y_chroma_offset: VkChromaLocation,
    pub chroma_filter: VkFilter,
    pub force_explicit_reconstruction: bool,
}

impl VkRawType<VkSamplerYcbcrConversionCreateInfo> for RawVkSamplerYcbcrConversionCreateInfo {
    fn vk_to_wrapped(src: &RawVkSamplerYcbcrConversionCreateInfo) -> VkSamplerYcbcrConversionCreateInfo {
        VkSamplerYcbcrConversionCreateInfo {
            format: RawVkFormat::vk_to_wrapped(&src.format),
            ycbcr_model: RawVkSamplerYcbcrModelConversion::vk_to_wrapped(&src.ycbcr_model),
            ycbcr_range: RawVkSamplerYcbcrRange::vk_to_wrapped(&src.ycbcr_range),
            components: RawVkComponentMapping::vk_to_wrapped(&src.components),
            x_chroma_offset: RawVkChromaLocation::vk_to_wrapped(&src.x_chroma_offset),
            y_chroma_offset: RawVkChromaLocation::vk_to_wrapped(&src.y_chroma_offset),
            chroma_filter: RawVkFilter::vk_to_wrapped(&src.chroma_filter),
            force_explicit_reconstruction: u32::vk_to_wrapped(&src.force_explicit_reconstruction),
        }
    }
}

impl VkWrappedType<RawVkSamplerYcbcrConversionCreateInfo> for VkSamplerYcbcrConversionCreateInfo {
    fn vk_to_raw(src: &VkSamplerYcbcrConversionCreateInfo, dst: &mut RawVkSamplerYcbcrConversionCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::SamplerYcbcrConversionCreateInfo);
        dst.next = ptr::null();
        dst.format = vk_to_raw_value(&src.format);
        dst.ycbcr_model = vk_to_raw_value(&src.ycbcr_model);
        dst.ycbcr_range = vk_to_raw_value(&src.ycbcr_range);
        dst.components = vk_to_raw_value(&src.components);
        dst.x_chroma_offset = vk_to_raw_value(&src.x_chroma_offset);
        dst.y_chroma_offset = vk_to_raw_value(&src.y_chroma_offset);
        dst.chroma_filter = vk_to_raw_value(&src.chroma_filter);
        dst.force_explicit_reconstruction = vk_to_raw_value(&src.force_explicit_reconstruction);
    }
}

impl Default for VkSamplerYcbcrConversionCreateInfo {
    fn default() -> VkSamplerYcbcrConversionCreateInfo {
        VkSamplerYcbcrConversionCreateInfo {
            format: VkFormat::default(),
            ycbcr_model: VkSamplerYcbcrModelConversion::default(),
            ycbcr_range: VkSamplerYcbcrRange::default(),
            components: VkComponentMapping::default(),
            x_chroma_offset: VkChromaLocation::default(),
            y_chroma_offset: VkChromaLocation::default(),
            chroma_filter: VkFilter::default(),
            force_explicit_reconstruction: false,
        }
    }
}

impl VkSetup for VkSamplerYcbcrConversionCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.components, fn_table, instance, device);
    }
}

impl VkFree for RawVkSamplerYcbcrConversionCreateInfo {
    fn vk_free(&mut self) {
        RawVkComponentMapping::vk_free(&mut self.components);
    }
}