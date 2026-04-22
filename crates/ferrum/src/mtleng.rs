use std::{ffi::c_void, fs, ops::Deref, ptr::NonNull};
use glam::Mat3;
use glfw::{GlfwReceiver, PWindow, WindowEvent, WindowMode};
use objc2_app_kit::NSWindow;
use objc2_foundation::{NSError, NSString};
use objc2_metal::{MTLBuffer, MTLCommandQueue, MTLCompileOptions, MTLCreateSystemDefaultDevice, MTLDevice, MTLLibrary, MTLPixelFormat, MTLRenderPipelineDescriptor, MTLRenderPipelineState, MTLResourceOptions};
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
    metal_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    metal_cmdq: Retained<ProtocolObject<dyn MTLCommandQueue>>,
    metal_renderer_pso: Result<Retained<ProtocolObject<dyn MTLRenderPipelineState>>, Retained<NSError>>
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

        // Compile all metal files
        let entries = fs::read_dir("crates/ferrum/src/shmet")
                .expect("Could not find shader directory");

        let mut combined_source = String::new();
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("metal") {
                    let content = fs::read_to_string(&path)
                        .expect(&format!("Could not read shader file: {:?}", path));

                    combined_source.push_str(&content);
                    combined_source.push_str("\n");
                }
            }
        }

        let source_ns = NSString::from_str(&combined_source);
        let options = MTLCompileOptions::new();
        let metal_library =
            device.newLibraryWithSource_options_error(&source_ns, Some(&options))
                .expect("Failed to compile Metal shaders at runtime");

        let metal_cmdq =
            device.newCommandQueue()
            .expect("Failed to create command queue");

        let metal_renderer_pso =
            MTLEngine::create_render_pipeline(&metal_library, &metal_layer, &device);

        Self
        {
            glfw,
            events,
            device,
            glfw_window,
            metal_window,
            metal_layer,
            metal_library,
            metal_buffer,
            metal_cmdq,
            metal_renderer_pso
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

    pub fn create_render_pipeline(
        metal_library: &Retained<ProtocolObject<dyn MTLLibrary>>,
        metal_layer: &Retained<CAMetalLayer>,
        device: &Retained<ProtocolObject<dyn MTLDevice>>
    )
    -> Result<Retained<ProtocolObject<dyn MTLRenderPipelineState>>, Retained<NSError>>
    {
        let vertex_shader =
            metal_library.newFunctionWithName(
                &NSString::from_str("vertexShader")
        ).expect("Failed to get vertex shader");
        let fragment_shader =
            metal_library.newFunctionWithName(
                &NSString::from_str("fragmentShader")
        ).expect("Failed to get fragment shader");

        let renderer_pipeline_descriptor =
            MTLRenderPipelineDescriptor::new();

        // Load the descriptor with label and vertex + fragment function
        renderer_pipeline_descriptor.setLabel(
            Some(&NSString::from_str("Triangle Rendering Pipeline"))
        );
        renderer_pipeline_descriptor.setVertexFunction(
            Some(&vertex_shader)
        );
        renderer_pipeline_descriptor.setFragmentFunction(
            Some(&fragment_shader)
        );

        unsafe {
            renderer_pipeline_descriptor
            .colorAttachments()
            .objectAtIndexedSubscript(0)
            .setPixelFormat(metal_layer.pixelFormat())
        };

        // we only need one renderer pipeline state for now, but we can create more
        let metal_renderer_pso =
            device.newRenderPipelineStateWithDescriptor_error(&renderer_pipeline_descriptor);

        metal_renderer_pso
    }

}
