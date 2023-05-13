use std::ffi::OsString;

pub fn _parse() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<OsString> = std::env::args_os().collect();
    let mut strings: Vec<String> = vec![];
    for v in &a {
        let x = v.clone().into_string();
        match x {
            Ok(s) => {strings.push(s.to_string())},
            Err(e) => {
                return Err(
                    Box::new(
                        std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Invalid unicode in args. ({:?})", e)
                        )
                    )
                );
            }
        }
    }
    for s in strings {
        let split_s = s.split("").collect::<Vec<&str>>();
        let mut better_s = vec![];
        for bs in &split_s {
            if bs != &"" {
                better_s.push(bs);
            };
        };
        match better_s.get(0).to_result("msg")?.to_owned() {
            &"-" => {
                match better_s[1] {
                    &"-" => {
                        todo!()
                    }
                    _ => {
                        todo!()
                    }
                }
            }
            _ => {
                todo!()
            }
        }
    }
    Ok(())
}

trait OptionToResult<T> {
    fn to_result(self, msg: &str) -> Result<T, Box<std::io::Error>>;
}

impl<T> OptionToResult<T> for Option<T> {
    fn to_result(self, msg: &str) -> Result<T, Box<std::io::Error>> {
        match self {
            Some(v) => {
                Ok(v)
            },
            None => {
                return Err(
                    Box::new(
                        std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Error: {} (Expected Some(), found None)", msg)
                        )
                    )
                );
            }
        }
    }
}