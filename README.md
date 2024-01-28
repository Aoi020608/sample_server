# HAHATOCO

## Deploy

```bash
anchor deploy --provider.cluster localnet
```

## Program ID

Program ID: *GLiyYWDzXiVfTKEx7T1SLsPyh6eohZeBYxvG5Q95ismh*

- [Explorer on devnet](https://explorer.solana.com/address/GLiyYWDzXiVfTKEx7T1SLsPyh6eohZeBYxvG5Q95ismh?cluster=devnet)

## Space
![space](https://hackmd.io/_uploads/SyxoWWrNj.png)

## Libraries

## Problems
- 0x1: Custom program error: 0x1 implies that you have insufficient funds to execute the transaction.
- 0x1004: 0x1004 is hexadecimal for 4100, or the DeclaredProgramIdMismatch error.
So somewhere in your program, you're declaring a different program id than the one you deployed with:


## REFERENCES
- [staking with anchor](https://buildspace.so/p/solana-core/lessons/staking-with-anchor)
- [Token Metadata](https://developers.metaplex.com/token-metadata/mint)
