// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

use slint::SharedString;
mod disk_functions;

const CACHE_FILE: &str = "cache.txt";

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    // Listar e arrumar texto dos discos
    let disk_names = disk_functions::get_disks();
    let drivers_text = disk_names.join("\n");
    
    main_window.set_drivers(SharedString::from(drivers_text));

    // Salvar tudo em cache
    let _drives = disk_functions::load_cache(CACHE_FILE);

    let ui_handle = main_window.as_weak();
    main_window.on_find_in(move | file_name: SharedString | {
        let main_window = ui_handle.unwrap();
        
        let option_folder = main_window.get_toggle_1();
        let option_file = main_window.get_toggle_2();
        let text = disk_functions::find_in(file_name.as_str(),option_folder,option_file);
        main_window.set_locations(text);
    });


    main_window.run()
}
