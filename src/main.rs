use rocket::{get,launch,routes};

// fn main() {
//     println!("Hello, world!");
// }

#[get("/")]
fn index()->&'static str{
   " Hello, world!"
}

#[launch]
fn rocket()-> _ {
    rocket::build().mount("/",routes![index])
}