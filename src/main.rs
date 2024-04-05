slint::include_modules!();

fn main() {
    match MainWindow::new() {
        Ok(ui) => {
            if let Err(e) = ui.run() {
                eprintln!("Failed to run the UI: {:?}", e);
            }
        },
        Err(e) => eprintln!("Failed to create MainWindow: {:?}", e),
    }
}
