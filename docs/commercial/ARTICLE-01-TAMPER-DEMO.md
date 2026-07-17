# Technical article draft #1 — Tamper demo

Audience: DFIR practitioners evaluating Community builds.
Status: **DRAFT** — publish when Gate 1 outreach starts.

## Thesis

An independent verifier that fails closed is more honest than a green UI card.

## Demo (synthetic)

```bash
./scripts/self-test.sh
cargo run -p acquire-slint --features gui   # Guided → Fill → Run
# Or follow docs/FOUNDATION-DEMO.md tamper steps:
cp -r PACKAGE PACKAGE-tampered
echo tampered >> PACKAGE-tampered/acquisitions/0001/evidence.raw
cargo run -p trareon-verifier -- verify PACKAGE-tampered
# expect INVALID, exit 2
```

## Takeaway

UI never decides success. Bytes + audit + verifier do. Unsigned ≠ unverifiable.
