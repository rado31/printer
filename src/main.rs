extern crate escposify;
extern crate tempfile;
use std::fs;
use std::io;

use escposify::device::File;
use escposify::printer::Printer;

fn main() -> io::Result<()> {
    // after you shares your printer, use the following code
    // the IP is the your computer's IP address
    // "Receipt Printer" is the shared name of your printer
    // (check the control panel -> devices and printer)
    let path = "usb://CUSTOM%20Engineering/VKP80?serial=VKP80%20200Num.:%200";

    // create a file and send it to the printer's path
    // if the path is not available/error you should handle it here.
    let file = File::<fs::File>::from_path(path)?;

    // prepare the variable
    let mut printer = Printer::new(file, None, None);

    printer
        .chain_font("C")?
        .chain_align("lt")?
        .chain_style("bu")?
        .chain_size(0, 0)?
        .chain_text("The quick brown fox jumps over the lazy dog")?
        .chain_text("敏捷的棕色狐狸跳过懒狗")?
        .chain_barcode("12345678", "EAN8", "", "", 0, 0)?
        .chain_feed(1)?
        .chain_cut(false)?
        .flush()
}

