/*!
Custom Error handling/logging example

*/
extern crate serde_json;

use std::fmt;


trait ErrorLog {
    fn log(self) -> Self;
}


#[derive(Debug)]
enum Error {
    JSON(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match *self {
            JSON(ref e) => write!(f, "JSON [de]serialization error: {}", e),
        }
    }
}


type Result<T> = std::result::Result<T, Error>;

impl<T> ErrorLog for Result<T> {
    fn log(self) -> Self {
        if let Err(ref e) = self {
            println!("LOG -> Error: {}", e);
        }
        self
    }
}


fn read_bad_data() -> Result<()> {
    let bad_data = r#"{
                    "name": "John Doe",
                    "age": 43,
                    "phones": [
                      "+44 1234567",
                      "+44 2345678
                    ]
                  }"#;
    let data: serde_json::Value = serde_json::from_str(bad_data)
        .map_err(Error::JSON).log()?;
    println!("read json: {:?}", data);
    Ok(())
}


fn main() {
    if let Err(e) = read_bad_data() {
        println!("Exiting with error: {}", e);
        std::process::exit(1);
    }
}
