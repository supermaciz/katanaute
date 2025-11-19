# GitHub Actions Workflows

This directory contains the GitHub Actions workflows for the Katanaute monorepo.

## Workflows

### CI Workflow (`ci.yml`)
**Trigger**: Push and Pull Requests to main/master branches

Runs comprehensive testing and validation for all components:

- **Phoenix Backend**: Tests on Elixir 1.15/1.19 with OTP 26/28
  - Dependency installation and caching
  - Compilation with warnings as errors
  - Code formatting check
  - Test suite execution

- **React Frontend**: Tests on Node 18/20
  - TypeScript type checking
  - ESLint linting
  - Vitest test suite
  - Production build verification

- **Rust GUI (Katarouille)**: Builds on Linux, macOS, and Windows
  - Code formatting check (rustfmt)
  - Clippy linting
  - Build verification
  - Test execution

- **Go TUI (Katago)**: Tests on Go 1.22/1.23
  - Dependency verification
  - Code formatting check (gofmt)
  - Go vet static analysis
  - Build verification
  - Test execution

### Security Workflow (`security.yml`)
**Trigger**: Push, Pull Requests, and Weekly Schedule (Mondays at 9am UTC)

Runs security audits for all components:

- **Rust**: cargo-audit for dependency vulnerabilities
- **Go**: Gosec security scanner
- **npm**: npm audit for React dependencies
- **Elixir**: hex.audit and Sobelow security scanner

### Coverage Workflow (`coverage.yml`)
**Trigger**: Push and Pull Requests to main/master branches

Generates code coverage reports:

- **Phoenix Backend**: ExUnit coverage reports
- **React Frontend**: Vitest coverage with Codecov upload

## Dependabot Configuration (`dependabot.yml`)

Automatically creates PRs for dependency updates:

- **GitHub Actions**: Weekly updates
- **Elixir (Mix)**: Weekly updates for Phoenix backend
- **npm**: Weekly updates for React frontend
- **Cargo**: Weekly updates for Rust GUI
- **Go modules**: Weekly updates for Go TUI

All dependency PRs are labeled appropriately for easy filtering.

## Badge Status

Add these badges to your main README.md:

```markdown
[![CI](https://github.com/YOUR_USERNAME/katanaute/actions/workflows/ci.yml/badge.svg)](https://github.com/YOUR_USERNAME/katanaute/actions/workflows/ci.yml)
[![Security](https://github.com/YOUR_USERNAME/katanaute/actions/workflows/security.yml/badge.svg)](https://github.com/YOUR_USERNAME/katanaute/actions/workflows/security.yml)
[![codecov](https://codecov.io/gh/YOUR_USERNAME/katanaute/branch/main/graph/badge.svg)](https://codecov.io/gh/YOUR_USERNAME/katanaute)
```

## Workflow Maintenance

- Workflows use version pinning (e.g., `@v4`) for stability
- Dependabot keeps action versions updated
- Matrix strategies test multiple versions for better compatibility
- Caching is configured for faster CI runs

## Troubleshooting

**CI Failures**:
1. Check the workflow logs in GitHub Actions tab
2. Reproduce locally with the same commands
3. Verify dependencies are up to date

**Security Alerts**:
1. Review Dependabot PRs for updates
2. Check security advisories in the Security tab
3. Update vulnerable dependencies promptly

**Coverage Issues**:
1. Run coverage locally: `mix test --cover` or `npm run test:coverage`
2. Ensure tests are comprehensive
3. Check Codecov dashboard for detailed reports
