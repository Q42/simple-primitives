/* ==== Structure ====

    This file consists of two main public fuctions.
    - Generate
    - Export

    The generate function returns a ShapeData enum which holds all the vertices and the amount of triangles.
    The export function returns a string which contains all shapedata in OBJ format.

    Every shape in this file has two functions. One that returns the amount of triangles and the other one that returns a list with all the vertices which makes up the shapes.
    In every second function you see groups of three temp_vec.push( ... ); lines. These push a single vertice in the temporary list. (x, y, z)

    Every shape has it's own custom options. There are passed to the shape functions via the args list. Each shape has the following options:
    - Plane         - Subdivisions (usize)
    - Disk          - Radius (f32), Sides (usize)
    - Cube          - Subdivisions (usize)
    - Sphere        - Subdivisions (usize)
    - cylinder      - Radius (f32), Sides (usize)
    - Tube          - Sides (usize), InnerRadius (f32), OuterRadius (f32)

    The order of the vertices is important as it is the definition of which way the face of the triangle is visible.

  =================== */ 

use wasm_bindgen::prelude::*;
use std::f32::consts::PI;

// WebGL needs vertices and how many triangles it's going to draw so this struct is passed back to the webCLient struct.
pub struct ShapeData {
    pub vertices: Vec<f32>,
    pub nr_of_triangles: usize,
}

// The shape enum.
#[wasm_bindgen]
pub enum Shape {
    Plane = 0,
    Disk = 1,
    Cube = 2,
    Sphere = 3,
    Cylinder = 4,
    Tube = 5,
}

// We export and generate function get the shape as an index but we want to be able to convert it to an enum.
#[wasm_bindgen]
pub fn usize2Shape(value: usize) -> Shape {
    match value {
        0 => Shape::Plane,
        1 => Shape::Disk,
        2 => Shape::Cube,
        3 => Shape::Sphere,
        4 => Shape::Cylinder,
        5 => Shape::Tube,
        _ => panic!(),
    }
}

// Returns the shape data as a string in OBJ format.
#[wasm_bindgen]
pub fn export_shape(shapeIndex: usize, scale: Vec<f32>, args: Vec<f32>) -> String {
    let mut data = String::new();

    let shape_data = generate_geometry(shapeIndex, scale, args);

    for i in (0..shape_data.vertices.len()).step_by(3) {
        data.push_str("v ");
        data.push_str(&shape_data.vertices[i    ].to_string());
        data.push_str(" ");
        data.push_str(&shape_data.vertices[i + 1].to_string());
        data.push_str(" ");
        data.push_str(&shape_data.vertices[i + 2].to_string());
        data.push_str("\n");
    }

    for i in (0..(shape_data.vertices.len() as f32 / 3.0) as i32).step_by(3) {
        data.push_str("f ");
        data.push_str(&(i + 1).to_string());
        data.push_str(" ");
        data.push_str(&(i + 2).to_string());
        data.push_str(" ");
        data.push_str(&(i + 3).to_string());
        data.push_str("\n");
    }

    data.into()
}

// Generate the geometry and returns the data as a "ShapeData" struct.
pub fn generate_geometry(shape_index: usize, scale: Vec<f32>, args: Vec<f32>) -> ShapeData {
    // Lets define the shape first so whe now what we need to generate. We also want to declare the variables that going to hold the definition of the shape.
    let shape: Shape = usize2Shape(shape_index);
    let mut vertices: Vec<f32> = Vec::new();
    let mut nr_of_triangles = 0;

    // Here we filter the selected shape and call the right functions to make the shape.
    if let Shape::Plane = shape {
        vertices = plane(args[0] as usize);
        nr_of_triangles = nr_of_plane_triangles(args[0] as usize);
    }
    else if let Shape::Disk = shape {
        vertices = disk(args[0] as usize, args[1] as f32);
        nr_of_triangles = nr_of_disk_triangles(args[0] as usize);
    }
    else if let Shape::Cube = shape {
        vertices = cube(args[0] as usize);
        nr_of_triangles = nr_of_cube_triangles(args[0] as usize);
    }
    else if let Shape::Sphere = shape {
        vertices = sphere(args[0] as usize);
        nr_of_triangles = nr_of_sphere_triangles(args[0] as usize);
    }
    else if let Shape::Cylinder = shape {
        vertices = cylinder(args[0] as usize, args[1] as f32);
        nr_of_triangles = nr_of_cylinder_triangles(args[0] as usize);
    }
    else if let Shape::Tube = shape {
        vertices = tube(args[0] as usize, args[1] as f32, args[2] as f32);
        nr_of_triangles = nr_of_tube_triangles(args[0] as usize);
    }

    // This is the scale part. We want to check first if we actually need to apply scaling if the set scale differs from x: 1, y: 1, z: 1
    if !((scale[0] == 1.0) && (scale[1] == 1.0) && (scale[2] == 1.0)) {
        // If the user wants a different scale we simply multiply the scale vector with the vertice component wise.
        for i in (0..vertices.len()).step_by(3) {
            vertices[i    ] *= scale[0];
            vertices[i + 1] *= scale[1];
            vertices[i + 2] *= scale[2];
        }
    }

    // Return all the info the webclient struct needs.
    ShapeData {
        vertices,
        nr_of_triangles,
    }
}

// ==== PLANE

// Returns the amount of triangles the wanted plane consists off.
fn nr_of_plane_triangles(subdivisions: usize) -> usize {
    return (subdivisions * subdivisions) * 2;
}

// Returns a Vec<f32> with all the vertices. Every trio of f32's forms a vector3.
fn plane(subdivisions: usize) -> Vec<f32> {
    let mut temp_vec: Vec<f32> = Vec::new();
    let step = 1.0 / subdivisions as f32;

    // In each iteration a quad is added to the list.
    for y in 0..subdivisions {
        for x in 0..subdivisions {
            temp_vec.push(step * x as f32 - 0.5);
            temp_vec.push(step * y as f32 - 0.5 + step);
            temp_vec.push(0.0);

            temp_vec.push(step * x as f32 - 0.5);
            temp_vec.push(step * y as f32 - 0.5);
            temp_vec.push(0.0);

            temp_vec.push(step * x as f32 - 0.5 + step);
            temp_vec.push(step * y as f32 - 0.5 + step);
            temp_vec.push(0.0);

            temp_vec.push(step * x as f32 - 0.5 + step);
            temp_vec.push(step * y as f32 - 0.5 + step);
            temp_vec.push(0.0);

            temp_vec.push(step * x as f32 - 0.5);
            temp_vec.push(step * y as f32 - 0.5);
            temp_vec.push(0.0);

            temp_vec.push(step * x as f32 - 0.5 + step);
            temp_vec.push(step * y as f32 - 0.5);
            temp_vec.push(0.0);
        }
    }

    return temp_vec;
}

// ==== DISK

// Returns the amount of triangles the wanted disk consists off.
fn nr_of_disk_triangles(sides: usize) -> usize {
    return sides;    
}

// Returns a Vec<f32> with all the vertices. Every trio of f32's forms a vector3.
fn disk(sides: usize, radius: f32) -> Vec<f32> {
    let mut temp_vec: Vec<f32> = Vec::new();
    let step = 2.0 * PI / sides as f32;

    for i in 0..sides {
        temp_vec.push((i       as f32 * step).cos() * radius);
        temp_vec.push((i       as f32 * step).sin() * radius);
        temp_vec.push(0.0                                   );
            
        temp_vec.push(((i + 1) as f32 * step).cos() * radius);
        temp_vec.push(((i + 1) as f32 * step).sin() * radius);
        temp_vec.push(0.0                                   );
            
        temp_vec.push(0.0                                   );
        temp_vec.push(0.0                                   );
        temp_vec.push(0.0                                   );
    }

    return temp_vec;
}

// ==== CUBE

// Returns the amount of triangles the wanted cube consists off.
fn nr_of_cube_triangles(subdivisions: usize) -> usize {
    return nr_of_plane_triangles(subdivisions) * 6;
}

// Returns a Vec<f32> with all the vertices. Every trio of f32's forms a vector3.
fn cube(subdivisions: usize) -> Vec<f32> {
    let mut temp_vec: Vec<f32> = Vec::new();
    let step = 1.0 / subdivisions as f32;

    // For each side we generat a plane, so six times in total.

    for y in 0..subdivisions {
        for x in 0..subdivisions {            
            temp_vec.push(step * x as f32 - 0.5);
            temp_vec.push(step * y as f32 - 0.5 + step);
            temp_vec.push(0.5);

            temp_vec.push(step * x as f32 - 0.5);
            temp_vec.push(step * y as f32 - 0.5);
            temp_vec.push(0.5);

            temp_vec.push(step * x as f32 - 0.5 + step);
            temp_vec.push(step * y as f32 - 0.5 + step);
            temp_vec.push(0.5);
            
            temp_vec.push(step * x as f32 - 0.5 + step);
            temp_vec.push(step * y as f32 - 0.5 + step);
            temp_vec.push(0.5);

            temp_vec.push(step * x as f32 - 0.5);
            temp_vec.push(step * y as f32 - 0.5);
            temp_vec.push(0.5);

            temp_vec.push(step * x as f32 - 0.5 + step);
            temp_vec.push(step * y as f32 - 0.5);
            temp_vec.push(0.5);
        }
    }

    for y in 0..subdivisions {
        for x in 0..subdivisions {
            temp_vec.push(step * x as f32 - 0.5);
            temp_vec.push(step * y as f32 - 0.5);
            temp_vec.push(-0.5);

            temp_vec.push(step * x as f32 - 0.5);
            temp_vec.push(step * y as f32 - 0.5 + step);
            temp_vec.push(-0.5);

            temp_vec.push(step * x as f32 - 0.5 + step);
            temp_vec.push(step * y as f32 - 0.5 + step);
            temp_vec.push(-0.5);

            temp_vec.push(step * x as f32 - 0.5);
            temp_vec.push(step * y as f32 - 0.5);
            temp_vec.push(-0.5);

            temp_vec.push(step * x as f32 - 0.5 + step);
            temp_vec.push(step * y as f32 - 0.5 + step);
            temp_vec.push(-0.5);

            temp_vec.push(step * x as f32 - 0.5 + step);
            temp_vec.push(step * y as f32 - 0.5);
            temp_vec.push(-0.5);
        }
    }

    for y in 0..subdivisions {
        for x in 0..subdivisions {
            temp_vec.push(step * x as f32 - 0.5);
            temp_vec.push(0.5);
            temp_vec.push(step * y as f32 - 0.5);

            temp_vec.push(step * x as f32 - 0.5);
            temp_vec.push(0.5);
            temp_vec.push(step * y as f32 - 0.5 + step);

            temp_vec.push(step * x as f32 - 0.5 + step);
            temp_vec.push(0.5);
            temp_vec.push(step * y as f32 - 0.5 + step);

            temp_vec.push(step * x as f32 - 0.5);
            temp_vec.push(0.5);
            temp_vec.push(step * y as f32 - 0.5);

            temp_vec.push(step * x as f32 - 0.5 + step);
            temp_vec.push(0.5);
            temp_vec.push(step * y as f32 - 0.5 + step);


            temp_vec.push(step * x as f32 - 0.5 + step);
            temp_vec.push(0.5);
            temp_vec.push(step * y as f32 - 0.5);
        }
    }

    for y in 0..subdivisions {
        for x in 0..subdivisions {
            temp_vec.push(step * x as f32 - 0.5 + step);
            temp_vec.push(-0.5);
            temp_vec.push(step * y as f32 - 0.5 + step);

            temp_vec.push(step * x as f32 - 0.5);
            temp_vec.push(-0.5);
            temp_vec.push(step * y as f32 - 0.5 + step);

            temp_vec.push(step * x as f32 - 0.5);
            temp_vec.push(-0.5);
            temp_vec.push(step * y as f32 - 0.5);

            temp_vec.push(step * x as f32 - 0.5 + step);
            temp_vec.push(-0.5);
            temp_vec.push(step * y as f32 - 0.5);

            temp_vec.push(step * x as f32 - 0.5 + step);
            temp_vec.push(-0.5);
            temp_vec.push(step * y as f32 - 0.5 + step);

            temp_vec.push(step * x as f32 - 0.5);
            temp_vec.push(-0.5);
            temp_vec.push(step * y as f32 - 0.5);
        }
    }

    for y in 0..subdivisions {
        for x in 0..subdivisions {
            temp_vec.push(0.5);
            temp_vec.push(step * x as f32 - 0.5 + step);
            temp_vec.push(step * y as f32 - 0.5 + step);

            temp_vec.push(0.5);
            temp_vec.push(step * x as f32 - 0.5);
            temp_vec.push(step * y as f32 - 0.5 + step);

            temp_vec.push(0.5);
            temp_vec.push(step * x as f32 - 0.5);
            temp_vec.push(step * y as f32 - 0.5);

            temp_vec.push(0.5);
            temp_vec.push(step * x as f32 - 0.5 + step);
            temp_vec.push(step * y as f32 - 0.5);

            temp_vec.push(0.5);
            temp_vec.push(step * x as f32 - 0.5 + step);
            temp_vec.push(step * y as f32 - 0.5 + step);

            temp_vec.push(0.5);
            temp_vec.push(step * x as f32 - 0.5);
            temp_vec.push(step * y as f32 - 0.5);
        }
    }

    for y in 0..subdivisions {
        for x in 0..subdivisions {
            temp_vec.push(-0.5);
            temp_vec.push(step * x as f32 - 0.5);
            temp_vec.push(step * y as f32 - 0.5 + step);

            temp_vec.push(-0.5);
            temp_vec.push(step * x as f32 - 0.5 + step);
            temp_vec.push(step * y as f32 - 0.5 + step);

            temp_vec.push(-0.5);
            temp_vec.push(step * x as f32 - 0.5);
            temp_vec.push(step * y as f32 - 0.5);

            temp_vec.push(-0.5);
            temp_vec.push(step * x as f32 - 0.5 + step);
            temp_vec.push(step * y as f32 - 0.5 + step);

            temp_vec.push(-0.5);
            temp_vec.push(step * x as f32 - 0.5 + step);
            temp_vec.push(step * y as f32 - 0.5);

            temp_vec.push(-0.5);
            temp_vec.push(step * x as f32 - 0.5);
            temp_vec.push(step * y as f32 - 0.5);
        }
    }

    return temp_vec;
}

// ==== SPHERE

// Returns the amount of triangles the wanted sphere consists off.
fn nr_of_sphere_triangles(subdivisions: usize) -> usize {
    return nr_of_cube_triangles(subdivisions);
}

// Returns a Vec<f32> with all the vertices. Every trio of f32's forms a vector3.
fn sphere(subdivisions: usize) -> Vec<f32> {
    let mut temp_vec = cube(subdivisions);
    let mut length: f32;

    // To generate a sphere we simply normalize all the vertices of a cube. Length = Sqrt(x^2 + y^2 + z^2) -> x / Length, y / Length, z / Length
    for i in (0..temp_vec.len()).step_by(3) {
        length = 
        (
            (temp_vec[i    ] * temp_vec[i    ]) + 
            (temp_vec[i + 1] * temp_vec[i + 1]) + 
            (temp_vec[i + 2] * temp_vec[i + 2])
        ).sqrt();
            
        temp_vec[i    ] = temp_vec[i    ] / length;
        temp_vec[i + 1] = temp_vec[i + 1] / length;
        temp_vec[i + 2] = temp_vec[i + 2] / length;
    }

    return temp_vec;
}

// ==== CYLINDER

// Returns the amount of triangles the wanted cylinder consists off.
fn nr_of_cylinder_triangles(sides: usize) -> usize {
    return (nr_of_disk_triangles(sides) * 2) + (sides * 2);
}

// Returns a Vec<f32> with all the vertices. Every trio of f32's forms a vector3.
fn cylinder(sides: usize, radius: f32) -> Vec<f32> {
    let mut temp_vec: Vec<f32> = Vec::new();
    let step = 2.0 * PI / sides as f32;

    // Top
    for i in 0..sides {
        temp_vec.push(((i + 1) as f32 * step).cos() * radius);
        temp_vec.push(0.5                                   );
        temp_vec.push(((i + 1) as f32 * step).sin() * radius);
            
        temp_vec.push((i       as f32 * step).cos() * radius);
        temp_vec.push(0.5                                   );
        temp_vec.push((i       as f32 * step).sin() * radius);
            
        temp_vec.push(0.0                                   );
        temp_vec.push(0.5                                   );
        temp_vec.push(0.0                                   );
    }

    // Outside ring
    for i in 0..sides {
        temp_vec.push((i       as f32 * step).cos() * radius);
        temp_vec.push(0.5                                   );
        temp_vec.push((i       as f32 * step).sin() * radius);
            
        temp_vec.push(((i + 1) as f32 * step).cos() * radius);
        temp_vec.push(0.5                                   );
        temp_vec.push(((i + 1) as f32 * step).sin() * radius);

        temp_vec.push((i       as f32 * step).cos() * radius);
        temp_vec.push(-0.5                                  );
        temp_vec.push((i       as f32 * step).sin() * radius);

        temp_vec.push(((i + 1) as f32 * step).cos() * radius);
        temp_vec.push(-0.5                                  );
        temp_vec.push(((i + 1) as f32 * step).sin() * radius);

        temp_vec.push((i       as f32 * step).cos() * radius);
        temp_vec.push(-0.5                                  );
        temp_vec.push((i       as f32 * step).sin() * radius);

        temp_vec.push(((i + 1) as f32 * step).cos() * radius);
        temp_vec.push(0.5                                   );
        temp_vec.push(((i + 1) as f32 * step).sin() * radius);
    }

    // Bottom
    for i in 0..sides {
        temp_vec.push((i       as f32 * step).cos() * radius);
        temp_vec.push(-0.5                                  );
        temp_vec.push((i       as f32 * step).sin() * radius);

        temp_vec.push(((i + 1) as f32 * step).cos() * radius);
        temp_vec.push(-0.5                                  );
        temp_vec.push(((i + 1) as f32 * step).sin() * radius);
            
        temp_vec.push(0.0                                   );
        temp_vec.push(-0.5                                  );
        temp_vec.push(0.0                                   );
    }

    return temp_vec;
}

// ==== TUBE

// Returns the amount of triangles the wanted tube consists off.
fn nr_of_tube_triangles(sides: usize) -> usize {
    return sides * 8;
}

// Returns a Vec<f32> with all the vertices. Every trio of f32's forms a vector3.
fn tube(sides: usize, inner_radius: f32, outer_radius: f32) -> Vec<f32> {
    let mut temp_vec: Vec<f32> = Vec::new();
    let step = 2.0 * PI / sides as f32;

    // top ring
    for i in 0..sides {
        temp_vec.push((i       as f32 * step).cos() * inner_radius);
        temp_vec.push(0.5                                         );
        temp_vec.push((i       as f32 * step).sin() * inner_radius);
        
        temp_vec.push(((i + 1) as f32 * step).cos() * inner_radius);
        temp_vec.push(0.5                                         );
        temp_vec.push(((i + 1) as f32 * step).sin() * inner_radius);
            
        temp_vec.push((i       as f32 * step).cos() * outer_radius);
        temp_vec.push(0.5                                         );
        temp_vec.push((i       as f32 * step).sin() * outer_radius);
        
        temp_vec.push((i       as f32 * step).cos() * outer_radius);
        temp_vec.push(0.5                                         );
        temp_vec.push((i       as f32 * step).sin() * outer_radius);

        temp_vec.push(((i + 1) as f32 * step).cos() * inner_radius);
        temp_vec.push(0.5                                         );
        temp_vec.push(((i + 1) as f32 * step).sin() * inner_radius);

        temp_vec.push(((i + 1) as f32 * step).cos() * outer_radius);
        temp_vec.push(0.5                                         );
        temp_vec.push(((i + 1) as f32 * step).sin() * outer_radius);
    }

    // outer cylinder
    for i in 0..sides {
        temp_vec.push((i       as f32 * step).cos() * outer_radius);
        temp_vec.push(0.5                                         );
        temp_vec.push((i       as f32 * step).sin() * outer_radius);
            
        temp_vec.push(((i + 1) as f32 * step).cos() * outer_radius);
        temp_vec.push(0.5                                         );
        temp_vec.push(((i + 1) as f32 * step).sin() * outer_radius);

        temp_vec.push((i       as f32 * step).cos() * outer_radius);
        temp_vec.push(-0.5                                        );
        temp_vec.push((i       as f32 * step).sin() * outer_radius);

        temp_vec.push(((i + 1) as f32 * step).cos() * outer_radius);
        temp_vec.push(-0.5                                        );
        temp_vec.push(((i + 1) as f32 * step).sin() * outer_radius);

        temp_vec.push((i       as f32 * step).cos() * outer_radius);
        temp_vec.push(-0.5                                        );
        temp_vec.push((i       as f32 * step).sin() * outer_radius);

        temp_vec.push(((i + 1) as f32 * step).cos() * outer_radius);
        temp_vec.push(0.5                                         );
        temp_vec.push(((i + 1) as f32 * step).sin() * outer_radius);
    }

    // inner cylinder
    for i in 0..sides {
        temp_vec.push(((i + 1) as f32 * step).cos() * inner_radius);
        temp_vec.push(0.5                                         );
        temp_vec.push(((i + 1) as f32 * step).sin() * inner_radius);
        
        temp_vec.push((i       as f32 * step).cos() * inner_radius);
        temp_vec.push(0.5                                         );
        temp_vec.push((i       as f32 * step).sin() * inner_radius);
            
        temp_vec.push((i       as f32 * step).cos() * inner_radius);
        temp_vec.push(-0.5                                        );
        temp_vec.push((i       as f32 * step).sin() * inner_radius);
        
        temp_vec.push((i       as f32 * step).cos() * inner_radius);
        temp_vec.push(-0.5                                        );
        temp_vec.push((i       as f32 * step).sin() * inner_radius);

        temp_vec.push(((i + 1) as f32 * step).cos() * inner_radius);
        temp_vec.push(-0.5                                        );
        temp_vec.push(((i + 1) as f32 * step).sin() * inner_radius);
        
        temp_vec.push(((i + 1) as f32 * step).cos() * inner_radius);
        temp_vec.push(0.5                                         );
        temp_vec.push(((i + 1) as f32 * step).sin() * inner_radius);
    }

    // bottom ring
    for i in 0..sides {
        temp_vec.push(((i + 1) as f32 * step).cos() * inner_radius);
        temp_vec.push(-0.5                                        );
        temp_vec.push(((i + 1) as f32 * step).sin() * inner_radius);
            
        temp_vec.push((i       as f32 * step).cos() * inner_radius);
        temp_vec.push(-0.5                                        );
        temp_vec.push((i       as f32 * step).sin() * inner_radius);
            
        temp_vec.push((i       as f32 * step).cos() * outer_radius);
        temp_vec.push(-0.5                                        );
        temp_vec.push((i       as f32 * step).sin() * outer_radius);

        temp_vec.push(((i + 1) as f32 * step).cos() * inner_radius);
        temp_vec.push(-0.5                                        );
        temp_vec.push(((i + 1) as f32 * step).sin() * inner_radius);

        temp_vec.push((i       as f32 * step).cos() * outer_radius);
        temp_vec.push(-0.5                                        );
        temp_vec.push((i       as f32 * step).sin() * outer_radius);

        temp_vec.push(((i + 1) as f32 * step).cos() * outer_radius);
        temp_vec.push(-0.5                                        );
        temp_vec.push(((i + 1) as f32 * step).sin() * outer_radius);
    }

    return temp_vec;
}