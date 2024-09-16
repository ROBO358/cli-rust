use std::path::Path;

pub fn handle_debug(debug_level: u8) {
    match debug_level {
        0 => (),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }
}

pub fn handle_config(config_path: &Path) {
    println!("Value for config: {}", config_path.display());
    // ここに設定ファイルの処理ロジックを追加できます
}
