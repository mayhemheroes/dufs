#![no_main]
use libfuzzer_sys::fuzz_target;
use dufs::utils::glob;

fuzz_target!(|data: &[u8]| {
    // Convert the fuzz input bytes to valid UTF-8 strings for pattern and target
    if let Ok(pattern) = std::str::from_utf8(&data[..data.len() / 2]) {
        if let Ok(target) = std::str::from_utf8(&data[data.len() / 2..]) {
            // Call the glob function with the fuzz inputs
            let _ = glob(pattern, target);
        }
    }
});
