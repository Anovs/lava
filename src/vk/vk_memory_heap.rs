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
use vk::vk_memory_heap_flags::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawVkMemoryHeap {
    pub size: u64,
    pub flags: RawVkMemoryHeapFlags,
}

#[derive(Debug, Clone)]
pub struct VkMemoryHeap {
    pub size: usize,
    pub flags: VkMemoryHeapFlags,
}

impl VkRawType<VkMemoryHeap> for RawVkMemoryHeap {
    fn vk_to_wrapped(src: &RawVkMemoryHeap) -> VkMemoryHeap {
        VkMemoryHeap {
            size: u64::vk_to_wrapped(&src.size),
            flags: RawVkMemoryHeapFlags::vk_to_wrapped(&src.flags),
        }
    }
}

impl VkWrappedType<RawVkMemoryHeap> for VkMemoryHeap {
    fn vk_to_raw(src: &VkMemoryHeap, dst: &mut RawVkMemoryHeap) {
        dst.size = vk_to_raw_value(&src.size);
        dst.flags = vk_to_raw_value(&src.flags);
    }
}

impl Default for VkMemoryHeap {
    fn default() -> VkMemoryHeap {
        VkMemoryHeap {
            size: 0,
            flags: VkMemoryHeapFlags::default(),
        }
    }
}

impl VkSetup for VkMemoryHeap {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkMemoryHeap {
    fn vk_free(&mut self) {
        
    }
}