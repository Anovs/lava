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
use vulkan::khr::{VkResolveModeFlags,RawVkResolveModeFlags};

/// Wrapper for [VkPhysicalDeviceDepthStencilResolvePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceDepthStencilResolvePropertiesKHR.html)
#[derive(Debug, Clone)]
pub struct VkPhysicalDeviceDepthStencilResolveProperties {
    pub supported_depth_resolve_modes: VkResolveModeFlags,
    pub supported_stencil_resolve_modes: VkResolveModeFlags,
    pub independent_resolve_none: bool,
    pub independent_resolve: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPhysicalDeviceDepthStencilResolveProperties {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub supported_depth_resolve_modes: RawVkResolveModeFlags,
    pub supported_stencil_resolve_modes: RawVkResolveModeFlags,
    pub independent_resolve_none: u32,
    pub independent_resolve: u32,
}

impl VkWrappedType<RawVkPhysicalDeviceDepthStencilResolveProperties> for VkPhysicalDeviceDepthStencilResolveProperties {
    fn vk_to_raw(src: &VkPhysicalDeviceDepthStencilResolveProperties, dst: &mut RawVkPhysicalDeviceDepthStencilResolveProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDeviceDepthStencilResolvePropertiesKhr);
        dst.next = ptr::null();
        dst.supported_depth_resolve_modes = vk_to_raw_value(&src.supported_depth_resolve_modes);
        dst.supported_stencil_resolve_modes = vk_to_raw_value(&src.supported_stencil_resolve_modes);
        dst.independent_resolve_none = vk_to_raw_value(&src.independent_resolve_none);
        dst.independent_resolve = vk_to_raw_value(&src.independent_resolve);
    }
}

impl VkRawType<VkPhysicalDeviceDepthStencilResolveProperties> for RawVkPhysicalDeviceDepthStencilResolveProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceDepthStencilResolveProperties) -> VkPhysicalDeviceDepthStencilResolveProperties {
        VkPhysicalDeviceDepthStencilResolveProperties {
            supported_depth_resolve_modes: RawVkResolveModeFlags::vk_to_wrapped(&src.supported_depth_resolve_modes),
            supported_stencil_resolve_modes: RawVkResolveModeFlags::vk_to_wrapped(&src.supported_stencil_resolve_modes),
            independent_resolve_none: u32::vk_to_wrapped(&src.independent_resolve_none),
            independent_resolve: u32::vk_to_wrapped(&src.independent_resolve),
        }
    }
}

impl Default for VkPhysicalDeviceDepthStencilResolveProperties {
    fn default() -> VkPhysicalDeviceDepthStencilResolveProperties {
        VkPhysicalDeviceDepthStencilResolveProperties {
            supported_depth_resolve_modes: VkResolveModeFlags::default(),
            supported_stencil_resolve_modes: VkResolveModeFlags::default(),
            independent_resolve_none: false,
            independent_resolve: false,
        }
    }
}

impl VkSetup for VkPhysicalDeviceDepthStencilResolveProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDeviceDepthStencilResolveProperties {
    fn vk_free(&mut self) {
        
    }
}