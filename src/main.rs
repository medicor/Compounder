#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]

//use std::error::Error;

slint::include_modules!();

fn main() 
{
    let window = AppWindow::new().unwrap();
    let handle = window.as_weak();

    window.on_change_counter (move || {
        let root = handle.unwrap(); // .upgrade(). needed?
        root.set_counter(root.get_counter() + 1);
    });

    window.run().unwrap();
}

// fn main() -> Result<(), Box<dyn Error>>
// {
//     let ui = AppWindow::new()?;

//     ui.on_request_increase({
//         let uiHandle = ui.as_weak();
//         move || {
//             let ui = uiHandle.unwrap();
//             ui.set_counter(ui.get_counter() + 1);
//         }
//     });

//     ui.run()?;
//     Ok(())
// }
