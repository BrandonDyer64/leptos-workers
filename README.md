# Leptos on Workers

A template for using Leptos ssr + server functions in a Cloudflare worker.

## Prerequisites

### Rust

Install Rust from [rustup.rs](https://rustup.rs). Then:

```bash
cargo install cargo-leptos --locked
```

### Wrangler

You'll need some version of Node.js. I personally recommend using [nvm](https://github.com/nvm-sh/nvm). Then:

```bash
npm install --global wrangler
```

## Run locally

```bash
wrangler dev
```

## Run on Cloudflare

```bash
wrangler deploy
```

## Troubleshooting

### function wbg:__something must be callable

**Q:** I'm getting an error like this in my console:

```diff
- Unhandled Promise Rejection: LinkError: import function wbg:__wbindgen_closure_wrapper268 must be callable
```

**A:** You need to hard refresh in your browser between compiles

- Firefox/Chrome: `Ctrl-Shift-R` or `⌘ R`
- Safari: `⌥ ⌘ E` + `⌘ R`

