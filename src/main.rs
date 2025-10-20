fn main() {
    let width : u32 = 256;
    let height : u32 = 256;

    println!("P3\n{} {}\n255", width, height);

    for j in (0..height).rev() {
        for i in 0..width {
            let r = i as f64 / ((height as f64) - 1.0 );
            let g = j as f64 / ((width as f64) - 1.0);
            let b = 0.0;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
