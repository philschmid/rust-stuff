# CLI for running vs-code devcontainers on AWS ec2 instances


1. start a ec2 instance 
2. update `~/.ssh/config`
3. make sure locker docker daemon is stopped
3. expose remote docker and start devcontainer 
```bash
DOCKER_HOST=ssh://ubuntu@infinity devcontainer open
``` 
4. get container id and copy local directory to remote container (only need done once)
4. docker cp . 06c56af81eed:/workspace
6. stop ec2 instnace


## Idea for tool 

provide git repo + aws profile 

```bash
aws-dev-container philschmid/my-repo --instance m6i.large --profile x1
```

