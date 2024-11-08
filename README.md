
RUST ON SHELL
---

# git-tracker

**git-tracker** is a Rust-based CLI tool to manage, track, and automate Git workflows with features to record changes, generate commit messages, and commit & push changes to remote repositories.

## Features

- **Track Changes:** Record changes with descriptions and categorize them by type.
- **Auto-Commit & Push:** Generate structured commit messages and push them automatically to a designated branch.
- **Change Listing:** View a list of recorded changes.
- **Customizable Commit Templates:** Use predefined templates for commit messages, modifiable in a configuration file.

## Installation

To use `git-tracker`, clone the repository and build the binary using Cargo:

```bash
git clone https://github.com/your-username/git-tracker.git
cd git-tracker
cargo build --release
```

The binary will be located in `target/release/gt`. You can add this path to your system’s PATH variable to use `gt` globally.

## Usage

### Commands Overview

The `git-tracker` CLI offers three main commands:

1. `add`: Record a change with a description and type.
2. `commit`: Commit recorded changes to a Git branch with auto-generated commit messages and push to the remote (optional).
3. `list`: Display a list of all recorded changes.

### Command Details

#### 1. Add a Change

Record a change with a description and specify the type of change (e.g., feature, fix, chore).

```bash
gt add -m "Refactor the login module" -t refactor
```

**Options**:
- `-m, --message`: Description of the change.
- `-t, --type`: Type of change (default: `feature`). Types include `feature`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`.

#### 2. Commit Changes

Commit the recorded changes and optionally push them to the remote branch.

```bash
gt commit -b main --no-push
```

**Options**:
- `-b, --branch`: Target branch for the commit. If not specified, defaults to `main` or the configured default branch.
- `--no-push`: Skip pushing changes to the remote repository.

#### 3. List Changes

View all recorded changes and their details:

```bash
gt list
```

This displays changes with timestamps, types, descriptions, and affected files.

## Configuration

The configuration file `.gt-config.json` in the root directory allows you to customize `git-tracker`'s behavior. This file is automatically generated on first use.

Example `.gt-config.json`:

```json
{
  "default_branch": "main",
  "commit_templates": {
    "feature": "feat: {message}",
    "fix": "fix: {message}",
    "docs": "docs: {message}",
    "style": "style: {message}",
    "refactor": "refactor: {message}",
    "test": "test: {message}",
    "chore": "chore: {message}"
  },
  "auto_push": true
}
```

- **default_branch**: Default branch for commits when not specified.
- **commit_templates**: Custom templates for different change types, where `{message}` is replaced with the change description.
- **auto_push**: If `true`, automatically pushes commits to the remote repository unless overridden by `--no-push`.

## Example Usage

1. Record a new feature:

    ```bash
    gt add -m "Add user profile page" -t feature
    ```

2. Record a bug fix:

    ```bash
    gt add -m "Fix login issue on mobile" -t fix
    ```

3. Commit all recorded changes to `main` branch and push:

    ```bash
    gt commit -b main
    ```

4. View the list of recorded changes:

    ```bash
    gt list
    ```

## Error Handling and Debugging

`git-tracker` includes various checks to handle errors:

- Verifies if the current directory is a Git repository.
- Checks for uncommitted Git changes before attempting to commit.
- Confirms that a remote named `origin` exists before pushing.
- Checks if the target branch exists on the remote and creates it if necessary.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/new-feature`)
3. Commit your changes (`git commit -m 'Add new feature'`)
4. Push to the branch (`git push origin feature/new-feature`)
5. Open a pull request

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---

Just Watching 😎