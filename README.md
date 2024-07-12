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
- `AnF3VpWyZAZU3Kzb4F31XpabJcqeVXgvpNgAMAURFtS4` 

Mint some in your admin wallet

```
spl-token create-account AnF3VpWyZAZU3Kzb4F31XpabJcqeVXgvpNgAMAURFtS4 
spl-token mint AnF3VpWyZAZU3Kzb4F31XpabJcqeVXgvpNgAMAURFtS4 10000 
```

### Create pool 

Price of 2000 lamports per native token  (2 SOL per 1 Token) 

```
cargo run --bin client -- -k target/admin.json create-pool --token AnF3VpWyZAZU3Kzb4F31XpabJcqeVXgvpNgAMAURFtS4 --quote So11111111111111111111111111111111111111112 --price 2000
```

Save the output
> Created pool: p6FHzSGqXdtH3nJ1oWZy2veAM9zcYCtADRfhpqjcUur
>  token vault: 61kAbzVFcw7bw75UBBfdR6zPo1pRJE2ynR9Z3dji1yEo
>  quote vault: 2JoJJFSCBxHp2PXkFoD8GiUjYChL93V6X6fpzhfRtasz


Deposit some amount from admin wallet into the program vault

```
cargo run --bin client -- -k target/admin.json deposit  --pool p6FHzSGqXdtH3nJ1oWZy2veAM9zcYCtADRfhpqjcUur --amount 1000
```

Withdraw from vault

```
cargo run --bin client --  -k target/admin.json withdraw --pool p6FHzSGqXdtH3nJ1oWZy2veAM9zcYCtADRfhpqjcUur --token --amount 50
```

Create buy authorization for some user

```
cargo run --bin client --  -k target/admin.json create-authorization  --pool p6FHzSGqXdtH3nJ1oWZy2veAM9zcYCtADRfhpqjcUur --user A3LMxSj28m9jXCTgYk44XRJFPMokQ7mtCDsQaV9wi8Sh  --amount 1000
```


Buy (as a user)

```
solana config set --keypair user.json
spl-token wrap 1
cargo run --bin client -- -k target/user.json buy  --pool p6FHzSGqXdtH3nJ1oWZy2veAM9zcYCtADRfhpqjcUur --amount 100
```

