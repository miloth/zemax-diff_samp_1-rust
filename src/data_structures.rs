use std::ffi::c_double;

/// Data structure to interpret the data returned by the DLL for a diffractive surface.
#[repr(C, align(8))]
#[derive(Debug)]
pub struct DiffractiveData {
    length: f64,                         // data 0 = overall length of the data structure
    pub position: [f64; 3], // data 1-3 = position of the ray in the local coordinate system
    pub cosine: [f64; 3],   // data 4-6 = cosine of the ray in the local coordinate system
    pub cosine_surface_normal: [f64; 3], // data 7-9 = cosine of the surface normal in the local coordinate system
    pub wavelength_um: f64,              // data 10 = wavelength in micrometers
    pub is_reflective: f64, // data 11 = 0 if refractive surface, 1 for reflective. May be set by DLL if data[31] = 1 or 2
    pub index_in: f64,      // data 12 = index of refraction, approaching side
    pub index_out: f64,     // data 13 = index of refraction, exit side
    pub current_order: f64, // data 14 = current order of the diffractive surface
    pub starting_order: f64, // data 15 = starting order of the diffractive surface
    pub ending_order: f64,  // data 16 = ending order of the diffractive surface
    pub mm_per_unit: f64, // data 17 = millimeters per user unit (1.0 for mm, 25.4 for inches, 10.0 for cm and 1000.0 for meters)
    pub seed: f64,        // data 18 = a random value to use as a seed

    _unknown_field_19: f64, // data 19 = unknown field

    pub electric_field_x: [f64; 2], // data 20-21 = electric field incident x-component
    pub electric_field_y: [f64; 2], // data 22-23 = electric field incident y-component
    pub electric_field_z: [f64; 2], // data 24-25 = electric field incident z-component

    _unknown_fields_26_29: [f64; 4], // data 26-28 = unknown fields

    pub relative_energy: f64, // data 30 = relative energy of the ray
    pub return_flag: f64, // data 31 = 1 if DLL returns phase and phase derivatives; = 2 if complete output ray data

    pub phase_shift: f64, // data 32 = output phase added to the ray in radians
    pub phase_derivatives: [f64; 2], // data 33-34 = phase derivatives with respect to x and y

    pub cosine_out: [f64; 3], // data 35-37 = cosine of the ray in the local coordinate system after the surface

    _unknown_fields_38_39: [f64; 2], // data 38-39 = unknown fields

    pub electric_field_x_out: [f64; 2], // data 40-41 = electric field outgoing x-component
    pub electric_field_y_out: [f64; 2], // data 42-43 = electric field outgoing y-component
    pub electric_field_z_out: [f64; 2], // data 44-45 = electric field outgoing z-component

    _reserved_for_future_use: [f64; 5], // data 46-50 = reserved for future use

    pub lines_per_um: f64, // data 51 = lines per um for the reflected ray
    _lines_per_um_refracted: f64, // data 52 = lines per um for the refracted ray, same as the one above
}

impl DiffractiveData {
    pub fn from_pointer(data_pointer: &*mut c_double) -> Option<&mut Self> {
        unsafe { data_pointer.cast::<DiffractiveData>().as_mut() }
    }
}
