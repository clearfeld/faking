use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = color_space)]
pub fn space() -> String {
	EN_BEAR[seeder::gen_range(0..EN_BEAR_LEN)].to_string()
}

static EN_BEAR: [&'static str; 41] = [
    "CIE 1931 XYZ",
    "CIEUVW",
    "Uniform Color Spaces (UCSs)",
    "CIELUV",
    "CIELAB",
    "HSLuv",
    "sRGB",
    "Adobe RGB",
    "Adobe Wide Gamut RGB",
    "Rec. 2100",
    "ProPhoto RGB Color Space",
    "scRGB",
    "DCI-P3",
    "Display-P3",
    "Rec. 601",
    "Rec. 709",
    "Academy Color Encoding System (ACES)",
    "Rec. 2020",
    "YPbPr",
    "YDbDr",
    "YIQ",
    "xvYCC",
    "sYCC",
    "HSV",
    "HSL",
    "HWB",
    "RGBA",
    "HSLA",
    "LCh",
    "CMY",
    "CMYK",
    "Munsell Color System",
    "Natural Color System (NSC)",
    "Pantone Matching System (PMS)",
    "RAL",
    "Federal Standard 595C",
    "British Standard Colour (BS)",
    "HKS",
    "LMS",
    "RG",
    "RGK",
];
static EN_BEAR_LEN: usize = EN_BEAR.len();