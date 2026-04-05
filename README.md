# Rewriting Project Claw Code

<p align="center">
  <strong>⭐ The fastest repo in history to surpass 50K stars, reaching the milestone in just 2 hours after publication ⭐</strong>
</p>

<p align="center">
  <a href="https://star-history.com/#ultraworkers/claw-code&Date">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=ultraworkers/claw-code&type=Date&theme=dark" />
      <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=ultraworkers/claw-code&type=Date" />
      <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=ultraworkers/claw-code&type=Date" width="600" />
    </picture>
  </a>
</p>

<p align="center">
  <img src="assets/clawd-hero.jpeg" alt="Claw" width="300" />
</p>

<p align="center">
  <strong>Autonomously maintained by lobsters/claws — not by human hands</strong>
</p>

<p align="center">
  <a href="https://github.com/Yeachan-Heo/clawhip">clawhip</a> ·
  <a href="https://github.com/code-yeongyu/oh-my-openagent">oh-my-openagent</a> ·
  <a href="https://github.com/Yeachan-Heo/oh-my-claudecode">oh-my-claudecode</a> ·
  <a href="https://github.com/Yeachan-Heo/oh-my-codex">oh-my-codex</a> ·
  <a href="https://discord.gg/6ztZB9jvWq">UltraWorkers Discord</a>
</p>

> [!IMPORTANT]
> **This repository is a Rust CLI project.** The canonical implementation lives in [`rust/`](./rust), where the `claw` binary and workspace crates are maintained. Start with [`USAGE.md`](./USAGE.md) for build, auth, CLI, session, and parity-harness workflows, then use [`rust/README.md`](./rust/README.md) for crate-level details. The top-level `src/` and `tests/` trees remain supporting parity/reference surfaces, not the primary runtime.

> Want the bigger idea behind this repo? Read [`PHILOSOPHY.md`](./PHILOSOPHY.md) and Sigrid Jin's public explanation: https://x.com/realsigridjin/status/2039472968624185713

> Shout-out to the UltraWorkers ecosystem powering this repo: [clawhip](https://github.com/Yeachan-Heo/clawhip), [oh-my-openagent](https://github.com/code-yeongyu/oh-my-openagent), [oh-my-claudecode](https://github.com/Yeachan-Heo/oh-my-claudecode), [oh-my-codex](https://github.com/Yeachan-Heo/oh-my-codex), and the [UltraWorkers Discord](https://discord.gg/6ztZB9jvWq).

---

## Backstory

This repo is maintained by **lobsters/claws**, not by a conventional human-only dev team.

The people behind the system are [Bellman / Yeachan Heo](https://github.com/Yeachan-Heo) and friends like [Yeongyu](https://github.com/code-yeongyu), but the repo itself is being pushed forward by autonomous claw workflows: parallel coding sessions, event-driven orchestration, recovery loops, and machine-readable lane state.

In practice, that means this project is not just *about* coding agents — it is being **actively built by them**. Features, tests, telemetry, docs, and workflow hardening are landed through claw-driven loops using [clawhip](https://github.com/Yeachan-Heo/clawhip), [oh-my-openagent](https://github.com/code-yeongyu/oh-my-openagent), [oh-my-claudecode](https://github.com/Yeachan-Heo/oh-my-claudecode), and [oh-my-codex](https://github.com/Yeachan-Heo/oh-my-codex).

This repository exists to prove that an open coding harness can be built **autonomously, in public, and at high velocity** — with humans setting direction and claws doing the grinding.

See the public build story here:

https://x.com/realsigridjin/status/2039472968624185713

![Tweet screenshot](assets/tweet-screenshot.png)

---

## Implementation Status

The product surface is now **Rust-first**.

- `rust/` contains the active Rust workspace and the shipping `claw` CLI
- [`USAGE.md`](./USAGE.md) is the top-level guide for build, auth, session, and parity-harness workflows
- [`PARITY.md`](./PARITY.md) tracks how the Rust port compares with the archived reference surface
- `src/` and `tests/` remain supporting parity/reference surfaces for mirrored inventories, audit helpers, and repo guidance

The Python tree is still useful for metadata exploration and archived-surface analysis, but it is **not** the primary implementation.

## Why this rewrite exists

I originally studied the exposed codebase to understand its harness, tool wiring, and agent workflow. After spending more time with the legal and ethical questions—and after reading the essay linked below—I did not want the exposed snapshot itself to remain the main tracked source tree.

This repository now centers the Rust CLI rewrite, with supporting parity and reference artifacts preserved where they are still useful.

## Repository Layout

```text
.
├── rust/                               # Active Rust workspace and `claw` CLI
│   ├── Cargo.toml
│   ├── crates/
│   ├── scripts/
│   └── README.md
├── src/                                # Supporting parity/reference helpers and mirrored inventories
├── tests/                              # Validation for the top-level parity/reference helpers
├── assets/omx/                         # OmX workflow screenshots
├── PARITY.md                           # Rust-port parity checkpoint and gap tracking
├── USAGE.md                            # Top-level Rust CLI usage guide
└── README.md
```

## Implementation Surfaces

### Rust workspace (`rust/`)

The Rust workspace is the canonical implementation surface for this repository. It contains the CLI binary, runtime crates, mock parity harness, and the current build/test workflows.

### Top-level `src/` and `tests/`

The top-level Python files are supporting artifacts for parity/reference work:

- mirrored command/tool inventories
- manifest and audit helpers
- guidance-generation helpers
- validation for those supporting surfaces

They help document and inspect the broader rewrite effort, but they are not the shipping CLI runtime.

## Quickstart

Build the Rust workspace:

```bash
cd rust
cargo build --workspace
```

Show the CLI help:

```bash
cd rust
./target/debug/claw --help
```

Run a prompt through the CLI:

```bash
cd rust
./target/debug/claw prompt "summarize this repository"
```

Run verification:

```bash
cd rust
cargo test --workspace
```

For auth, session-management, permission-mode, and parity-harness details, start with [`USAGE.md`](./USAGE.md).

## Current Parity Checkpoint

The active parity effort is the Rust port tracked in [`PARITY.md`](./PARITY.md). The workspace now carries a substantial Rust implementation footprint, but parity closure against the archived TypeScript reference is still in progress. The top-level Python helpers remain useful for mirrored inventories and audit/reporting flows, not as the main runtime replacement.


## Built with `oh-my-codex`

The restructuring and documentation work on this repository was AI-assisted and orchestrated with Yeachan Heo's [oh-my-codex (OmX)](https://github.com/Yeachan-Heo/oh-my-codex), layered on top of Codex.

- **`$team` mode:** used for coordinated parallel review and architectural feedback
- **`$ralph` mode:** used for persistent execution, verification, and completion discipline
- **Codex-driven workflow:** used to evolve the Rust CLI workspace and keep parity/docs work aligned with the current repo reality

### OmX workflow screenshots

![OmX workflow screenshot 1](assets/omx/omx-readme-review-1.png)

*Ralph/team orchestration view while the README and essay context were being reviewed in terminal panes.*

![OmX workflow screenshot 2](assets/omx/omx-readme-review-2.png)

*Split-pane review and verification flow during the final README wording pass.*

## Community

<p align="center">
  <a href="https://discord.gg/6ztZB9jvWq"><img src="https://img.shields.io/badge/UltraWorkers-Discord-5865F2?logo=discord&style=for-the-badge" alt="UltraWorkers Discord" /></a>
</p>

Join the [**UltraWorkers Discord**](https://discord.gg/6ztZB9jvWq) — the community around clawhip, oh-my-openagent, oh-my-claudecode, oh-my-codex, and claw-code. Come chat about LLMs, harness engineering, agent workflows, and autonomous software development.

[![Discord](https://img.shields.io/badge/Join%20Discord-UltraWorkers-5865F2?logo=discord&style=for-the-badge)](https://discord.gg/6ztZB9jvWq)

## Star History

See the chart at the top of this README.

## Ownership / Affiliation Disclaimer

- This repository does **not** claim ownership of the original Claude Code source material.
- This repository is **not affiliated with, endorsed by, or maintained by Anthropic**.
