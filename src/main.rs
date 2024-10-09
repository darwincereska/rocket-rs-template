#[macro_use] extern crate rocket;
use rocket::{
	response::{Redirect},
	fs::{FileServer, NamedFile},
	
};

// Redirects to /home route
#[get("/")]
fn index() -> Redirect {
	Redirect::to(uri!("/home"))
}

#[get("/home")]
async fn home() -> Option<NamedFile> {
	let path = "pages/home.html";

	NamedFile::open(path).await.ok()
	
}

// 404 Catcher
#[catch(404)]
async fn not_found() -> Option<NamedFile> {
	let path = "pages/404.html";

	NamedFile::open(path).await.ok()
}



#[launch]
fn rocket() -> _ {
    rocket::build()
    .register("/", catchers![not_found])
    .mount("/", routes![index,home])
    // Serves static files ex: public/styles.css
    .mount("/public", FileServer::from("static"))
}
