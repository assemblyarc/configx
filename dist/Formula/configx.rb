class Configx < Formula
  desc "Offline TUI wizard that generates a modular, well-commented Neovim config"
  homepage "https://github.com/assemblyarc/configx"
  url "https://github.com/assemblyarc/configx/archive/refs/tags/v0.1.1.tar.gz"
  sha256 "REPLACE_WITH_SOURCE_TARBALL_SHA256_AFTER_TAGGING"
  license "MIT"
  head "https://github.com/assemblyarc/configx.git", branch: "main"
  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/configx --version")
  end
end
