# rescript-zed

ReScript support for [Zed](https://zed.dev) editor.

This extension plugs in the following projects:

- [tree-sitter-rescript](https://github.com/rescript-lang/tree-sitter-rescript) parser
- [@rescript/language-server](https://github.com/rescript-lang/rescript-vscode) LSP

Because this extension uses the same language server as the official VS Code extension, most core language features live or die with `@rescript/language-server`: diagnostics, formatting, hover, go-to-definition, references, rename, code actions, and signature help.

## Settings

```json
{
  "lsp": {
    "rescript-language-server": {
      "initialization_options": {
        "extensionConfiguration": {
          "askToStartBuild": false,
          "inlayHints": {
            "enable": true
          }
        }
      },
      "settings": {
        "zed": {
          "version": "1.72.0"
        }
      }
    }
  }
}
```

`initialization_options` are passed to the language server when it is started. This extension now mirrors the official `rescript-vscode` server defaults for `askToStartBuild`, `logLevel`, `inlayHints`, `codeLens`, `signatureHelp`, `incrementalTypechecking`, and project-config caching, and your local `initialization_options` are merged on top so you can override them per project. See [extensionConfiguration](https://github.com/rescript-lang/rescript-vscode/blob/master/server/src/config.ts).

`initialization_options.extensionConfiguration` accepts the same server-side settings as the official VS Code extension, including `askToStartBuild`, `logLevel`, `inlayHints.{enable,maxLength}`, `codeLens`, `binaryPath`, `platformPath`, `runtimePath`, `signatureHelp.{enabled,forConstructorPayloads}`, `incrementalTypechecking.{enable,acrossFiles}`, and `cache.projectConfig.enable`.

`settings.zed.version` is consumed by this extension and lets you pin a specific npm version of [@rescript/language-server](https://www.npmjs.com/package/@rescript/language-server?activeTab=versions). The legacy `settings.version` key is still supported for backwards compatibility.

## Snippets

The extension now bundles baseline ReScript snippets under `snippets/rescript.json`, including the main official VS Code templates for `module`, `try`, `for`, and common `external` interop patterns, alongside extra convenience snippets like `let`, `switch`, `type`, and `@react.component`.

## Semantic Tokens

Zed keeps semantic tokens disabled by default. If you want the ReScript language server to add richer symbol-aware highlighting on top of tree-sitter, enable them for ReScript:

```json
{
  "languages": {
    "ReScript": {
      "semantic_tokens": "combined"
    }
  }
}
```

This extension ships default semantic-token rules for ReScript's `interface`, `modifier`, and `type` tokens so their styling lands closer to the official VS Code extension, while still letting you override them with your own `global_lsp_settings.semantic_token_rules`.

## Parity Backlog

### Quick wins

- Keep improving tree-sitter queries for highlighting, outline quality, locals, and injections as new syntax edge cases turn up.
- Audit the live semantic-token stream from `@rescript/language-server` and add any remaining ReScript-specific overrides.
- Fill any snippet gaps that still show up in day-to-day ReScript and React usage beyond the core VS Code set.

### Medium-term work

- Audit the current `tree-sitter-rescript` grammar against modern ReScript syntax edge cases.
- Improve completion and symbol labels through `zed_extension_api` hooks where ReScript-specific rendering helps.
- Expand semantic-token coverage once more ReScript-specific token streams are audited in Zed.

### Blocked on Zed extension capabilities

- VS Code-style editor title buttons such as "switch impl/intf" or "open compiled JS".
- Status bar integrations like build state or Code Analyzer controls.
- Command palette actions that are specific to this language extension outside LSP, snippets, slash commands, or MCP servers.

## Developing

See [CONTRIBUTING.md](CONTRIBUTING.md) for instructions on how to develop this extension locally.

## Acknowledgements

This project was originally created by [humaans](https://github.com/humaans/). We're grateful for their initial work in bringing ReScript support to Zed.
