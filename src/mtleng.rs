use std::{ffi::c_void, fs, ptr::NonNull};
use glam::Mat3;
use glfw::{GlfwReceiver, PWindow, WindowEvent, WindowMode};
use objc2_app_kit::NSWindow;
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use objc2_foundation::{NSError, NSString};
use objc2_metal::{MTLBuffer, MTLCommandBuffer, MTLCommandEncoder, MTLCommandQueue, MTLCompileOptions,
    MTLCreateSystemDefaultDevice, MTLDevice, MTLLibrary, MTLLoadAction, MTLPixelFormat, MTLPrimitiveType,
    MTLRenderCommandEncoder, MTLRenderPassDescriptor, MTLRenderPipelineDescriptor, MTLRenderPipelineState,
    MTLResourceOptions, MTLStoreAction, MTLViewport};
use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_quartz_core::{CAAutoresizingMask, CAMetalDrawable, CAMetalLayer};

pub enum WindowSize {
    Fullscreen,
    Windowed
}


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
    metal_renderer_pso: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    metal_drawable: Retained<ProtocolObject<dyn CAMetalDrawable>>
}

impl MTLEngine {

    pub fn new(
        width: u32,
        height: u32,
        title: &str,
        window_mode: WindowSize
    ) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).expect("Failed to init glfw");
        // Tell GLFW to not create OPENGL graphics context
        glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi);
        let (mut glfw_window, events) =
            match window_mode {
                WindowSize::Fullscreen => glfw.with_primary_monitor(|glfw, m| {
                    glfw.create_window(
                        width,
                        height,
                        title,
                        m.map_or(glfw::WindowMode::Windowed, |m| {
                            glfw::WindowMode::FullScreen(m)
                        }),
                    )
                    .expect("Failed to create window")
                }),
                WindowSize::Windowed =>
                glfw.create_window(width, height, title, WindowMode::Windowed)
                    .expect("Failed to create window")
            };


        glfw_window.set_framebuffer_size_polling(true);

        let (width_screen, height_height) = glfw_window.get_framebuffer_size();

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

        metal_layer.setDrawableSize(CGSize::new(width_screen as f64, height_height as f64));
        metal_window.contentView().unwrap().setLayer(Some(&metal_layer));
        metal_window.contentView().unwrap().setWantsLayer(true);

        // Create the object
        let metal_buffer =
            MTLEngine::create_triangle(&device);

        // Compile all metal files
        let entries = fs::read_dir("src/shmet")
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
            MTLEngine::create_render_pipeline(&metal_library, &metal_layer, &device)
                .expect("Failed to create render pipeline");

        let metal_drawable: Retained<ProtocolObject<dyn CAMetalDrawable>>
            = metal_layer.nextDrawable().expect("Failed to get drawable");

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
            metal_renderer_pso,
            metal_drawable
        }
    }

    pub fn run(&mut self) {
        while !glfw::Window::should_close(&self.glfw_window) {

            // poll events first
            self.glfw.poll_events();

            // handle events loop
            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                        self.glfw_window.set_should_close(true);
                    }
                    glfw::WindowEvent::FramebufferSize(width, height) => {
                        self.metal_layer.setDrawableSize(
                            CGSize { width: width as f64, height: height as f64 }
                        );
                    }
                    _ => {}
                }
            }

            // drawing
            // FIX: REZING THE WINDOW DOES NOT DRAW THE TRIANGLE ONCE AGAIN
            self.metal_drawable = self.metal_layer.nextDrawable().expect("Failed to get drawable");
            MTLEngine::send_render_command(self);
        }
    }

    /// Sends the render commands to the GPU and draws it on the window
    pub fn send_render_command(&mut self) {

        let metal_cmd_buffer =
            self.metal_cmdq.commandBuffer().expect("Failed to get command buffer");

        let render_pass_descriptor = MTLRenderPassDescriptor::new();
        let cd = unsafe {
            render_pass_descriptor
                .colorAttachments()
                .objectAtIndexedSubscript(0)
        };

        cd.setTexture(Some(&self.metal_drawable.texture()));
        cd.setLoadAction(MTLLoadAction::Clear);
        cd.setStoreAction(MTLStoreAction::Store);

        let render_cmd_encoder =
            metal_cmd_buffer.renderCommandEncoderWithDescriptor(&render_pass_descriptor)
                .expect("failed to create the render command encoder.");
        MTLEngine::encode_render_command(self, &render_cmd_encoder);
        render_cmd_encoder.endEncoding();

        metal_cmd_buffer.presentDrawable(self.metal_drawable.as_ref());
        metal_cmd_buffer.commit(); // Send commands to the GPU
        metal_cmd_buffer.waitUntilCompleted();
    }

    /// Encodes the rendering commands to send to the GPU
    pub fn encode_render_command(
        &mut self,
        render_cmd_encoder: &Retained<ProtocolObject<dyn MTLRenderCommandEncoder>>,
    ) {
        unsafe {
            render_cmd_encoder.setRenderPipelineState(&self.metal_renderer_pso);
            // render_cmd_encoder.setViewport??
            render_cmd_encoder.setVertexBuffer_offset_atIndex(Some(&self.metal_buffer), 0, 0);
        }
        let (width, height) = self.glfw_window.get_framebuffer_size();
        let viewport = MTLViewport {
            originX: 0.0,
            originY: 0.0,
            width: width as f64,
            height: height as f64,
            znear: 0.0,
            zfar: 1.0,
        };
        render_cmd_encoder.setViewport(viewport);
        let type_triangle = MTLPrimitiveType::Triangle;
        let vtx_start = 0;
        let vtx_count = 3;
        unsafe {
            render_cmd_encoder.drawPrimitives_vertexStart_vertexCount(
            type_triangle, vtx_start, vtx_count
        );
        }
    }

    // Creates Vertex Buffer for a Triangle
    pub fn create_triangle
    (
        device: &Retained<ProtocolObject<dyn MTLDevice>>
    )
    -> Retained<ProtocolObject<dyn MTLBuffer>>
    {
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

    /// Creates a renderer pipeline with default configuration
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
