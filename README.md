# Gated Token Pool

## Testing

We will use SOL as quote token, and create a new mint for our testing

### Run local validator & deploy program

```
solana-test-validator
```

### Create wallet for admin and user

In another terminal, crea   te new wallet for admin & user, and also airdrop some Sol:

```
solana-keygen new -o admin.json
solana config set -C admin.yml  --url http://localhost:8899 -k admin.json
solana airdrop 100 -C admin.yml
```

```
solana-keygen new -o user.json
solana config set -C user.yml  --url http://localhost:8899 -k user.json
solana airdrop 100 -C user.yml
```

We will need user wallet address later:

```
export USER=$(solana address -C user.yml)
```

### Create token mint 

```
export MINT=$(spl-token create-token --decimals 6 -C admin.yml | grep Address | cut -d ':' -f 2 | xargs)
export QUOTE="So11111111111111111111111111111111111111112"
```


Mint some in your admin wallet

```
spl-token create-account $MINT -C admin.yml
spl-token mint $MINT 10000 -C admin.yml
```

### Build and deploy the program

```
anchor build
anchor deploy
```

### Create pool 

Price of 2000 lamports per native token  (2 SOL per 1 Token) 

```
export POOL=$(cargo run --bin client -- -k admin.json create-pool --token $MINT --quote $QUOTE --price 2000 | grep 'pool:' | cut -d ':' -f 2 | xargs)
```

Deposit some amount from admin wallet into the program vault

```
cargo run --bin client -- -k admin.json deposit  --pool $POOL --amount 1000
```

Withdraw from vault

```
cargo run --bin client --  -k admin.json withdraw --pool $POOL --token --amount 50
```

Create buy authorization for some user

```
cargo run --bin client --  -k admin.json create-authorization  --pool $POOL --user $USER  --amount 1000
```

Buy (as a user)
- First wrap some sol to be used to purchase token
- Then issue the buy instruction

```
spl-token wrap 1 -C user.yml
cargo run --bin client -- -k user.json buy --pool $POOL --amount 100
```

See balance:

``` 
UQB=$(spl-token balance $QUOTE -C user.yml)
UTB=$(spl-token balance $MINT -C user.yml)
AQB=$(spl-token balance $QUOTE -C admin.yml)
ATB=$(spl-token balance $MINT -C admin.yml)
VQB=$(spl-token balance $QUOTE -C admin.yml --owner $POOL)
VTB=$(spl-token balance $MINT -C admin.yml --owner $POOL)
echo "User Token = $UTB | Quote = $UQB" 
echo "Admin Token = $ATB | Quote = $AQB" 
echo "Vault Token = $VTB | Quote = $VQB" 
```