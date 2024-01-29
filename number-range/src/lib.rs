use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn numrng_unsigned(rng: *const c_char, len: *mut u32) -> *mut u32 {
    let rng = unsafe { CStr::from_ptr(rng).to_string_lossy() };

    let mut numbers = match number_range::NumberRangeOptions::default()
        .with_range_sep('-')
        .parse(&rng)
    {
        Ok(n) => n.collect(),
        Err(e) => {
            eprintln!("Parsing Failed: {}", e);
            vec![]
        }
    };
    unsafe {
        *len = numbers.len() as u32;
    }
    let n = numbers.as_mut_ptr();
    std::mem::forget(numbers);
    n
}

#[no_mangle]
pub extern "C" fn numrng_signed(rng: *const c_char, len: *mut u32) -> *mut i32 {
    let rng = unsafe { CStr::from_ptr(rng).to_string_lossy() };

    let mut numbers = match number_range::NumberRangeOptions::default().parse(&rng) {
        Ok(n) => n.collect(),
        Err(e) => {
            eprintln!("Parsing Failed: {}", e);
            vec![]
        }
    };
    unsafe {
        *len = numbers.len() as u32;
    }
    let n = numbers.as_mut_ptr();
    std::mem::forget(numbers);
    n
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
