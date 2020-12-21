extern crate collada_io; // Export
extern crate clap; // CLI
extern crate dot_vox; // Import
extern crate stl_io; // Export

pub mod app;

const INPUT_FILEPATH: &'static str = "input.vox";
const OUTPUT_STL_FILEPATH: &'static str = "output.stl";
const OUTPUT_DAE_FILEPATH: &'static str = "output.dae";
const OUTPUT_PAL_FILEPATH: &'static str = "output.txt";

use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;

struct Color
{
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

#[derive(Copy, Clone)]
struct MetaVoxel
{
    pub voxel: dot_vox::Voxel,
    pub faces: u8
}

#[derive(Copy, Clone, PartialEq)]
struct Vertex
{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Copy, Clone, PartialEq)]
struct Normal
{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Copy, Clone, PartialEq)]
struct Triangle
{
    pub normal: Normal,
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex
}

#[derive(Copy, Clone, PartialEq)]
struct IndexedTriangle
{
    pub normal: Normal, // STL
    pub normal_index: usize, // DAE
    pub a: usize,
    pub b: usize,
    pub c: usize
}

impl Into<stl_io::Normal> for Normal
{
    fn into(self) -> stl_io::Normal
    { 
        stl_io::Normal::new([self.x, self.y, self.z])
    }
}

impl Into<stl_io::Vertex> for Vertex
{
    fn into(self) -> stl_io::Vertex
    { 
        stl_io::Vertex::new([self.x, self.y, self.z])
    }
}

impl Into<stl_io::Triangle> for Triangle
{
    fn into(self) -> stl_io::Triangle
    { 
        stl_io::Triangle {
            normal: self.normal.into(),
            vertices: [self.a.into(), self.b.into(), self.c.into()]
        }
    }
}

impl Into<stl_io::IndexedTriangle> for IndexedTriangle
{
    fn into(self) -> stl_io::IndexedTriangle
    { 
        stl_io::IndexedTriangle {
            normal: self.normal.into(),
            vertices: [self.a, self.b, self.c]
        }
    }
}

impl From<u32> for Color
{
    fn from(src: u32) -> Self 
    { 
        let r = ((src & 0x00_00_00_FF) >> 0) as u8;
        let g = ((src & 0x00_00_FF_00) >> 8) as u8;
        let b = ((src & 0x00_FF_00_00) >> 16) as u8;
        let a = ((src & 0xFF_00_00_00) >> 24) as u8;
        Self {
            r,
            g,
            b,
            a
        } 
    }
}

impl Into<String> for Color
{
    fn into(self) -> String
    {
        format!("{} {} {}", self.r, self.g, self.b)
    }
}

impl MetaVoxel
{
    pub fn has_left(&self) -> bool
    {
        return ((self.faces & (1 << 0)) >> 0) == 1;
    }
    
    pub fn has_back(&self) -> bool
    {
        return ((self.faces & (1 << 1)) >> 1) == 1;
    }

    pub fn has_bottom(&self) -> bool
    {
        return ((self.faces & (1 << 2)) >> 2) == 1;
    }

    pub fn has_right(&self) -> bool
    {
        return ((self.faces & (1 << 3)) >> 3) == 1;
    }
    
    pub fn has_front(&self) -> bool
    {
        return ((self.faces & (1 << 4)) >> 4) == 1;
    }

    pub fn has_top(&self) -> bool
    {
        return ((self.faces & (1 << 5)) >> 5) == 1;
    }
}

fn get_voxel_idx(vox: &dot_vox::Voxel, size: &dot_vox::Size) -> usize
{
    return (vox.x as usize) + 
           ((vox.y as usize) * (size.x as usize)) + 
           ((vox.z as usize) * (size.x as usize) * (size.y as usize));
}

// Turn the Vec
fn reorder_voxels(voxels: &mut Vec<dot_vox::Voxel>, size: &dot_vox::Size) -> Vec<Option<dot_vox::Voxel>>
{
    let mut ret_voxels: Vec<Option<dot_vox::Voxel>> = Vec::new();
    let len: usize = (size.x as usize) * (size.y as usize) * (size.z as usize);
    ret_voxels.resize(len, None);
    for voxel in voxels
    {
        let idx = get_voxel_idx(voxel, size);
        ret_voxels[idx] = Some(*voxel);
    }
    // voxels.sort_by(|a_vox: &dot_vox::Voxel, b_vox: &dot_vox::Voxel| {
    //     let a: usize = get_voxel_idx(&a_vox, size);
    //     let b: usize = get_voxel_idx(&b_vox, size);
        
    //     return a.partial_cmp(&b).unwrap();
    // });
    return ret_voxels;
}

fn has_neighbor(voxels: &Vec<Option<dot_vox::Voxel>>, voxel: &dot_vox::Voxel, size: &dot_vox:: Size, x: i16, y: i16, z: i16) -> Result<bool, std::num::TryFromIntError>
{
    use std::convert::TryFrom;
    let vx: i16 = voxel.x.into();
    let vy: i16 = voxel.y.into();
    let vz: i16 = voxel.z.into();

    let pos = dot_vox::Voxel {
        i: voxel.i,
        x: u8::try_from(vx + x)?,
        y: u8::try_from(vy + y)?,
        z: u8::try_from(vz + z)?
    };

    let neighbor: Option<dot_vox::Voxel>;
    let idx = get_voxel_idx(&pos, size);
    neighbor = voxels[idx];
    // println!("Base: {}, {}, {}", voxel.x, voxel.y, voxel.z);
    // println!("Base (Signed): {}, {}, {}", vx, vy, vz);
    // println!("By: {}, {}, {}", x, y, z);
    // println!("Relative: {}, {}, {}", pos.x, pos.y, pos.z);
    // println!("Index: {}", idx);
    // println!("Some: {}", neighbor.is_some());
    // println!("--------------------------------------------");

    return Ok(neighbor.is_some());
}

fn convert_meta_voxels(voxels: &Vec<Option<dot_vox::Voxel>>, size: &dot_vox::Size) -> Vec<Option<MetaVoxel>>
{
    let mut mvoxels: Vec<Option<MetaVoxel>> = Vec::new();
    let len: usize = (size.x as usize) * (size.y as usize) * (size.z as usize);
    mvoxels.resize(len, None);
    for (i, opt_voxel) in voxels.iter().enumerate()
    {
        match opt_voxel
        {
            Some(voxel) => {
                let mut faces: u8 = 0;
                let mut surrounded: bool = true;
                // Need to add check to see if there are empty spaces in order to add a face
                if voxel.x == 0 || !has_neighbor(voxels, voxel, size, -1, 0, 0).unwrap()// Left
                {
                    faces |= 1 << 0;
                    surrounded = false;
                }
                
                if voxel.y == 0 || !has_neighbor(voxels, voxel, size, 0, -1, 0).unwrap() // Back
                {
                    faces |= 1 << 1;
                    surrounded = false;
                }

                if voxel.z == 0 || !has_neighbor(voxels, voxel, size, 0, 0, -1).unwrap() // Bottom
                {
                    faces |= 1 << 2;
                    surrounded = false;
                }

                if ((voxel.x + 1) == (size.x as u8)) || !has_neighbor(voxels, voxel, size, 1, 0, 0).unwrap() // Right
                {
                    faces |= 1 << 3;
                    surrounded = false;
                }

                if (voxel.y + 1) == (size.y as u8) || !has_neighbor(voxels, voxel, size, 0, 1, 0).unwrap()  // Front
                {
                    faces |= 1 << 4;
                    surrounded = false;
                }

                if ((voxel.z + 1) == (size.z as u8)) || !has_neighbor(voxels, voxel, size, 0, 0, 1).unwrap() // Top
                {
                    faces |= 1 << 5;
                    surrounded = false;
                }

                if !surrounded
                {
                    mvoxels[i] = Some(MetaVoxel {
                        voxel: *voxel,
                        faces
                    });
                }
            }
            None => {

            }
        }
    }

    return mvoxels;
}

fn convert_triangles(mvoxels: &Vec<Option<MetaVoxel>>) -> Vec<Triangle>
{
    let mut triangles = Vec::new();

    for opt_mvoxel in mvoxels
    {
        match opt_mvoxel
        {
            Some(mvoxel) => {
                // Should this part be turned into a triangles() function on the MetaVoxel struct?
                let vleft_back_top = Vertex {
                    x: mvoxel.voxel.x as f32 + 0.0,
                    y: mvoxel.voxel.y as f32 + 0.0,
                    z: mvoxel.voxel.z as f32 + 1.0,
                };

                let vleft_front_top = Vertex {
                    x: mvoxel.voxel.x as f32 + 0.0,
                    y: mvoxel.voxel.y as f32 + 1.0,
                    z: mvoxel.voxel.z as f32 + 1.0,
                };

                let vleft_back_bottom = Vertex {
                    x: mvoxel.voxel.x as f32 + 0.0,
                    y: mvoxel.voxel.y as f32 + 0.0,
                    z: mvoxel.voxel.z as f32 + 0.0,
                };

                let vleft_front_bottom = Vertex {
                    x: mvoxel.voxel.x as f32 + 0.0,
                    y: mvoxel.voxel.y as f32 + 1.0,
                    z: mvoxel.voxel.z as f32 + 0.0,
                };

                let vright_back_top = Vertex {
                    x: mvoxel.voxel.x as f32 + 1.0,
                    y: mvoxel.voxel.y as f32 + 0.0,
                    z: mvoxel.voxel.z as f32 + 1.0,
                };

                let vright_front_top = Vertex {
                    x: mvoxel.voxel.x as f32 + 1.0,
                    y: mvoxel.voxel.y as f32 + 1.0,
                    z: mvoxel.voxel.z as f32 + 1.0,
                };

                let vright_back_bottom = Vertex {
                    x: mvoxel.voxel.x as f32 + 1.0,
                    y: mvoxel.voxel.y as f32 + 0.0,
                    z: mvoxel.voxel.z as f32 + 0.0,
                };

                let vright_front_bottom = Vertex {
                    x: mvoxel.voxel.x as f32 + 1.0,
                    y: mvoxel.voxel.y as f32 + 1.0,
                    z: mvoxel.voxel.z as f32 + 0.0,
                };

                if mvoxel.has_left()
                {
                    let normal = Normal {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0
                    };
                    triangles.push(Triangle {
                        normal,
                        a: vleft_back_top,
                        b: vleft_back_bottom,
                        c: vleft_front_bottom
                    });

                    triangles.push(Triangle {
                        normal,
                        a: vleft_back_top,
                        b: vleft_front_top,
                        c: vleft_front_bottom
                    });
                }

                if mvoxel.has_back()
                {
                    let normal = Normal {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0
                    };
                    triangles.push(Triangle {
                        normal,
                        a: vleft_back_top,
                        b: vright_back_top,
                        c: vright_back_bottom
                    });

                    triangles.push(Triangle {
                        normal,
                        a: vleft_back_top,
                        b: vleft_back_bottom,
                        c: vright_back_bottom
                    });
                }

                if mvoxel.has_bottom()
                {
                    let normal = Normal {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0
                    };
                    triangles.push(Triangle {
                        normal,
                        a: vleft_back_bottom,
                        b: vleft_front_bottom,
                        c: vright_front_bottom
                    });

                    triangles.push(Triangle {
                        normal,
                        a: vleft_back_bottom,
                        b: vright_back_bottom,
                        c: vright_front_bottom
                    });                    
                }

                if mvoxel.has_right()
                {
                    let normal = Normal {
                        x: -1.0,
                        y: 0.0,
                        z: 0.0
                    };
                    triangles.push(Triangle {
                        normal,
                        a: vright_back_top,
                        b: vright_back_bottom,
                        c: vright_front_bottom
                    });

                    triangles.push(Triangle {
                        normal,
                        a: vright_back_top,
                        b: vright_front_top,
                        c: vright_front_bottom
                    });
                }

                if mvoxel.has_front()
                {
                    let normal = Normal {
                        x: 0.0,
                        y: -1.0,
                        z: 0.0
                    };
                    triangles.push(Triangle {
                        normal,
                        a: vleft_front_top,
                        b: vright_front_top,
                        c: vright_front_bottom
                    });

                    triangles.push(Triangle {
                        normal,
                        a: vleft_front_top,
                        b: vleft_front_bottom,
                        c: vright_front_bottom
                    });
                }

                if mvoxel.has_top()
                {
                    let normal = Normal {
                        x: 0.0,
                        y: 0.0,
                        z: -1.0
                    };
                    triangles.push(Triangle {
                        normal,
                        a: vleft_back_top,
                        b: vleft_front_top,
                        c: vright_front_top
                    });

                    triangles.push(Triangle {
                        normal,
                        a: vleft_back_top,
                        b: vright_back_top,
                        c: vright_front_top
                    });
                }
            },
            None => {

            }
        }
    }

    return triangles;
}

fn index_triangles(triangles: &Vec<Triangle>, vertices: &mut Vec<Vertex>, normals: &mut Vec<Normal>, idx_triangles: &mut Vec<IndexedTriangle>)
{
    for triangle in triangles
    {
        let normal_index: usize;
        let a_position: usize;
        let b_position: usize;
        let c_position: usize;
        
        match vertices.iter().position(|&x| x == triangle.a)
        {
            Some(idx) => {
                a_position = idx;
            },
            None => {
                a_position = vertices.len();
                vertices.push(triangle.a);
            }
        }

        match vertices.iter().position(|&x| x == triangle.b)
        {
            Some(idx) => {
                b_position = idx;
            },
            None => {
                b_position = vertices.len();
                vertices.push(triangle.b);
            }
        }

        match vertices.iter().position(|&x| x == triangle.c)
        {
            Some(idx) => {
                c_position = idx;
            },
            None => {
                c_position = vertices.len();
                vertices.push(triangle.c);
            }
        }

        match normals.iter().position(|&x| x == triangle.normal)
        {
            Some(idx) => {
                normal_index = idx;
            },
            None => {
                normal_index = vertices.len();
                normals.push(triangle.normal);
            }
        }

        idx_triangles.push(IndexedTriangle {
            normal: triangle.normal,
            normal_index,
            a: a_position,
            b: b_position,
            c: c_position
        });
    }
}


fn convert_vox_stl(ifpath: &str, ofpath: &str)
{
    use std::fs::OpenOptions;
    let mut in_data = dot_vox::load(ifpath).unwrap();
    println!("Vox Version #: {}", in_data.version);
    println!("# of Models: {}", in_data.models.len());

    // let mut vertices: Vec<Vertex> = Vec::new();
    // let mut normals: Vec<Normal> = Vec::new();
    let mut triangles: Vec<Triangle> = Vec::new();
    // let mut indexed_triangles: Vec<IndexedTriangle> = Vec::new();
    for model in &mut in_data.models
    {
        let size = model.size;
        let voxels = reorder_voxels(&mut model.voxels, &size);
        // Need to change this to Vec<Option<Voxel>>
        // reorder_voxels(&mut model.voxels, &size);    
        let mvoxels = convert_meta_voxels(&voxels, &size);
        triangles.append(&mut convert_triangles(&mvoxels));
        println!("# Of Triangles in Model: {}", triangles.len());
    }

    // index_triangles(&triangles, &mut vertices, &mut normals, &mut indexed_triangles);
    // let stl_vertices: Vec<stl_io::Vertex>;
    let stl_triangles: Vec<stl_io::Triangle>;
    // let stl_indexed_triangles: Vec<stl_io::IndexedTriangle>;
    // stl_vertices = vertices.iter().map(|vertex| (*vertex).into()).collect::<Vec<_>>();
    stl_triangles = triangles.iter().map(|triangle| (*triangle).into()).collect::<Vec<_>>();
    // stl_indexed_triangles = indexed_triangles.iter().map(|idx_triangle| (*idx_triangle).into()).collect::<Vec<_>>();

    // let indexed_mesh: stl_io::IndexedMesh = stl_io::IndexedMesh {
    //     vertices: stl_vertices,
    //     faces: stl_indexed_triangles
    // };
    // indexed_mesh.validate().unwrap();
    let mut file = OpenOptions::new().write(true).create(true).open(ofpath).unwrap();
    stl_io::write_stl(&mut file, stl_triangles.iter()).unwrap();
}

fn convert_vox_dae(ifpath: &str, ofpath: &str)
{
    let mut in_data = dot_vox::load(ifpath).unwrap();
    println!("Vox Version #: {}", in_data.version);
    println!("# of Models: {}", in_data.models.len());

    let collada: collada_io::collada::Collada;
    let mut geometries: Vec<collada_io::geometry::Geometry> = Vec::new();
    for model in &mut in_data.models
    {
        let size = model.size;
        println!("Size: ({}, {}, {})", size.x, size.y, size.z);
        let voxels = reorder_voxels(&mut model.voxels, &size);
        println!("Length: {}", voxels.len());
        // Need to change this to Vec<Option<Voxel>>
        // reorder_voxels(&mut model.voxels, &size);    
        let mvoxels = convert_meta_voxels(&voxels, &size);
        let triangles = convert_triangles(&mvoxels);
        println!("# Of Triangles in Model: {}", triangles.len());
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut normals: Vec<Normal> = Vec::new();
        let mut indexed_triangles: Vec<IndexedTriangle> = Vec::new();
        index_triangles(&triangles, &mut vertices, &mut normals, &mut indexed_triangles);

        let mut primitive: Vec<usize> = Vec::new();
        primitive.reserve(indexed_triangles.len() * 3);
        let mut mesh_positions: Vec<f32> = Vec::new();
        mesh_positions.reserve(vertices.len() * 3);

        // TODO: Add normals
        for idx_triangle in indexed_triangles
        {
            primitive.push(idx_triangle.a);
            primitive.push(idx_triangle.b);
            primitive.push(idx_triangle.c);
        }

        for vertex in vertices
        {
            mesh_positions.push(vertex.x);
            mesh_positions.push(vertex.y);
            mesh_positions.push(vertex.z);
        }

        geometries.push(collada_io::geometry::Geometry {
            id: Some("Voxel-mesh".to_string()),
            name: Some("Voxel".to_string()),
            mesh: collada_io::geometry::Mesh {
                triangles: collada_io::geometry::Triangles {
                    vertices: "#Cube-mesh-vertices".to_string(),
                    normals: None,
                    tex_vertices: None,
                    primitive: Some(primitive),
                    material: None
                },
                vertices: collada_io::geometry::Vertices {
                    id: "Voxel-mesh-vertices".to_string(),
                    name: None,
                    source: "#Voxel-mesh-positions".to_string()
                },
                sources: vec! {
                    collada_io::geometry::Source {
                        id: "Voxel-mesh-positions".to_string(),
                        float_array: collada_io::geometry::FloatArray {
                            id: "Voxel-mesh-positions-array".to_string(),    
                            data: mesh_positions
                        },
                        accessor: collada_io::geometry::Accessor {
                            params: vec! { "X".to_string(), "Y".to_string(), "Z".to_string() }
                        }
                    },
                }
            }
        });   
    }

    let mut file = File::create(ofpath).unwrap();

    collada = collada_io::collada::Collada {
        scene: Some(collada_io::scene::Scene {
            visual_scenes: vec!{
                "#Scene".to_string()
            }
        }),
        visual_scenes: Some(vec!{
            collada_io::scene::VisualScene {
                id: "Scene".to_string(),
                name: "Scene".to_string(),
                nodes: vec!{
                    collada_io::scene::Node {
                        id: "Voxel".to_string(),
                        name: "Voxel".to_string(),
                        transformation_elements: vec!{
                            collada_io::scene::TransformationElement::Matrix {
                                sid: "transform".to_string(),
                                matrix: vec! {
                                    1.0, 0.0, 0.0, 0.0, 
                                    0.0, 1.0, 0.0, 0.0, 
                                    0.0, 0.0, 1.0, 0.0,
                                    0.0, 0.0, 0.0, 1.0,
                                }
                            }
                        },
                        instances: vec!{
                            collada_io::scene::Instance::Geometry {
                                url: "#Voxel-mesh".to_string(),
                                name: Some("Voxel".to_string()),
                                sid: None,
                                bind_material: None
                            }
                        }
                    }
                }
            }
        }),
        asset: collada_io::meta::Asset::default(),
        geometries: Some(geometries)
    };

    collada.write_to(&mut file).unwrap();
}

fn _export_jasc_palette(ifpath: &str, ofpath: &str) -> std::io::Result<()>
{
    let in_data = dot_vox::load(ifpath).unwrap();
    let file = File::create(ofpath)?;
    let mut file = LineWriter::new(file);

    let len_str: String = in_data.palette.len().to_string() + "\n";
    file.write_all(b"JASC\n")?;
    file.write_all(b"0100\n")?;
    file.write_all(len_str.as_bytes())?;
    for element in &in_data.palette
    {
        let color: Color = (*element).into();
        let color_str: String = color.into();
        file.write_all((color_str + "\n").as_bytes())?;
    }

    Ok(())
}

fn main() 
{
    let app = app::new_app();
    let matches = app.get_matches();
    let in_file = matches.value_of("input").unwrap_or(INPUT_FILEPATH);
    let out_file: &str;

    if matches.is_present("stl")
    {
        out_file = matches.value_of("output").unwrap_or(OUTPUT_STL_FILEPATH);
        convert_vox_stl(in_file, out_file);
    } else if matches.is_present("dae")
    {
        out_file = matches.value_of("output").unwrap_or(OUTPUT_DAE_FILEPATH);
        convert_vox_dae(in_file, out_file);
    } else {
        // Need either stl or dae so not sure what to do here
    }
}