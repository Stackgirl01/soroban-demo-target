# Soroban Demo Target

A deliberately vulnerable [Soroban](https://soroban.stellar.org/) smart contract repository used to demonstrate [Stellar Security Gate](https://github.com/Stackgirl01/stellar-security-gate) — a GitHub Action that scans Soroban contracts for security issues directly in your CI pipeline.

## What's in here

This repo is **not** meant to be a real, deployable contract. It exists purely as a test target to show the Security Gate catching real vulnerability classes in pull requests.

```
.
├── .github/workflows/
│   └── security-gate.yml     # CI workflow that runs Stellar Security Gate on every PR
└── contracts/
    └── vulnerable.rs         # Intentionally vulnerable Soroban contract
```

## The vulnerability

`contracts/vulnerable.rs` implements a simple `withdraw` function with a missing balance check:

```rust
pub fn withdraw(env: Env, from: Address, amount: i128) {
    from.require_auth();
    let balance: i128 = env.storage().instance().get(&from).unwrap_or(0);
    let new_balance = balance - amount;
    env.storage().instance().set(&from, &new_balance);
}
```

**Issue:** there's no check that `amount <= balance` before subtracting. A caller can withdraw more than their actual balance, driving `new_balance` negative — a classic underflow/insufficient-balance-check bug that Security Gate is designed to flag.

## How the CI check works

The workflow at `.github/workflows/security-gate.yml` runs on every pull request targeting `main`:

```yaml
- name: Run Stellar Security Gate
  uses: Stackgirl01/stellar-security-gate@v1
  with:
    fail-on-severity: high
    soroban-path: contracts
```

It scans everything under `contracts/`, and fails the check if it finds any issue rated `high` severity or above — which the missing balance check in `vulnerable.rs` should trigger.

## Why this repo exists

Rather than just describing what Security Gate does, this repo gives it something real to catch — a working example you can point to, fork, or open a PR against to see the Action flag a genuine Soroban vulnerability pattern in action.

## Related

- [Stellar Security Gate](https://github.com/Stackgirl01/stellar-security-gate) — the GitHub Action being demonstrated here
