# 🚕 Pamasahe

**Fast, cashless fare payments for public transport powered by Stellar.**

---

## 🎯 Problem & Solution

**Problem:**
Commuters struggle with lack of coins and cash. Drivers lose income due to fare evasion. Current systems are slow or expensive.

**Solution:**
Use Stellar's speed and low fees to enable instant QR payments between commuter and driver, with immutable proof of payment.

---

## 📅 Timeline (Bootcamp)
- **Day 1:** Setup project structure & `pay_fare` function
- **Day 2:** Implement verification & history storage
- **Day 3:** Write all 5 test cases
- **Day 4:** Integrate simple frontend QR scanner & demo

---

## ✨ Stellar Features Used
- XLM / USDC Transfers
- Soroban Smart Contracts
- Persistent Storage
- Event Logging

---

## 🛠️ Prerequisites
- Rust toolchain
- Soroban CLI v22.0.0+

---

## 🚀 Build & Deploy

```bash
# Build
soroban contract build

# Test
cargo test -- --nocapture

# Deploy
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/pamasahe.wasm \
  --source <YOUR_KEY> \
  --network testnet
