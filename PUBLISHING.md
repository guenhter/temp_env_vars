# Publishing crate

Here are some notes when publishing a new version of the crate

1. Ensure all versions are updated
2. Run this

```bash
# Publish the macro first
cd temp_env_vars_macro
sed -i 's/^temp_env_vars/# temp_env_vars/' Cargo.toml
cargo publish --allow-dirty
git checkout -- .

# Publish the "root" crate
cd ..
cargo publish
```
