```sh
docker run --platform=linux/x86_64 -it -v $(pwd)/src:/src --name my-centos-container centos:7 /bin/bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
yum install -y gcc

docker start -i my-centos-container
```

ビルド時、openssl周りでエラー
```sh
yum install openssl openssl-devel
```
