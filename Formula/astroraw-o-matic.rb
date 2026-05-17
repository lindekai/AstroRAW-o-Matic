class AstrorawOMatic < Formula
  desc "Mostly harmless RAW conversion // DSLR RAW to FITS for astrophotography"
  homepage "https://github.com/lindekai/AstroRAW-o-Matic"
  url "https://github.com/lindekai/AstroRAW-o-Matic/archive/refs/tags/v0.3.0.tar.gz"
  sha256 "PLACEHOLDER_SHA256"
  license "MIT"
  head "https://github.com/lindekai/AstroRAW-o-Matic.git", branch: "main"

  bottle do
    # Bottles are generated automatically via GitHub Actions after each release.
    # Run: brew test-bot --only-formulae astroraw-o-matic
  end

  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release", "--bin", "astroraw-o-matic",
           *std_cargo_args(root: buildpath, path: "crates/astroraw-cli")
    bin.install "target/release/astroraw-o-matic"
  end

  def caveats
    <<~EOS
      AstroRAW-o-Matic // Mostly harmless RAW conversion

      Quick start:
        astroraw-o-matic inspect image.CR2
        astroraw-o-matic convert ./RAW --output ./FITS --metadata session.json

      Alias (optional):
        echo "alias arom='astroraw-o-matic'" >> ~/.zshrc

      Community: Dark Matters Discord — https://discord.gg/mvgC6aXY
    EOS
  end

  test do
    assert_match "AstroRAW-o-Matic", shell_output("#{bin}/astroraw-o-matic --version")
  end
end
