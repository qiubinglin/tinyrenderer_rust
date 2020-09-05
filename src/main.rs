include!("geometry.rs");

fn main() {
    let a: Vec3D = Vec3D {
        arr: [1.0, 0.0, 0.0]
    };
    let _b: Vec3D = a * 2.0;
    let _c: f64 = a * _b;
    println!("{}", _c)
}
