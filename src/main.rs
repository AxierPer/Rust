#[macro_use]
extern crate diesel;
pub mod schema;
pub mod models;
pub mod funciones;


fn main() {
    funciones::edit_data();
    // funciones::add_data("Mi cuarto blog post".to_string(), "cuarto Post".to_string(), "Lorem Ipsum".to_string());
    funciones::select_data();

}