from solana.rpc.api import Client
solana_client = Client("https://api.mainnet-beta.solana.com")

currentslot = solana_client.get_epoch_info()['result']['absoluteSlot']
print(solana_client.get_epoch_info()['result']['absoluteSlot'])

buffer = 0

Anchor_toml =f'''
[features]
seeds = false
[programs.localnet]
splendor = "xmtn2vByiRMraod2aVHYXB9mxRJQ3Z3Y7SnvSdAy8qn"

[registry]
url = "https://anchor.projectserum.com"

[[test.validator.clone]]
address = "4bcFeLv4nydFrsZqV5CgwCVrPhkQKsXtzfy2KyMz7ozM" # TULIP PROGRAM
[[test.validator.clone]]
address = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v" # USDC
[[test.validator.clone]]
address = "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB" # USDT
[[test.validator.clone]]
address = "Amig8TisuLpzun8XyGfC5HJHHGUQEscjLgoTWsCCKihg" # TUUSDC
[[test.validator.clone]]
address = "gLhY2arqFpmVGkpbBbTi3TeWbsWevA8dqrwbKacK3vJ"  # TUUSDT
[[test.validator.clone]]
address = "8ojYYusVjAKTB7tHjKb9oCBubhhU4vHQDNCcUAdg2ikb" # DEVLET (2)
[[test.validator.clone]]
address = "DpxS9YTv7e7MD9USgniaG4G5n4gVGAk8HJ6p7AXDXcxq" # DEVLET USDC (2)
[[test.validator.clone]]
address = "5KuYEobaCTkQntwdHBZZJpo8HLAb3ruvcWKC1z5RHGJk" # DEVLET USDT (2)
[[test.validator.clone]]
address = "6wQkK76HdRLtVB11V6Tcvp92WmSwSSf6V64kvYbe3xTd" # DEVLET TUUSDC (2)
[[test.validator.clone]]
address = "34bmioDCWJqaig6SN3b28TAAtrbWL35byabBVmagVzR7" # DEVLET TUUSDT (2)
[[test.validator.clone]]
address = "FTkSmGsJ3ZqDSHdcnY7ejN1pWV3Ej7i88MYpZyyaqgGt" # Tulip USDC reserve account
[[test.validator.clone]]
address = "64QJd6MYXUjCBvCaZKaqxiKmaMkPUdNonE1KuY1YoGGb" # Tulip's USDC token account (i.e. the lending pool)
[[test.validator.clone]]
address = "D1cqtVThyebK9KXKGXrCEuiqaNf5L4UfM1vHgCqiJxym" # lendingMarket
[[test.validator.clone]]
address = "HsUZsMSrGBvGBncppitE4Ug8qJypZwHRgStYzCEW8fiD"
[[test.validator.clone]]
address = "2U6kk4iTVqeypBydVPKA8mLTLAQEBfWf4KYfmkcvomPE" # destinationCollateral
[[test.validator.clone]]
address = "ExzpbWgczTgd8J58BrnESndmzBkRVfc6PhFjSGiQXgAB" # USDC price oracle
[[test.validator.clone]]
address = "5JQ8Mhdp2wv3HWcfjq9Ts8kwzCAeBADFBDAgBznzRsE4" # USDC price oracle metadata program id
[[test.validator.clone]]
address = "897c9JQNbYRR7RaDew7mqiZzFF3TXB67gSmk27XNn9vZ" # USDC price
[[test.validator.clone]]
address = "uo3MK2mD9KogjNLxTWVaB5XqA9Hg4mx4QuRm9SRtKdE"# "3vxLXJqLqF3JG5TCbYycbKWRBbCJQLxQmBGCkyqEEefL" # USDT price
[[test.validator.clone]]
address = "Av6XyAMJnyi68FdsKSPYgzfXGjYrrt6jcAMwtvzLCqaM" # USDT product
[[test.validator.clone]]
address = "Csn3exasdhDzxYApmnci3d8Khb629VmgK4NQqdeyZBNt" # Tulip USDT reserve account
[[test.validator.clone]]
address = "124J21csiR1FdDywteXa8LhAmeqBXZRvozhoE7zq9znc" # Tulip USDT reserve liquidity supply

[test.validator]
#url = "m"
url = "https://api.mainnet-beta.solana.com"
warp_slot = "{currentslot+buffer}"
ticks_per_slot = 10000000

[provider]
cluster = "localnet"
wallet = "./devlet2.json"

[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"

[test]
startup_wait = 60000
'''
f = open('Anchor.toml', 'w')
f.write(Anchor_toml)
f.close()