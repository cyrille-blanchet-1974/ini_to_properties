use std::env;
use std::fs::File;

#[derive(Debug)]
pub struct Paramcli
//parameters from command line and/or confFile
{
    pub fic: String,
}

impl Default for Paramcli {
    fn default() -> Self {
        Paramcli::new()
    }
}

impl Paramcli {
    pub fn new() -> Paramcli {
        let mut fic = String::new();

        let args: Vec<String> = env::args().skip(1).collect();
        let name = env::args()
            .take(1)
            .next()
            .unwrap_or_else(|| String::from("ini_to_properties"));
        println!("{} 1.0 (2021)", name);
        if args.is_empty() {
            help(&name);
        }
        for arg in args {
            if arg == "/?"
                || arg == "-?"
                || arg.to_lowercase() == "/help"
                || arg.to_lowercase() == "-help"
            {
                help(&name);
            }
            if let Some(n) = get_param(&arg, String::from("/fic:")) {
                fic = n;
                continue;
            }
        }
        //checks
        if fic.is_empty() {
            println!("ERROR! no file to work with!");
            println!("--------------------------------------------------");
            help(&name);
        }
        if !fic.ends_with(".ini") {
            fic.push_str(".ini");
        }
        //check if file exists
        if File::open(&fic).is_err() {
            println!("Error file {} doesn't exists or unereadable", &fic);
            help(&name);
        };
        Paramcli { fic }
    }
}

fn get_param(arg: &str, switch: String) -> Option<String> {
    if arg.to_lowercase().starts_with(&switch) {
        let mut a = String::from(arg);
        return Some(a.split_off(switch.len()));
    }
    None
}

fn help(name: &str) {
    println!("syntax : {} /fic:file", name);
    println!("paramerters between [] are optionnals");
    println!("------------------------------------");
    println!("fic: ini file");
    println!("output file will have the same name only changing extention");
    std::process::exit(0);
}
