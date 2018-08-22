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
use vk::vk_external_fence_handle_type_flags::*;
use vk::vk_external_fence_feature_flags::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawVkExternalFenceProperties {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub export_from_imported_handle_types: RawVkExternalFenceHandleTypeFlags,
    pub compatible_handle_types: RawVkExternalFenceHandleTypeFlags,
    pub external_fence_features: RawVkExternalFenceFeatureFlags,
}

#[derive(Debug, Clone)]
pub struct VkExternalFenceProperties {
    pub export_from_imported_handle_types: VkExternalFenceHandleTypeFlags,
    pub compatible_handle_types: VkExternalFenceHandleTypeFlags,
    pub external_fence_features: VkExternalFenceFeatureFlags,
}

impl VkRawType<VkExternalFenceProperties> for RawVkExternalFenceProperties {
    fn vk_to_wrapped(src: &RawVkExternalFenceProperties) -> VkExternalFenceProperties {
        VkExternalFenceProperties {
            export_from_imported_handle_types: RawVkExternalFenceHandleTypeFlags::vk_to_wrapped(&src.export_from_imported_handle_types),
            compatible_handle_types: RawVkExternalFenceHandleTypeFlags::vk_to_wrapped(&src.compatible_handle_types),
            external_fence_features: RawVkExternalFenceFeatureFlags::vk_to_wrapped(&src.external_fence_features),
        }
    }
}

impl VkWrappedType<RawVkExternalFenceProperties> for VkExternalFenceProperties {
    fn vk_to_raw(src: &VkExternalFenceProperties, dst: &mut RawVkExternalFenceProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ExternalFenceProperties);
        dst.next = ptr::null();
        dst.export_from_imported_handle_types = vk_to_raw_value(&src.export_from_imported_handle_types);
        dst.compatible_handle_types = vk_to_raw_value(&src.compatible_handle_types);
        dst.external_fence_features = vk_to_raw_value(&src.external_fence_features);
    }
}

impl Default for VkExternalFenceProperties {
    fn default() -> VkExternalFenceProperties {
        VkExternalFenceProperties {
            export_from_imported_handle_types: VkExternalFenceHandleTypeFlags::default(),
            compatible_handle_types: VkExternalFenceHandleTypeFlags::default(),
            external_fence_features: VkExternalFenceFeatureFlags::default(),
        }
    }
}

impl VkSetup for VkExternalFenceProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkExternalFenceProperties {
    fn vk_free(&mut self) {
        
    }
}