use ::libnetplan::utils::netplan_create_yaml_patch;
use libnetplan::parser::Parser;
use libnetplan::state::State;

pub fn set(key_value: &str, origin_hint: Option<&String>, root_dir: Option<&String>) {
    let keyvalue_split: Vec<&str> = key_value.split("=").collect();

    if keyvalue_split.len() != 2 {
        println!("set error invalid key value");
        return;
    }

    let rootdir: &str;
    let originhint: String;

    if root_dir.is_none() {
        rootdir = "/";
    } else {
        rootdir = root_dir.unwrap();
    }

    if origin_hint.is_none() {
        originhint = "70-netplan-set.yaml".to_string();
    } else {
        originhint = [origin_hint.unwrap(), ".yaml"].join(".");
    }

    let mut key = keyvalue_split[0].to_string();
    let value = keyvalue_split[1].to_string();

    if !key.starts_with("network") {
        key = format!("network.{key}");
    }

    let parser = Parser::new();

    let yaml_patch = netplan_create_yaml_patch(&key, &value).unwrap();

    parser.load_nullable_fields(&yaml_patch).unwrap();

    if let Err(error) = parser.load_yaml_hierarchy(rootdir) {
        println!("error: {error:?}");
        return;
    }

    if let Err(_) = parser.load_yaml_from_string(&yaml_patch) {
        println!("Bad yaml patch!");
        return;
    }

    let state = State::new();
    _ = state.import_parser_state(parser);

    if origin_hint.is_some() {
        let parser_output_file = Parser::new();
        parser_output_file.load_nullable_fields(&yaml_patch).unwrap();

        parser_output_file.load_nullable_overrides(&yaml_patch, &originhint).unwrap();

    if let Err(error) = parser_output_file.load_yaml_hierarchy(rootdir) {
        println!("error: {error:?}");
        return;
    }
    parser_output_file.load_yaml_from_string(&yaml_patch).unwrap();

    let state_output_file = State::new();
    let _ = state_output_file.import_parser_state(parser_output_file);
    let _ = state_output_file.write_yaml_file(&originhint, rootdir);

    } else {

        state
            .update_yaml_hierarchy(&originhint, rootdir)
            .unwrap()
    }
}
