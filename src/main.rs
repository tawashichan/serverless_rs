
use lambda_runtime::*;
use serde_derive::*;

use lambda_runtime::error::HandlerError;
use std::error::Error;

#[derive(Deserialize, Clone)]
struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
}
 
#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}
 
fn main() -> Result<(), Box<dyn Error>> {
    lambda!(my_handler);
    Ok(())
}
 
fn my_handler(e: CustomEvent, _c: lambda_runtime::Context) -> Result<CustomOutput, HandlerError> {
    Ok(CustomOutput {
        message: format!("Hello, {}!", e.first_name),
    })
}