use std::process::Command;
use rocket::http::{Status};
#[macro_use] extern crate rocket;


#[get("/?<resource>")]
fn index(resource: &str) -> Result<String, Status>{

    let command = "az";
    let args = ["account", "get-access-token", &format!("--resource={}", resource)];

    let output = Command::new(command)
        .args(&args)
        .output()
        .expect("Error happened while executing az command");

    if output.status.success(){
        let token_response = String::from_utf8(output.stdout).expect("Error parsing token response");
        return Ok(token_response);
    }
    else {
        return Err(Status::BadRequest);
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

