use std::{fmt::Write, path::PathBuf};

pub fn convert_3d_lut() {
    let dds_path = PathBuf::from("tony_mc_mapface_f32.dds");

    let dds =
        &ddsfile::Dds::read(&mut std::io::Cursor::new(std::fs::read(&dds_path).unwrap())).unwrap();

    assert_eq!(
        dds.get_dxgi_format().unwrap(),
        ddsfile::DxgiFormat::R32G32B32A32_Float
    );

    let dds_data = dds.get_data(0).unwrap();
    assert_eq!(dds_data.as_ptr() as usize % 4, 0);
    let dims = [dds.get_width(), dds.get_height(), dds.get_depth()];

    let mut output = format!("SPILUT 1.0\n3 3\n{} {} {}\n", dims[0], dims[1], dims[2]);

    let dds_data: &[f32] =
        unsafe { std::slice::from_raw_parts(dds_data.as_ptr().cast(), dds_data.len() / 4) };

    for z in 0..dims[2] {
        for y in 0..dims[1] {
            for x in 0..dims[0] {
                let offset = ((((z * dims[1]) + y) * dims[0] + x) * 4) as usize;
                let pixel = &dds_data[offset..offset + 4];
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];
                writeln!(&mut output, "{x} {y} {z} {r:.8} {g:.8} {b:.8}").unwrap();
            }
        }
    }

    std::fs::write("../LUTs/tony_mc_mapface.spi3d", output).unwrap();
}
