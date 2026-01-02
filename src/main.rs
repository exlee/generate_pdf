use bytesize::ByteSize;
use clap::Parser;
use std::time::SystemTime;
use csscolorparser::Color as CColor;
use std::fs::File;

extern crate bytesize;

mod utils;
mod pdf;
mod messenger;
mod divider;

//use messenger::Messenger;
use pdf::generate_pdf;
use crate::messenger::Messenger;

#[derive(Parser, Debug, Clone)]
/// Example PDF generator
///
/// Use it to generate PDFs to test with development account.
/// Can be used to generate visually distinguishable PDFs with
/// defined filesize and pages.
///
///

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
    debug: bool,
    /// Enables super-precise size generation
    #[arg(long, default_value_t = false)]
    super_precision: bool
}

fn main() {
    let start_time = SystemTime::now();
    let mut args = Args::parse();

    let desired_size = args.size.clone();

    args.random_string = utils::generate_random(10);

    let msg = Messenger::new(args.clone());

    let output_file = match args.output {
        Some(ref name) => String::from(name),
        None =>  {
            let color_name = match args.color.name() {
                Some(name) => name,
                None => {
                    // Returns early
                    utils::print_color_without_name();
                    std::process::exit(1);
                }
            };
            format!("sample-{}-{}.pdf", color_name, &args.random_string)
        }
    };

    let output_handle = File::create(&output_file).expect("Can't open file");
    let file_size = generate_pdf(args, &msg, output_handle);

    //let delta =  desired_size.unwrap().as_u64() as i64 - file_size.as_u64() as i64;
    //println!("Ask for: {:?}", desired_size.unwrap().as_u64() as i64 + delta +);

    let milliseconds = start_time.elapsed().expect("SystemTime Failure");
    msg.stats(format!("Finished in {milliseconds:?}. Final file size: {file_size}.\nFile name: {output_file}"));
    msg.silent(format!("{output_file}"));
}
