mod interface;
mod slicer;

fn main() {
    println!("<svg viewBox=\"0 0 20 23\" xmlns=\"http://www.w3.org/2000/svg\">");
    println!("<defs><path id=\"field\" vector-effect=\"non-scaling-stroke\" stroke=\"black\" stroke-width=\"2px\" fill=\"none\" d=\"M 1 0");

    let quarter: Vec<(f64, f64)> = (0..=120)
        .map(|i| {
            let z = i as f64 / 120.0 * 2f64.cbrt();
            let x = slicer::radius(z);
            (x, z)
        })
        .collect();

    let mut half = quarter.clone();
    half.extend(quarter.iter().map(|(x, y)| (-x, *y)).rev());

    let mut full = half.clone();
    full.extend(half.iter().map(|(x, y)| (-x, -y)));

    for (x, y) in full {
        println!("L {} {}", x, y);
    }

    println!("\"/></defs>");

    for scale in 1..10 {
        println!(
            "<use href=\"#field\" transform=\"translate(10 11.5) scale({})\" />",
            scale
        );
    }

    println!("<line stroke=\"red\" vector-effect=\"non-scaling-stroke\" stroke-width=\"2px\" x1=\"0\" x2=\"20\" y1=\"7\" y2=\"7\" />");

    println!("</svg>");
}
