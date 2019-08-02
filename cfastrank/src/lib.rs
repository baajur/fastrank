#![crate_type = "dylib"]
use libc::{c_char, c_void};
use serde_json;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::mem;

use fastrank::dataset::DatasetRef;
use fastrank::model::Model;
use fastrank::{FeatureId, InstanceId};

#[repr(C)]
pub struct CModel {
    inner: Box<Model>,
}
#[repr(C)]
pub struct CDataset {
    inner: Box<DatasetRef>,
}

#[no_mangle]
pub extern "C" fn free_str(originally_from_rust: *mut c_char) {
    let _will_drop: CString = unsafe { CString::from_raw(originally_from_rust) };
}

#[no_mangle]
pub extern "C" fn exec_json(json_cmd_str: *mut c_char) -> *const c_char {
    let json_cmd_str: &CStr = unsafe { CStr::from_ptr(json_cmd_str) };
    let output = match result_exec_json(json_cmd_str) {
        Ok(response) => response,
        Err(e) => format!("Error: {:?}", e),
    };
    let c_output: CString = CString::new(output).expect("Conversion to CString should succeed!");
    CString::into_raw(c_output)
}

fn result_exec_json(query_str: &CStr) -> Result<String, Box<Error>> {
    let query_str: &str = query_str
        .to_str()
        .map_err(|_| "Could not convert your query string to UTF-8!")?;

    let args: serde_json::Value = serde_json::from_str(query_str)?;
    Ok(format!("result_exec_json: {:?}", args))
}