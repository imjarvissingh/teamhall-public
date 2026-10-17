# TeamHall Public CI Harness

This repository contains only a public CI smoke harness and synthetic fixtures.
The workflow checks a GitHub-hosted runner, Rust stable, Node.js 24, pnpm, and
small dependency-free example tests.

**PUBLIC HARNESS VALIDATION — NOT FULL PRIVATE TEAMHALL CI**

The workflow checks out only this public repository. It does not fetch or build
the private TeamHall application, and a passing run does not verify TeamHall's
Rust or frontend source, container isolation, Issue #126, or any private commit.
