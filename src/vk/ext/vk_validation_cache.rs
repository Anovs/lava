// Generated by `scripts/generate_vk.js`

use utils::c_bindings::*;
use utils::vk_traits::*;
use utils::vk_ptr::*;
use utils::vk_convert::*;
use std::os::raw::c_char;
use std::ptr;
use std::mem;
use std::cmp;
use vk::*;

pub type RawVkValidationCache = u64;

#[derive(Debug, Copy, Clone)]
pub struct VkValidationCache {
    _handle: RawVkValidationCache,
    _parent_instance: RawVkInstance,
    _parent_device: RawVkDevice,
    _fn_table: *mut VkInstanceFunctionTable
}

impl VkRawType<VkValidationCache> for RawVkValidationCache {
    fn vk_to_wrapped(src: &RawVkValidationCache) -> VkValidationCache {
        VkValidationCache {
            _handle: *src,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkValidationCache> for VkValidationCache {
    fn vk_to_raw(src: &VkValidationCache, dst: &mut RawVkValidationCache) {
        *dst = src._handle
    }
}

impl Default for VkValidationCache {
    fn default() -> VkValidationCache {
        VkValidationCache {
            _handle: 0,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkSetup for VkValidationCache {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        self._parent_instance = instance;
        self._parent_device = device;
        self._fn_table = fn_table;
    }
}

impl VkValidationCache {
    
    pub fn handle(&self) -> u64 {
        self._handle
    }
    
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyValidationCacheEXT)(self._parent_device, self._handle, ptr::null());
        }
    }
    
    pub fn merge(&self, src_caches: &[&ext::VkValidationCache]) -> VkResult {
        unsafe {
            let raw_src_cache_count = src_caches.len() as u32;
            let raw_src_caches = new_ptr_vk_array_from_ref(src_caches);
            let vk_result = ((&*self._fn_table).vkMergeValidationCachesEXT)(self._parent_device, self._handle, raw_src_cache_count, raw_src_caches);
            free_ptr(raw_src_caches);
            RawVkResult::vk_to_wrapped(&vk_result)
        }
    }
    
    pub fn get_data(&self) -> Result<Vec<c_void>, VkResult> {
        unsafe {
            let mut raw_data : *mut c_void = ptr::null_mut();
            let raw_data_size = &mut mem::uninitialized() as *mut usize;
            let vk_result = ((&*self._fn_table).vkGetValidationCacheDataEXT)(self._parent_device, self._handle, raw_data_size, raw_data);
            if vk_result != 0 { return Err(RawVkResult::vk_to_wrapped(&vk_result)) }
            raw_data = malloc((*raw_data_size as usize) * mem::size_of::<c_void>()) as *mut c_void;
            
            let vk_result = ((&*self._fn_table).vkGetValidationCacheDataEXT)(self._parent_device, self._handle, raw_data_size, raw_data);
            if vk_result != 0 { return Err(RawVkResult::vk_to_wrapped(&vk_result)) }
            
            let data = Vec::from_raw_parts(raw_data, *raw_data_size, *raw_data_size);
            Ok(data)
        }
    }
}