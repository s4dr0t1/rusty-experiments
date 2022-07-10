//use std::collections::HashMap; 

//fn query_crtsh(url: &str) -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
fn query_crtsh(url: &str) -> Result<String, Box<dyn std::error::Error>> {

    let url: String = "https://crt.sh/?q=%25.".to_owned() + &url + "&output=json";
    println!("{}", &url);
//    let response = reqwest::blocking::get(&url)?
//        .json::<HashMap<String, String>>()?;

    let response = reqwest::blocking::get(&url)?
        .text()?;
    Ok(response)
}

pub fn enumerate(url: &str) {
    match query_crtsh(url) {
        Ok(success) => {
            println!("Success:\n {}", success);
        },
        Err(error) => println!("Error: {:?}", error),
    }
}

