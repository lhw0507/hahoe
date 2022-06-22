use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};
use rand::*;

fn SetRectangle(program : &WebGlProgram, context : &WebGl2RenderingContext, width : u32, height : u32){

    let width_ratio = 2. / width as f32;
    let height_ratio = 2./ height as f32;

    let start_width_ratio = -1.;
    let start_height_ratio = -1.;

    for x in 0..width
    {
        for y in 0..height
        {
            let x1 =  start_width_ratio + x as f32 * width_ratio;
            let x2 = start_width_ratio + (x + 1) as f32 * width_ratio;
            let y1 = start_height_ratio + y as f32* height_ratio;
            let y2 = start_height_ratio + (y + 1) as f32 * height_ratio;
            
            let vertices = [
              x1, y1, 0.0, 
              x2, y1, 0.0,
              x1, y2, 0.0,
              x1, y2, 0.0, 
              x2, y1, 0.0,
              x2, y2, 0.0
              ];

           
            unsafe {

                let positions_array_buf_view = js_sys::Float32Array::view(&vertices);
        
                context.buffer_data_with_array_buffer_view(
                    WebGl2RenderingContext::ARRAY_BUFFER,
                    &positions_array_buf_view,
                    WebGl2RenderingContext::STATIC_DRAW,
                );
                
                let color_uniform_location = context.get_uniform_location(&program, "u_color");
                
                
                //context.uniform4f(color_uniform_location.as_ref(), 0.0, js_sys::Math::random() as f32, 0.0, 1.0);
                context.uniform4f(color_uniform_location.as_ref(), 0.0, rand::random::<f32>(), 0.0, 1.0);
                
                //context.uniform4f(color_uniform_location.as_ref(), rand::random(), rand::random(), rand::random(), 1.0);
            let vert_count = (vertices.len() / 3) as i32;
            draw(&context, vert_count);
            }
        }
    }
    
}


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r##"#version 300 es
 
        in vec4 position;

        void main() {
        
            gl_Position = position;
        }
        "##,
    )?;

    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r##"#version 300 es
    
        precision highp float;

        uniform vec4 u_color;

        out vec4 outColor;

        void main() {
            outColor = u_color;
        }
        "##,
    )?;

    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));
    
    let position_attribute_location = context.get_attrib_location(&program, "position");
    

    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")?;
    context.bind_vertex_array(Some(&vao));
   
    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    



    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));


    context.enable_vertex_attrib_array(position_attribute_location as u32);
    context.vertex_attrib_pointer_with_i32(position_attribute_location as u32, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);


    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    SetRectangle(&program, &context, 400, 300);
    
    Ok(())
}

fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
    

    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
