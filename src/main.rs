use std::env::args;
use clap::Parser;
use crate::args::PngMeArgs;

mod args;
// mod chunk;
mod chunk_type;
mod png;
mod chunk;
mod commands;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let args = args::Cli::parse();
//     println!("{:?}", args);
//     match args.subcommand {
//         args::PngMeArgs::Encode(encode_args) => commands::encode(encode_args),
//         PngMeArgs::Decode(encode_args) => commands::decode(encode_args),
//         args::PngMeArgs::Remove(remove_args) => commands::remove(remove_args),
//         args::PngMeArgs::Print(print_args) => commands::print(print_args),
//     }
// }
//
// fn main() -> Result<(), slint::PlatformError> {
//     let ui = AppWindow::new()?;
//
//     let ui_handle = ui.as_weak();
//     ui.on_request_increase_value(move || {
//         let ui = ui_handle.unwrap();
//         ui.set_counter(ui.get_counter() + 1);
//     });
//
//     ui.run()
// }


// slint::include_modules!();
//
// fn main() -> Result<(), slint::PlatformError> {
//     let ui = AppWindow::new()?;
//
//     let ui_handle = ui.as_weak();
//     ui.on_request_increase_value(move || {
//         let ui = ui_handle.unwrap();
//         ui.set_counter(ui.get_counter() + 1);
//     });
//
//     ui.run()
// }


fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    export component MainWindow inherits Window {
        Text {
            text: "hello world";
            color: green;
        }
    }
}