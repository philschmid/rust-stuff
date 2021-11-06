use core::time;
use std::process::Command;
use std::{env, fs, thread};

use rs_docker::Docker;
use structopt::StructOpt;

fn watch_container_running(mut docker: Docker, container_id: &str) {
    let two_seconds = time::Duration::from_secs(2);

    loop {
        let containers = match docker.get_containers(false) {
            Ok(containers) => containers,
            Err(e) => {
                panic!("{}", e);
            }
        };

        let container_exists = containers
            .iter()
            .any(|container| container.Id == container_id);

        if container_exists {
            thread::sleep(two_seconds);
        } else {
            break;
        }
        println!("{}", container_exists);
    }
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Clone, Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct AwsDevcontainer {
    /// Directory from where the devcontainer should be started
    #[structopt(default_value = ".")]
    directory: String,

    /// The interface address where to listen for incoming request
    #[structopt(short = "i", default_value = "m6i.large")]
    instance_type: String,

    /// The interface address where to listen for incoming request
    #[structopt(short = "p", default_value = "hf-inf")]
    profile: String,
}

fn main() {
    // Parse the command line arguments
    let args = AwsDevcontainer::from_args();

    // start ec2 instance
    // TODO

    // update ssh config
    // TODO

    // set docker host
    // env::set_var("DOCKER_HOST", "ssh://ubuntu@infinity");

    // create docker client
    let mut docker = match Docker::connect("unix:///var/run/docker.sock") {
        Ok(docker) => docker,
        Err(e) => {
            panic!("{}", e);
        }
    };

    // start devcontainer
    Command::new("devcontainer")
        .args(["open"])
        .output()
        .expect("failed to execute process");

    // get container id
    let containers = match docker.get_containers(false) {
        Ok(containers) => containers,
        Err(e) => {
            panic!("{}", e);
        }
    };

    let container_id = containers
        .iter()
        .find(|container| container.Image.contains("vsc"))
        .unwrap()
        .clone()
        .Id;

    // copy repository to container
    Command::new("docker")
        .args(["cp", ".", format!("{}:/workspace", container_id).as_str()])
        .output()
        .expect("failed to execute process");

    // // listen until container is stopped
    watch_container_running(docker, container_id.as_str());

    // stop ec2 instance
    println!("stop ec2 instance");
}
