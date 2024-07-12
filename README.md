# Gated Token Pool

## Testing setup

We will use SOL as quote token, and create a new mint for our testing

### Run local validator & deploy program

```
solana-test-validator
```

### Create wallet for admin and user

In another terminal, crea   te new wallet for admin & user, and also airdrop some Sol:

```
solana-keygen new -o admin.json --force --no-passphrase
solana config set -C admin.yml  --url http://localhost:8899 -k admin.json
solana airdrop 100 -C admin.yml
spl-token wrap 1 -C admin.yml
```

```
solana-keygen new -o user.json  --force --no-passphrase
solana config set -C user.yml  --url http://localhost:8899 -k user.json
solana airdrop 100 -C user.yml
spl-token wrap 1 -C user.yml
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
spl-token mint $MINT 1000000 -C admin.yml
```

### Build and deploy the program

```
anchor build
anchor deploy
```

## Usage

### Create pool 

Price of 2000 lamports per native token  (2 SOL per 1 Token) 

```
export POOL=$(./target/debug/client -k admin.json create-pool --token $MINT --quote $QUOTE --price 2000 | grep 'pool:' | cut -d ':' -f 2 | xargs)
echo $POOL
```

### Deposit and or withdraw (as admin)

Deposit some amount from admin wallet into the program vault

```
./target/debug/client -k admin.json deposit  --pool $POOL --amount 100000
```

Withdraw from vault

```
./target/debug/client  -k admin.json withdraw --pool $POOL --token --amount 5000
```

### Create user buy allowance

Create buy authorization for some user

```
./target/debug/client  -k admin.json create-authorization  --pool $POOL --user $USER  --amount 100000
```

### Buy

Buy (as a user)

```
./target/debug/client -k user.json buy --pool $POOL --amount 10000
```