# Security Cheatsheet

## Checklist

- [ ] Utiliser cargo audit
- [ ] Valider tous les inputs
- [ ] Éviter unsafe sauf si nécessaire
- [ ] Utiliser types sûrs (Option, Result)
- [ ] Ne pas logger de secrets
- [ ] Vérifier les bounds
- [ ] Utiliser Send/Sync correctement

## Commandes

```bash
cargo audit
cargo clippy
cargo test
```

