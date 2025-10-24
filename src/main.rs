slint::include_modules!();

fn main() {
    let main_window = GeneralWindow::new().unwrap();
    main_window
        .window()
        .set_size(slint::LogicalSize::new(405., 600.));
    main_window.run().unwrap();
}
