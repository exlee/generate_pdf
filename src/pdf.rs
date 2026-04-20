use pdf_canvas::{Pdf, BuiltinFont, Canvas};
use std::fs::File;
use bytesize::ByteSize;
use clap::Parser;

use crate::utils::PDFColor;
use crate::Args;
use crate::messenger::Messenger;
use crate::divider::divide_at;


extern crate bytesize;

const A4_WIDTH: f32 = 595.0;
const A4_HEIGHT: f32 = 842.0;
const FONT_FAMILY: BuiltinFont = BuiltinFont::Helvetica;
const FONT_SIZE: f32 = 24.0;
const LINE_SPACING: f32 = 10.0;
const SHOW_LINE_PAD: usize = 6;
const BLOCK_SIZE_BYTES: usize = 1024;


pub fn generate_pdf(args: Args, msg: &Messenger, output_handle: File) -> ByteSize {
    let desired = args.size.map(|s| s.as_u64() as usize);

    match desired {
        None => do_generate(args, msg, output_handle),
        Some(target) => {
            let mut current_target = target;

            for _ in 0..10 {
                let temp = tempfile::tempfile().expect("Can't create temp file");
                let actual = generate_with_data_size(args.clone(), msg, temp, current_target);
                let actual_size = actual.as_u64() as usize;

                if actual_size == target {
                    return generate_with_data_size(args, msg, output_handle, current_target);
                }

                let delta = target as i64 - actual_size as i64;
                let new_target = current_target as i64 + delta;
                if new_target <= 0 {
                    return generate_with_data_size(args, msg, output_handle, current_target);
                }
                current_target = new_target as usize;
            }

            generate_with_data_size(args, msg, output_handle, current_target)
        }
    }
}

fn generate_with_data_size(mut args: Args, msg: &Messenger, output_handle: File, data_size: usize) -> ByteSize {
    args.size = Some(ByteSize::b(data_size as u64));
    do_generate(args, msg, output_handle)
}

fn do_generate(args: Args, msg: &Messenger, output_handle: File) -> ByteSize {
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

    ByteSize::b(cloned_handle.metadata().unwrap().len())
}

fn render_page(pdf: &mut Pdf, args: &Args, page: &u16, msg: &Messenger) {
    pdf.render_page(A4_WIDTH, A4_HEIGHT, |canvas| {
        let canvas_font = canvas.get_font(FONT_FAMILY);
        let text = args.text.join(" ");

        let mut lines: Vec<String> = vec![text];

        // Stamp with random string
        if !args.no_random_string {
            lines.push(args.random_string.clone())
        }

        // Stamp page size
        match (args.no_sizeinfo, args.size) {
            (false, Some(size)) => lines.push(format!("File size: {}", size)),
            (true, _) | (_, None) => {}
        };

        // Stamp page number
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

        if let Some(size) = args.size {
            generate_data(
                canvas,
                divide_at(size.as_u64() as usize, args.pages as usize, *page as usize)
            )?
        }

        Ok(())
    }).expect("Failed to generate page 0");
}


/// Function generating data insiide the Canvas
fn generate_data(canvas: &mut Canvas, size: usize) -> Result<(), std::io::Error> {
    if size < SHOW_LINE_PAD {
        return Ok(());
    }

    let line_gen: String = "_".repeat(BLOCK_SIZE_BYTES);
    canvas.text(|t| {
        t.pos(0.0, 0.0)?;

        let mut k = 0;

        while k + BLOCK_SIZE_BYTES + 2 * SHOW_LINE_PAD + 1 <= size {
            t.show(&line_gen)?;
            k += BLOCK_SIZE_BYTES + SHOW_LINE_PAD;
        }

        let missing_bytes = size - k - SHOW_LINE_PAD;
        t.show(&"_".repeat(missing_bytes))?;
        Ok(())
    })?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::tempfile;

    macro_rules! header {
        () => {
            use tempfile::tempfile;
            use super::*;

            fn build_args(pages: usize, expected_size: usize) -> Args {
                let mut args = Args::parse_from([
                    "test",
                    "--no-sizeinfo",
                    "--size",  &expected_size.to_string(),
                    "--pages", &pages.to_string(),
                    "red" ,
                    "Example Text"
                ]);
                args.random_string = String::from("0000000000");
                args
            }
            fn file_size_checker(pages: usize, expected_size: usize) -> usize {
                let args = build_args(pages, expected_size);
                let msg = Messenger::new(args.clone());
                let file = tempfile().unwrap();
                generate_pdf(args, &msg, file).as_u64() as usize
            }
        }
    }

    macro_rules! test_size {
        ($name:ident: $pages:expr,$size:expr) => {
            #[test]
            fn $name() {
                assert_eq!(file_size_checker($pages, $size), $size);
            }
        };
        ($name:ident: $size:expr) => {
            #[test]
            fn $name => {
                assert_eq!(file_size_checker(1, $size), $size);
            }
        }


    }

    mod file_size_check {
        header!();

        test_size!(check_1_4096        :  1      , 4096        );
        test_size!(check_3_8192        :  3      , 8192        );
        test_size!(check_9_999999      :  9      , 999999      );
        test_size!(check_10_50000      :  10     , 50000       );
        test_size!(check_25_999999     :  25     , 999999      );
        test_size!(check_8_9999999     :  8      , 9999999     );
        test_size!(check_8_9999995     :  1      , 9999995     );
        test_size!(check_3_123456      :  3      , 123456      );
        test_size!(check_7_987654      :  3      , 987654      );
        test_size!(check_3_10000       :  3      , 10000       );
        test_size!(check_12_94798      :  12     , 94798       );
    }
}

// Local Variables:
// my-local-templates: ((ts "test_size!(check_" (p "1" page) "_"(p "1" size)": " (s page)", "(s size) ")"))
// End:
