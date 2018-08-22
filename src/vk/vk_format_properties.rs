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
use vk::vk_format_feature_flags::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawVkFormatProperties {
    pub linear_tiling_features: RawVkFormatFeatureFlags,
    pub optimal_tiling_features: RawVkFormatFeatureFlags,
    pub buffer_features: RawVkFormatFeatureFlags,
}

#[derive(Debug, Clone)]
pub struct VkFormatProperties {
    pub linear_tiling_features: VkFormatFeatureFlags,
    pub optimal_tiling_features: VkFormatFeatureFlags,
    pub buffer_features: VkFormatFeatureFlags,
}

impl VkRawType<VkFormatProperties> for RawVkFormatProperties {
    fn vk_to_wrapped(src: &RawVkFormatProperties) -> VkFormatProperties {
        VkFormatProperties {
            linear_tiling_features: RawVkFormatFeatureFlags::vk_to_wrapped(&src.linear_tiling_features),
            optimal_tiling_features: RawVkFormatFeatureFlags::vk_to_wrapped(&src.optimal_tiling_features),
            buffer_features: RawVkFormatFeatureFlags::vk_to_wrapped(&src.buffer_features),
        }
    }
}

impl VkWrappedType<RawVkFormatProperties> for VkFormatProperties {
    fn vk_to_raw(src: &VkFormatProperties, dst: &mut RawVkFormatProperties) {
        dst.linear_tiling_features = vk_to_raw_value(&src.linear_tiling_features);
        dst.optimal_tiling_features = vk_to_raw_value(&src.optimal_tiling_features);
        dst.buffer_features = vk_to_raw_value(&src.buffer_features);
    }
}

impl Default for VkFormatProperties {
    fn default() -> VkFormatProperties {
        VkFormatProperties {
            linear_tiling_features: VkFormatFeatureFlags::default(),
            optimal_tiling_features: VkFormatFeatureFlags::default(),
            buffer_features: VkFormatFeatureFlags::default(),
        }
    }
}

impl VkSetup for VkFormatProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkFormatProperties {
    fn vk_free(&mut self) {
        
    }
}