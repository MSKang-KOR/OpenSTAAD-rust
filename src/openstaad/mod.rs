pub mod api;
pub mod new_api;
pub mod tauri;
pub mod tools;
pub use tools::safe_array::{safe_array_from_vec1d, safe_array_from_vec2d};
pub use tools::variant::{SafeArray, SafeArrayP, variant_with_ptr_from};
