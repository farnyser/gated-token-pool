UQB=$(spl-token balance $QUOTE -C user.yml)
UTB=$(spl-token balance $MINT -C user.yml)
AQB=$(spl-token balance $QUOTE -C admin.yml)
ATB=$(spl-token balance $MINT -C admin.yml)
VQB=$(spl-token balance $QUOTE -C admin.yml --owner $POOL)
VTB=$(spl-token balance $MINT -C admin.yml --owner $POOL)

echo "User Token = $UTB | Quote = $UQB"
echo "Admin Token = $ATB | Quote = $AQB"
echo "Vault Token = $VTB | Quote = $VQB"
