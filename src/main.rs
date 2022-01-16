mod conv;
mod paramcli;
mod read;
mod write;

use conv::*;
use paramcli::*;
use read::*;
use std::sync::mpsc::channel;
use write::*;

pub fn traitement(p: &Paramcli) {
    let mut fic_out = String::from(&p.fic);
    fic_out = fic_out.replace(".ini", ".properties");

    //MPSC chanels
    let (to_conv, from_read) = channel();
    let (to_write, from_conv) = channel();

    let hread = start_thread_read(to_conv, &p.fic);
    let hconv = start_thread_conv(from_read, to_write);
    let hwrite = start_thread_write(from_conv, &fic_out);

    //wait for threads to stop
    if hread.join().is_err() {
        println!("Thread read finished with error");
    }
    if hconv.join().is_err() {
        println!("Thread conv finished with error");
    }
    if hwrite.join().is_err() {
        println!("Thread write finished with error");
    }
}

fn main() {
    let param = Paramcli::new();
    traitement(&param);
}
