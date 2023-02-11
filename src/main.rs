use bytesize::ByteSize;
use clap::Parser;
use pdf_canvas::{Pdf, BuiltinFont, Canvas};
use std::fs::File;
use std::time::SystemTime;
use csscolorparser::Color as CColor;

extern crate bytesize;

const A4_WIDTH: f32 = 595.0;
const A4_HEIGHT: f32 = 842.0;
const FONT_FAMILY: BuiltinFont = BuiltinFont::Helvetica;
const FONT_SIZE: f32 = 24.0;
const LINE_SPACING: f32 = 10.0;

mod messenger;
mod utils;

use messenger::Messenger;
use utils::PDFColor;


#[derive(Parser, Debug, Clone)]
#[command(author, version, about, arg_required_else_help(true))]
pub struct Args {
    /// Text color, web color names and hex codes (without #) are supported
    /// e.g. red, blue, hotpink, 00ff00, rgb(100,100,100)
    #[arg(id="color")]
    color: CColor,
    /// Number of pages to generate
    #[arg(long, default_value_t = 1)]
    pages: u16,
    #[arg(short, long)]
    output: Option<String>,
    /// Generated file size
    /// ByteSize strings accepted, eg: 10Mb, 1024Kb etc.
    /// Data is distributed across all pages.
    /// (might not be accurate)
    #[arg(short, long)]
    size: Option<ByteSize>,
    /// Text to print
    #[arg(id="text")]
    text: Vec<String>,
    /// Don't print random number inside
    #[arg(long, default_value_t = false)]
    no_random_string: bool,
    /// Don't print page numbers
    #[arg(long, default_value_t = false)]
    no_pagenum: bool,
    /// Don't print PDF size information
    #[arg(long, default_value_t = false)]
    no_sizeinfo: bool,
    /// Print stats (file size, generation time)
    #[arg(long, default_value_t = false)]
    no_stats: bool,
    /// Print only resulting file name on success
    #[arg(long, default_value_t = false)]
    silent: bool,
    #[arg(skip)]
    random_string: String,
    /// Show debug messages
    #[arg(long, default_value_t = false)]
    debug: bool
}

fn render_page(pdf: &mut Pdf, args: &Args, page: &u16, msg: &Messenger) {
    pdf.render_page(A4_WIDTH, A4_HEIGHT, |canvas| {
        let canvas_font = canvas.get_font(FONT_FAMILY);
        let text = args.text.join(" ");

        let mut lines: Vec<String> = vec![text];

        if !args.no_random_string {
            lines.push(args.random_string.clone())
        }

        match (args.no_sizeinfo, args.size) {
            (false, Some(size)) => lines.push(format!("File size: {}", size)),
            (true, _) | (_, None) => {}
        };

        if args.pages > 1 && !args.no_pagenum {
            lines.push(format!("Page {} of {}", *page + 1, args.pages))
        }


        let mut y_position = (A4_HEIGHT +
            ((FONT_SIZE + LINE_SPACING) * (lines.len() as f32) - LINE_SPACING)
        ) / 2.0 - FONT_SIZE;

        for (idx, line) in lines.into_iter().enumerate() {
            msg.debug(format!("CUR: {} Y: {y_position} Text: {line}", idx + 1));

            canvas.text(|t| {
                t.set_font(&canvas_font, FONT_SIZE)?;
                t.set_fill_color(args.color.as_pdf_color())?;
                let line_width = canvas_font.get_width(FONT_SIZE, &line);

                t.pos(
                    (A4_WIDTH - line_width)/2.0,
                    y_position
                )?;

                // Center of the page
                t.show(&line)?;

                y_position -= LINE_SPACING + FONT_SIZE;

                Ok(())
            })?;
            }

        // If more pages - mark page
        // Put random marker
        if let Some(size) = args.size {
            generate_data(
                canvas,
                ByteSize::b(
                    size.as_u64() / (args.pages as u64)
                )
            )?
        }

        Ok(())
    }).expect("Failed to generate page 0");
}

fn generate_data(canvas: &mut Canvas, size: ByteSize) -> Result<(), std::io::Error> {
    let line_gen: String = "_".repeat(1024-9);
    canvas.text(|t| {
        t.pos(0.0, 0.0)?;
        let mut k = ByteSize::b(0);

        while k < size {
            t.show(&line_gen)?;
            k += ByteSize::b(1024)
        };
        Ok(())
    })?;
    Ok(())
}

fn main() {
    let start_time = SystemTime::now();

    let mut args = Args::parse();

    let msg = Messenger::new(args.clone());

    args.random_string = utils::generate_random(10);

    let output_file = match args.output {
        Some(ref name) => String::from(name),
        None =>  {
            let color_name = match args.color.name() {
                Some(name) => name,
                None =>
                    // Returns early
                    utils::print_color_without_name_and_exit()
            };
            format!("sample-{}-{}.pdf", color_name, &args.random_string)
        }
    };

    let output_handle = File::create(&output_file).expect("Can't open file");
    let cloned_handle = output_handle.try_clone().expect("Can't clone handle");

    let mut document = Pdf::new(output_handle).expect("Can't create PDF!");
    let mut cur_page = 0;

    while cur_page < args.pages {
        msg.debug(format!("Render page {}/{}", cur_page + 1, args.pages));
        render_page(&mut document, &args, &cur_page, &msg);
        msg.debug(format!("File size: {}", cloned_handle.metadata().unwrap().len()));
        cur_page += 1;
    }

    msg.debug(format!("Final file size: {}", cloned_handle.metadata().unwrap().len()));
    document.finish().expect("Can't finish!");

    let milliseconds = start_time.elapsed().expect("SystemTime Failure");
    let file_size = ByteSize::b(cloned_handle.metadata().unwrap().len());

    msg.stats(format!("Finished in {milliseconds:?}. Final file size: {file_size}.\nFile name: {output_file}"));
    msg.silent(format!("{output_file}"));
}
