mod dds_to_spi3d;
mod lut_1d;

use dds_to_spi3d::convert_3d_lut;
use lut_1d::generate_1d_luts;

fn main() {
    generate_1d_luts();
    convert_3d_lut();
}
