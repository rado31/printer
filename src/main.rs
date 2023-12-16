extern crate escposify;
extern crate tempfile;
use std::fs;
use std::io;

use escposify::device::File;
use escposify::img::Image;
use escposify::printer::Printer;

fn main() -> io::Result<()> {
    // after you shares your printer, use the following code
    // the IP is the your computer's IP address
    // "Receipt Printer" is the shared name of your printer
    // (check the control panel -> devices and printer)
    let path = "/dev/usb/lp0";

    // create a file and send it to the printer's path
    // if the path is not available/error you should handle it here.
    let file = File::<fs::File>::from_path(path)?;

    // prepare the variable
    let mut printer = Printer::new(file, None, None);

    let image = Image::new("railways.png").unwrap();

    printer.bit_image(&image, Some("D24")).unwrap();

    printer
        .chain_font("C")?
        .chain_align("lt")?
        .chain_style("bu")?
        .chain_size(0, 0)?
        .chain_text("The quick brown fox jumps over the lazy dog")?
        .chain_barcode("12345678", "EAN8", "", "", 0, 0)?
        .chain_feed(1)?
        .chain_cut(false)?
        .flush()
}
