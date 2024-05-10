use std::env;
use std::process::Command;

use which::which;

pub fn generate(root_dir: &str, mapping: Option<&String>) {
    if env::var("SNAP").is_ok() {
        let busctl = which("busctl");

        if busctl.is_err() {
            println!("missing busctl utility");
            return;
        }

        let cmd = Command::new(busctl.unwrap())
            .arg("call")
            .arg("--quiet")
            .arg("--system")
            .arg("io.netplan.Netplan")
            .arg("/io/netplan/Netplan")
            .arg("io.netplan.Netplan")
            .arg("Generate")
            .output()
            .expect("busctl failed");

        if let Some(code) = cmd.status.code() {
            if code == 130 {
                println!("PermissionError: failed to communicate with dbus service");
            } else if code != 0 {
                println!("RuntimeError: failed to communicate with dbus service: error {code}");
            }
        }

        return;
    }

    let generate_path;

    if let Ok(path) = env::var("NETPLAN_GENERATE_PATH") {
        generate_path = path;
    } else {
        generate_path = "/usr/libexec/netplan/generate".to_string();
    }

    let mut cmd = Command::new(generate_path);

    cmd.arg("--root-dir").arg(root_dir);

    if let Some(map) = mapping {
        cmd.arg("--mapping").arg(map);
    }

    let res = cmd.output().expect("generate failed");

    if res.stdout.len() > 0 {
        if let Ok(stdout) = String::from_utf8(res.stdout.to_owned()) {
            println!("{stdout}");
        }
    }

    if res.stderr.len() > 0 {
        if let Ok(stderr) = String::from_utf8(res.stderr) {
            println!("{stderr}");
        }
    }

    if mapping.is_none() {
        if let Ok(systemctl) = which("systemctl") {
            if let Ok(status) = Command::new(systemctl)
                .arg("daemon-reload")
                .arg("--no--ask-password")
                .output()
            {
                if let Some(code) = status.status.code() {
                    if code != 0 {
                        println!("systemctl daemon-reload failed with exit code {code}");
                    }
                }
            }
        }
    }
}
