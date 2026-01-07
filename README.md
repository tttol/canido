# canido

A CLI tool to view IAM policies attached to your current AWS role. `canido` means "Can I do?".

## Features

- 🔍 Automatically detects the current IAM role from your AWS credentials
- 📋 Displays both managed and inline policies
- 🎨 Colored output for better readability
- 📄 JSON output option for scripting

## Installation

### Using Homebrew (macOS/Linux)

```bash
brew tap tttol/tap
brew install canido
```

### Using Cargo (requires Rust)

```bash
cargo install canido
```

### From GitHub Releases

Download the binary for your platform from the [Releases](https://github.com/tttol/canido/releases) page.

## Usage

```bash
# View policies for the current role
canido

# View policies for a specific role
canido --role MyRoleName

# Output in JSON format
canido --json

# Show help
canido --help
```

## Example Output

```
--- Checking credentials ---
Target role: MyDeveloperRole

==================================================
  1. Managed Policies
==================================================
[Policy ARN]: arn:aws:iam::123456789012:policy/MyPolicy
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": "s3:GetObject",
      "Resource": "*"
    }
  ]
}
--------------------------------------------------

==================================================
  2. Inline Policies
==================================================
(No inline policies attached)
```

## Prerequisites

- AWS credentials configured (via `aws configure`, environment variables, or SSO)
- Sufficient IAM permissions to read policies:
  - `iam:ListAttachedRolePolicies`
  - `iam:ListRolePolicies`
  - `iam:GetPolicy`
  - `iam:GetPolicyVersion`
  - `iam:GetRolePolicy`
  - `sts:GetCallerIdentity`

## License

MIT
