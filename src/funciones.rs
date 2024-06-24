#[macro_use]
use crate:: diesel;

use dotenv::dotenv;
// use schema::posts;
use std::env;
use super::models::{Post, PostSimplificado,NewPost};


use diesel::prelude::*;
use diesel::pg::PgConnection;

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DB URL no encontrada");
    PgConnection::establish(&database_url).expect("Error al conectarse a la base de datos")
}

pub fn select_data() {
    use super::schema::posts::dsl::*;
    // use self::schema::posts;
    let mut conn = establish_connection();

    // //TODO: SELECT * FROM posts limit 
    // let posts_result_l = posts.limit(1).load::<Post>(&mut conn).expect("Error al ejecutar la consulta");
    // for post in posts_result_l {
    //     println!("Datos limitados de la base de datos\n{:?}", post);
    // }

    // //TODO: SELECT * FROM posts limit order by
    // let posts_result_l = posts.order(id.desc()).limit(1).load::<Post>(&mut conn).expect("Error al ejecutar la consulta");
    // for post in posts_result_l {
    //     println!("Datos limitados y ordenado de manera descendiente de la base de datos\n{:?}", post);
    // }

    //TODO: SELECT * FROM posts 
    let posts_result_nl = posts.order(id.asc()).load::<Post>(&mut conn).expect("Error al ejecutar la consulta");
    for post in posts_result_nl {
        println!("Datos completos de la base de datos\n{:?}", post);
    }

    // //TODO: SELECT title, body FROM posts 
    // let posts_result= posts.select((title, body)).limit(1).load::<PostSimplificado>(&mut conn).expect("Error al ejecutar la consulta");

    // for post in posts_result {
    //     println!("ciertos datos de la base de datos\n{:?}", post);
    // }

    // //TODO: SELECT * FROM posts where
    // let posts_result_l = posts.filter(slug.eq("tercer Post")).load::<Post>(&mut conn).expect("Error al ejecutar la consulta");
    // for post in posts_result_l {
    //     println!("Datos filtrados de la base de datos\n{:?}", post);
    // }
    
}

pub fn add_data(titulo: String, slu:String, cuerpo:String) {
    let mut conn = establish_connection();
    use super::schema::posts::dsl::*;
    // use super::schema::posts;

    let new_post = NewPost {
        title: &titulo,
        slug: &slu,
        body: &cuerpo,
    };

    diesel::insert_into(posts).values(&new_post).execute(&mut conn).expect("Error al agregar");
}

pub fn edit_data(){
    let mut conn = establish_connection();
    use super::schema::posts::dsl::*;

    diesel::update(posts.filter(id.eq(4))).set(slug.eq("cuarto Post")).get_result::<Post>(&mut conn).expect("Fallo al actualizar datos");
}