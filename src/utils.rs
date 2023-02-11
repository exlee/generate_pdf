use rand::{thread_rng, Rng};
use csscolorparser::Color as CColor;
use pdf_canvas::graphicsstate::Color;


pub fn print_color_without_name_and_exit() -> &'static str {
    println!("
Color can't be converted to human-readable name.
Use --output option or choose different color.
");
    std::process::exit(1);
}

pub fn generate_random(length: u8) -> String {
    let mut rng = thread_rng();
    let mut result = String::new();
    let mut i = 0;

    //TODO: This probalby should be rewritten into collect
    while i < length {
        result.push(rng.gen_range::<u8,_>(48..58) as char);
        i += 1;
    }

    result
}

pub trait PDFColor {
    fn as_pdf_color(&self) -> Color;
}

impl PDFColor for CColor {
    fn as_pdf_color(&self) -> Color {
        let [r,g,b,_] = self.to_rgba8();
        Color::RGB { red: r, green: g, blue: b }
    }
}
