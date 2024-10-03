#![allow(non_snake_case)]

fn main() 
{
    slint_build::compile_with_config(
        "ui/appwindow.slint", 
        slint_build::CompilerConfiguration::new().with_style("material-dark".into())
    ).unwrap();
}
