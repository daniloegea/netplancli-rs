use ::libnetplan::utils::netplan_create_yaml_patch;
use libnetplan::parser::Parser;
use libnetplan::state::State;

pub fn set(key_value: &str, root_dir: &str) {
    let keyvalue_split: Vec<&str> = key_value.split("=").collect();

    if keyvalue_split.len() != 2 {
        println!("set error invalid key value");
        return;
    }

    let mut key = keyvalue_split[0].to_string();
    let value = keyvalue_split[1].to_string();

    if !key.starts_with("network") {
        key = format!("network.{key}");
    }

    let parser = Parser::new();

    let yaml_patch = netplan_create_yaml_patch(&key, &value).unwrap();

    parser.load_nullable_fields(&yaml_patch).unwrap();

    if let Err(error) = parser.load_yaml_hierarchy(root_dir) {
        println!("error: {error:?}");
        return;
    }

    parser.load_yaml_from_string(&yaml_patch).unwrap();

    let state = State::new();
    _ = state.import_parser_state(parser);

    state
        .update_yaml_hierarchy("netplan.yaml", root_dir)
        .unwrap()
}
