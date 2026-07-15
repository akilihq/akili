# akili

Deterministic-first code retrieval for coding agents — grep/AST for the cheap 90%, escalate to the type-checker/LSP semantic layer only when an edge crosses a file boundary, so the model spends its tokens on judgment, not retrieval.

The approach is language-general (grep is universal; tree-sitter and LSP cover most languages). **The current implementation targets TypeScript first**, using the TypeScript compiler for the semantic layer.

## License

[MIT](LICENSE)
