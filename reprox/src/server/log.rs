#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        use std::io::Write;

        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("server.log")
            .unwrap();

        writeln!(file, $($arg)*).unwrap();
        let v = std::env::var("REPROX_LOG_MODE");
        if v.is_ok() {
            match v.unwrap().as_str() {
                "true" => {
                    writeln!(std::io::stdout(), $($arg)*).unwrap()
                }
                _ => {}
            }
        }
    }};
}