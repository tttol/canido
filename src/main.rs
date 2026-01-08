use anyhow::{Context, Result};
use aws_sdk_iam::Client as IamClient;
use aws_sdk_sts::Client as StsClient;
use clap::Parser;
use colored::*;

/// A CLI tool to view IAM policies attached to the current AWS role
#[derive(Parser, Debug)]
#[command(name = "canido")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Show output in JSON format
    #[arg(short, long)]
    json: bool,

    /// Show only policy names
    #[arg(short, long)]
    short: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Load AWS configuration from environment/credentials
    let config = aws_config::load_from_env().await;
    let sts_client = StsClient::new(&config);
    let iam_client = IamClient::new(&config);

    // Get the role name
    let role_name = get_current_role_name(&sts_client).await?;

    if args.short {
        print_policies_short(&iam_client, &role_name).await?;
    } else if args.json {
        print_policies_json(&iam_client, &role_name).await?;
    } else {
        print_policies_human(&iam_client, &role_name).await?;
    }

    Ok(())
}

/// Print section header with title
fn print_section_header(title: &str) {
    println!("{}", "==================================================".blue());
    println!("{}", format!("  {}", title).blue().bold());
    println!("{}", "==================================================".blue());
}

/// Extract the role name from current AWS credentials
async fn get_current_role_name(sts_client: &StsClient) -> Result<String> {
    println!("{}", "--- Checking AWS credentials ---".dimmed());

    let identity = sts_client
        .get_caller_identity()
        .send()
        .await
        .context("Failed to get caller identity. Please check your AWS credentials (e.g., run 'aws sso login').")?;

    let arn = identity
        .arn()
        .context("ARN not found in caller identity")?;

    // ARN format: arn:aws:sts::123456789012:assumed-role/RoleName/SessionName
    let role_name = arn
        .split('/')
        .nth(1)
        .context("Failed to extract role name from ARN")?
        .to_string();

    println!("Target role: {}\n", role_name.green());

    Ok(role_name)
}

/// Print policies in human-readable format
async fn print_policies_human(iam_client: &IamClient, role_name: &str) -> Result<()> {
    print_section_header("1. Managed Policies");

    let managed_policies = list_managed_policies(iam_client, role_name).await?;

    if managed_policies.is_empty() {
        println!("{}", "(No managed policies attached)".dimmed());
    } else {
        for policy_arn in &managed_policies {
            println!("[Policy ARN]: {}", policy_arn.cyan());

            match get_policy_document(iam_client, policy_arn).await {
                Ok(document) => {
                    let formatted = format_json(&document)?;
                    println!("{}", formatted);
                }
                Err(e) => {
                    println!("{}: {}", "Error".red(), e);
                }
            }
            println!("{}", "--------------------------------------------------".dimmed());
        }
    }

    println!();
    print_section_header("2. Inline Policies");

    let inline_policies = list_inline_policies(iam_client, role_name).await?;

    if inline_policies.is_empty() {
        println!("{}", "(No inline policies attached)".dimmed());
    } else {
        for policy_name in &inline_policies {
            println!("[Policy Name]: {}", policy_name.cyan());

            match get_inline_policy_document(iam_client, role_name, policy_name).await {
                Ok(document) => {
                    let formatted = format_json(&document)?;
                    println!("{}", formatted);
                }
                Err(e) => {
                    println!("{}: {}", "Error".red(), e);
                }
            }
            println!("{}", "--------------------------------------------------".dimmed());
        }
    }

    Ok(())
}

/// Print policies in JSON format
async fn print_policies_json(iam_client: &IamClient, role_name: &str) -> Result<()> {
    let managed_policies = list_managed_policies(iam_client, role_name).await?;
    let inline_policies = list_inline_policies(iam_client, role_name).await?;

    let mut managed_docs = Vec::new();
    for policy_arn in &managed_policies {
        if let Ok(doc) = get_policy_document(iam_client, policy_arn).await {
            managed_docs.push(serde_json::json!({
                "arn": policy_arn,
                "document": serde_json::from_str::<serde_json::Value>(&doc).unwrap_or_default()
            }));
        }
    }

    let mut inline_docs = Vec::new();
    for policy_name in &inline_policies {
        if let Ok(doc) = get_inline_policy_document(iam_client, role_name, policy_name).await {
            inline_docs.push(serde_json::json!({
                "name": policy_name,
                "document": serde_json::from_str::<serde_json::Value>(&doc).unwrap_or_default()
            }));
        }
    }

    let output = serde_json::json!({
        "role_name": role_name,
        "managed_policies": managed_docs,
        "inline_policies": inline_docs
    });

    println!("{}", serde_json::to_string_pretty(&output)?);

    Ok(())
}

/// Print only policy names
async fn print_policies_short(iam_client: &IamClient, role_name: &str) -> Result<()> {
    let managed_policy_names = list_managed_policy_names(iam_client, role_name).await?;
    let inline_policies = list_inline_policies(iam_client, role_name).await?;

    print_section_header("1. Managed Policies");

    if managed_policy_names.is_empty() {
        println!("{}", "(No managed policies attached)".dimmed());
    } else {
        for policy_name in &managed_policy_names {
            println!("{}", policy_name);
        }
    }

    println!();
    print_section_header("2. Inline Policies");

    if inline_policies.is_empty() {
        println!("{}", "(No inline policies attached)".dimmed());
    } else {
        for policy_name in &inline_policies {
            println!("{}", policy_name);
        }
    }

    Ok(())
}

/// List all managed policies attached to a role
async fn list_managed_policies(iam_client: &IamClient, role_name: &str) -> Result<Vec<String>> {
    let response = iam_client
        .list_attached_role_policies()
        .role_name(role_name)
        .send()
        .await
        .context("Failed to list managed policies")?;

    let policies = response
        .attached_policies()
        .iter()
        .filter_map(|p| p.policy_arn().map(|s| s.to_string()))
        .collect();

    Ok(policies)
}

/// List all managed policy names attached to a role
async fn list_managed_policy_names(iam_client: &IamClient, role_name: &str) -> Result<Vec<String>> {
    let response = iam_client
        .list_attached_role_policies()
        .role_name(role_name)
        .send()
        .await
        .context("Failed to list managed policies")?;

    let policy_names = response
        .attached_policies()
        .iter()
        .filter_map(|p| p.policy_name().map(|s| s.to_string()))
        .collect();

    Ok(policy_names)
}

/// List all inline policies attached to a role
async fn list_inline_policies(iam_client: &IamClient, role_name: &str) -> Result<Vec<String>> {
    let response = iam_client
        .list_role_policies()
        .role_name(role_name)
        .send()
        .await
        .context("Failed to list inline policies")?;

    Ok(response.policy_names().to_vec())
}

/// Get the policy document for a managed policy
async fn get_policy_document(iam_client: &IamClient, policy_arn: &str) -> Result<String> {
    // Get the default version ID
    let policy = iam_client
        .get_policy()
        .policy_arn(policy_arn)
        .send()
        .await
        .context("Failed to get policy")?;

    let version_id = policy
        .policy()
        .and_then(|p| p.default_version_id())
        .context("Failed to get default version ID")?;

    // Get the policy document
    let version = iam_client
        .get_policy_version()
        .policy_arn(policy_arn)
        .version_id(version_id)
        .send()
        .await
        .context("Failed to get policy version")?;

    let document = version
        .policy_version()
        .and_then(|v| v.document())
        .context("Failed to get policy document")?;

    // The document is URL-encoded, so we need to decode it
    let decoded = urlencoding::decode(document)
        .context("Failed to decode policy document")?
        .to_string();

    Ok(decoded)
}

/// Get the policy document for an inline policy
async fn get_inline_policy_document(
    iam_client: &IamClient,
    role_name: &str,
    policy_name: &str,
) -> Result<String> {
    let response = iam_client
        .get_role_policy()
        .role_name(role_name)
        .policy_name(policy_name)
        .send()
        .await
        .context("Failed to get inline policy")?;

    let document = response.policy_document();

    // The document is URL-encoded, so we need to decode it
    let decoded = urlencoding::decode(document)
        .context("Failed to decode policy document")?
        .to_string();

    Ok(decoded)
}

/// Format JSON string with indentation
fn format_json(json_str: &str) -> Result<String> {
    let value: serde_json::Value = serde_json::from_str(json_str)
        .context("Failed to parse JSON")?;
    
    serde_json::to_string_pretty(&value)
        .context("Failed to format JSON")
}
