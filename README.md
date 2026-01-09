![Static Badge](https://img.shields.io/badge/Rust-gray?logo=rust)
![Static Badge](https://img.shields.io/badge/LICENSE-MIT-blue)

# canido - Can I do?

A CLI tool to view IAM policies attached to your current AWS role. `canido` means "Can I do?".
![demo](demo-canido.gif)

## ✨ Highlights

- 🦀 **Written in Rust** - Fast, safe, and reliable
- 🔒 **Secure** - Read-only operations, no credentials storage
- ⚡ **Fast** - Instant policy retrieval

## 🤔 Why canido?

**❌ Before:** Running multiple AWS CLI commands to check permissions...
```bash
aws sts get-caller-identity
aws iam list-attached-role-policies --role-name MyRole
aws iam get-policy-version --policy-arn ... --version-id ...
# And then inline policies too...
```

**✅ After:** Just one command with `canido`!
```bash
canido
```

- No need to login to AWS Console and navigate through multiple screens
- No more copying JSON and formatting in your editor
- Quickly verify your current permissions for troubleshooting

## Features

- 🔍 Automatically detects the current IAM role from your AWS credentials
- 📋 Displays both managed and inline policies
- 🎨 Colored output for better readability
- 📄 JSON output option for scripting

## 💡 Use Cases

- 🔧 **During Development**: "Why is this API returning 403? Do I have the right permissions?" - Check instantly
- 🚨 **Troubleshooting**: Quickly investigate permission issues
- 📚 **Learning & Auditing**: Understand team members' role permissions

## Installation

### Using Homebrew (macOS)

```bash
brew tap tttol/tap
brew install canido
```

### From GitHub Releases

Download the binary for your platform from the [Releases](https://github.com/tttol/canido/releases) page.

## Usage

```bash
# View policies for the current role
canido

# Output in JSON format
canido --json

# Show only policy names
canido --short

# Show help
canido --help
```

## Example Output

<details>
<summary>Click to see example output</summary>

```
--- Checking AWS credentials ---
Target role: AWSReservedSSO_CanidoInlinePolicy_f1d7ab46757a3473

==================================================
  1. Managed Policies
==================================================
[Policy ARN]: arn:aws:iam::aws:policy/IAMFullAccess
{
  "Statement": [
    {
      "Action": [
        "iam:*",
        "organizations:DescribeAccount",
        "organizations:DescribeOrganization",
        "organizations:DescribeOrganizationalUnit",
        "organizations:DescribePolicy",
        "organizations:ListChildren",
        "organizations:ListParents",
        "organizations:ListPoliciesForTarget",
        "organizations:ListRoots",
        "organizations:ListPolicies",
        "organizations:ListTargetsForPolicy"
      ],
      "Effect": "Allow",
      "Resource": "*"
    }
  ],
  "Version": "2012-10-17"
}
--------------------------------------------------

==================================================
  2. Inline Policies
==================================================
[Policy Name]: AwsSSOInlinePolicy
{
  "Statement": [
    {
      "Action": [
        "s3:GetObject",
        "s3:ListBucket"
      ],
      "Effect": "Allow",
      "Resource": [
        "*"
      ],
      "Sid": "Statement1"
    },
    {
      "Action": [
        "secretsmanager:DescribeSecret",
        "secretsmanager:GetRandomPassword",
        "secretsmanager:GetResourcePolicy",
        "secretsmanager:GetSecretValue",
        "secretsmanager:ListSecretVersionIds",
        "secretsmanager:ListSecrets",
        "secretsmanager:BatchGetSecretValue"
      ],
      "Effect": "Allow",
      "Resource": [
        "*"
      ],
      "Sid": "Statement2"
    }
  ],
  "Version": "2012-10-17"
}
--------------------------------------------------
```

</details>

## Prerequisites

- AWS credentials configured (via `aws configure`, environment variables, or SSO)
- Sufficient IAM permissions to read policies:
  - `iam:ListAttachedRolePolicies`
  - `iam:ListRolePolicies`
  - `iam:GetPolicy`
  - `iam:GetPolicyVersion`
  - `iam:GetRolePolicy`
  - `sts:GetCallerIdentity`

## 🌟 Community

- ⭐ **Star this repo** if you find it useful!
- 🐛 **[Report issues](https://github.com/tttol/canido/issues)** - Found a bug? Let us know
- 🤝 **Contributing** - PRs are welcome!

## License

MIT
