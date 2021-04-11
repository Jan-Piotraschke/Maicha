#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::databases::diesel;
use std::collections::HashMap;

#[database("sqlite_logs")]
struct LogsDbConn(diesel::SqliteConnection);

/*
struct --> data / input structure
*/
#[derive(FromForm)]
struct LoginData {
    username: String,
    password: String,
}

/*
Functions can return values to the code that calls them. We don’t name
return values, but we do declare their type after an arrow (->).
In Rust, the return value of the function is synonymous with the value
of the final expression in the block of the body of a function
*/
#[get("/")]
fn index(
    cookies: Cookies
) -> Result<Template, Redirect> {
    // Redirect unauthorized users to the login page
    if !is_user_logged_in(&cookies) {
        Err(Redirect::found("/login"))
    } else {
        let mut context = HashMap::new();
        context.insert("username", cookies.get("username").unwrap().value());

        Ok(Template::render("index", context))
    }
}

#[get("/blog")]
fn blog_form(
    cookies: Cookies
) -> Result<Template, Redirect> {
    // Redirect unauthorized users to the login page
    if !is_user_logged_in(&cookies) {
        Err(Redirect::found("/login"))
    } else {
        let mut context = HashMap::new();
        context.insert("username", cookies.get("username").unwrap().value());
        Ok(Template::render("blog", context))
    }
}

#[get("/login")]
fn login_form(
    cookies: Cookies
) -> Result<Template, Redirect> {
    // Redirect already logged in user to the index page
    if is_user_logged_in(&cookies) {
        Err(Redirect::found("/"))
    } else {
        let context: HashMap<&str, &str> = HashMap::new();

        Ok(Template::render("login", context))
    }
}

/*
the input of the Jinja-like templates are defined in a `struct`

To tell Rocket that you're expecting request body data, the data route
argument is used with the name of the parameter in the request handler
*/
#[post("/login", data = "<login_data>")]
fn login_verify(
    login_data: Form<LoginData>,
    mut cookies: Cookies
) -> Result<Template, Redirect> {
    // Redirect already logged in user to the index page
    if is_user_logged_in(&cookies) {
        Err(Redirect::to("/"))
    } else if !is_login_data_valid(&login_data) {
        let mut context = HashMap::new();
        context.insert("error", "Invalid credentials!");
        context.insert("username", &login_data.username);

        Ok(Template::render("login", context))
    } else {
        cookies.add(Cookie::new("username", login_data.into_inner().username));

        Err(Redirect::to("/"))
    }
}


#[get("/logout")]
fn logout(
    mut cookies: Cookies
) -> Redirect {
    cookies.remove(Cookie::named("username"));

    Redirect::to("/login")
}

/* COORDINATE the web app

the function which coordinates the web application which it`s settings
*/
fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .attach(LogsDbConn::fairing())
        .mount("/", routes![index, blog_form, login_form, login_verify, logout])
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static"))) // include usage of static files
        .launch();
}

fn is_login_data_valid(login_data: &Form<LoginData>) -> bool {
    login_data.username == "JNP" && login_data.password != ""
}

fn is_user_logged_in(cookies: &Cookies) -> bool {
    cookies.get("username").is_some()
}

