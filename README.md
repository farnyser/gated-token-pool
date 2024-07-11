# Gated Token Pool

## Testing

We will use SOL as quote token, and create a new mint for our testing

### Run local validator

```
solana-test-validator
```

### Create wallet for admin and user

And airdrop some SOL

```
solana-keygen new -o admin.json
solana-keygen new -o user.json
solana airdrop 100 -k admin.json
solana airdrop 100 -k user.json
```

### Create token mint 

```
solana config set --keypair admin.json
spl-token create-token --decimals 6 
```

Save the mints (example): 
- `5cHdJFawj7Fac42UddATY3iBdv4N7VSekJxMZqPc2yb7` 

Mint some in your admin wallet

```
spl-token create-account 5cHdJFawj7Fac42UddATY3iBdv4N7VSekJxMZqPc2yb7 
spl-token mint 5cHdJFawj7Fac42UddATY3iBdv4N7VSekJxMZqPc2yb7 1000 
```

### Create pool 

Price of 2000 lamports per native token  (2 SOL per 1 Token) 

```
cargo run --bin client create-pool -k admin.json --token 5cHdJFawj7Fac42UddATY3iBdv4N7VSekJxMZqPc2yb7 --quote So11111111111111111111111111111111111111112 --price 2000
```

Deposit some amount from admin wallet into the program vault

```

```