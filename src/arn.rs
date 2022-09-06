use std::fs;
use configparser::ini::Ini;

pub fn parse_arn_of_profile(profile: &String) -> String {
    // read file
    let mut path = home::home_dir().unwrap();
    path.push(".aws");
    path.push("config");
    let aws_config = fs::read_to_string(path);
    if aws_config.is_err() {
        panic!("AWS Config not found!")
    }
    let aws_config_content = aws_config.ok().unwrap();

    // parse
    let mut config = Ini::new();
    match config.read(aws_config_content) {
        Err(e) => panic!("{:?}", e),
        _ => ()
    }

    // fetch section
    let mut section_name = "profile ".to_string();
    section_name.push_str(profile);    

    let value = config.get(&section_name, "credential_process");
    match value {
        None => {
            panic!("Profile name or key not found")
        },
        _ => ()
    }

    // extract arn
    let value_unwraped = value.unwrap();
    let pos = value_unwraped.rfind(' ');
    if pos.is_none() {
        panic!("Invalid 'credential_process' value");
    }
    let index = pos.unwrap();
    let arn = value_unwraped[(index + 1)..].trim().to_string();
    return arn
}