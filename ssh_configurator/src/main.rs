use ssh_configurator::{parse_config_file, SshConfig};
fn main() {
    // cli args dummy
    let ssh_filename = "/Users/philipp/.ssh/config";

    let mut ssh_config = parse_config_file(&ssh_filename);

    let mut inferentia = ssh_config.get_config("inferentia").unwrap().clone();
    inferentia.hostname = String::from("ec-1-1-1-1.compute-1.amazonaws.com");

    // updates config
    ssh_config.update_config(inferentia);

    // add new config
    let new_config = SshConfig {
        host: "Test".to_string(),
        hostname: "1.1.1.1".to_string(),
        user: "ubuntu".to_string(),
        identity_file: "pwd/ssh".to_string(),
        port: 22,
    };
    ssh_config.update_or_insert_config(new_config);

    ssh_config.write_file()
}
