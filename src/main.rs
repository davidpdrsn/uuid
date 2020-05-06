use clipboard::{ClipboardContext, ClipboardProvider};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "uuid")]
struct Opt {
    /// Copy UUID to the clipboard
    #[structopt(short)]
    copy: bool,
}

fn main() {
    let opt = Opt::from_args();

    let uuid = uuid::Uuid::new_v4();

    if opt.copy {
        let mut clipboard = ClipboardContext::new().unwrap();
        clipboard.set_contents(uuid.to_string()).unwrap();
    }

    println!("{}", uuid);
}
