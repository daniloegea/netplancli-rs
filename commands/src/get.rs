use libnetplan::parser::Parser;
use libnetplan::state::State;

pub fn get(key: &str, root_dir: &str) {
    let mut parser = Parser::new();
    if let Err(error) = parser.load_yaml_hierarchy(root_dir) {
        println!("error: {error:?}");
        return;
    }

    let state = match State::try_from(parser) {
        Ok(s) => s,
        Err(_) => {
            println!("Can't import parser state");
            return;
        }
    };

    if key == "all" {
        let yaml = state.dump_yaml().unwrap();
        print!("{yaml}");
    } else {
        let yaml = state.dump_yaml_subtree(key).unwrap();
        print!("{yaml}");
    }
}
