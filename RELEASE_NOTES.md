# Changelog

All notable changes to this project will be documented in this file.
## [0.1.1] - 2026-04-01

### Bug Fixes

- *(ci)* Use variables for git identity instead of API lookup (#15)
- *(ci)* Use GitHub App token for release workflow (#14)
- *(ci)* Use official crates-io-auth-action for trusted publishing (#13)
- *(ci)* Remove unsupported copilot review param from ruleset
- *(ci)* Use ADMIN_TOKEN secret for ruleset sync

### CI

- Add dependabot for cargo and github-actions (#4)
- Add on-demand publish workflow (#3)
- Add CI workflow and enforce squash merges (#1)
- Auto-apply branch rulesets on push to master
- Add branch ruleset for master

### Documentation

- Add crate-level documentation to all crates (#10)

### Features

- Add pre-generated type crates for 20 new tree-sitter languages (#9)
- Add pre-generated type crates for 5 languages (#2)
- Implement Spanned for all generated enums and AnyNode
- Initial implementation of treesitter-types

### Miscellaneous

- Use trusted publishing for crates.io (#12)
- Set up cargo-release with git-cliff integration (#11)
- *(deps)* Bump actions/checkout from 4 to 6 (#5)
- *(deps)* Bump amannn/action-semantic-pull-request from 5 to 6 (#6)
- *(deps)* Bump extractions/setup-just from 2 to 3 (#8)
- Trigger ruleset sync
- Trigger ruleset sync
- Add CODEOWNERS
- Add version to treesitter-types-macros path dependency
