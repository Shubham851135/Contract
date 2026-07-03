# 🗳️ Stellar Soroban Delegation Smart Contract

Contract ID = CCVPYTJLQUHP6KLT7A2HK3XE4MLJIRIFVQBQNJOIWDJMPTUYLN5WQV75

A basic **vote delegation smart contract** built on Stellar using Soroban (Rust).

This project allows token holders to delegate their voting power to a trusted representative, forming the foundation for DAO-style governance on Stellar.




---

## 📌 What it does

This smart contract enables:

- Token holders to **delegate voting rights**
- Changing delegation anytime
- Revoking delegation
- Tracking how many users support a representative

It stores delegation relationships **on-chain** using Soroban contract storage.

---

## 🚀 Features

- ✅ One wallet → one delegate
- 🔄 Change delegation anytime
- ❌ Revoke delegation support
- 📊 Delegator count tracking per representative
- 🔐 Secure authentication using `require_auth()`
- 📡 Event emission for actions (delegate & revoke)
- ⚡ Lightweight and gas-efficient design

---

## 🧠 How it works

1. A user (delegator) selects a representative  
2. The contract stores:
   - Delegator → Representative mapping  
3. Representative's supporter count increases  
4. If delegation changes:
   - Old representative count decreases  
   - New representative count increases

<img width="1920" height="1128" alt="image" src="https://github.com/user-attachments/assets/a536135a-510e-4c4b-826c-746ef95ecdd3" />

-----

## 📂 Project Structure

```bash
.
├── Cargo.toml
└── src
    └── lib.rs
wallet Address = GABCRLENNCRCYQAOJF2BICWLVAFA32EB24RUQAX3H2PFE343MIY4YLRX
Contract ID = CCVPYTJLQUHP6KLT7A2HK3XE4MLJIRIFVQBQNJOIWDJMPTUYLN5WQV75
<img width="1920" height="1128" alt="image" src="https://github.com/user-attachments/assets/a536135a-510e-4c4b-826c-746ef95ecdd3" />

