use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn cpu(_: TokenStream) -> TokenStream {
    let vendor = std::fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|x| {
            x.lines()
                .filter_map(|line| {
                    let mut split = line.split(':');
                    let field1 = split.next()?;
                    let field2 = split.next()?;
                    Some((field1.trim(), field2.trim()))
                })
                .find(|(x, _)| *x == "model name")
                .map(|(_, x)| x.to_owned())
        })
        .unwrap_or_else(|| "Unknown CPU".to_string());

    quote!({ #vendor }).into()
}

#[proc_macro]
pub fn compiler(_: TokenStream) -> TokenStream {
    let mut version = std::process::Command::new("rustc")
        .arg("--version")
        .output()
        .expect("failed to get rustc version")
        .stdout;
    if version.last().cloned() == Some(b'\n') {
        version.pop();
    }

    let version = String::from_utf8(version).expect("not UTF-8 output from rustc --version");

    quote!({ #version }).into()
}

#[proc_macro]
pub fn os(_: TokenStream) -> TokenStream {
    let mut uname = std::process::Command::new("uname")
        .arg("-r")
        .output()
        .expect("failed to fetch output")
        .stdout;

    if uname.last().cloned() == Some(b'\n') {
        uname.pop();
    }

    let linux = String::from_utf8(uname).expect("cannot convert uname -r to utf-8");

    let ubuntu = std::fs::read_to_string("/etc/lsb-release");
    let ubuntu = ubuntu
        .ok()
        .and_then(|x| {
            x.lines()
                .filter_map(|line| {
                    let mut split = line.split('=');
                    let field1 = split.next()?;
                    let field2 = split.next()?;
                    Some((field1.trim(), field2.trim()))
                })
                .find(|(_, x)| *x == "DISTRIB_RELEASE")
                .map(|(_, x)| x.to_owned())
        })
        .unwrap_or_else(|| "Unknown".to_string());

    let final_info = format!("Linux: {}, Ubuntu: {}", linux, ubuntu);

    quote!({ #final_info }).into()
}
