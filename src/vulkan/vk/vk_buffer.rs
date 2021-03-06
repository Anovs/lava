// Generated by `scripts/generate.js`

use utils::c_bindings::*;
use utils::vk_traits::*;
use utils::vk_ptr::*;
use utils::vk_convert::*;
use std::os::raw::c_char;
use std::ops::Drop;
use std::ptr;
use std::mem;
use std::cmp;
use std::slice;
use vulkan::*;
use vulkan::vk::*;

#[doc(hidden)]
pub type RawVkBuffer = u64;

/// Wrapper for [VkBuffer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBuffer.html).
#[derive(Debug, Clone, Copy)]
pub struct VkBuffer {
    _handle: RawVkBuffer,
    _fn_table: *mut VkFunctionTable
}

impl VkRawType<VkBuffer> for RawVkBuffer {
    fn vk_to_wrapped(src: &RawVkBuffer) -> VkBuffer {
        VkBuffer {
            _handle: *src,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkBuffer> for VkBuffer {
    fn vk_to_raw(src: &VkBuffer, dst: &mut RawVkBuffer) {
        *dst = src._handle
    }
}

impl Default for VkBuffer {
    fn default() -> VkBuffer {
        VkBuffer {
            _handle: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkBuffer {
    fn eq(&self, other: &VkBuffer) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkBuffer {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        self._fn_table = fn_table;
    }
}

impl VkBuffer {
    
    /// Returns the internal Vulkan handle for the object.
    pub fn vk_handle(&self) -> u64 {
        self._handle
    }
    
    /// Indicates if the Vulkan internal handle for this object is 0.
    pub fn is_null(&self) -> bool {
        self._handle == 0
    }
    
    /// Creates an object with a null Vulkan internal handle.
    ///
    /// Calling a method with a null handle will most likely result in a crash.
    pub fn null() -> Self {
        Self {
            _handle: 0,
            _fn_table: ptr::null_mut()
        }
    }
    
    /// Wrapper for [vkBindBufferMemory](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindBufferMemory.html).
    pub fn bind_memory(&self, memory: VkDeviceMemory, memory_offset: usize) -> LavaResult<()> {
        unsafe {
            let raw_memory = vk_to_raw_value(&memory);
            let raw_memory_offset = vk_to_raw_value(&memory_offset);
            let vk_result = ((&*self._fn_table).vkBindBufferMemory)((*self._fn_table).device, self._handle, raw_memory, raw_memory_offset);
            if vk_result == 0 { Ok(()) } else { Err((RawVkResult::vk_to_wrapped(&vk_result), ())) }
        }
    }
    
    /// Wrapper for [vkGetBufferMemoryRequirements](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferMemoryRequirements.html).
    pub fn get_memory_requirements(&self) -> VkMemoryRequirements {
        unsafe {
            let raw_memory_requirements = &mut mem::zeroed() as *mut RawVkMemoryRequirements;
            
            ((&*self._fn_table).vkGetBufferMemoryRequirements)((*self._fn_table).device, self._handle, raw_memory_requirements);
            
            let mut memory_requirements = new_vk_value(raw_memory_requirements);
            let fn_table = self._fn_table;
            VkSetup::vk_setup(&mut memory_requirements, fn_table);
            memory_requirements
        }
    }
    
    /// Wrapper for [vkDestroyBuffer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyBuffer.html).
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyBuffer)((*self._fn_table).device, self._handle, ptr::null());
        }
    }
}