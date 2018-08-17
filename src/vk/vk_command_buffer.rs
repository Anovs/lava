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

pub type RawVkCommandBuffer = u64;

#[derive(Debug, Copy, Clone)]
pub struct VkCommandBuffer {
    _handle: RawVkCommandBuffer,
    _parent_instance: RawVkInstance,
    _parent_device: RawVkDevice,
    _fn_table: *mut VkInstanceFunctionTable
}

impl VkRawType<VkCommandBuffer> for RawVkCommandBuffer {
    fn vk_to_wrapped(src: &RawVkCommandBuffer) -> VkCommandBuffer {
        VkCommandBuffer {
            _handle: *src,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkCommandBuffer> for VkCommandBuffer {
    fn vk_to_raw(src: &VkCommandBuffer, dst: &mut RawVkCommandBuffer) {
        *dst = src._handle
    }
}

impl Default for VkCommandBuffer {
    fn default() -> VkCommandBuffer {
        VkCommandBuffer {
            _handle: 0,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkSetup for VkCommandBuffer {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        self._parent_instance = instance;
        self._parent_device = device;
        self._fn_table = fn_table;
    }
}

impl VkCommandBuffer {
    
    pub fn handle(&self) -> u64 {
        self._handle
    }
    
    pub fn begin(&self, begin_info: &VkCommandBufferBeginInfo) -> VkResult {
        unsafe {
            let raw_begin_info = new_ptr_vk_value(begin_info);
            let vk_result = ((&*self._fn_table).vkBeginCommandBuffer)(self._handle, raw_begin_info);
            free_vk_ptr(raw_begin_info);
            RawVkResult::vk_to_wrapped(&vk_result)
        }
    }
    
    pub fn end(&self) -> VkResult {
        unsafe {
            let vk_result = ((&*self._fn_table).vkEndCommandBuffer)(self._handle);
            RawVkResult::vk_to_wrapped(&vk_result)
        }
    }
    
    pub fn reset(&self, flags: VkCommandBufferResetFlags) -> VkResult {
        unsafe {
            let raw_flags = vk_to_raw_value(&flags);
            let vk_result = ((&*self._fn_table).vkResetCommandBuffer)(self._handle, raw_flags);
            RawVkResult::vk_to_wrapped(&vk_result)
        }
    }
    
    pub fn cmd_bind_pipeline(&self, pipeline_bind_point: VkPipelineBindPoint, pipeline: &VkPipeline) {
        unsafe {
            let raw_pipeline_bind_point = vk_to_raw_value(&pipeline_bind_point);
            let raw_pipeline = vk_to_raw_value(pipeline);
            ((&*self._fn_table).vkCmdBindPipeline)(self._handle, raw_pipeline_bind_point, raw_pipeline);
        }
    }
    
    pub fn cmd_set_viewport(&self, first_viewport: usize, viewports: &[VkViewport]) {
        unsafe {
            let raw_first_viewport = vk_to_raw_value(&first_viewport);
            let raw_viewport_count = viewports.len() as u32;
            let raw_viewports = new_ptr_vk_array(viewports);
            ((&*self._fn_table).vkCmdSetViewport)(self._handle, raw_first_viewport, raw_viewport_count, raw_viewports);
            free_vk_ptr_array(raw_viewport_count as usize, raw_viewports);
        }
    }
    
    pub fn cmd_set_scissor(&self, first_scissor: usize, scissors: &[VkRect2D]) {
        unsafe {
            let raw_first_scissor = vk_to_raw_value(&first_scissor);
            let raw_scissor_count = scissors.len() as u32;
            let raw_scissors = new_ptr_vk_array(scissors);
            ((&*self._fn_table).vkCmdSetScissor)(self._handle, raw_first_scissor, raw_scissor_count, raw_scissors);
            free_vk_ptr_array(raw_scissor_count as usize, raw_scissors);
        }
    }
    
    pub fn cmd_set_line_width(&self, line_width: f32) {
        unsafe {
            let raw_line_width = line_width;
            ((&*self._fn_table).vkCmdSetLineWidth)(self._handle, raw_line_width);
        }
    }
    
    pub fn cmd_set_depth_bias(&self, depth_bias_constant_factor: f32, depth_bias_clamp: f32, depth_bias_slope_factor: f32) {
        unsafe {
            let raw_depth_bias_constant_factor = depth_bias_constant_factor;
            let raw_depth_bias_clamp = depth_bias_clamp;
            let raw_depth_bias_slope_factor = depth_bias_slope_factor;
            ((&*self._fn_table).vkCmdSetDepthBias)(self._handle, raw_depth_bias_constant_factor, raw_depth_bias_clamp, raw_depth_bias_slope_factor);
        }
    }
    
    pub fn cmd_set_blend_constants(&self, blend_constants: [f32; 4]) {
        unsafe {
            let raw_blend_constants = { let mut dst_array : [f32; 4] = mem::uninitialized(); to_array(&blend_constants, &mut dst_array); dst_array };
            ((&*self._fn_table).vkCmdSetBlendConstants)(self._handle, raw_blend_constants);
        }
    }
    
    pub fn cmd_set_depth_bounds(&self, min_depth_bounds: f32, max_depth_bounds: f32) {
        unsafe {
            let raw_min_depth_bounds = min_depth_bounds;
            let raw_max_depth_bounds = max_depth_bounds;
            ((&*self._fn_table).vkCmdSetDepthBounds)(self._handle, raw_min_depth_bounds, raw_max_depth_bounds);
        }
    }
    
    pub fn cmd_set_stencil_compare_mask(&self, face_mask: VkStencilFaceFlags, compare_mask: u32) {
        unsafe {
            let raw_face_mask = vk_to_raw_value(&face_mask);
            let raw_compare_mask = compare_mask;
            ((&*self._fn_table).vkCmdSetStencilCompareMask)(self._handle, raw_face_mask, raw_compare_mask);
        }
    }
    
    pub fn cmd_set_stencil_write_mask(&self, face_mask: VkStencilFaceFlags, write_mask: u32) {
        unsafe {
            let raw_face_mask = vk_to_raw_value(&face_mask);
            let raw_write_mask = write_mask;
            ((&*self._fn_table).vkCmdSetStencilWriteMask)(self._handle, raw_face_mask, raw_write_mask);
        }
    }
    
    pub fn cmd_set_stencil_reference(&self, face_mask: VkStencilFaceFlags, reference: usize) {
        unsafe {
            let raw_face_mask = vk_to_raw_value(&face_mask);
            let raw_reference = vk_to_raw_value(&reference);
            ((&*self._fn_table).vkCmdSetStencilReference)(self._handle, raw_face_mask, raw_reference);
        }
    }
    
    pub fn cmd_bind_descriptor_sets(&self, pipeline_bind_point: VkPipelineBindPoint, layout: &VkPipelineLayout, first_set: usize, descriptor_sets: &[VkDescriptorSet], dynamic_offsets: &[usize]) {
        unsafe {
            let raw_pipeline_bind_point = vk_to_raw_value(&pipeline_bind_point);
            let raw_layout = vk_to_raw_value(layout);
            let raw_first_set = vk_to_raw_value(&first_set);
            let raw_descriptor_set_count = descriptor_sets.len() as u32;
            let raw_descriptor_sets = new_ptr_vk_array(descriptor_sets);
            let raw_dynamic_offset_count = dynamic_offsets.len() as u32;
            let raw_dynamic_offsets = new_ptr_vk_array(dynamic_offsets);
            ((&*self._fn_table).vkCmdBindDescriptorSets)(self._handle, raw_pipeline_bind_point, raw_layout, raw_first_set, raw_descriptor_set_count, raw_descriptor_sets, raw_dynamic_offset_count, raw_dynamic_offsets);
            free_ptr(raw_descriptor_sets);
            free_ptr(raw_dynamic_offsets);
        }
    }
    
    pub fn cmd_bind_index_buffer(&self, buffer: &VkBuffer, offset: usize, index_type: VkIndexType) {
        unsafe {
            let raw_buffer = vk_to_raw_value(buffer);
            let raw_offset = vk_to_raw_value(&offset);
            let raw_index_type = vk_to_raw_value(&index_type);
            ((&*self._fn_table).vkCmdBindIndexBuffer)(self._handle, raw_buffer, raw_offset, raw_index_type);
        }
    }
    
    pub fn cmd_bind_vertex_buffers(&self, first_binding: usize, buffers: &[VkBuffer], offsets: &[usize]) {
        unsafe {
            let raw_first_binding = vk_to_raw_value(&first_binding);
            let raw_binding_count = cmp::max(buffers.len(), offsets.len()) as u32;
            let raw_buffers = new_ptr_vk_array(buffers);
            let raw_offsets = new_ptr_vk_array(offsets);
            ((&*self._fn_table).vkCmdBindVertexBuffers)(self._handle, raw_first_binding, raw_binding_count, raw_buffers, raw_offsets);
            free_ptr(raw_buffers);
            free_ptr(raw_offsets);
        }
    }
    
    pub fn cmd_draw(&self, vertex_count: usize, instance_count: usize, first_vertex: usize, first_instance: usize) {
        unsafe {
            let raw_vertex_count = vk_to_raw_value(&vertex_count);
            let raw_instance_count = vk_to_raw_value(&instance_count);
            let raw_first_vertex = vk_to_raw_value(&first_vertex);
            let raw_first_instance = vk_to_raw_value(&first_instance);
            ((&*self._fn_table).vkCmdDraw)(self._handle, raw_vertex_count, raw_instance_count, raw_first_vertex, raw_first_instance);
        }
    }
    
    pub fn cmd_draw_indexed(&self, index_count: usize, instance_count: usize, first_index: usize, vertex_offset: isize, first_instance: usize) {
        unsafe {
            let raw_index_count = vk_to_raw_value(&index_count);
            let raw_instance_count = vk_to_raw_value(&instance_count);
            let raw_first_index = vk_to_raw_value(&first_index);
            let raw_vertex_offset = vk_to_raw_value(&vertex_offset);
            let raw_first_instance = vk_to_raw_value(&first_instance);
            ((&*self._fn_table).vkCmdDrawIndexed)(self._handle, raw_index_count, raw_instance_count, raw_first_index, raw_vertex_offset, raw_first_instance);
        }
    }
    
    pub fn cmd_draw_indirect(&self, buffer: &VkBuffer, offset: usize, draw_count: usize, stride: usize) {
        unsafe {
            let raw_buffer = vk_to_raw_value(buffer);
            let raw_offset = vk_to_raw_value(&offset);
            let raw_draw_count = vk_to_raw_value(&draw_count);
            let raw_stride = vk_to_raw_value(&stride);
            ((&*self._fn_table).vkCmdDrawIndirect)(self._handle, raw_buffer, raw_offset, raw_draw_count, raw_stride);
        }
    }
    
    pub fn cmd_draw_indexed_indirect(&self, buffer: &VkBuffer, offset: usize, draw_count: usize, stride: usize) {
        unsafe {
            let raw_buffer = vk_to_raw_value(buffer);
            let raw_offset = vk_to_raw_value(&offset);
            let raw_draw_count = vk_to_raw_value(&draw_count);
            let raw_stride = vk_to_raw_value(&stride);
            ((&*self._fn_table).vkCmdDrawIndexedIndirect)(self._handle, raw_buffer, raw_offset, raw_draw_count, raw_stride);
        }
    }
    
    pub fn cmd_dispatch(&self, group_count_x: usize, group_count_y: usize, group_count_z: usize) {
        unsafe {
            let raw_group_count_x = vk_to_raw_value(&group_count_x);
            let raw_group_count_y = vk_to_raw_value(&group_count_y);
            let raw_group_count_z = vk_to_raw_value(&group_count_z);
            ((&*self._fn_table).vkCmdDispatch)(self._handle, raw_group_count_x, raw_group_count_y, raw_group_count_z);
        }
    }
    
    pub fn cmd_dispatch_indirect(&self, buffer: &VkBuffer, offset: usize) {
        unsafe {
            let raw_buffer = vk_to_raw_value(buffer);
            let raw_offset = vk_to_raw_value(&offset);
            ((&*self._fn_table).vkCmdDispatchIndirect)(self._handle, raw_buffer, raw_offset);
        }
    }
    
    pub fn cmd_copy_buffer(&self, src_buffer: &VkBuffer, dst_buffer: &VkBuffer, regions: &[VkBufferCopy]) {
        unsafe {
            let raw_src_buffer = vk_to_raw_value(src_buffer);
            let raw_dst_buffer = vk_to_raw_value(dst_buffer);
            let raw_region_count = regions.len() as u32;
            let raw_regions = new_ptr_vk_array(regions);
            ((&*self._fn_table).vkCmdCopyBuffer)(self._handle, raw_src_buffer, raw_dst_buffer, raw_region_count, raw_regions);
            free_vk_ptr_array(raw_region_count as usize, raw_regions);
        }
    }
    
    pub fn cmd_copy_image(&self, src_image: &VkImage, src_image_layout: VkImageLayout, dst_image: &VkImage, dst_image_layout: VkImageLayout, regions: &[VkImageCopy]) {
        unsafe {
            let raw_src_image = vk_to_raw_value(src_image);
            let raw_src_image_layout = vk_to_raw_value(&src_image_layout);
            let raw_dst_image = vk_to_raw_value(dst_image);
            let raw_dst_image_layout = vk_to_raw_value(&dst_image_layout);
            let raw_region_count = regions.len() as u32;
            let raw_regions = new_ptr_vk_array(regions);
            ((&*self._fn_table).vkCmdCopyImage)(self._handle, raw_src_image, raw_src_image_layout, raw_dst_image, raw_dst_image_layout, raw_region_count, raw_regions);
            free_vk_ptr_array(raw_region_count as usize, raw_regions);
        }
    }
    
    pub fn cmd_blit_image(&self, src_image: &VkImage, src_image_layout: VkImageLayout, dst_image: &VkImage, dst_image_layout: VkImageLayout, regions: &[VkImageBlit], filter: VkFilter) {
        unsafe {
            let raw_src_image = vk_to_raw_value(src_image);
            let raw_src_image_layout = vk_to_raw_value(&src_image_layout);
            let raw_dst_image = vk_to_raw_value(dst_image);
            let raw_dst_image_layout = vk_to_raw_value(&dst_image_layout);
            let raw_region_count = regions.len() as u32;
            let raw_regions = new_ptr_vk_array(regions);
            let raw_filter = vk_to_raw_value(&filter);
            ((&*self._fn_table).vkCmdBlitImage)(self._handle, raw_src_image, raw_src_image_layout, raw_dst_image, raw_dst_image_layout, raw_region_count, raw_regions, raw_filter);
            free_vk_ptr_array(raw_region_count as usize, raw_regions);
        }
    }
    
    pub fn cmd_copy_buffer_to_image(&self, src_buffer: &VkBuffer, dst_image: &VkImage, dst_image_layout: VkImageLayout, regions: &[VkBufferImageCopy]) {
        unsafe {
            let raw_src_buffer = vk_to_raw_value(src_buffer);
            let raw_dst_image = vk_to_raw_value(dst_image);
            let raw_dst_image_layout = vk_to_raw_value(&dst_image_layout);
            let raw_region_count = regions.len() as u32;
            let raw_regions = new_ptr_vk_array(regions);
            ((&*self._fn_table).vkCmdCopyBufferToImage)(self._handle, raw_src_buffer, raw_dst_image, raw_dst_image_layout, raw_region_count, raw_regions);
            free_vk_ptr_array(raw_region_count as usize, raw_regions);
        }
    }
    
    pub fn cmd_copy_image_to_buffer(&self, src_image: &VkImage, src_image_layout: VkImageLayout, dst_buffer: &VkBuffer, regions: &[VkBufferImageCopy]) {
        unsafe {
            let raw_src_image = vk_to_raw_value(src_image);
            let raw_src_image_layout = vk_to_raw_value(&src_image_layout);
            let raw_dst_buffer = vk_to_raw_value(dst_buffer);
            let raw_region_count = regions.len() as u32;
            let raw_regions = new_ptr_vk_array(regions);
            ((&*self._fn_table).vkCmdCopyImageToBuffer)(self._handle, raw_src_image, raw_src_image_layout, raw_dst_buffer, raw_region_count, raw_regions);
            free_vk_ptr_array(raw_region_count as usize, raw_regions);
        }
    }
    
    pub fn cmd_update_buffer(&self, dst_buffer: &VkBuffer, dst_offset: usize, data: &[c_void]) {
        unsafe {
            let raw_dst_buffer = vk_to_raw_value(dst_buffer);
            let raw_dst_offset = vk_to_raw_value(&dst_offset);
            let raw_data_size = data.len() as u64;
            let raw_data = data.as_ptr();
            ((&*self._fn_table).vkCmdUpdateBuffer)(self._handle, raw_dst_buffer, raw_dst_offset, raw_data_size, raw_data);
        }
    }
    
    pub fn cmd_fill_buffer(&self, dst_buffer: &VkBuffer, dst_offset: usize, size: usize, data: usize) {
        unsafe {
            let raw_dst_buffer = vk_to_raw_value(dst_buffer);
            let raw_dst_offset = vk_to_raw_value(&dst_offset);
            let raw_size = vk_to_raw_value(&size);
            let raw_data = vk_to_raw_value(&data);
            ((&*self._fn_table).vkCmdFillBuffer)(self._handle, raw_dst_buffer, raw_dst_offset, raw_size, raw_data);
        }
    }
    
    pub fn cmd_clear_color_image(&self, image: &VkImage, image_layout: VkImageLayout, color: &VkClearColorValue, ranges: &[VkImageSubresourceRange]) {
        unsafe {
            let raw_image = vk_to_raw_value(image);
            let raw_image_layout = vk_to_raw_value(&image_layout);
            let raw_color = new_ptr_vk_value(color);
            let raw_range_count = ranges.len() as u32;
            let raw_ranges = new_ptr_vk_array(ranges);
            ((&*self._fn_table).vkCmdClearColorImage)(self._handle, raw_image, raw_image_layout, raw_color, raw_range_count, raw_ranges);
            free_ptr(raw_color);
            free_vk_ptr_array(raw_range_count as usize, raw_ranges);
        }
    }
    
    pub fn cmd_clear_depth_stencil_image(&self, image: &VkImage, image_layout: VkImageLayout, depth_stencil: &VkClearDepthStencilValue, ranges: &[VkImageSubresourceRange]) {
        unsafe {
            let raw_image = vk_to_raw_value(image);
            let raw_image_layout = vk_to_raw_value(&image_layout);
            let raw_depth_stencil = new_ptr_vk_value(depth_stencil);
            let raw_range_count = ranges.len() as u32;
            let raw_ranges = new_ptr_vk_array(ranges);
            ((&*self._fn_table).vkCmdClearDepthStencilImage)(self._handle, raw_image, raw_image_layout, raw_depth_stencil, raw_range_count, raw_ranges);
            free_vk_ptr(raw_depth_stencil);
            free_vk_ptr_array(raw_range_count as usize, raw_ranges);
        }
    }
    
    pub fn cmd_clear_attachments(&self, attachments: &[VkClearAttachment], rects: &[VkClearRect]) {
        unsafe {
            let raw_attachment_count = attachments.len() as u32;
            let raw_attachments = new_ptr_vk_array(attachments);
            let raw_rect_count = rects.len() as u32;
            let raw_rects = new_ptr_vk_array(rects);
            ((&*self._fn_table).vkCmdClearAttachments)(self._handle, raw_attachment_count, raw_attachments, raw_rect_count, raw_rects);
            free_vk_ptr_array(raw_attachment_count as usize, raw_attachments);
            free_vk_ptr_array(raw_rect_count as usize, raw_rects);
        }
    }
    
    pub fn cmd_resolve_image(&self, src_image: &VkImage, src_image_layout: VkImageLayout, dst_image: &VkImage, dst_image_layout: VkImageLayout, regions: &[VkImageResolve]) {
        unsafe {
            let raw_src_image = vk_to_raw_value(src_image);
            let raw_src_image_layout = vk_to_raw_value(&src_image_layout);
            let raw_dst_image = vk_to_raw_value(dst_image);
            let raw_dst_image_layout = vk_to_raw_value(&dst_image_layout);
            let raw_region_count = regions.len() as u32;
            let raw_regions = new_ptr_vk_array(regions);
            ((&*self._fn_table).vkCmdResolveImage)(self._handle, raw_src_image, raw_src_image_layout, raw_dst_image, raw_dst_image_layout, raw_region_count, raw_regions);
            free_vk_ptr_array(raw_region_count as usize, raw_regions);
        }
    }
    
    pub fn cmd_set_event(&self, event: &VkEvent, stage_mask: VkPipelineStageFlags) {
        unsafe {
            let raw_event = vk_to_raw_value(event);
            let raw_stage_mask = vk_to_raw_value(&stage_mask);
            ((&*self._fn_table).vkCmdSetEvent)(self._handle, raw_event, raw_stage_mask);
        }
    }
    
    pub fn cmd_reset_event(&self, event: &VkEvent, stage_mask: VkPipelineStageFlags) {
        unsafe {
            let raw_event = vk_to_raw_value(event);
            let raw_stage_mask = vk_to_raw_value(&stage_mask);
            ((&*self._fn_table).vkCmdResetEvent)(self._handle, raw_event, raw_stage_mask);
        }
    }
    
    pub fn cmd_wait_events(&self, events: &[VkEvent], src_stage_mask: VkPipelineStageFlags, dst_stage_mask: VkPipelineStageFlags, memory_barriers: &[VkMemoryBarrier], buffer_memory_barriers: &[VkBufferMemoryBarrier], image_memory_barriers: &[VkImageMemoryBarrier]) {
        unsafe {
            let raw_event_count = events.len() as u32;
            let raw_events = new_ptr_vk_array(events);
            let raw_src_stage_mask = vk_to_raw_value(&src_stage_mask);
            let raw_dst_stage_mask = vk_to_raw_value(&dst_stage_mask);
            let raw_memory_barrier_count = memory_barriers.len() as u32;
            let raw_memory_barriers = new_ptr_vk_array(memory_barriers);
            let raw_buffer_memory_barrier_count = buffer_memory_barriers.len() as u32;
            let raw_buffer_memory_barriers = new_ptr_vk_array(buffer_memory_barriers);
            let raw_image_memory_barrier_count = image_memory_barriers.len() as u32;
            let raw_image_memory_barriers = new_ptr_vk_array(image_memory_barriers);
            ((&*self._fn_table).vkCmdWaitEvents)(self._handle, raw_event_count, raw_events, raw_src_stage_mask, raw_dst_stage_mask, raw_memory_barrier_count, raw_memory_barriers, raw_buffer_memory_barrier_count, raw_buffer_memory_barriers, raw_image_memory_barrier_count, raw_image_memory_barriers);
            free_ptr(raw_events);
            free_vk_ptr_array(raw_memory_barrier_count as usize, raw_memory_barriers);
            free_vk_ptr_array(raw_buffer_memory_barrier_count as usize, raw_buffer_memory_barriers);
            free_vk_ptr_array(raw_image_memory_barrier_count as usize, raw_image_memory_barriers);
        }
    }
    
    pub fn cmd_pipeline_barrier(&self, src_stage_mask: VkPipelineStageFlags, dst_stage_mask: VkPipelineStageFlags, dependency_flags: VkDependencyFlags, memory_barriers: &[VkMemoryBarrier], buffer_memory_barriers: &[VkBufferMemoryBarrier], image_memory_barriers: &[VkImageMemoryBarrier]) {
        unsafe {
            let raw_src_stage_mask = vk_to_raw_value(&src_stage_mask);
            let raw_dst_stage_mask = vk_to_raw_value(&dst_stage_mask);
            let raw_dependency_flags = vk_to_raw_value(&dependency_flags);
            let raw_memory_barrier_count = memory_barriers.len() as u32;
            let raw_memory_barriers = new_ptr_vk_array(memory_barriers);
            let raw_buffer_memory_barrier_count = buffer_memory_barriers.len() as u32;
            let raw_buffer_memory_barriers = new_ptr_vk_array(buffer_memory_barriers);
            let raw_image_memory_barrier_count = image_memory_barriers.len() as u32;
            let raw_image_memory_barriers = new_ptr_vk_array(image_memory_barriers);
            ((&*self._fn_table).vkCmdPipelineBarrier)(self._handle, raw_src_stage_mask, raw_dst_stage_mask, raw_dependency_flags, raw_memory_barrier_count, raw_memory_barriers, raw_buffer_memory_barrier_count, raw_buffer_memory_barriers, raw_image_memory_barrier_count, raw_image_memory_barriers);
            free_vk_ptr_array(raw_memory_barrier_count as usize, raw_memory_barriers);
            free_vk_ptr_array(raw_buffer_memory_barrier_count as usize, raw_buffer_memory_barriers);
            free_vk_ptr_array(raw_image_memory_barrier_count as usize, raw_image_memory_barriers);
        }
    }
    
    pub fn cmd_begin_query(&self, query_pool: &VkQueryPool, query: usize, flags: VkQueryControlFlags) {
        unsafe {
            let raw_query_pool = vk_to_raw_value(query_pool);
            let raw_query = vk_to_raw_value(&query);
            let raw_flags = vk_to_raw_value(&flags);
            ((&*self._fn_table).vkCmdBeginQuery)(self._handle, raw_query_pool, raw_query, raw_flags);
        }
    }
    
    pub fn cmd_end_query(&self, query_pool: &VkQueryPool, query: usize) {
        unsafe {
            let raw_query_pool = vk_to_raw_value(query_pool);
            let raw_query = vk_to_raw_value(&query);
            ((&*self._fn_table).vkCmdEndQuery)(self._handle, raw_query_pool, raw_query);
        }
    }
    
    pub fn cmd_reset_query_pool(&self, query_pool: &VkQueryPool, first_query: usize, query_count: usize) {
        unsafe {
            let raw_query_pool = vk_to_raw_value(query_pool);
            let raw_first_query = vk_to_raw_value(&first_query);
            let raw_query_count = vk_to_raw_value(&query_count);
            ((&*self._fn_table).vkCmdResetQueryPool)(self._handle, raw_query_pool, raw_first_query, raw_query_count);
        }
    }
    
    pub fn cmd_write_timestamp(&self, pipeline_stage: VkPipelineStageFlags, query_pool: &VkQueryPool, query: usize) {
        unsafe {
            let raw_pipeline_stage = vk_to_raw_value(&pipeline_stage);
            let raw_query_pool = vk_to_raw_value(query_pool);
            let raw_query = vk_to_raw_value(&query);
            ((&*self._fn_table).vkCmdWriteTimestamp)(self._handle, raw_pipeline_stage, raw_query_pool, raw_query);
        }
    }
    
    pub fn cmd_copy_query_pool_results(&self, query_pool: &VkQueryPool, first_query: usize, query_count: usize, dst_buffer: &VkBuffer, dst_offset: usize, stride: usize, flags: VkQueryResultFlags) {
        unsafe {
            let raw_query_pool = vk_to_raw_value(query_pool);
            let raw_first_query = vk_to_raw_value(&first_query);
            let raw_query_count = vk_to_raw_value(&query_count);
            let raw_dst_buffer = vk_to_raw_value(dst_buffer);
            let raw_dst_offset = vk_to_raw_value(&dst_offset);
            let raw_stride = vk_to_raw_value(&stride);
            let raw_flags = vk_to_raw_value(&flags);
            ((&*self._fn_table).vkCmdCopyQueryPoolResults)(self._handle, raw_query_pool, raw_first_query, raw_query_count, raw_dst_buffer, raw_dst_offset, raw_stride, raw_flags);
        }
    }
    
    pub fn cmd_push_constants(&self, layout: &VkPipelineLayout, stage_flags: VkShaderStageFlags, offset: usize, values: &[c_void]) {
        unsafe {
            let raw_layout = vk_to_raw_value(layout);
            let raw_stage_flags = vk_to_raw_value(&stage_flags);
            let raw_offset = vk_to_raw_value(&offset);
            let raw_size = values.len() as u32;
            let raw_values = values.as_ptr();
            ((&*self._fn_table).vkCmdPushConstants)(self._handle, raw_layout, raw_stage_flags, raw_offset, raw_size, raw_values);
        }
    }
    
    pub fn cmd_begin_render_pass(&self, render_pass_begin: &VkRenderPassBeginInfo, contents: VkSubpassContents) {
        unsafe {
            let raw_render_pass_begin = new_ptr_vk_value(render_pass_begin);
            let raw_contents = vk_to_raw_value(&contents);
            ((&*self._fn_table).vkCmdBeginRenderPass)(self._handle, raw_render_pass_begin, raw_contents);
            free_vk_ptr(raw_render_pass_begin);
        }
    }
    
    pub fn cmd_next_subpass(&self, contents: VkSubpassContents) {
        unsafe {
            let raw_contents = vk_to_raw_value(&contents);
            ((&*self._fn_table).vkCmdNextSubpass)(self._handle, raw_contents);
        }
    }
    
    pub fn cmd_end_render_pass(&self) {
        unsafe {
            ((&*self._fn_table).vkCmdEndRenderPass)(self._handle);
        }
    }
    
    pub fn cmd_execute_commands(&self, command_buffers: &[VkCommandBuffer]) {
        unsafe {
            let raw_command_buffer_count = command_buffers.len() as u32;
            let raw_command_buffers = new_ptr_vk_array(command_buffers);
            ((&*self._fn_table).vkCmdExecuteCommands)(self._handle, raw_command_buffer_count, raw_command_buffers);
            free_ptr(raw_command_buffers);
        }
    }
    
    pub fn cmd_set_device_mask(&self, device_mask: u32) {
        unsafe {
            let raw_device_mask = device_mask;
            ((&*self._fn_table).vkCmdSetDeviceMask)(self._handle, raw_device_mask);
        }
    }
    
    pub fn cmd_dispatch_base(&self, base_group_x: usize, base_group_y: usize, base_group_z: usize, group_count_x: usize, group_count_y: usize, group_count_z: usize) {
        unsafe {
            let raw_base_group_x = vk_to_raw_value(&base_group_x);
            let raw_base_group_y = vk_to_raw_value(&base_group_y);
            let raw_base_group_z = vk_to_raw_value(&base_group_z);
            let raw_group_count_x = vk_to_raw_value(&group_count_x);
            let raw_group_count_y = vk_to_raw_value(&group_count_y);
            let raw_group_count_z = vk_to_raw_value(&group_count_z);
            ((&*self._fn_table).vkCmdDispatchBase)(self._handle, raw_base_group_x, raw_base_group_y, raw_base_group_z, raw_group_count_x, raw_group_count_y, raw_group_count_z);
        }
    }
    
    pub fn cmd_push_descriptor_set(&self, pipeline_bind_point: VkPipelineBindPoint, layout: &VkPipelineLayout, set: usize, descriptor_writes: &[VkWriteDescriptorSet]) {
        unsafe {
            let raw_pipeline_bind_point = vk_to_raw_value(&pipeline_bind_point);
            let raw_layout = vk_to_raw_value(layout);
            let raw_set = vk_to_raw_value(&set);
            let raw_descriptor_write_count = descriptor_writes.len() as u32;
            let raw_descriptor_writes = new_ptr_vk_array(descriptor_writes);
            ((&*self._fn_table).vkCmdPushDescriptorSetKHR)(self._handle, raw_pipeline_bind_point, raw_layout, raw_set, raw_descriptor_write_count, raw_descriptor_writes);
            free_vk_ptr_array(raw_descriptor_write_count as usize, raw_descriptor_writes);
        }
    }
    
    pub fn cmd_push_descriptor_set_with_template(&self, descriptor_update_template: &VkDescriptorUpdateTemplate, layout: &VkPipelineLayout, set: usize, data: &c_void) {
        unsafe {
            let raw_descriptor_update_template = vk_to_raw_value(descriptor_update_template);
            let raw_layout = vk_to_raw_value(layout);
            let raw_set = vk_to_raw_value(&set);
            let raw_data = data as *const c_void;
            ((&*self._fn_table).vkCmdPushDescriptorSetWithTemplateKHR)(self._handle, raw_descriptor_update_template, raw_layout, raw_set, raw_data);
        }
    }
    
    pub fn cmd_begin_render_pass_2(&self, render_pass_begin: &VkRenderPassBeginInfo, subpass_begin_info: &khr::VkSubpassBeginInfo) {
        unsafe {
            let raw_render_pass_begin = new_ptr_vk_value(render_pass_begin);
            let raw_subpass_begin_info = new_ptr_vk_value(subpass_begin_info);
            ((&*self._fn_table).vkCmdBeginRenderPass2KHR)(self._handle, raw_render_pass_begin, raw_subpass_begin_info);
            free_vk_ptr(raw_render_pass_begin);
            free_vk_ptr(raw_subpass_begin_info);
        }
    }
    
    pub fn cmd_next_subpass_2(&self, subpass_begin_info: &khr::VkSubpassBeginInfo, subpass_end_info: &khr::VkSubpassEndInfo) {
        unsafe {
            let raw_subpass_begin_info = new_ptr_vk_value(subpass_begin_info);
            let raw_subpass_end_info = new_ptr_vk_value(subpass_end_info);
            ((&*self._fn_table).vkCmdNextSubpass2KHR)(self._handle, raw_subpass_begin_info, raw_subpass_end_info);
            free_vk_ptr(raw_subpass_begin_info);
            free_vk_ptr(raw_subpass_end_info);
        }
    }
    
    pub fn cmd_end_render_pass_2(&self, subpass_end_info: &khr::VkSubpassEndInfo) {
        unsafe {
            let raw_subpass_end_info = new_ptr_vk_value(subpass_end_info);
            ((&*self._fn_table).vkCmdEndRenderPass2KHR)(self._handle, raw_subpass_end_info);
            free_vk_ptr(raw_subpass_end_info);
        }
    }
    
    pub fn cmd_draw_indirect_count(&self, buffer: &VkBuffer, offset: usize, count_buffer: &VkBuffer, count_buffer_offset: usize, max_draw_count: usize, stride: usize) {
        unsafe {
            let raw_buffer = vk_to_raw_value(buffer);
            let raw_offset = vk_to_raw_value(&offset);
            let raw_count_buffer = vk_to_raw_value(count_buffer);
            let raw_count_buffer_offset = vk_to_raw_value(&count_buffer_offset);
            let raw_max_draw_count = vk_to_raw_value(&max_draw_count);
            let raw_stride = vk_to_raw_value(&stride);
            ((&*self._fn_table).vkCmdDrawIndirectCountKHR)(self._handle, raw_buffer, raw_offset, raw_count_buffer, raw_count_buffer_offset, raw_max_draw_count, raw_stride);
        }
    }
    
    pub fn cmd_draw_indexed_indirect_count(&self, buffer: &VkBuffer, offset: usize, count_buffer: &VkBuffer, count_buffer_offset: usize, max_draw_count: usize, stride: usize) {
        unsafe {
            let raw_buffer = vk_to_raw_value(buffer);
            let raw_offset = vk_to_raw_value(&offset);
            let raw_count_buffer = vk_to_raw_value(count_buffer);
            let raw_count_buffer_offset = vk_to_raw_value(&count_buffer_offset);
            let raw_max_draw_count = vk_to_raw_value(&max_draw_count);
            let raw_stride = vk_to_raw_value(&stride);
            ((&*self._fn_table).vkCmdDrawIndexedIndirectCountKHR)(self._handle, raw_buffer, raw_offset, raw_count_buffer, raw_count_buffer_offset, raw_max_draw_count, raw_stride);
        }
    }
    
    pub fn cmd_debug_marker_begin(&self, marker_info: &ext::VkDebugMarkerMarkerInfo) {
        unsafe {
            let raw_marker_info = new_ptr_vk_value(marker_info);
            ((&*self._fn_table).vkCmdDebugMarkerBeginEXT)(self._handle, raw_marker_info);
            free_vk_ptr(raw_marker_info);
        }
    }
    
    pub fn cmd_debug_marker_end(&self) {
        unsafe {
            ((&*self._fn_table).vkCmdDebugMarkerEndEXT)(self._handle);
        }
    }
    
    pub fn cmd_debug_marker_insert(&self, marker_info: &ext::VkDebugMarkerMarkerInfo) {
        unsafe {
            let raw_marker_info = new_ptr_vk_value(marker_info);
            ((&*self._fn_table).vkCmdDebugMarkerInsertEXT)(self._handle, raw_marker_info);
            free_vk_ptr(raw_marker_info);
        }
    }
    
    pub fn cmd_draw_indirect_count_amd(&self, buffer: &VkBuffer, offset: usize, count_buffer: &VkBuffer, count_buffer_offset: usize, max_draw_count: usize, stride: usize) {
        unsafe {
            let raw_buffer = vk_to_raw_value(buffer);
            let raw_offset = vk_to_raw_value(&offset);
            let raw_count_buffer = vk_to_raw_value(count_buffer);
            let raw_count_buffer_offset = vk_to_raw_value(&count_buffer_offset);
            let raw_max_draw_count = vk_to_raw_value(&max_draw_count);
            let raw_stride = vk_to_raw_value(&stride);
            ((&*self._fn_table).vkCmdDrawIndirectCountAMD)(self._handle, raw_buffer, raw_offset, raw_count_buffer, raw_count_buffer_offset, raw_max_draw_count, raw_stride);
        }
    }
    
    pub fn cmd_draw_indexed_indirect_count_amd(&self, buffer: &VkBuffer, offset: usize, count_buffer: &VkBuffer, count_buffer_offset: usize, max_draw_count: usize, stride: usize) {
        unsafe {
            let raw_buffer = vk_to_raw_value(buffer);
            let raw_offset = vk_to_raw_value(&offset);
            let raw_count_buffer = vk_to_raw_value(count_buffer);
            let raw_count_buffer_offset = vk_to_raw_value(&count_buffer_offset);
            let raw_max_draw_count = vk_to_raw_value(&max_draw_count);
            let raw_stride = vk_to_raw_value(&stride);
            ((&*self._fn_table).vkCmdDrawIndexedIndirectCountAMD)(self._handle, raw_buffer, raw_offset, raw_count_buffer, raw_count_buffer_offset, raw_max_draw_count, raw_stride);
        }
    }
    
    pub fn cmd_begin_conditional_rendering(&self, conditional_rendering_begin: &ext::VkConditionalRenderingBeginInfo) {
        unsafe {
            let raw_conditional_rendering_begin = new_ptr_vk_value(conditional_rendering_begin);
            ((&*self._fn_table).vkCmdBeginConditionalRenderingEXT)(self._handle, raw_conditional_rendering_begin);
            free_vk_ptr(raw_conditional_rendering_begin);
        }
    }
    
    pub fn cmd_end_conditional_rendering(&self) {
        unsafe {
            ((&*self._fn_table).vkCmdEndConditionalRenderingEXT)(self._handle);
        }
    }
    
    pub fn cmd_process_commands(&self, process_commands_info: &nvx::VkCmdProcessCommandsInfo) {
        unsafe {
            let raw_process_commands_info = new_ptr_vk_value(process_commands_info);
            ((&*self._fn_table).vkCmdProcessCommandsNVX)(self._handle, raw_process_commands_info);
            free_vk_ptr(raw_process_commands_info);
        }
    }
    
    pub fn cmd_reserve_space_for_commands(&self, reserve_space_info: &nvx::VkCmdReserveSpaceForCommandsInfo) {
        unsafe {
            let raw_reserve_space_info = new_ptr_vk_value(reserve_space_info);
            ((&*self._fn_table).vkCmdReserveSpaceForCommandsNVX)(self._handle, raw_reserve_space_info);
            free_vk_ptr(raw_reserve_space_info);
        }
    }
    
    pub fn cmd_set_viewport_wscaling(&self, first_viewport: usize, viewport_wscalings: &[nv::VkViewportWScaling]) {
        unsafe {
            let raw_first_viewport = vk_to_raw_value(&first_viewport);
            let raw_viewport_count = viewport_wscalings.len() as u32;
            let raw_viewport_wscalings = new_ptr_vk_array(viewport_wscalings);
            ((&*self._fn_table).vkCmdSetViewportWScalingNV)(self._handle, raw_first_viewport, raw_viewport_count, raw_viewport_wscalings);
            free_vk_ptr_array(raw_viewport_count as usize, raw_viewport_wscalings);
        }
    }
    
    pub fn cmd_set_discard_rectangle(&self, first_discard_rectangle: usize, discard_rectangles: &[VkRect2D]) {
        unsafe {
            let raw_first_discard_rectangle = vk_to_raw_value(&first_discard_rectangle);
            let raw_discard_rectangle_count = discard_rectangles.len() as u32;
            let raw_discard_rectangles = new_ptr_vk_array(discard_rectangles);
            ((&*self._fn_table).vkCmdSetDiscardRectangleEXT)(self._handle, raw_first_discard_rectangle, raw_discard_rectangle_count, raw_discard_rectangles);
            free_vk_ptr_array(raw_discard_rectangle_count as usize, raw_discard_rectangles);
        }
    }
    
    pub fn cmd_begin_debug_utils_label(&self, label_info: &ext::VkDebugUtilsLabel) {
        unsafe {
            let raw_label_info = new_ptr_vk_value(label_info);
            ((&*self._fn_table).vkCmdBeginDebugUtilsLabelEXT)(self._handle, raw_label_info);
            free_vk_ptr(raw_label_info);
        }
    }
    
    pub fn cmd_end_debug_utils_label(&self) {
        unsafe {
            ((&*self._fn_table).vkCmdEndDebugUtilsLabelEXT)(self._handle);
        }
    }
    
    pub fn cmd_insert_debug_utils_label(&self, label_info: &ext::VkDebugUtilsLabel) {
        unsafe {
            let raw_label_info = new_ptr_vk_value(label_info);
            ((&*self._fn_table).vkCmdInsertDebugUtilsLabelEXT)(self._handle, raw_label_info);
            free_vk_ptr(raw_label_info);
        }
    }
    
    pub fn cmd_set_sample_locations(&self, sample_locations_info: &ext::VkSampleLocationsInfo) {
        unsafe {
            let raw_sample_locations_info = new_ptr_vk_value(sample_locations_info);
            ((&*self._fn_table).vkCmdSetSampleLocationsEXT)(self._handle, raw_sample_locations_info);
            free_vk_ptr(raw_sample_locations_info);
        }
    }
    
    pub fn cmd_write_buffer_marker(&self, pipeline_stage: VkPipelineStageFlags, dst_buffer: &VkBuffer, dst_offset: usize, marker: usize) {
        unsafe {
            let raw_pipeline_stage = vk_to_raw_value(&pipeline_stage);
            let raw_dst_buffer = vk_to_raw_value(dst_buffer);
            let raw_dst_offset = vk_to_raw_value(&dst_offset);
            let raw_marker = vk_to_raw_value(&marker);
            ((&*self._fn_table).vkCmdWriteBufferMarkerAMD)(self._handle, raw_pipeline_stage, raw_dst_buffer, raw_dst_offset, raw_marker);
        }
    }
    
    pub fn cmd_set_checkpoint(&self, checkpoint_marker: &c_void) {
        unsafe {
            let raw_checkpoint_marker = checkpoint_marker as *const c_void;
            ((&*self._fn_table).vkCmdSetCheckpointNV)(self._handle, raw_checkpoint_marker);
        }
    }
}