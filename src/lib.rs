#[no_mangle]
pub unsafe extern "C" fn api_init(port: u16) {
    match era_server::init(port) {
        Ok(_) => (),
        Err(err) => {
            println!("API failed: {:?}", err);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn api_init_thread(port: u16) {
    std::thread::spawn(move || api_init(port));
}

#[no_mangle]
pub unsafe extern "C" fn is_free(port: u16) -> bool {
    era_server::utils::is_free(port)
}

#[no_mangle]
pub unsafe extern "C" fn api_init_thread_find() -> u16 {
    for i in 6770..u16::MAX {
        if is_free(i) {
            api_init_thread(i);
            return i;
        }
    }
    return 0;
}