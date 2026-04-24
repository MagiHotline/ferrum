use std::{ffi::c_void, io::{Error, ErrorKind}, ptr::NonNull};

use glam::Mat3;
use objc2::{MainThreadMarker, MainThreadOnly, rc::Retained, runtime::ProtocolObject};
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use objc2_foundation::{NSArray, NSError, NSMutableArray, NSUInteger};
use objc2_metal::{MTL4ArgumentTable, MTL4ArgumentTableDescriptor, MTL4CommandAllocator, MTL4CommandAllocatorDescriptor, MTL4CommandBuffer, MTL4CommandQueue, MTLBuffer, MTLCreateSystemDefaultDevice, MTLDevice, MTLGPUFamily, MTLLibrary, MTLResidencySet, MTLResidencySetDescriptor, MTLResourceOptions, MTLSharedEvent, MTLViewport};
use objc2_metal_kit::{MTKView, MTKViewDelegate};

const K_FRAMES_IN_FLIGHT: u8 = 3;

pub struct MTL4Renderer {
    device: Retained<ProtocolObject<dyn MTLDevice>>,
    arg_table: Retained<ProtocolObject<dyn MTL4ArgumentTable>>,
    res_set: Retained<ProtocolObject<dyn MTLResidencySet>>,
    cmd_q: Retained<ProtocolObject<dyn MTL4CommandQueue>>,
    cmd_buffer: Retained<ProtocolObject<dyn MTL4CommandBuffer>>,
    buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    def_lib: Retained<ProtocolObject<dyn MTLLibrary>>,
    view: Retained<MTKView>,
    viewport_size_buffer:Retained<ProtocolObject<dyn MTLBuffer>>
}

impl MTL4Renderer {

    pub fn new() -> Self {
        let device =
            MTLCreateSystemDefaultDevice()
                .expect("Failed to create the device. You might not be on MacOs.");


        if !device.supportsFamily(MTLGPUFamily::Metal4) {
            panic!("Your Mac does not support Metal 4.");
        }

       let cmd_q =
           device.newMTL4CommandQueue().expect("Failed to create the Command queue");

       let cmd_buffer =
           device.newCommandBuffer().expect("Failed to create the Command buffer");

       // The default library will contain the project's shader
       let def_lib =
           device.newDefaultLibrary().expect("Failed to create the Default Library");

       let frame_rect = CGRect::new(
           CGPoint { x: 0.0, y: 0.0 },
           CGSize { width: 800.0, height: 600.0 });

       let view = MTKView::initWithFrame_device(
           MTKView::alloc(MainThreadMarker::new().unwrap()),
           frame_rect,
           Some(&device));

       // ESSENTIAL RESOURCES
       let buffer = Self::create_triangle_buffer(&device);
       let arg_table = Self::create_argument_table(&device).unwrap();
       let res_set = Self::make_residency_set(&device).unwrap();

       let viewport = MTLViewport {
            originX: 0.0,
            originY: 0.0,
            width: 800.0,
            height: 600.0,
            znear: 0.0,
            zfar: 0.0
       };

       let viewport_size_buffer =
           device.newBufferWithLength_options(size_of_val(&viewport), MTLResourceOptions::StorageModeManaged)
               .expect("Failed to create the viewport buffer");



        MTL4Renderer {
            device,
            arg_table,
            res_set,
            cmd_q,
            cmd_buffer,
            buffer,
            def_lib,
            view,
            viewport_size_buffer
        }
    }

    pub fn create_triangle_buffer(device: &Retained<ProtocolObject<dyn MTLDevice>>)
    -> Retained<ProtocolObject<dyn MTLBuffer>> {
        let triangle_vert = Mat3::from_cols_array(&[
            -0.5, -0.5, 0.0,
             0.5, -0.5, 0.0,
             0.0,  0.5, 0.0,
        ]);

        let length = size_of_val(&triangle_vert);
        let pointer = NonNull::new(
            &triangle_vert as *const Mat3 as *mut c_void)
                .expect("Pointer to vertex data is null");


        let triangle_vertex_buff = unsafe {
            device.newBufferWithBytes_length_options(
                pointer,
                length,
                MTLResourceOptions::StorageModeShared
            )
        }.expect("Unable to create the vertex buffer.");

        triangle_vertex_buff
    }

    pub fn create_argument_table(
        device: &Retained<ProtocolObject<dyn MTLDevice>>
    )
    -> Result<Retained<ProtocolObject<dyn MTL4ArgumentTable>>, Retained<NSError>> {

        let argument_table_descriptor =
            MTL4ArgumentTableDescriptor::new();

        argument_table_descriptor.setMaxBufferBindCount(2);
        let argument_table =
            device.newArgumentTableWithDescriptor_error(&argument_table_descriptor);

        argument_table
    }

    pub fn make_residency_set(device: &Retained<ProtocolObject<dyn MTLDevice>>) ->
        Result<Retained<ProtocolObject<dyn MTLResidencySet>>, Retained<NSError>> {

        let res_set_descriptor = MTLResidencySetDescriptor::new();

        let residency_set =
            device.newResidencySetWithDescriptor_error(&res_set_descriptor);

        residency_set
    }

    pub fn make_cmd_allocators(&mut self) {
        self.device.newCommandAllocator().expect("Failed to create command allocator");
    }

    pub fn render(&mut self) {

    }
}
