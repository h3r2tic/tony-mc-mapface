fn tony_log(x: f64) -> f64 {
    let range_min = -13.0;
    let range_max = 8.0;
    let x = 2.0f64.powf(x * (range_max - range_min) + range_min);
    x / (x + 1.0)
}

fn srgb_oetf(x: f64) -> f64 {
    if x <= 0.0031308 {
        12.92 * x
    } else {
        1.055 * x.powf(1.0 / 2.4) - 0.055
    }
}

fn generate_lut(lut_fn: fn(f64) -> f64, filename: &str) {
    let table_size = 4096;

    let x_values: Vec<f64> = (0..table_size)
        .map(|i| i as f64 / (table_size - 1) as f64)
        .collect();
    let y_values: Vec<f64> = x_values.iter().map(|&x| lut_fn(x)).collect();

    let preamble = format!(
        "Version 1\nFrom 0.0000000 1.0000000\nLength {}\nComponents 1\n{{\n",
        table_size
    );
    let postamble = "\n}\n";

    let lut_str = y_values
        .iter()
        .map(|&x| format!("{:.10}", x))
        .collect::<Vec<String>>()
        .join("\n");

    let spi1d_data = format!("{}{}{}", preamble, lut_str, postamble);

    std::fs::write(filename, spi1d_data).unwrap();
}

pub fn generate_1d_luts() {
    generate_lut(tony_log, "../LUTs/tony_log_to_rational.spi1d");
    generate_lut(srgb_oetf, "../LUTs/sRGB_OETF.spi1d");
}
