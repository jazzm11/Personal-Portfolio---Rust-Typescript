# Personal Portfolio

A personal portfolio project with a TypeScript/Lit frontend and a Rust/Axum API.

## Requirements

- [Git](https://git-scm.com/)
- [Node.js](https://nodejs.org/) and npm
- [Rust](https://www.rust-lang.org/tools/install) and Cargo

## Clone the repository

```bash
git clone https://github.com/jazzm11/Personal-Portfolio---Rust-Typescript.git
cd Personal-Portfolio---Rust-Typescript
```

## Run the frontend

In a terminal, install the frontend dependencies and start the Vite development server:

```bash
cd packages/site
npm install
npm run dev
```

Open the local URL printed by Vite (usually `http://localhost:5173`). The server supports hot reloading while you edit the frontend.

To create and locally preview a production build:

```bash
npm run build
npm run preview
```

Run these commands from `packages/site`.

## Run the Rust API

In a second terminal, from the repository root, start the API:

```bash
cargo run --manifest-path crates/portfolio_backend/Cargo.toml
```

The API listens at `http://127.0.0.1:3000`. Check that it is running by visiting `http://127.0.0.1:3000/api/health`; it returns a JSON health status.

## Project layout

- `packages/site/` — Lit and TypeScript frontend, built with Vite.
- `crates/portfolio_backend/` — Axum HTTP API.
- `crates/portfolio_models/` — Rust models crate.
- `crates/portfolio_repository/` — Rust repository crate.

The frontend and API are started separately. The API currently provides a health-check endpoint at `/api/health`.
