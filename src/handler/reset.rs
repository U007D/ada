#[allow(unsafe_code, clippy::missing_const_for_fn)]
#[no_mangle]
pub extern "C" fn reset_handler() -> ! {
    let _x = 42;

    #[allow(clippy::empty_loop)]
    loop {}
}

// The reset vector, a pointer to the reset handler
#[link_section = ".vector_table.reset_vector"]
#[allow(unsafe_code)]
#[no_mangle]
pub static RESET_VECTOR: extern "C" fn() -> ! = reset_handler;
