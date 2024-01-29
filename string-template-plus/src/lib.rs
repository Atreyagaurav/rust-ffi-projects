use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr::null_mut;
use string_template_plus::{Render, RenderOptions, Template};

macro_rules! return_null_if_err {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(e) => {
                eprintln!("{}", e);
                return null_mut();
            }
        }
    };
}

#[no_mangle]
pub extern "C" fn strtp_render_template(
    str_templ: *const c_char,
    variables: *const c_char,
) -> *mut u8 {
    let str_templ = unsafe { CStr::from_ptr(str_templ).to_string_lossy() };
    let variables = unsafe { CStr::from_ptr(variables).to_string_lossy() };

    let mut ops = RenderOptions::default();
    ops.variables = variables
        .split('\n')
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| l.split_once('=').unwrap_or((l, "")))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    let templ = return_null_if_err!(Template::parse_template(&str_templ));
    let mut text = return_null_if_err!(templ.render(&ops));
    // coz C needs null terminated strings
    text.push('\0');
    let t = text.as_mut_ptr();
    std::mem::forget(text);
    t
}
