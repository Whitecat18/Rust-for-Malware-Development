# Cross Compile Rust-Programs using Docker

## Prerequisites

* Docker

## Steps

I have created an Dockerfile in my main branch of [Rust-for-Malware-Development](https://github.com/Whitecat18/Rust-for-Malware-Development.git). lets build our image


```
docker build . -t windows_compile 
```

> ⚠️ : This build takes upto 3 GB Space

Next run the container in the following project Directory ! 

Once you’ve created the image, then you can run the container by executing the following command:

```
docker run --rm -v ‘your-pwd’:/app rust_cross_compile/windows
```

The -rm option will remove the container when the command completes. The -v command allows you to persist data after a container has existed by linking your container storage with your local machine. Replace ‘your-pwd’ with the absolute path to your Rust directory

Example i have compiled [EDRChecker](https://github.com/Whitecat18/Rust-for-Malware-Development/tree/main/EDRChecker) and executed on my Virtual Machine. 

![image](https://github.com/user-attachments/assets/f36dd530-9da8-4e64-9180-078af6bfb37c)

![image](https://github.com/user-attachments/assets/57272c70-de5c-4435-acc8-7899c1ea7a7a)


## Modifying Dockerfile 

Here is the Default version of Rust Container
```
FROM rust:latest

RUN apt update ; apt upgrade -y 
RUN apt install -y g++-mingw-w64-x86-64

RUN rustup target add x86_64-pc-windows-gnu 
RUN rustup toolchain install stable-x86_64-pc-windows-gnu 

WORKDIR /app

CMD ["cargo", "build", "--target", "x86_64-pc-windows-gnu"]
```

For **--release** you can modify the Dockerfile as Follows 

```
FROM rust:latest

RUN apt update ; apt upgrade -y 
RUN apt install -y g++-mingw-w64-x86-64

RUN rustup target add x86_64-pc-windows-gnu 
RUN rustup toolchain install stable-x86_64-pc-windows-gnu 

WORKDIR /app

CMD ["cargo", "build", "--target", "x86_64-pc-windows-gnu", "--release"]
```

Here is the Image: 

![image](https://github.com/user-attachments/assets/b1220909-c0c8-4645-b7a2-94acb5722bf6)

![image](https://github.com/user-attachments/assets/35efd900-2f61-4c7d-89d2-815ec318639e)

THE PATH WHERE IT COMPILES AND KEEPS BINARY FILES: 

![image](https://github.com/user-attachments/assets/66fc1c4f-6759-413d-a417-3609d5bd14c2)

