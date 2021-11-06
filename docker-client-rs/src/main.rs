extern crate rs_docker;
use std::{thread, time};

use rs_docker::Docker;

fn watch_container_running(container_name: &str) {
    let two_seconds = time::Duration::from_secs(2);

    let mut docker = match Docker::connect("unix:///var/run/docker.sock") {
        Ok(docker) => docker,
        Err(e) => {
            panic!("{}", e);
        }
    };

    loop {
        let containers = match docker.get_containers(false) {
            Ok(containers) => containers,
            Err(e) => {
                panic!("{}", e);
            }
        };

        let container_exists = containers
            .iter()
            .any(|container| container.Names[0] == format!("/{}", &container_name));

        if container_exists {
            thread::sleep(two_seconds);
        } else {
            break;
        }
        println!("{}", container_exists);
    }
}

fn main() {
    let devcontainer_id = "python-3";

    watch_container_running(devcontainer_id);

    println!("do something after container is not running")
}
