pub fn ppm_print_header(nx: i32, ny: i32)
{
    println!("P3\n{} {}\n255", nx, ny);
}

pub fn ppm_print_rgb(r: f64, g: f64, b: f64)
{
    let ir = (255.99 * r) as i32;
    let ig = (255.99 * g) as i32;
    let ib = (255.99 * b) as i32;
    println!("{} {} {}", ir, ig, ib);
}