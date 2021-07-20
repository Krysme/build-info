use proc_macro::TokenStream;
use quote::ToTokens;

fn command(cmd: &str, args: &[&str]) -> Option<String> {
    std::process::Command::new(cmd)
        .args(args)
        .output()
        .map(|out| -> Vec<u8> { out.stdout })
        .ok()
        .and_then(|mut out| {
            if out.last().cloned() == Some(b'\n') {
                out.pop();
            }
            String::from_utf8(out).ok()
        })
}

#[proc_macro]
pub fn cpu(_: TokenStream) -> TokenStream {
    let s = command("sh", &["-c", "grep 'model name' /proc/cpuinfo | head -n 1"])
        .expect("cannot fetch cpu info, check /proc/cpuinfo");

    s.split(':')
        .nth(1)
        .expect("wrong info in /proc/cpuinfo")
        .trim()
        .into_token_stream()
        .into()
}

#[proc_macro]
/// rustc --version
pub fn compiler(_: TokenStream) -> TokenStream {
    command("rustc", &["--version"])
        .expect("cannot get rustc --version")
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn os(_: TokenStream) -> TokenStream {
    let linux = command("uname", &["-r"]).expect("failed to run uname -r");

    let ubuntu = command("grep", &["DISTRIB_RELEASE", "/etc/lsb-release"]).unwrap_or_default();

    let unknown = "Unknown";
    format!(
        "Linux: {}, Ubuntu: {}",
        linux,
        if let Some(ubuntu) = ubuntu.split('=').nth(1) {
            ubuntu.trim()
        } else {
            unknown
        }
    )
    .into_token_stream()
    .into()
}

#[proc_macro]
pub fn date_time(_: TokenStream) -> TokenStream {
    command("date", &[])
        .expect("cannot get date")
        .into_token_stream()
        .into()
}

#[proc_macro]
pub fn ip(_: TokenStream) -> TokenStream {
    static IP_COMMAND: &str = r#"ifconfig | grep inet[[:space:]] | awk ' { print $2 } ' | grep -v '127\.0\.0\.1' | head -n 1"#;
    command("sh", &["-c", IP_COMMAND])
        .expect("cannot get ip")
        .into_token_stream()
        .into()
}
