# Homebrew Formula for canido
# 
# To use this formula:
# 1. Create a repository named 'homebrew-tap' on GitHub
# 2. Create a 'Formula' directory in that repository
# 3. Copy this file to 'Formula/canido.rb'
# 4. Replace YOUR_USERNAME with your GitHub username
# 5. After releasing, update the sha256 values with actual checksums
#
# Users can then install with:
#   brew tap YOUR_USERNAME/tap
#   brew install canido

class IamPolicyViewer < Formula
  desc "A CLI tool to view IAM policies attached to the current AWS role"
  homepage "https://github.com/YOUR_USERNAME/canido"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/YOUR_USERNAME/canido/releases/download/v#{version}/canido-aarch64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256_FOR_MACOS_ARM64"
    end
    on_intel do
      url "https://github.com/YOUR_USERNAME/canido/releases/download/v#{version}/canido-x86_64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256_FOR_MACOS_X86_64"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/YOUR_USERNAME/canido/releases/download/v#{version}/canido-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256_FOR_LINUX_ARM64"
    end
    on_intel do
      url "https://github.com/YOUR_USERNAME/canido/releases/download/v#{version}/canido-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256_FOR_LINUX_X86_64"
    end
  end

  def install
    bin.install "canido"
  end

  test do
    # The tool requires AWS credentials, so we just test that it runs
    assert_match "View IAM policies", shell_output("#{bin}/canido --help")
  end
end
