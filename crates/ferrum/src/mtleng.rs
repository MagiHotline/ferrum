use std::{ffi::c_void, fs, ptr::{NonNull, null_mut}};

use glam::Mat3;
use glfw::{GlfwReceiver, PWindow, WindowEvent, WindowMode};
use objc2_app_kit::NSWindow;
use objc2_foundation::NSString;
use objc2_metal::{MTLBuffer, MTLCompileOptions, MTLCreateSystemDefaultDevice, MTLDevice, MTLLibrary, MTLPixelFormat, MTLResourceOptions};
use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_quartz_core::CAMetalLayer;
// Only OBJ-C objects need Retained

pub struct MTLEngine {
    glfw: glfw::Glfw,
    events: GlfwReceiver<(f64, WindowEvent)>,
    device: Retained<ProtocolObject<dyn MTLDevice>>,
    glfw_window: PWindow,
    metal_window: Retained<NSWindow>,
    metal_layer: Retained<CAMetalLayer>,
    metal_library: Retained<ProtocolObject<dyn MTLLibrary>>,
    metal_buffer: Retained<ProtocolObject<dyn MTLBuffer>>
}

impl MTLEngine {

    pub fn new() -> Self {
        let mut glfw = glfw::init_no_callbacks().expect("Failed to init glfw");
        // Tell GLFW to not create OPENGL graphics context
        glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi);
        // HARDCODED WIDHT AND LENGTH FOR NOW
        let (glfw_window, events) = glfw.create_window(
            800, 600,
            "Metal Window",
            WindowMode::Windowed
        ).expect("Failed to create window.");

        let device =
            MTLCreateSystemDefaultDevice()
            .expect("Failed to find a Metal device.");

        let metal_window = unsafe {
                Retained::retain(glfw::Window::get_cocoa_window(&glfw_window) as *mut NSWindow)
                    .expect("Unable to find Objc handle")
        };

        // Link device and layer to the window
        let metal_layer = CAMetalLayer::new();
        metal_layer.setDevice(Some(&device));
        metal_layer.setPixelFormat(MTLPixelFormat::BGRA8Unorm);
        metal_window.contentView().unwrap().setLayer(Some(&metal_layer));
        metal_window.contentView().unwrap().setWantsLayer(true);

        // Create the object
        let metal_buffer =
            MTLEngine::create_triangle(&device);

        let shader_source = include_str!("shmet/triangle.metal");
        let source_ns = NSString::from_str(&shader_source);
        let options = MTLCompileOptions::new();
        let metal_library =
            device.newLibraryWithSource_options_error(&source_ns, Some(&options))
                .expect("Failed to compile Metal shaders at runtime");

        Self
        {
            glfw,
            events,
            device,
            glfw_window,
            metal_window,
            metal_layer,
            metal_library,
            metal_buffer
        }
    }

    pub fn run(&mut self) {
        while !glfw::Window::should_close(&self.glfw_window) {
            self.glfw.poll_events();
        }
    }

    pub fn create_triangle(device: &Retained<ProtocolObject<dyn MTLDevice>>) -> Retained<ProtocolObject<dyn MTLBuffer>> {

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

}
