```sh
docker run -it -v $(pwd)/src:/src --name my-centos-container centos7 /bin/bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
yum install -y gcc

docker start -i my-centos-container
```
