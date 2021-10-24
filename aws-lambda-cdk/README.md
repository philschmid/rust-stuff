# Serverless Example: CDK meets Rust🦀


# Deploy

```bash
cdk bootstrap
```

```bash
./build-function.sh && cdk deploy
```


## Compiling on Mac OS X
First, install rustup if you don’t already have it. Then, add the x86_64-unknown-linux-musl target:

```
rustup target add x86_64-unknown-linux-musl
```
Before we build the application, we’ll also need to install a linker for the target platform. Fortunately, the musl-cross tap from Homebrew provides a complete cross-compilation toolchain for Mac OS.

```
brew install filosottile/musl-cross/musl-cross
```
Now we need to inform Cargo that our project uses the newly-installed linker when building for the x86_64-unknown-linux-musl platform. Create a new directory called .cargo in your project folder and a new file called config inside the new folder.

```
mkdir .cargo
echo '[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"' > .cargo/config
```