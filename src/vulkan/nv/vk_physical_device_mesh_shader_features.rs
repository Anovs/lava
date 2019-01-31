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

/// Wrapper for [VkPhysicalDeviceMeshShaderFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceMeshShaderFeaturesNV.html)
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceMeshShaderFeatures {
    pub task_shader: bool,
    pub mesh_shader: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceMeshShaderFeatures {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub task_shader: u32,
    pub mesh_shader: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceMeshShaderFeatures> for VkPhysicalDeviceMeshShaderFeatures {
    fn vk_to_raw(src: &VkPhysicalDeviceMeshShaderFeatures, dst: &mut RawVkPhysicalDeviceMeshShaderFeatures) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceMeshShaderFeaturesNv);
        dst.next = ptr::null();
        dst.task_shader = vk_to_raw_value(&src.task_shader);
        dst.mesh_shader = vk_to_raw_value(&src.mesh_shader);
    }
}

impl VkRawType<VkPhysicalDeviceMeshShaderFeatures> for RawVkPhysicalDeviceMeshShaderFeatures {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceMeshShaderFeatures) -> VkPhysicalDeviceMeshShaderFeatures {
        VkPhysicalDeviceMeshShaderFeatures {
            task_shader: u32::vk_to_wrapped(&src.task_shader),
            mesh_shader: u32::vk_to_wrapped(&src.mesh_shader),
        }
    }
}

impl Default for VkPhysicalDeviceMeshShaderFeatures {
    fn default() -> VkPhysicalDeviceMeshShaderFeatures {
        VkPhysicalDeviceMeshShaderFeatures {
            task_shader: false,
            mesh_shader: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceMeshShaderFeatures {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceMeshShaderFeatures {
    fn vk_free(&mut self) {
        
    }
}