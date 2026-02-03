# @tego/bot-cli

CLI wrapper for @tego/botjs.

## Installation

```bash
# Install in the workspace
pnpm install

# Build the package
pnpm --filter @tego/bot-cli build
```

## Usage

### Generic invocation

```bash
bot call moveMouse --json '[100,200]'
bot call mouseClick --json '["left", true]'
bot call captureScreen --json '[]' --out screenshot.png
```

### Args input modes

```bash
bot call moveMouse --file args.json
bot call moveMouse --stdin
```

### Dry-run validation

```bash
bot call moveMouse --json '[100,200]' --dry-run
```

### Doctor command

```bash
bot doctor
bot doctor --json-output
```

## Output

- Default output is human-readable.
- Use `--json` to emit machine JSON output.
- For Buffer results, use `--out` to write to file.

## License

MIT
