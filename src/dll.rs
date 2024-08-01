use crate::utils::{init_contents, is32_bits_dll};

#[derive(Debug)]
pub struct Dll {
    pub path: String,
    pub architecture: i32,
}

impl Dll {
    pub fn new(path: &str) -> Result<Dll, std::io::Error> {
        let buffer = init_contents(path)?;
        Ok(Dll {
            path: path.to_string(),
            architecture: if is32_bits_dll(buffer.clone()) {
                32
            } else {
                64
            },
        })
    }
}
