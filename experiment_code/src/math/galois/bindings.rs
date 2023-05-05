/* automatically generated by rust-bindgen 0.64.0 */

// Manually added attributes to suppress some warnings.
#![allow(dead_code)]

extern "C" {
    pub fn galois_single_multiply(
        a: ::std::os::raw::c_int,
        b: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn galois_single_divide(
        a: ::std::os::raw::c_int,
        b: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn galois_log(
        value: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn galois_ilog(
        value: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn galois_create_log_tables(w: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn galois_logtable_multiply(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn galois_logtable_divide(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn galois_create_mult_tables(w: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn galois_multtable_multiply(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn galois_multtable_divide(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn galois_shift_multiply(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn galois_shift_divide(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn galois_create_split_w8_tables() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn galois_split_w8_multiply(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn galois_inverse(
        x: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn galois_shift_inverse(
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn galois_get_mult_table(w: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn galois_get_div_table(w: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn galois_get_log_table(w: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn galois_get_ilog_table(w: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn galois_region_xor(
        r1: *mut ::std::os::raw::c_char,
        r2: *mut ::std::os::raw::c_char,
        r3: *mut ::std::os::raw::c_char,
        nbytes: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn galois_w08_region_multiply(
        region: *mut ::std::os::raw::c_char,
        multby: ::std::os::raw::c_int,
        nbytes: ::std::os::raw::c_int,
        r2: *mut ::std::os::raw::c_char,
        add: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn galois_w16_region_multiply(
        region: *mut ::std::os::raw::c_char,
        multby: ::std::os::raw::c_int,
        nbytes: ::std::os::raw::c_int,
        r2: *mut ::std::os::raw::c_char,
        add: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn galois_w32_region_multiply(
        region: *mut ::std::os::raw::c_char,
        multby: ::std::os::raw::c_int,
        nbytes: ::std::os::raw::c_int,
        r2: *mut ::std::os::raw::c_char,
        add: ::std::os::raw::c_int,
    );
}