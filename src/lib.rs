use std::ffi::{c_char, c_double, c_int};

#[cfg(target_os = "windows")]
use winapi::um::winnt::{
    DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH,
};

mod data_structures;
mod functions;

/// Entrypoint for the DLL, more info [here](https://learn.microsoft.com/en-us/windows/win32/dlls/dllmain).
///
/// Rust version inspired by [this](https://github.com/rust-lang/rust/issues/67399#issue-539755149).
#[cfg(target_os = "windows")]
#[no_mangle]
pub extern "system" fn DllMain(_: *const u8, ul_reason_for_call: u32, _: *const u8) -> u32 {
    match ul_reason_for_call {
        DLL_PROCESS_ATTACH => (),
        DLL_PROCESS_DETACH => (),
        DLL_THREAD_ATTACH => (),
        DLL_THREAD_DETACH => (),
        _ => unreachable!(
            "DllMain called with unknown reason {:?}",
            ul_reason_for_call
        ),
    }
    1
}

/// This is called when the ray hits the surface that uses the diffraction DLL.
///
/// # Safety
///
/// This function is unsafe because it dereferences a raw pointer and writes to it, so Zemax can fetch results from it.
#[no_mangle]
pub unsafe extern "C" fn UserDiffraction(data_pointer: *mut c_double) -> c_int {
    let data = unsafe {
        data_pointer
            .cast::<data_structures::DiffractiveData>()
            .as_mut()
    }
    .expect("pointer is not NULL");

    // Place 95% of the energy in the transmitted path, 5% in the reflected path
    data.relative_energy = if data.is_reflective == 0.0 {
        functions::get_total_power(data.ending_order, data.starting_order) * 0.95
    } else {
        functions::get_total_power(data.ending_order, data.starting_order) * 0.05
    };

    // Also return the phase and derivatives
    data.return_flag = 1.0;
    data.phase_derivatives = [
        0.0,
        data.current_order * data.lines_per_um * data.wavelength_um,
    ];
    data.phase_shift = data.phase_derivatives[1] * data.position[1];

    0
}

/// This is called to load the parameter names and when the user changes their value.
///
/// # Safety
///
/// This function is unsafe because it dereferences a raw pointer and writes to it, so Zemax can fetch results from it.
#[no_mangle]
pub unsafe extern "C" fn UserParamNames(param_data_pointer: *mut c_char) -> c_int {
    // TODO: What is the correct max length of the pointer?
    let param_data = unsafe { std::slice::from_raw_parts_mut(param_data_pointer, 31) };
    let data_num = param_data[0] as usize;
    unsafe { param_data_pointer.copy_from(functions::str_to_mut_i8_ptr(""), 1) };
    if data_num == 1 {
        unsafe { param_data_pointer.copy_from(functions::str_to_mut_i8_ptr("lines/um"), 9) }
    }

    0
}
