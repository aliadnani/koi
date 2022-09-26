# Koi

An open-source & self-hostable feedback platform

## Stack

- Client: React & Vite
- Server: Rust & SQLite
  - Am considering PostgreSQL in the future!

## Getting started

### Prerequisites

- `Node.js` & `pnpm`
- Rust toolchain (`rustup`)

### Starting Server

```bash
cd server

# Within /koi/server directory
cargo run

...

2022-09-26T17:12:53.959997Z DEBUG koi: Server started on 0.0.0.0:6122!
```

### Starting Client

```bash
cd client

# Within /koi/client directory
pnpm install
pnpm run dev

  VITE v3.1.0  ready in 402 ms

  ➜  Local:   http://localhost:5173/
  ➜  Network: use --host to expose
```

### Building/Deploying Production

TODO: Set up GitHub packages & Actions to make readily available standalone releases.

### Client

```bash
cd client

# Within /koi/client directory
pnpm install
pnpm run build
```

Build artifacts are emitted at `/dist` folder which served on any web server.

Alternatively, you can configure `vercel` among others to build and deploy from this repo automatically.

### Server

A standalone binary can be built like so:

```bash
cd server

# Within /koi/server directory
cargo build --release
```

Otherwise if you prefer Docker:

```bash
cd server

# Within /koi/server directory
pnpm install

# Or any image tag really
docker build -t aliadnani/koi:latest .
```

Standalone binaries and Docker images can then be deployed as you like.