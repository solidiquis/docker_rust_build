# Docker Rust Build

Combination of dummy libs/bins that depend on arrow/data-fusion/tokio to test docker build speeds.

## No Chef

```
┌─ faster_build 
└ 🧋 docker buildx bake --no-cache
[+] Building 260.5s (16/16) FINISHED                                                                                                                                              docker:orbstack
 => [internal] load local bake definitions                                                                                                                                                   0.0s
 => => reading docker-bake.hcl 343B / 343B                                                                                                                                                   0.0s
 => [csv internal] load build definition from Dockerfile                                                                                                                                     0.2s
 => => transferring dockerfile: 719B                                                                                                                                                         0.0s
 => [csv_again internal] load metadata for docker.io/library/debian:bookworm-slim                                                                                                            0.7s
 => [csv_again internal] load metadata for docker.io/library/rust:latest                                                                                                                     0.7s
 => [csv_again internal] load .dockerignore                                                                                                                                                  0.2s
 => => transferring context: 2B                                                                                                                                                              0.0s
 => [csv_again builder 1/4] FROM docker.io/library/rust:latest@sha256:4a29b0db5c961cd530f39276ece3eb6e66925b59599324c8c19723b72a423615                                                       0.0s
 => [csv_again internal] load build context                                                                                                                                                  0.4s
 => => transferring context: 662.98kB                                                                                                                                                        0.2s
 => CACHED [csv runtime 1/2] FROM docker.io/library/debian:bookworm-slim@sha256:b4aa902587c2e61ce789849cb54c332b0400fe27b1ee33af4669e1f7e7c3e22f                                             0.0s
 => CACHED [csv builder 2/4] WORKDIR /app                                                                                                                                                    0.0s
 => [csv_again builder 3/4] COPY . .                                                                                                                                                        11.6s
 => [csv builder 4/4] RUN cargo build --release --bin csv                                                                                                                                  246.3s
 => [csv_again builder 4/4] RUN cargo build --release --bin csv_again                                                                                                                      246.2s
 => [csv_again runtime 2/2] COPY --from=builder /app/target/release/csv_again /usr/local/bin/app                                                                                             0.3s
 => [csv runtime 2/2] COPY --from=builder /app/target/release/csv /usr/local/bin/app                                                                                                         0.3s 
 => [csv_again] exporting to image                                                                                                                                                           0.2s 
 => => exporting layers                                                                                                                                                                      0.2s 
 => => writing image sha256:be84234a1008ceb1bea660b78277a8b2d0e61f8f530ea40ad88cb26decfeab33                                                                                                 0.0s 
 => => naming to docker.io/library/csv_again:latest                                                                                                                                          0.0s 
 => [csv] exporting to image                                                                                                                                                                 0.3s 
 => => exporting layers                                                                                                                                                                      0.2s 
 => => writing image sha256:7f44d1fab001b5ca6258a3b20f66239830afcc22ae86524b2305901a855e44a9                                                                                                 0.0s 
 => => naming to docker.io/library/csv:latest                                                                                                                                                0.0s 
 ```

## With Chef

```
┌─ faster_build 
└ 🧋 docker buildx bake --no-cache
[+] Building 159.9s (21/21) FINISHED                                                                                                                                              docker:orbstack
 => [internal] load local bake definitions                                                                                                                                                   0.0s
 => => reading docker-bake.hcl 343B / 343B                                                                                                                                                   0.0s
 => [csv internal] load build definition from Dockerfile                                                                                                                                     0.1s
 => => transferring dockerfile: 534B                                                                                                                                                         0.0s
 => [csv internal] load metadata for docker.io/library/debian:bookworm-slim                                                                                                                  0.4s
 => [csv internal] load metadata for docker.io/library/rust:latest                                                                                                                           0.4s
 => [csv_again internal] load .dockerignore                                                                                                                                                  0.2s
 => => transferring context: 2B                                                                                                                                                              0.0s
 => [csv_again chef 1/3] FROM docker.io/library/rust:latest@sha256:4a29b0db5c961cd530f39276ece3eb6e66925b59599324c8c19723b72a423615                                                          0.0s
 => CACHED [csv_again runtime 1/2] FROM docker.io/library/debian:bookworm-slim@sha256:b4aa902587c2e61ce789849cb54c332b0400fe27b1ee33af4669e1f7e7c3e22f                                       0.0s
 => [csv_again internal] load build context                                                                                                                                                  0.4s
 => => transferring context: 662.65kB                                                                                                                                                        0.3s
 => CACHED [csv_again chef 2/3] WORKDIR /app                                                                                                                                                 0.0s
 => [csv_again chef 3/3] RUN cargo install cargo-chef --locked                                                                                                                              10.2s
 => [csv planner 1/2] COPY . .                                                                                                                                                              10.7s
 => [csv_again planner 2/2] RUN cargo chef prepare --recipe-path recipe.json                                                                                                                 0.4s 
 => [csv builder 1/4] COPY --from=planner /app/recipe.json recipe.json                                                                                                                       0.2s 
 => [csv builder 2/4] RUN cargo chef cook --release --recipe-path recipe.json                                                                                                              109.6s 
 => [csv builder 3/4] COPY . .                                                                                                                                                               6.2s 
 => [csv builder 4/4] RUN cargo build --release --bin csv                                                                                                                                   20.4s 
 => [csv_again builder 4/4] RUN cargo build --release --bin csv_again                                                                                                                       20.0s 
 => [csv_again runtime 2/2] COPY --from=builder /app/target/release/csv_again /usr/local/bin/app                                                                                             0.3s 
 => [csv_again] exporting to image                                                                                                                                                           0.4s 
 => => exporting layers                                                                                                                                                                      0.2s 
 => => writing image sha256:6b36beb46ae4f178e271dff98dbc7d0ee0e6e527c4801b9a55c7076e1c26d4d5                                                                                                 0.0s 
 => => naming to docker.io/library/csv_again:latest                                                                                                                                          0.0s 
 => [csv runtime 2/2] COPY --from=builder /app/target/release/csv /usr/local/bin/app                                                                                                         0.2s 
 => [csv] exporting to image                                                                                                                                                                 0.3s
 => => exporting layers                                                                                                                                                                      0.3s
 => => writing image sha256:4736e9968a05db560966b848cd03160eaeabd2223237290e056dd3967f775581                                                                                                 0.0s
 => => naming to docker.io/library/csv:latest                                                                                                                                                0.0s
```

## With Chef + Mold

```
 => [internal] load local bake definitions                                                                                                                                                   0.0s
 => => reading docker-bake.hcl 343B / 343B                                                                                                                                                   0.0s
 => [csv internal] load build definition from Dockerfile                                                                                                                                     0.2s
 => => transferring dockerfile: 825B                                                                                                                                                         0.0s
 => [csv internal] load metadata for docker.io/library/debian:bookworm-slim                                                                                                                  1.0s
 => [csv internal] load metadata for docker.io/library/rust:latest                                                                                                                           0.6s
 => [csv_again internal] load .dockerignore                                                                                                                                                  0.1s
 => => transferring context: 2B                                                                                                                                                              0.0s
 => CACHED [csv runtime 1/2] FROM docker.io/library/debian:bookworm-slim@sha256:b4aa902587c2e61ce789849cb54c332b0400fe27b1ee33af4669e1f7e7c3e22f                                             0.0s
 => [csv_again internal] load build context                                                                                                                                                  0.4s
 => => transferring context: 663.44kB                                                                                                                                                        0.3s
 => [csv chef 1/4] FROM docker.io/library/rust:latest@sha256:4a29b0db5c961cd530f39276ece3eb6e66925b59599324c8c19723b72a423615                                                                0.0s
 => CACHED [csv chef 2/4] WORKDIR /app                                                                                                                                                       0.0s
 => [csv chef 3/4] RUN apt-get update && apt-get install -y --no-install-recommends     mold clang lld pkg-config ca-certificates  && rm -rf /var/lib/apt/lists/*                            6.8s
 => [csv_again chef 4/4] RUN cargo install cargo-chef --locked                                                                                                                               9.2s
 => [csv_again planner 1/2] COPY . .                                                                                                                                                         8.4s 
 => [csv_again planner 2/2] RUN cargo chef prepare --recipe-path recipe.json                                                                                                                 0.3s 
 => [csv builder 1/4] COPY --from=planner /app/recipe.json recipe.json                                                                                                                       0.2s 
 => [csv_again builder 2/4] RUN cargo chef cook --release --recipe-path recipe.json                                                                                                         95.6s 
 => [csv builder 3/4] COPY . .                                                                                                                                                              10.5s 
 => [csv_again builder 4/4] RUN cargo build --release --bin csv_again                                                                                                                       15.4s 
 => [csv builder 4/4] RUN cargo build --release --bin csv                                                                                                                                   15.3s 
 => [csv runtime 2/2] COPY --from=builder /app/target/release/csv /usr/local/bin/app                                                                                                         0.3s 
 => [csv_again runtime 2/2] COPY --from=builder /app/target/release/csv_again /usr/local/bin/app                                                                                             0.2s 
 => [csv] exporting to image                                                                                                                                                                 0.3s 
 => => exporting layers                                                                                                                                                                      0.3s 
 => => writing image sha256:bfe659a3f585f953af0c76146486000188b667479bd5d64a31ec2eaaad9f6ef8                                                                                                 0.0s 
 => => naming to docker.io/library/csv:latest                                                                                                                                                0.0s
 => [csv_again] exporting to image                                                                                                                                                           0.3s
 => => exporting layers                                                                                                                                                                      0.2s
 => => writing image sha256:9c8fd69323b47bb2fc15c0b2deda8ccf0bee845600d7d5cfa5f9042a4e4e6f61                                                                                                 0.0s
 => => naming to docker.io/library/csv_again:latest                                                                                                                                          0.0s
```
