use hlsdk_sys::{BOOL, TRUE};

#[no_mangle]
pub extern "C" fn Meta_Detach(/* TODO */) -> BOOL {
    TRUE
}

#[no_mangle]
pub extern "C" fn Meta_Query(/* TODO */) -> BOOL {
    TRUE
}

#[no_mangle]
pub unsafe extern "C" fn GiveFnptrsToDll(/* TODO */) {}
