use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};

#[derive(Debug, Clone)]
pub struct SshConfig {
    pub host: String,
    pub hostname: String,
    pub user: String,
    pub identity_file: String,
    pub port: u8,
}

impl SshConfig {
    pub fn new() -> SshConfig {
        SshConfig {
            host: String::from(""),
            hostname: String::from(""),
            user: String::from(""),
            identity_file: String::from(""),
            port: 22,
        }
    }
    fn to_string(&self) -> String {
        format!(
            r#"Host {host}
  HostName {hostname}
  User {user}
  IdentityFile {identity_file}
  Port {port}      

"#,
            host = self.host,
            hostname = self.hostname,
            user = self.user,
            identity_file = self.identity_file,
            port = self.port
        )
    }
}

#[derive(Debug)]
pub struct SshConfigFile {
    pub configs: HashMap<String, SshConfig>,
    file_path: String,
}

impl SshConfigFile {
    fn new(file_path: &str) -> SshConfigFile {
        SshConfigFile {
            configs: HashMap::new(),
            file_path: file_path.to_string(),
        }
    }
    pub fn get_config(&self, key: &str) -> Option<&SshConfig> {
        self.configs.get(key)
    }
    pub fn update_or_insert_config(&mut self, config: SshConfig) {
        match self.configs.get_mut(&config.host) {
            Some(new_config) => *new_config = config,
            _ => {
                self.configs.insert(config.host.clone(), config);
                ()
            }
        }
    }
    pub fn update_config(&mut self, config: SshConfig) {
        if let Some(new_config) = self.configs.get_mut(&config.host) {
            *new_config = config
        }
    }

    pub fn write_file(&self) {
        let whole_string: String = self
            .configs
            .iter()
            .map(|(_, config)| config.to_string())
            .collect();
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            // .open(self.file_path)
            .open("config.txt")
            .unwrap();
        file.write_all(whole_string.as_bytes())
            .expect("Unable to write data");
    }
}

pub fn parse_config_file(path: &str) -> SshConfigFile {
    let file_content = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut ssh_config = SshConfigFile::new(path);

    for confiugration in file_content.split("\n\n") {
        let mut ssh = SshConfig::new();
        for key_value_pair in confiugration.lines() {
            let mut key_value_iter = key_value_pair.split_whitespace();
            match key_value_iter.next().unwrap() {
                "Host" => ssh.host = key_value_iter.next().unwrap().to_string(),
                "HostName" => ssh.hostname = key_value_iter.next().unwrap().to_string(),
                "User" => ssh.user = key_value_iter.next().unwrap().to_string(),
                "IdentityFile" => ssh.identity_file = key_value_iter.next().unwrap().to_string(),
                "Port" => ssh.port = key_value_iter.next().unwrap().parse().unwrap_or(22),
                _ => (),
            }
        }
        ssh_config.configs.insert(ssh.host.clone(), ssh);
    }
    return ssh_config;
}
