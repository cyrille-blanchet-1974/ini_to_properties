use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread::{spawn, JoinHandle};

pub fn start_thread_conv(from_read: Receiver<String>, to_write: Sender<String>) -> JoinHandle<()> {
    spawn(move || {
        let mut str_section = String::new();
        for l in from_read {
            let mut res = String::new();
            //if starts with ; => replace add # before and push to write
            if l.trim().starts_with(';') {
                res.push_str("# ");
                res.push_str(&l);
            }
            //if start with [  and ends with ]  keep inside in str_section
            else if l.trim().starts_with('[') && l.trim().ends_with(']') {
                str_section = String::new();
                //TODO: remove [ and ]
                str_section.push_str(l.trim());
                str_section = str_section.replace("[", " ");
                str_section = str_section.replace("]", " ");
                str_section = str_section.trim().to_string();
                //add # before and push to write
                res.push_str("# ");
                res.push_str(&l);
            }
            //if trim() = "" send to write
            else if l.trim() == "" {
                res.push_str(&l);
            }
            //else add str_section and '.' before and push to write
            else {
                res.push_str(&str_section);
                res.push('.');
                res.push_str(&l);
            }

            if to_write.send(res).is_err() {
                println!("error sending to write");
                return;
            }
        }
    })
}
