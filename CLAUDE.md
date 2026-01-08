# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`canido` (means "Can I do?") is a Rust CLI tool that displays IAM policies attached to AWS roles. It automatically detects the current IAM role from AWS credentials and shows both managed and inline policies with colored output or JSON format.

Binary name in Cargo.toml is `canido`, but the project/repository name is `canido`.

## Build & Development Commands

```bash
# Build the project
cargo build

# Build release version
cargo build --release

# Run the tool (detects current role from AWS credentials)
cargo run

# Run with JSON output
cargo run -- --json

# Run with short output (policy names only)
cargo run -- --short

# Build for specific target (cross-compilation)
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
```

## Architecture

Single-file Rust application (`main.rs`) with the following flow:

1. **Credential Detection**: Uses AWS STS `GetCallerIdentity` to extract role name from ARN (format: `arn:aws:sts::123456789012:assumed-role/RoleName/SessionName`)
2. **Policy Retrieval**: Fetches both managed policies (via `ListAttachedRolePolicies` + `GetPolicy` + `GetPolicyVersion`) and inline policies (via `ListRolePolicies` + `GetRolePolicy`)
3. **Document Decoding**: AWS returns URL-encoded policy documents, which must be decoded using `urlencoding` crate
4. **Output Formatting**: Supports human-readable colored output (using `colored` crate) or JSON format (using `serde_json`)

Key dependencies:
- `aws-sdk-sts` and `aws-sdk-iam` for AWS API calls
- `tokio` for async runtime
- `clap` for CLI argument parsing
- `anyhow` for error handling

## Release Process

Releases are automated via GitHub Actions (see `release.yml`):
1. Tag version with `v*` pattern (e.g., `v1.0.0`)
2. Automatically publishes to crates.io
3. Builds binaries for macOS (ARM64/x86_64) and Linux (x86_64/ARM64)
4. Creates GitHub release with binaries
5. Updates Homebrew tap formula

## AWS Permissions Required

The tool needs these IAM permissions:
- `iam:ListAttachedRolePolicies`
- `iam:ListRolePolicies`
- `iam:GetPolicy`
- `iam:GetPolicyVersion`
- `iam:GetRolePolicy`
- `sts:GetCallerIdentity`
