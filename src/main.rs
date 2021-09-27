// use std::{
// fs::File,
// io::{BufWriter, Write},
//     path::PathBuf,
//     str::FromStr,
// };

use nalgebra::DMatrix;
use parry3d::{
    math::{Point, Real, Vector},
    shape::HeightField,
};
// use ply_rs::{
//     ply::{
//         Addable, DefaultElement, ElementDef, Encoding, Ply, Property, PropertyDef, PropertyType,
//         ScalarType,
//     },
//     writer::Writer,
// };

fn main() {
    let (particle_radius, size) = (0.05, 1.0);
    let particles = raytrace_particles(particle_radius, Vector::new(size, 0., size), 1.0);
    println!("{}, \n {:?}", particles.len(), particles); // particles.len() should be 100
    return;

    //comment out above to run through various
    for particle_radius in (1..10).map(|a| a as f32 / 5.) {
        for size in (1..20).map(|s| s as f32) {
            let scale = Vector::new(size, 0., size);
            let particles = raytrace_particles(particle_radius, scale, 1.0);

            // let to_file =
            //     PathBuf::from_str(&format!("tmp/{}-{}-{}.ply", particle_radius, size, scale.x))
            //         .unwrap();

            // println!(
            //     "outputting particles: {}, len: {}",
            //     to_file.to_string_lossy(),
            //     particles.len()
            // );
            // output_particles_to_file(&particles, None, &to_file)
        }
    }
}

fn raytrace_particles(
    particle_radius: Real,
    scale: Vector<Real>,
    height: Real,
    // bodies: &mut RigidBodySet,
    // colliders: &mut ColliderSet,
) -> Vec<Point<Real>> {
    /*
     * Ground
     */
    let nsubdivs = 40;

    let heights = DMatrix::from_fn(nsubdivs + 1, nsubdivs + 1, |i, j| height);

    // println!("heights: {:?}", heights);
    let heightfield = HeightField::new(heights, scale);

    let samples =
        salva3d::sampling::shape_surface_ray_sample(&heightfield, particle_radius).unwrap();

    samples
}

// pub fn output_particles_to_file(
//     particles: &Vec<Point<Real>>,
//     velocities: Option<&Vec<Vector<Real>>>,
//     to_file: &PathBuf,
// ) {
//     let file = File::create(to_file).unwrap();
//     let mut buf = Vec::<u8>::new();

//     let mut ply = create_ply();

//     let particles = particles
//         .iter()
//         .enumerate()
//         .map(|(i, particle)| {
//             let mut point = DefaultElement::new();

//             let velocity: Vector<Real> = if let Some(velocities) = velocities {
//                 velocities[i]
//             } else {
//                 Vector::y()
//             };

//             point.insert("x".to_string(), Property::Float(particle.x));
//             point.insert("y".to_string(), Property::Float(particle.y));
//             point.insert("z".to_string(), Property::Float(particle.z));

//             point.insert("nx".to_string(), Property::Float(velocity.x));
//             point.insert("ny".to_string(), Property::Float(velocity.y));
//             point.insert("nz".to_string(), Property::Float(velocity.z));
//             point
//         })
//         .collect();

//     ply.payload.insert("vertex".to_string(), particles);

//     let w = Writer::new();
//     let _written = w.write_ply(&mut buf, &mut ply).unwrap();

//     let mut buf_writer = BufWriter::new(file);
//     match buf_writer.write_all(&buf) {
//         Ok(r) => r,
//         _ => (),
//     }
// }

// fn create_ply() -> Ply<DefaultElement> {
//     let mut ply = Ply::<DefaultElement>::new();
//     ply.header.encoding = Encoding::BinaryBigEndian;
//     let mut point_element = ElementDef::new("vertex".to_string());
//     let p = PropertyDef::new("x".to_string(), PropertyType::Scalar(ScalarType::Float));
//     point_element.properties.add(p);
//     let p = PropertyDef::new("y".to_string(), PropertyType::Scalar(ScalarType::Float));
//     point_element.properties.add(p);
//     let p = PropertyDef::new("z".to_string(), PropertyType::Scalar(ScalarType::Float));
//     point_element.properties.add(p);
//     let p = PropertyDef::new("nx".to_string(), PropertyType::Scalar(ScalarType::Float));
//     point_element.properties.add(p);
//     let p = PropertyDef::new("ny".to_string(), PropertyType::Scalar(ScalarType::Float));
//     point_element.properties.add(p);
//     let p = PropertyDef::new("nz".to_string(), PropertyType::Scalar(ScalarType::Float));
//     point_element.properties.add(p);
//     ply.header.elements.add(point_element);

//     ply
// }
