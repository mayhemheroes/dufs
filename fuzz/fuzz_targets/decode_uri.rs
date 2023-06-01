#![no_main]


use libfuzzer_sys::fuzz_target;
use dufs::utils::decode_uri;

fuzz_target!(|data: &[u8]| {
    // Convert the fuzz input bytes to a valid UTF-8 string
    if let Ok(input) = std::str::from_utf8(data) {
        // Call the decode_uri function with the fuzz input
        let _ = decode_uri(input);
    }
});
