# Usage

```
anchor build
anchor test

anchor deploy --provider.cluster devnet

solana program deploy --use-rpc --url devnet --buffer prompt:// --program-id target/deploy/anchor_counter-keypair.json target/deploy/anchor_counter.so
```

Bug with update contract
> Error: Deploying program failed: RPC response error -32002: Transaction simulation failed: Error processing Instruction 0: account data too small for instruction [3 log messages]

Solves:
- Redeploy with new program id
- extend program data with next code

```
solana program extend EX6RpRe8PhX8DzcWzAm481E7nfF3iDsqhnteL1Rqxtd6 20000 -u d 
```