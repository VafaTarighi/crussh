use std::ffi::CString;

use crate::utils::extract_shell_ident;


#[derive(Debug, PartialEq)]
pub(crate) struct FileName(String);

impl FileName {
    
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        extract_shell_ident(s)
            .map(|(s, filename)| {
                (s, Self(filename.to_string()))
        })
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0.as_str()
    }

    pub(crate) fn as_cstring(&self) -> CString {
        CString::new(self.0.clone()).unwrap()
    }
}