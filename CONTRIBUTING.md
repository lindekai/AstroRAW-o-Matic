# Contributing to AstroRAW-o-Matic

Thank you for wanting to contribute. The project is in early development — issues and PRs are very welcome.

## Ground rules

- Keep engine (`astroraw-core`) free of CLI/UI dependencies.
- All public interfaces should have at least a brief doc comment.
- Tests for metadata resolution, FITS header generation, and session JSON validation are expected.
- Commit messages should be clear and reference the affected crate where relevant.

## Development setup

```bash
git clone https://github.com/lindekai/AstroRAW-o-Matic.git
cd AstroRAW-o-Matic
cargo build
cargo test
```

## Areas where help is most needed

- LibRaw FFI integration for actual RAW pixel extraction
- Additional camera format support (CR3, NEF, ARW)
- Shell completions
- CI/CD pipeline
- GUI prototype (Tauri)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
