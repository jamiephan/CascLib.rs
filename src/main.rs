use casclib::CASCLIB_VERSION_STRING;

fn main() {
    println!(
        "CascLib Binding Version: {}",
        core::str::from_utf8(CASCLIB_VERSION_STRING).unwrap()
    )
}
