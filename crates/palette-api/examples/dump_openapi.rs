fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", palette_api::openapi::openapi_json_pretty()?);
    Ok(())
}
