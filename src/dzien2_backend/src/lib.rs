#[ic_cdk::query]
fn greet(name: String, last_name: String, wiek : i8) -> String {
    format!("Hello, {} {} {} !", name,last_name,wiek)
}
