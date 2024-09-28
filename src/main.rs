use furbrowser_rs::interactive;

fn main() {
    if let Err(e) = interactive() {
        eprintln!("error: {e}")
    }
}
