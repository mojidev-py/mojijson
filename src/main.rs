use json::json_parser_main;

mod json;


fn main() {
    let parsercontent = json_parser_main::JsonObject::from_line("{goop:true}".to_string());
    println!("{parsercontent:#?}")
}