# Solana PDA Creation Program (Rust)

This Solana program demonstrates **how to create a PDA (Program Derived Address)** using the native `solana_program` SDK (NO Anchor).  
It shows how to:

- Derive a PDA using seeds  
- Verify the PDA passed into the instruction  
- Create the PDA account on-chain  
- Pay rent to make the PDA permanent  
- Use `invoke_signed()` to let the PDA "sign"  

This is a minimal, beginner-friendly template for learning how PDAs work in Rust.

---

## 📌 Program Logic Summary

When the program is called, it expects **three accounts**:

1. **PDA Account**  
2. **User (payer)**  
3. **System Program**

The program:

1. Derives a PDA using:
   ```rust
   seeds = [user.key, b"user"]
