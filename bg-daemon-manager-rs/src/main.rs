use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process::{Command, Stdio};
use std::{thread, time};

/// might since the command runs in foreground with the c.wait()
// Spawned successfully
// do something before waiting for the docker run to finish
// WARNING: The requested image's platform (linux/amd64) does not match the detected host platform (linux/arm64/v8) and no specific platform was requested
// [2022-01-10 21:17:54 +0000] [1] [INFO] Starting gunicorn 19.9.0
// [2022-01-10 21:17:54 +0000] [1] [INFO] Listening at: http://0.0.0.0:80 (1)
// [2022-01-10 21:17:54 +0000] [1] [INFO] Using worker: gevent
// [2022-01-10 21:17:54 +0000] [15] [INFO] Booting worker with pid: 15
// Blocked thread for sleep and bash command sleep: 5.006450166s
fn if_let_example() {
    let now = time::Instant::now();
    let cmd = "docker";
    let arg = "run -p 80:80 kennethreitz/httpbin";
    // let cmd = "sleep";
    // let arg = "4";
    if let Ok(mut c) = Command::new(cmd).args(arg.split(' ')).spawn() {
        println!("Spawned successfully");
        println!("do something before waiting for the docker run to finish");

        let ten_millis = time::Duration::from_secs(5);
        thread::sleep(ten_millis);
        println!(
            "Blocked thread for sleep and bash command sleep: {:?}",
            now.elapsed()
        );
        // keeps to docker process
        println!("Exit with: {:?}", c.wait());
        // exists and keeps the docker process running
        // println!("Exit but keeps the docker container running");
    } else {
        panic!("panic");
    }
}
// sudo lsof -i :8000
fn normal_call() -> std::process::Child {
    // let now = time::Instant::now();
    let cmd = "gunicorn";
    let arg = "httpbin:app";
    let child = Command::new(cmd)
        .args(arg.split(' '))
        .env("GUNICORN_CMD_ARGS", "--capture-output --error-logfile - --access-logfile - --access-logformat '%(h)s %(t)s %(r)s %(s)s Host: %({Host}i)s}'")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    println!("Spawned successfully");

    println!("{:?}", child.stdout);
    child
}

fn main() {
    // if_let_example();
    let mut t = normal_call();
    println!("after starting");
    println!("{:?}", t.id());
    thread::sleep(time::Duration::from_secs(5));
    println!("after 5s delay");

    if let Some(stdout) = &mut t.stdout {
        let lines = BufReader::new(stdout).lines().enumerate().take(2);
        for (counter, line) in lines {
            println!("{}, {:?}", counter, line);
        }
    }

    t.kill().expect("failed to kill");
    println!("process killed")
}
