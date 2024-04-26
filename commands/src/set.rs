use ::libnetplan::utils::netplan_create_yaml_patch;
use libnetplan::parser::Parser;
use libnetplan::state::State;

pub fn set(key_value: &str, origin_hint: Option<&String>, root_dir: Option<&String>) {
    let keyvalue_split: Vec<&str> = key_value.split("=").collect();

    if keyvalue_split.len() != 2 {
        println!("set error invalid key value");
        return;
    }

    let filename = [origin_hint.unwrap(), "yaml"].join(".");
    let rootdir = root_dir.unwrap();

    let key = keyvalue_split[0];
    let value = keyvalue_split[1];

    let full_key = if key.starts_with("network") {
        key.to_string()
    } else {
        format!("network.{key}")
    };

    let yaml_patch = netplan_create_yaml_patch(&full_key, &value).unwrap();

    if filename != "70-netplan-set.yaml" {
        let mut parser = Parser::new();
        parser.load_nullable_fields(&yaml_patch).unwrap();

        parser
            .load_nullable_overrides(&yaml_patch, &filename)
            .unwrap();

        if let Err(error) = parser.load_yaml_hierarchy(rootdir) {
            println!("error: {error:?}");
            return;
        }
        parser.load_yaml_from_string(&yaml_patch).unwrap();

        let state_output_file = State::new();
        let _ = state_output_file.import_parser_state(parser);
        let _ = state_output_file.write_yaml_file(&filename, rootdir);
    } else {
        let mut parser = Parser::new();
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

        state.update_yaml_hierarchy(&filename, rootdir).unwrap()
    }
}
