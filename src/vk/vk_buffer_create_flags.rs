// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkBufferCreateFlags = u32;

#[derive(Debug, Clone, Copy)]
pub struct VkBufferCreateFlags {
    pub sparse_binding: bool,
    pub sparse_residency: bool,
    pub sparse_aliased: bool,
    pub protected: bool,
}

impl VkRawType<VkBufferCreateFlags> for RawVkBufferCreateFlags {
    fn vk_to_wrapped(src: &RawVkBufferCreateFlags) -> VkBufferCreateFlags {
        VkBufferCreateFlags {
            sparse_binding: (src & 0x00000001) != 0,
            sparse_residency: (src & 0x00000002) != 0,
            sparse_aliased: (src & 0x00000004) != 0,
            protected: (src & 0x00000008) != 0,
        }
    }
}

impl VkWrappedType<RawVkBufferCreateFlags> for VkBufferCreateFlags {
    fn vk_to_raw(src: &VkBufferCreateFlags, dst: &mut RawVkBufferCreateFlags) {
        *dst = 0;
        if src.sparse_binding { *dst |= 0x00000001; }
        if src.sparse_residency { *dst |= 0x00000002; }
        if src.sparse_aliased { *dst |= 0x00000004; }
        if src.protected { *dst |= 0x00000008; }
    }
}

impl Default for VkBufferCreateFlags {
    fn default() -> VkBufferCreateFlags {
        VkBufferCreateFlags {
            sparse_binding: false,
            sparse_residency: false,
            sparse_aliased: false,
            protected: false,
        }
    }
}

impl VkBufferCreateFlags {
    
    pub fn none() -> VkBufferCreateFlags {
        VkBufferCreateFlags {
            sparse_binding: false,
            sparse_residency: false,
            sparse_aliased: false,
            protected: false,
        }
    }
    
    pub fn all() -> VkBufferCreateFlags {
        VkBufferCreateFlags {
            sparse_binding: true,
            sparse_residency: true,
            sparse_aliased: true,
            protected: true,
        }
    }
}

impl VkBufferCreateFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.sparse_binding { 0x00000001 } else { 0 }
        + if self.sparse_residency { 0x00000002 } else { 0 }
        + if self.sparse_aliased { 0x00000004 } else { 0 }
        + if self.protected { 0x00000008 } else { 0 }
    }
}