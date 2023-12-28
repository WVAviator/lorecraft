use log::info;

pub struct Logger {}

impl Logger {
    pub fn setup(app: &mut tauri::App) {
        let log_file = app
            .path_resolver()
            .app_log_dir()
            .expect("Failed to identify log directory.")
            .join("debug.log");

        std::fs::create_dir_all(log_file.parent().unwrap()).expect("Failed to create log dir.");

        fern::Dispatch::new()
            .chain(
                fern::Dispatch::new()
                    .format(|out, message, record| {
                        out.finish(format_args!("[{}] {}", record.level(), message,))
                    })
                    .level(log::LevelFilter::Info)
                    .chain(std::io::stdout()),
            )
            .chain(
                fern::Dispatch::new()
                    .format(|out, message, record| {
                        out.finish(format_args!(
                            "[{} | {}] {}",
                            record.level(),
                            record.target(),
                            message,
                        ))
                    })
                    .level(log::LevelFilter::Trace)
                    .chain(fern::log_file(&log_file).expect("Failed to create log file.")),
            )
            .apply()
            .expect("Failed to initialize logger.");
        info!("Log file initialized at '{}'.", log_file.display());
    }
}
