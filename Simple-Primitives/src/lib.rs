#![allow(non_snake_case)]

extern crate js_sys;
extern crate mat4;
extern crate wasm_bindgen;
extern crate web_sys;
use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    EventTarget, MouseEvent, WebGlProgram, WebGlRenderingContext, HtmlCanvasElement,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::f32::consts::PI;

#[allow(dead_code)]
mod utils;
pub mod shapes;
use utils::{compile_shader, link_program, log};
use shapes::{
    generate_geometry, export_shape
};

// Returns the canvas element with the id canvas.
pub fn get_canvas() -> Result<HtmlCanvasElement, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    Ok(canvas)
}

// Returns the webgl context from the canvas
pub fn get_webgl_context() -> Result<WebGlRenderingContext, JsValue> {
    let canvas = get_canvas().unwrap();
    
    let gl = canvas
    .get_context("webgl")?
    .unwrap()
    .dyn_into::<WebGlRenderingContext>()?;

    Ok(gl)
}

// Exports the shape as OBJ format.
#[wasm_bindgen]
pub fn _export_shape(shapeIndex: usize, scale: Vec<f32>, args: Vec<f32>) -> String {
    return export_shape(shapeIndex, scale, args);
}

// The webclient struct is responsible for drawing the generated geomtry on the html canvas. It uses a simple shader where the triangles have distinc colors so that the user can differentiate them from each other.
#[wasm_bindgen]
pub struct WebClient {
    nr_of_vertices: usize,
    
    // Html related fields
    gl: WebGlRenderingContext,
    canvas: HtmlCanvasElement,

    // Shader related fields
    shaderProgram: WebGlProgram,
    location_modelViewMatrix:  Result<web_sys::WebGlUniformLocation, String>,
    location_projectionMatrix: Result<web_sys::WebGlUniformLocation, String>,

    // Rotation related fields
    drag: Rc<RefCell<bool>>,
    theta: Rc<RefCell<f32>>,
    phi: Rc<RefCell<f32>>,
    dX: Rc<RefCell<f32>>,
    dY: Rc<RefCell<f32>>,
    canvas_width: Rc<RefCell<f32>>,
    canvas_height: Rc<RefCell<f32>>,
}

#[wasm_bindgen]
impl WebClient {
    // To be able to use this struct we first need to initialize it.
    pub fn new() -> WebClient {
        let nr_of_vertices = 0;
        let gl = get_webgl_context().unwrap();
        let canvas = get_canvas().unwrap();

        // Vertex shader program
        let vsSource = 
        r#"
        attribute vec4 aVertexPosition;
        attribute vec3 aVertexColor;
        
        uniform mat4 uModelViewMatrix;
        uniform mat4 uProjectionMatrix;
        
        varying vec3 vColor;
        
        void main(void) {
            gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
            vColor = aVertexColor;
        }
        "#;
        
        // Fragment shader program
        let fsSource = 
        r#"
        precision mediump float;
        varying vec3 vColor;
        
        void main() {
            gl_FragColor = vec4(vColor, 1.0);
        }
        "#;
        
        // We need to compile the shader to be able to make a webgl program.
        let v_shader = compile_shader(&gl, WebGlRenderingContext::VERTEX_SHADER, vsSource);
        let f_shader = compile_shader(&gl, WebGlRenderingContext::FRAGMENT_SHADER, fsSource);
        
        // Linking and making the program.
        let shaderProgram = link_program(&gl, &v_shader.unwrap(), &f_shader.unwrap()).unwrap();

        // Tell webgl to use our webglprogram
        gl.use_program(Some(&shaderProgram));

        // We only want to see one side of the triangle for a greater performance
        gl.enable(WebGlRenderingContext::CULL_FACE);
        gl.cull_face(WebGlRenderingContext::BACK);

        // Retrieve the matrix locations so we can set these during run time.
        let location_projectionMatrix = gl
            .get_uniform_location(&shaderProgram, "uProjectionMatrix")
            .ok_or_else(|| String::from("cannot get uProjectionMatrix"));
        let location_modelViewMatrix = gl
            .get_uniform_location(&shaderProgram, "uModelViewMatrix")
            .ok_or_else(|| String::from("cannot get uModelViewMatrix"));

        // Here we declare some refcell so we can acces this from different parts of our program.
        let drag = Rc::new(RefCell::new(false));
        let theta = Rc::new(RefCell::new(-0.785398));
        let phi = Rc::new(RefCell::new(0.392699));
        let dX = Rc::new(RefCell::new(0.0));
        let dY = Rc::new(RefCell::new(0.0));
        let canvas_width = Rc::new(RefCell::new(canvas.client_width() as f32));
        let canvas_height = Rc::new(RefCell::new(canvas.client_height() as f32));

        log(&"==== WebClient new() ====");

        WebClient { 
            nr_of_vertices, 

            gl,
            canvas,
            shaderProgram, 

            location_modelViewMatrix, 
            location_projectionMatrix,

            drag,
            theta,
            phi,
            dX,
            dY,
            canvas_width,
            canvas_height,
        }
    }

    // Here we generate the geometry and push it to de gpu and the shader.
    pub fn generate(
        &mut self, 
        shape_index: usize, 
        scale: Vec<f32>, 
        args: Vec<f32>
    ) 
        -> Result<(), JsValue> 
        {
        // Creating the raw data we need.
        let shape_data = generate_geometry(shape_index, scale, args);
        let nr_of_triangles = shape_data.nr_of_triangles;
        self.nr_of_vertices = nr_of_triangles * 3;
        let vertices = shape_data.vertices;

        // We also need to make a color array to tell webgl which color eacht vertice is.
        let mut colors: Vec<f32> = Vec::new();
        
        // We switch between light and dark gray
        for _ in (0..nr_of_triangles).step_by(2) {
            let light_gray: Vec<f32> = vec![0.75294, 0.75294, 0.75294];
            let dark_gray: Vec<f32> = vec![0.50196, 0.50196, 0.50196];
            
            colors.extend(&light_gray);
            colors.extend(&light_gray);
            colors.extend(&light_gray);
            
            colors.extend(&dark_gray);
            colors.extend(&dark_gray);
            colors.extend(&dark_gray);
        }

        // Here's where we call the routine that builds all the
        // Objects we'll be drawing.
        self.init_buffers(vertices, colors).unwrap();

        log(&"==== WebClient generate() ====");

        Ok(())
    }

    pub fn initCallBacks(&mut self) {        
        // Get canvas as event target
        let event_target: EventTarget = self.canvas.clone().into();
        
        // Add event listeners
        // MOUSEDOWN
        {
            let drag = self.drag.clone();
            let mousedown_cb = Closure::wrap(Box::new(move |_event: MouseEvent| {
                *drag.borrow_mut() = true;
            }) as Box<dyn FnMut(MouseEvent)>);
            event_target
            .add_event_listener_with_callback("mousedown", mousedown_cb.as_ref().unchecked_ref())
            .unwrap();
            mousedown_cb.forget();
        }
        // MOUSEUP and MOUSEOUT
        {
            let drag = self.drag.clone();
            let mouseup_cb = Closure::wrap(Box::new(move |_event: MouseEvent| {
                *drag.borrow_mut() = false;
            }) as Box<dyn FnMut(MouseEvent)>);
            event_target
            .add_event_listener_with_callback("mouseup", mouseup_cb.as_ref().unchecked_ref())
            .unwrap();
            event_target
            .add_event_listener_with_callback("mouseout", mouseup_cb.as_ref().unchecked_ref())
            .unwrap();
            mouseup_cb.forget();
        }
        // MOUSEMOVE
        {
            let theta = self.theta.clone();
            let phi = self.phi.clone();
            let canvas_width = self.canvas_width.clone();
            let canvas_height = self.canvas_height.clone();
            let dX = self.dX.clone();
            let dY = self.dY.clone();
            let drag = self.drag.clone();
            let mousemove_cb = Closure::wrap(Box::new(move |event: MouseEvent| {
                if *drag.borrow() {
                    let cw = *canvas_width.borrow();
                    let ch = *canvas_height.borrow();
                    let factor = 0.25; // to reduce the scrollspeed
                    *dX.borrow_mut() = (event.movement_x() as f32) * 2.0 * PI / cw * factor; // dX is in radians
                    *dY.borrow_mut() = (event.movement_y() as f32) * 2.0 * PI / ch * factor; // dY is in radians
                    *theta.borrow_mut() += *dX.borrow();
                    *phi.borrow_mut() += *dY.borrow();
                    
                    // Numbers in radians.
                    if *phi.borrow() > 1.5708 {
                        *phi.borrow_mut() = 1.5708;
                    }
                    else if *phi.borrow() < -1.5708 {
                        *phi.borrow_mut() = -1.5708;
                    }
                }
            }) as Box<dyn FnMut(web_sys::MouseEvent)>);
            event_target
            .add_event_listener_with_callback("mousemove", mousemove_cb.as_ref().unchecked_ref())
            .unwrap();
            mousemove_cb.forget();
        }

        log(&"==== WebClient initCallBacks() ====");
    }
    
    #[allow(non_snake_case)]
    fn init_buffers(
        &self, 
        vertices: Vec<f32>, 
        colors: Vec<f32>, 
    ) -> Result<(), JsValue> {
        // ==== VERTICES
        
        // Create a buffer for the vertex positions.
        let verticesBuffer = self.gl
            .create_buffer()
            .ok_or("failed to create positionBuffer buffer")?;
        
        // Select the verticesBuffer as the one to apply buffer
        // operations to from here out.
        self.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&verticesBuffer));
        
        let vertices_array = float_32_array!(vertices);
        
        // Now pass the list of vetices into WebGL to build the
        // shape. We do this by creating a Float32Array from the
        // Rust array, then use it to fill the current buffer.
        self.gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertices_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
        
        // Tell WebGL how to pull out the positions from the position buffer into the vertexPosition attribute
        {
            let vertexPosition = self.gl.get_attrib_location(&self.shaderProgram, "aVertexPosition") as u32;
            let numComponents = 3;
            let type_ = WebGlRenderingContext::FLOAT;
            let normalize = false;
            let stride = 0;
            let offset = 0;
            self.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&verticesBuffer));
            
            self.gl.vertex_attrib_pointer_with_i32(
                vertexPosition,
                numComponents,
                type_,
                normalize,
                stride,
                offset,
            );
            self.gl.enable_vertex_attrib_array(vertexPosition);
        }
        
        // ==== COLORS
        
        // Create a buffer for the color positions.
        let colorBuffer = self.gl
            .create_buffer()
            .ok_or("failed to create colorBuffer buffer")?;
        
        // Select the colorBuffer as the one to apply buffer
        // operations to from here out.
        self.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&colorBuffer));
        
        let colors_array = float_32_array!(colors);
        
        // Now pass the list of colors into WebGL to build the
        // shape. We do this by creating a Float32Array from the
        // Rust array, then use it to fill the current buffer.
        self.gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &colors_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
        
        // Tell WebGL how to pull out the positions from the color buffer into the vertexColor attribute
        {
            let vertexColor = self.gl.get_attrib_location(&self.shaderProgram, "aVertexColor") as u32;
            let numComponents = 3;
            let type_ = WebGlRenderingContext::FLOAT;
            let normalize = false;
            let stride = 0;
            let offset = 0;
            self.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&colorBuffer));
            self.gl.vertex_attrib_pointer_with_i32(
                vertexColor,
                numComponents,
                type_,
                normalize,
                stride,
                offset,
            );
            self.gl.enable_vertex_attrib_array(vertexColor);
        }

        log(&"==== WebClient initBuffers() ====");
        
        Ok(())
    }

    // This function checks if the user is rotating the shape. We only want te redraw the scene if the shape gets rotated.
    pub fn drawSceneIf(&self) {
        if *self.drag.borrow() {
            self.drawScene().unwrap();
        }
    }
    
    // Draws the scene on the canvas.
    pub fn drawScene(&self) -> Result<(), JsValue> {
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear_depth(1.0); // Clear everything
        self.gl.enable(WebGlRenderingContext::DEPTH_TEST);
        
        // Clear the canvas before we start drawing on it.
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);
        
        // Set the drawing position to the "identity" point, which is
        // the center of the scene.
        let mut modelViewMatrix = mat4::new_identity();
        
        // Now move the drawing position a bit to where we want to
        // start drawing the square.
        let mat_to_translate = modelViewMatrix.clone();
        mat4::translate(
            &mut modelViewMatrix, // destination matrix
            &mat_to_translate,    // matrix to translate
            &[-0.0, 0.0, -6.0],
        ); // amount to translate
        
        let mat_to_rotate = modelViewMatrix.clone();
        mat4::rotate_x(
            &mut modelViewMatrix, // destination matrix
            &mat_to_rotate,       // matrix to rotate
            &*self.phi.borrow(),
        );
        let mat_to_rotate = modelViewMatrix.clone();
        mat4::rotate_y(
            &mut modelViewMatrix, // destination matrix
            &mat_to_rotate,       // matrix to rotate
            &*self.theta.borrow(),
        );

        let location_modelViewMatrix = self.location_modelViewMatrix.clone();     

        // Create a perspective matrix, a special matrix that is
        // used to simulate the distortion of perspective in a camera.
        // Our field of view is 45 degrees, with a width/height
        // ratio that matches the display size of the canvas
        // and we only want to see objects between 0.1 units
        // and 100 units away from the camera.
        self.gl.viewport(0, 0, self.canvas.width() as i32, self.canvas.height() as i32);
        let fieldOfView = 45.0 * PI / 180.0; // in radians
        let aspect: f32 = self.canvas.width() as f32 / self.canvas.height() as f32;
        let zNear = 1.0;
        let zFar = 100.0;
        let mut projectionMatrix = mat4::new_zero();
        
        mat4::perspective(&mut projectionMatrix, &fieldOfView, &aspect, &zNear, &zFar);
        
        let location_projectionMatrix = self.location_projectionMatrix.clone();

        // Set the shader uniforms
        self.gl.uniform_matrix4fv_with_f32_array(
            Some(&location_projectionMatrix?),
            false,
            &projectionMatrix,
        );
        self.gl.uniform_matrix4fv_with_f32_array(
            Some(&location_modelViewMatrix?),
            false, 
            &modelViewMatrix
        );
        
        // Draw the triangles
        self.gl.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            self.nr_of_vertices as i32,
        );

        log(&"==== WebClient drawScene() ====");

        Ok(())
    }
}