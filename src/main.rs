slint::include_modules!();

fn main() {
    let main_window = GeneralWindow::new().unwrap();

    // callback for language changing
    main_window.on_language_changed(move |index| {
        let lang_str = match index {
            1 => "ru",
            _ => "en",
        };

        if let Err(e) = slint::select_bundled_translation(lang_str) {
            eprintln!("Couldn't select translate: {}", e);
        }
    });


    main_window
        .window()
        .set_size(slint::LogicalSize::new(405., 600.));
    main_window.run().unwrap();

}
