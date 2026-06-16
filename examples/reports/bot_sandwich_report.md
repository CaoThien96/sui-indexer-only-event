# Bot Sandwich Checkpoint Report

- Generated: 2026-06-13 02:43:30 UTC
- Bot: `0xf3981a28e88f86255713dada5d7b1ebb23b0b9e499e80fa1406bdd74c3364735`
- SwapEvent: `0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::SwapEvent`
- Ordering: `(transaction_sequence_in_checkpoint, event_sequence_in_transaction)`

## Summary

| Metric | Count |
|--------|------:|
| Checkpoints with bot swaps | 461 |
| Bot buys (atob=false) | 268 |
| Full sandwiches (buy→victim→sell) | 75 |
| Partial (buy→victim, no sell in cp) | 58 |
| Sandwich rate (full / bot buys) | 28.0% |
| Bot buy → victim sell (same/later cp) | 263 (98.1%) |
| — same checkpoint | 8 |
| — later checkpoint | 255 |
| Bot buys, no victim sell after | 5 |

## Bot buy → victim sell

After each **bot buy** (`atob=false`), first **victim sell** (`atob=true`, sender≠bot, same pool) in the same checkpoint or any later checkpoint.

### Pair 1

- **pool** `0x9661cca01a5b9b3536883568fa967a2943e237de11a97976795f5adb293892e9` (same checkpoint)
  - **bot_buy** cp=285676352 `De56n7PhEhDeCZ5atdHoxUGtitWPbZB3qPQ9NF22hhA4` (tx_idx=6, ev_idx=0, amount_in=116000000000)
  - **victim_sell** cp=285676352 `7TjCFo5jgrpHYLwofsSRq4PXFF7GbcvUumgh3K9HEzdK` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=7, ev_idx=1, amount_in=4816224617605)

### Pair 2

- **pool** `0x9661cca01a5b9b3536883568fa967a2943e237de11a97976795f5adb293892e9` (+14 checkpoints)
  - **bot_buy** cp=285678074 `2NYqSGVdbsNh4V4Zqv8LZ58LbKuWQSxWYSk1aoQwhf4Y` (tx_idx=12, ev_idx=0, amount_in=61448085277)
  - **victim_sell** cp=285678088 `FduR7sGDzAnt3N11ZL7gDuySymFqfLp2yWLLKynSdYwt` (sender=`0xa8a6670d32e66762b8ee6d66f57aa847f718551099752a87cfa4ee7058e9b392`, tx_idx=8, ev_idx=2, amount_in=662110255471)

### Pair 3

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+2 checkpoints)
  - **bot_buy** cp=285709468 `FZzzShZFz3cPMbF8vxA1ePtD1LZ1FeBLtNDFrYbn88RW` (tx_idx=2, ev_idx=0, amount_in=104193856784)
  - **victim_sell** cp=285709470 `6nzUyRsQWi2Lt6tQ1fWqHUiNFd4nrYWjzCTY1QbijgBE` (sender=`0xdef166e88048b9a44048b71528529bfa7a956db14b68c04fc8db1e66cf1bd32c`, tx_idx=6, ev_idx=11, amount_in=937199649)

### Pair 4

- **pool** `0x3227fe6ef46d38c05896a65e8365d5812d03b198a51b323ddd4ec13817661442` (+2 checkpoints)
  - **bot_buy** cp=285711508 `CPCbtBoT9YPumbqX15ui6jiJ2mBwbPo5iPtyYW84S3h9` (tx_idx=8, ev_idx=0, amount_in=133774004)
  - **victim_sell** cp=285711510 `E4VJz76K2TMQqe7TtHutqxJSgP84oMh51zAe3GmYW69s` (sender=`0x89a1c807393670de16b055f0316232a5627b94bf74dfaa7ac34d3124109acf19`, tx_idx=8, ev_idx=4, amount_in=25254724)

### Pair 5

- **pool** `0x3227fe6ef46d38c05896a65e8365d5812d03b198a51b323ddd4ec13817661442` (+3 checkpoints)
  - **bot_buy** cp=285712023 `uc69HptKw9xoutPGsn3HHiK48DDdTxU3hcDXkBHSraq` (tx_idx=11, ev_idx=0, amount_in=199984366)
  - **victim_sell** cp=285712026 `HyiPNZ41v5yFCxxG3wW1DkqPZbb3Eq7iDAyqYJTNMw58` (sender=`0x8af2133a24d1097119305ec4262319ebd54e0e6473976a13e94bfe8f3341716f`, tx_idx=6, ev_idx=4, amount_in=20234869)

### Pair 6

- **pool** `0x3227fe6ef46d38c05896a65e8365d5812d03b198a51b323ddd4ec13817661442` (+45 checkpoints)
  - **bot_buy** cp=285712427 `CyofY9ENxEi6KGoTDN9reZw2qV9ek7HkHthRC6aAtGD8` (tx_idx=2, ev_idx=0, amount_in=217016408)
  - **victim_sell** cp=285712472 `GUT5R27t1DVZmTn696ohmguNtNnKKmrCNGrF3yGKtVbN` (sender=`0x70a561b0ecf5b64a473cf6763d53975095ded30630a4666fa46fea07a9f18aba`, tx_idx=12, ev_idx=6, amount_in=14083492)

### Pair 7

- **pool** `0x9661cca01a5b9b3536883568fa967a2943e237de11a97976795f5adb293892e9` (+3 checkpoints)
  - **bot_buy** cp=285736227 `134MeVNQammaaij5MRGm1i6zwx5jFVPjCHHVrLjMdr4H` (tx_idx=1, ev_idx=0, amount_in=12270412000)
  - **victim_sell** cp=285736230 `935nb3mT7D3KbjAjT8pdDgxCmgrqRx1Q19uWhfSkXU2c` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=5, ev_idx=1, amount_in=2644999113176)

### Pair 8

- **pool** `0x51e883ba7c0b566a26cbc8a94cd33eb0abd418a77cc1e60ad22fd9b1f29cd2ab` (+3 checkpoints)
  - **bot_buy** cp=285738270 `5oXvDCg1fFRFCq31Rn9PnzBXHboWJXdGP2pBoZgSq9pw` (tx_idx=12, ev_idx=0, amount_in=104491949812)
  - **victim_sell** cp=285738273 `F6RigZQv5qwLaGZor44qTeienbcSafpXxSu5DBQ1n7Sj` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=9, ev_idx=0, amount_in=229744072)

### Pair 9

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+2 checkpoints)
  - **bot_buy** cp=285740249 `zR3BMdR6bTgEzLzr3scS6MH25P1rin3FrzgzodfegkC` (tx_idx=3, ev_idx=0, amount_in=104527693243)
  - **victim_sell** cp=285740251 `28D5nbg7rSW11d5mbo4zsrPCfBjpoMgFD7STXR1tuWXp` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=4, ev_idx=1, amount_in=7257427885)

### Pair 10

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+2 checkpoints)
  - **bot_buy** cp=285740637 `C1Wm4UtAHiDoj5VY4qVr8b5AQCwwscwjmk9pVD3RcvV8` (tx_idx=9, ev_idx=0, amount_in=104723967914)
  - **victim_sell** cp=285740639 `ex5Scau7UZw4sa4LXzRef3vuVX8gQvrsT8M3JHWQ6dY` (sender=`0x1eb59ebed1febea954bdf8b1a17f4ea388a326f32ad3a4ae357015216092e834`, tx_idx=16, ev_idx=0, amount_in=3448595840338)

### Pair 11

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+3 checkpoints)
  - **bot_buy** cp=285750290 `7o21YHnQFGFx7ffq25tmmCiQe3pKWSqrwoAASityFJ83` (tx_idx=5, ev_idx=0, amount_in=104484439699)
  - **victim_sell** cp=285750293 `3t5wBdjeMTLccEDjMC7mGbRVXn8zHMJDmL5P2qJJPbmb` (sender=`0x9cad98bde3e40d10fec68a6d6de179f53b2fcdce339519db9599fb8fe2b7f6c2`, tx_idx=3, ev_idx=4, amount_in=330166761070)

### Pair 12

- **pool** `0x8049d009116269ac04ee14206b7afd8b64b5801279f85401ee4b39779f809134` (+3 checkpoints)
  - **bot_buy** cp=285752967 `BZw5dt7tkMcz9Yv6fWXWCsx4Hfji3BDmG152gyWK1Hjb` (tx_idx=6, ev_idx=0, amount_in=228386486)
  - **victim_sell** cp=285752970 `57ZZtgRUn2cSb3CD7AkSr2fxNZ8nvTFiW1bmJXGc1MqK` (sender=`0xd9785193d47c87d346301a29b318f2b1f393b3290f46047e84dc77e25ef7a362`, tx_idx=25, ev_idx=2, amount_in=225943454147)

### Pair 13

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+4 checkpoints)
  - **bot_buy** cp=285768408 `qraeFtmnbyMXnjNx76LwegJZFCeMkBFsBcaUib36XU4` (tx_idx=6, ev_idx=0, amount_in=105112663172)
  - **victim_sell** cp=285768412 `JBnZrTK2sQRWCHbGw78hNRGWHkj184GKdncQwNSdoqBU` (sender=`0xdef166e88048b9a44048b71528529bfa7a956db14b68c04fc8db1e66cf1bd32c`, tx_idx=1, ev_idx=5, amount_in=858069699342)

### Pair 14

- **pool** `0xd6918afa64d432b84b48088d165b0dda0b7459463a7d66365f7ff890cae22d2d` (same checkpoint)
  - **bot_buy** cp=285770942 `CnisAAHQ6L6vJBrYDYGJR5A6BwBURHizHtRXY27deuSx` (tx_idx=5, ev_idx=0, amount_in=105237587143)
  - **victim_sell** cp=285770942 `4RkusTaz2kPEThZNitq7nNFE5vntCLpaNJYo8xXqr6XA` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=13, ev_idx=2, amount_in=11734086856495)

### Pair 15

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+7 checkpoints)
  - **bot_buy** cp=285772436 `FZmNKeWaccsVHzvXGERK6HcYmfec5WkBDyzUv7qHz2oL` (tx_idx=13, ev_idx=0, amount_in=105896194273)
  - **victim_sell** cp=285772443 `GHURkP6PU2m15D2xMDQTRN7gmHBhYc5FcNGWaTdkBtM7` (sender=`0x1eb59ebed1febea954bdf8b1a17f4ea388a326f32ad3a4ae357015216092e834`, tx_idx=7, ev_idx=0, amount_in=13289119620094)

### Pair 16

- **pool** `0x3f77391f6b33ca2967430490c68dab38596608d05fc19d1ac9c3797595a8fddb` (+113 checkpoints)
  - **bot_buy** cp=285774668 `7JYkttQJUDWYW8E73GRFGvUUxFUNduSFH9CgEKts4PTU` (tx_idx=1, ev_idx=0, amount_in=2313565508)
  - **victim_sell** cp=285774781 `3wbwvJPEfHb6Q9ntpemdzhV83f1iq9gPfGNqHP21Cpaz` (sender=`0xa7d896a7fe4709e8dc46c62d1328bb89fdf19fe4b659a7423b614acb438b1726`, tx_idx=12, ev_idx=0, amount_in=120805931162)

### Pair 17

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+2 checkpoints)
  - **bot_buy** cp=285778003 `Akujx6zWr1PiDYGP2zswDpUEsdFpnuEiysmVsivbt2uK` (tx_idx=7, ev_idx=0, amount_in=105800762067)
  - **victim_sell** cp=285778005 `BxMxEMzJ3uCrvEJGtNVJB5cTiBWTUkvU5w1fuP4EcneA` (sender=`0x788a9ada3f7ee01cb93352878d84e68dce92a3ebcdd418f7dde34ccba262db6b`, tx_idx=14, ev_idx=6, amount_in=409346354670)

### Pair 18

- **pool** `0x8049d009116269ac04ee14206b7afd8b64b5801279f85401ee4b39779f809134` (+3 checkpoints)
  - **bot_buy** cp=285779267 `2fr4nsQYPxRD9BdvnTZbSBF77UkNgUv7F6mbUtPWeDxc` (tx_idx=12, ev_idx=0, amount_in=285958529)
  - **victim_sell** cp=285779270 `EgoXWoXh9NWWMeuZK4GgwyxsPkddFhZodMudCtZaMFjE` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=13, ev_idx=3, amount_in=971438148482)

### Pair 19

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+21 checkpoints)
  - **bot_buy** cp=285780033 `ADsemBx8Pnjva44RP3GifjSWr69KyrYKNfT6hKBfZPNL` (tx_idx=11, ev_idx=0, amount_in=105981469530)
  - **victim_sell** cp=285780054 `Cm6jekx3j8xuNPiCugQSrRqKBu8eTWMDrD3LujtaHfJK` (sender=`0x12929fa913c0c584746b4894704859da4440682193820517905fdee09eeb32e5`, tx_idx=6, ev_idx=1, amount_in=309097703071)

### Pair 20

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+2 checkpoints)
  - **bot_buy** cp=285781303 `4dgLRQ71vaeF8uFKCoqTrSDwsWTnc7UE9uGP9q8xezcL` (tx_idx=6, ev_idx=0, amount_in=106003900201)
  - **victim_sell** cp=285781305 `7XrVyibMyjCQ7Cj9xnuxf7nvZ1vyohmJLgi7Fmcno1gF` (sender=`0x1eb59ebed1febea954bdf8b1a17f4ea388a326f32ad3a4ae357015216092e834`, tx_idx=9, ev_idx=0, amount_in=3022941079586)

### Pair 21

- **pool** `0x31063a1f5775edae48d137f96a30c500733a54f64646e97f71561ba3abe49bd6` (+107709 checkpoints)
  - **bot_buy** cp=285781569 `Cb8K5oCdLvd22p2Zc8hap5uHfJAerfysxZcJoMQrLKtd` (tx_idx=25, ev_idx=0, amount_in=20369220451)
  - **victim_sell** cp=285889278 `CaZ3M6ahsW9JUZ29ShX1M4FMFynWF9itMRGG7Ec1bTho` (sender=`0xa6c8aa8ddcc3ffebb59c0a38d8019466d6e5b7bf51c230e7b76c853082c499b2`, tx_idx=7, ev_idx=2, amount_in=7999918)

### Pair 22

- **pool** `0x2d3230025b4615087656952bf5ddb49d7a9b6712ac9aa14977a877f02a16f165` (+3 checkpoints)
  - **bot_buy** cp=285786538 `B7CPhFHiPv9Y1FzokacAjeNKHdZTFwWKCz5ebFrqtHyM` (tx_idx=7, ev_idx=0, amount_in=4239247464)
  - **victim_sell** cp=285786541 `FVkka4BwBehvddzeaNLn39gwqnYc6RtPuukTebVpbia2` (sender=`0x55c169e02937a275b1467d622f33fc48595fe754ecb92360301272d2d9debbbf`, tx_idx=7, ev_idx=3, amount_in=12958425105)

### Pair 23

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=285791533 `9aE2JUdZmvntfE6zZ376pEv3Tj67idV7HkKWDqgiXm6m` (tx_idx=5, ev_idx=0, amount_in=8241294518)
  - **victim_sell** cp=285791536 `EHNf5WFQV2CKzu7J5vvd8QiuFHzh4b1y2vtJZ29PWaw3` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=10, ev_idx=7, amount_in=9684898734318)

### Pair 24

- **pool** `0x0254747f5ca059a1972cd7f6016485d51392a3fde608107b93bbaebea550f703` (+2 checkpoints)
  - **bot_buy** cp=285794179 `6MDCaqtbn35EC9qnAWfK2EU6KmwJ4xyJLTF1sn6VKfYs` (tx_idx=1, ev_idx=0, amount_in=105977622614)
  - **victim_sell** cp=285794181 `5RiwmRMqB4KqjxchrUyrM67m7K9Pdiws8NkTuxzoYmZf` (sender=`0xa5a4b4c353d6033ce739c0f5be43af578969ebdc9a5ddce7bd65aabb96888bbc`, tx_idx=11, ev_idx=9, amount_in=289023534412)

### Pair 25

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+24 checkpoints)
  - **bot_buy** cp=285794980 `GSQsXvv8PnE4wd9aT2Rcf46Z1j5U9WqKFx9H14L8dq2W` (tx_idx=37, ev_idx=0, amount_in=106039606165)
  - **victim_sell** cp=285795004 `Daxw9G7MVMW4pEJR3JaE3Fcf59pV1xDE1taFu9MKrSQq` (sender=`0x54fb1157d7e3a63822e1ac91b154c432d70ea294cb4de5ccfe9b70a38165affc`, tx_idx=13, ev_idx=1, amount_in=144877882416)

### Pair 26

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3` (+252 checkpoints)
  - **bot_buy** cp=285797664 `A1DtgRmb3a7uWdekSBadMFRv3HVvkwGa2bRsaHWSrfXj` (tx_idx=6, ev_idx=0, amount_in=13158949717)
  - **victim_sell** cp=285797916 `4DYgTMSy8iXBfnZb8wtvvKkQMwkfTL1n2Dmr1poVCTQW` (sender=`0x329bb29dc45c568388673fa79f7f0cd0c442496ae22aa21646b103728e11bc33`, tx_idx=16, ev_idx=0, amount_in=983691446859903)

### Pair 27

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3` (+245 checkpoints)
  - **bot_buy** cp=285797671 `Bo9zjoW7hSo1BgjvyAE138VQNM3HHc2eN8Jpy7oHo5y7` (tx_idx=8, ev_idx=0, amount_in=15747696093)
  - **victim_sell** cp=285797916 `4DYgTMSy8iXBfnZb8wtvvKkQMwkfTL1n2Dmr1poVCTQW` (sender=`0x329bb29dc45c568388673fa79f7f0cd0c442496ae22aa21646b103728e11bc33`, tx_idx=16, ev_idx=0, amount_in=983691446859903)

### Pair 28

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3` (+581 checkpoints)
  - **bot_buy** cp=285798420 `Gbt8Yvi7jNbkz8VPR3dQsHdJGj6NaaL7QoW6GeZfP4SJ` (tx_idx=9, ev_idx=0, amount_in=23981035955)
  - **victim_sell** cp=285799001 `G1i7BsM98tcyvWsAy6soHcioqqwrJUdfV8jnfkKUvykb` (sender=`0x8613e98201d4d1d5d3fc2d203ec56acdbc798832f09e2e61aa50573339e37e76`, tx_idx=2, ev_idx=0, amount_in=683750185973700)

### Pair 29

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3` (+67 checkpoints)
  - **bot_buy** cp=285798934 `9N5wT1m59RMiaSkouvJ3HkvNzfhoLsBQiLQg2Z4MTDb5` (tx_idx=9, ev_idx=0, amount_in=27822074128)
  - **victim_sell** cp=285799001 `G1i7BsM98tcyvWsAy6soHcioqqwrJUdfV8jnfkKUvykb` (sender=`0x8613e98201d4d1d5d3fc2d203ec56acdbc798832f09e2e61aa50573339e37e76`, tx_idx=2, ev_idx=0, amount_in=683750185973700)

### Pair 30

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3` (+472 checkpoints)
  - **bot_buy** cp=285799353 `4Gg6X6UBB8jpfxxXuHoRdJHqvj67wixRytLeN7jau3wp` (tx_idx=3, ev_idx=0, amount_in=27778520234)
  - **victim_sell** cp=285799825 `9c4UjvsWsswzZknkC45HpUagEHbBbRr2iyZBtRJzzoru` (sender=`0x1807983e21bffd37ac715723e6ecdc98813e24ffca82ccb9fddd56fbad337d9d`, tx_idx=9, ev_idx=0, amount_in=718502404149891)

### Pair 31

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3` (+409 checkpoints)
  - **bot_buy** cp=285799416 `AY2NGKvjeF5TFbWurSoPF225AkZVSdU74AkuVM1QMNhv` (tx_idx=1, ev_idx=0, amount_in=27809860002)
  - **victim_sell** cp=285799825 `9c4UjvsWsswzZknkC45HpUagEHbBbRr2iyZBtRJzzoru` (sender=`0x1807983e21bffd37ac715723e6ecdc98813e24ffca82ccb9fddd56fbad337d9d`, tx_idx=9, ev_idx=0, amount_in=718502404149891)

### Pair 32

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3` (+330 checkpoints)
  - **bot_buy** cp=285799495 `9qmLPNCZ28RGVzUuwu3WkiBY6V9ny5pBQCJuL6k1upPy` (tx_idx=11, ev_idx=0, amount_in=27855748834)
  - **victim_sell** cp=285799825 `9c4UjvsWsswzZknkC45HpUagEHbBbRr2iyZBtRJzzoru` (sender=`0x1807983e21bffd37ac715723e6ecdc98813e24ffca82ccb9fddd56fbad337d9d`, tx_idx=9, ev_idx=0, amount_in=718502404149891)

### Pair 33

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3` (+151 checkpoints)
  - **bot_buy** cp=285799980 `3dt5JjJQTkqon8dieWnCjGonpAB88fJTYBcyopm2zwgB` (tx_idx=1, ev_idx=0, amount_in=27945042126)
  - **victim_sell** cp=285800131 `Ek3ofcoNXEjMPFrRSQSQjMh8My5P7hctnA2LDci4FBTK` (sender=`0xfa511cbf6cf29c3e0207e35a77504aced1b52ce6c53fd3abe88382db31288f31`, tx_idx=3, ev_idx=0, amount_in=736508686050108)

### Pair 34

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3` (+65 checkpoints)
  - **bot_buy** cp=285800066 `9w2Pb7hZ4digEFz4cwhxwLEAN3NH58quKRJuGnvDkCdJ` (tx_idx=13, ev_idx=0, amount_in=27996031940)
  - **victim_sell** cp=285800131 `Ek3ofcoNXEjMPFrRSQSQjMh8My5P7hctnA2LDci4FBTK` (sender=`0xfa511cbf6cf29c3e0207e35a77504aced1b52ce6c53fd3abe88382db31288f31`, tx_idx=3, ev_idx=0, amount_in=736508686050108)

### Pair 35

- **pool** `0xfc6a11998f1acf1dd55acb58acd7716564049cfd5fd95e754b0b4fe9444f4c9d` (+4 checkpoints)
  - **bot_buy** cp=285800202 `Gbjh7iJ88HQJEjJes6AF6qwRwUFLjPAi41EbWtuZzHeP` (tx_idx=26, ev_idx=0, amount_in=13103500193)
  - **victim_sell** cp=285800206 `3DqToMYDrcreM6D1wX86ScGHfGZxFUkns9RbvzwumtCQ` (sender=`0x00006f748f809057fd1ca9ff8d02d89947f9079c26029ea2348657d8467b0000`, tx_idx=1, ev_idx=4, amount_in=4324620548914)

### Pair 36

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3` (+66 checkpoints)
  - **bot_buy** cp=285800286 `9bRPiuQamtFtDx7RvNZa9R5mYMWpmr2dneWAPowoG7Dk` (tx_idx=9, ev_idx=0, amount_in=28007130258)
  - **victim_sell** cp=285800352 `CmZP2AV68KT1rZEj5aZHoVV4btjJsMDSuXD2DU8PjKWh` (sender=`0x092d21d8796de90b8b10c15e016df7263bb9e7c58f924dbee7fe76a991986e24`, tx_idx=10, ev_idx=0, amount_in=312812624845488)

### Pair 37

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3` (+69 checkpoints)
  - **bot_buy** cp=285800569 `t95dWUynYv8hKnyCFyrbGErNTuXgf5cKJoQ8fMZTXCo` (tx_idx=1, ev_idx=0, amount_in=28019982090)
  - **victim_sell** cp=285800638 `2k9XL9uHvjtdSEiptfnykD1AGfuvPbuqCmLTEkJ3UnfC` (sender=`0x690e6a7505b19fec372d03ca77b08028f45cbcc761d5142d855ad9ec89dc02a4`, tx_idx=9, ev_idx=0, amount_in=466471002778635)

### Pair 38

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3` (+149 checkpoints)
  - **bot_buy** cp=285801080 `BaRFNxYyVSCvtNdNwc4RrhQZcRRwUJ53mP4F7mtgkYRP` (tx_idx=4, ev_idx=0, amount_in=28072238785)
  - **victim_sell** cp=285801229 `5awXiP7jR6CskMLrzU8XDgS5W2HD77au3vpWCYH4Shou` (sender=`0x86f05c502a2d887f88042a12f4578057fb56c4a169a599e1cc336f9b2541d9c8`, tx_idx=1, ev_idx=0, amount_in=370962955242182)

### Pair 39

- **pool** `0x2d3230025b4615087656952bf5ddb49d7a9b6712ac9aa14977a877f02a16f165` (+3 checkpoints)
  - **bot_buy** cp=285801346 `22cSfx2BRUqnDWZL1gqiB3oC1okyeDE5Y2GkdWbyAMs5` (tx_idx=1, ev_idx=0, amount_in=4240495021)
  - **victim_sell** cp=285801349 `Eecy3Yhzsqw62xYeFUFK1RzhWRN8GabmDQp4dF9faGGm` (sender=`0x8af2133a24d1097119305ec4262319ebd54e0e6473976a13e94bfe8f3341716f`, tx_idx=10, ev_idx=4, amount_in=25504418835)

### Pair 40

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3` (+57 checkpoints)
  - **bot_buy** cp=285801353 `6Fk4thks9WtLx413wJk5vzPtU1sMaxNDJjoZD1q4UTic` (tx_idx=2, ev_idx=0, amount_in=28098121706)
  - **victim_sell** cp=285801410 `Gb4CLW3guGSZ5kQYrdoZYaRbSMWANkMFivGc6Lhzhsta` (sender=`0xd3a2258bae9214e0e136023814d37c436e504ea6c5939d25b04aeaea3e1ff2c7`, tx_idx=1, ev_idx=0, amount_in=326587557432432)

### Pair 41

- **pool** `0xc23e7e8a74f0b18af4dfb7c3280e2a56916ec4d41e14416f85184a8aab6b7789` (same checkpoint)
  - **bot_buy** cp=285801398 `HKXtaeEN3KgEM7BSbrLi1Jcbm9QtpsMYsLWvFamahtqL` (tx_idx=25, ev_idx=0, amount_in=125015767782)
  - **victim_sell** cp=285801398 `28y73bowqU485vgQQ7RLBHBTmMjcj9Njfr18u69UTjvC` (sender=`0xdef166e88048b9a44048b71528529bfa7a956db14b68c04fc8db1e66cf1bd32c`, tx_idx=26, ev_idx=3, amount_in=18357979416229)

### Pair 42

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3` (+66 checkpoints)
  - **bot_buy** cp=285801501 `3hHt9u38Gphq9s9MKE7QVMiHsxKkoEiuyE7pVvv4J87x` (tx_idx=1, ev_idx=0, amount_in=28109994784)
  - **victim_sell** cp=285801567 `ENwz8s6eK5DjN9hrS6jy3GZfVAja4UJU3jtfFGDwVzqU` (sender=`0x786b77d80b2280c615a12985f894bdf2507caed3ae2bcc07bb58abe26ab0513f`, tx_idx=12, ev_idx=0, amount_in=452160917177914)

### Pair 43

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3` (+179 checkpoints)
  - **bot_buy** cp=285801627 `CoQibSgbarAvNiJKAMYU9wjN1k2wtDRuGYAcVz142jCs` (tx_idx=5, ev_idx=0, amount_in=28120409480)
  - **victim_sell** cp=285801806 `DwB4dEeYqrDRXMZQNFW6vbN9ZnoZ5mJVj1Z4QhZ8bNrj` (sender=`0x0ea94a26e3d941f572d4332b3e13633dd34e25dab705c90eb9a81600a0bd162b`, tx_idx=9, ev_idx=0, amount_in=773170973780487)

### Pair 44

- **pool** `0xc23e7e8a74f0b18af4dfb7c3280e2a56916ec4d41e14416f85184a8aab6b7789` (+11 checkpoints)
  - **bot_buy** cp=285801775 `AgR37VuCGtSBCAXuyucgjUJSd6fB3JrgiEadmYXSbvCA` (tx_idx=3, ev_idx=0, amount_in=124411843592)
  - **victim_sell** cp=285801786 `2ne7g2ii3q5bwZyuZ7eQ6SQiuvHubpmHAS5B2gxn86LN` (sender=`0x931aa7fa369d0384ec4553542454b574ae4736e6da5bdab8ea8d1f147e893528`, tx_idx=5, ev_idx=1, amount_in=3209112304440)

### Pair 45

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3` (+138 checkpoints)
  - **bot_buy** cp=285802400 `5jgMm3nqoLKP5tx4YDY1n348bMHMtBqMKjcDhuLsw3kS` (tx_idx=12, ev_idx=0, amount_in=28278719794)
  - **victim_sell** cp=285802538 `7Whmc48LLfVWvybh4BVydZV8f3ruHEGFWsddhEP1xBRV` (sender=`0x2df297c84c1ce04113819d3f8f7f72036b9d3b48fdfb1b6818426d485c345aa3`, tx_idx=10, ev_idx=0, amount_in=602373903235560)

### Pair 46

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3` (+880 checkpoints)
  - **bot_buy** cp=285802695 `DRYVn5o4cWygsD3mX5QMys2fN1qmL7EH2KQtg8nSt3Qg` (tx_idx=10, ev_idx=0, amount_in=28232810168)
  - **victim_sell** cp=285803575 `XL431HJFi5KYooFw4fcobrjb4UY6NFcQjRGjwQUxg4K` (sender=`0x46a5d763cb06fe9accef4212a7c44c15cb099ff3e4d43eba092572bffc8f5fa3`, tx_idx=3, ev_idx=0, amount_in=503850907514450)

### Pair 47

- **pool** `0x03d135b439d55511a6d7d98fafe5b92093f78b14c522d9d4a8cb004df5aead4f` (+3 checkpoints)
  - **bot_buy** cp=285802753 `FVkbVcvbPqf5S9WNHrUtE57G94xtihzpSj8EJnUPeQ57` (tx_idx=5, ev_idx=0, amount_in=123598019276)
  - **victim_sell** cp=285802756 `4ounP4YuEi75T5J8yqi245erH9HLN1ozortBa8vVP7wV` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=31, ev_idx=7, amount_in=7317446754)

### Pair 48

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3` (+39 checkpoints)
  - **bot_buy** cp=285804024 `4pbsWadv4MP4QKNYpjVaRfhcGuLKCgqFjr2AzvAmgHTD` (tx_idx=1, ev_idx=0, amount_in=28506488196)
  - **victim_sell** cp=285804063 `EJZcFXRAjKHH3ubtjFYZZaTfLwGQuZiYqERCu5KmGruE` (sender=`0xb9e8412ec49b4530e1b78a54a5eaf56cad0d9d0636c2677d6b4f5be6d0672c58`, tx_idx=2, ev_idx=0, amount_in=506354961493239)

### Pair 49

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3` (+9 checkpoints)
  - **bot_buy** cp=285804946 `CFz6ddg3bD5Gr5ePfxNo7XX5GVZ4Hbh2P2sVuNXyhK4e` (tx_idx=4, ev_idx=0, amount_in=29071069360)
  - **victim_sell** cp=285804955 `71d8j4EUQ88nq7AT7cmAfUcL2118ch6nQUzJGnuTrD5Z` (sender=`0x8a167b71445110865124a3600ba6f7d1c0426697f4d53218d04e34701446a8e5`, tx_idx=11, ev_idx=0, amount_in=1084850528679026)

### Pair 50

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3` (+317171 checkpoints)
  - **bot_buy** cp=285805047 `BxwtBmzAWZLsVTqGPSCRQqqDJcG2jDDPA2dKQJbYuFrn` (tx_idx=1, ev_idx=0, amount_in=29242866687)
  - **victim_sell** cp=286122218 `28nXBird5cKphdYUQ6Lpt68JVBGAF5Btop6LSotmuZBn` (sender=`0x33a6dcc43399a6ee9d0540059854a0842283a75d92178e6d5076125d1763bd40`, tx_idx=3, ev_idx=0, amount_in=85678671)

### Pair 51

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=285807472 `F87tjx2qotG2q9UZye96gYtKAK1RPem4f7JWA8tUaBeo` (tx_idx=4, ev_idx=0, amount_in=8320444625)
  - **victim_sell** cp=285807475 `FvS6N1jeay1NYEHNtVZyBVvccGiAg4YanT1s49fZHHy8` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=4, ev_idx=12, amount_in=16791000000000)

### Pair 52

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=285807503 `BGkLnm15vFB8etV3mn9JsnseKJf7usPcjNRS9HWFz3jJ` (tx_idx=10, ev_idx=0, amount_in=8329679752)
  - **victim_sell** cp=285807506 `6JG44Mnrt94Rt1ijdkkkXuKUvR8KzJLG9vx6gbEAnfNQ` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=13, ev_idx=7, amount_in=8423060224411)

### Pair 53

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+4 checkpoints)
  - **bot_buy** cp=285807765 `9EdfJZq2XLAWQwhoxboCvdtYCmMnXUuRL36KtxZnyKNL` (tx_idx=11, ev_idx=0, amount_in=8501722685)
  - **victim_sell** cp=285807769 `6CrD41pQuMDJyQ7fosegokLS95jVGiWDLbsExGZvTcy8` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=2, ev_idx=7, amount_in=24424102600016)

### Pair 54

- **pool** `0x9661cca01a5b9b3536883568fa967a2943e237de11a97976795f5adb293892e9` (+5 checkpoints)
  - **bot_buy** cp=285808231 `rRYBbBFG6J6VP1XyZRnQaa9dZBKFMFou7uvNZ3mdN9d` (tx_idx=12, ev_idx=0, amount_in=12232187832)
  - **victim_sell** cp=285808236 `28AyxY15HeZrb4oJstDUEZku7ME5pbajwaXqzWc8beuT` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=9, ev_idx=1, amount_in=2805957403002)

### Pair 55

- **pool** `0x9661cca01a5b9b3536883568fa967a2943e237de11a97976795f5adb293892e9` (+4 checkpoints)
  - **bot_buy** cp=285808268 `GSq3bG6kNUGiiuYCfxCodmoJ1g82L3rtLYQReqNiWhUY` (tx_idx=9, ev_idx=0, amount_in=12253496126)
  - **victim_sell** cp=285808272 `FehCm9VkTmLzHtU5Hhw65i7MEypo8uqTLdpDbD6jyvwT` (sender=`0x89a1c807393670de16b055f0316232a5627b94bf74dfaa7ac34d3124109acf19`, tx_idx=3, ev_idx=6, amount_in=1111327799163)

### Pair 56

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+1 checkpoints)
  - **bot_buy** cp=285809713 `EKrXeT3H8b9EqnkweZS9QFBu9tnwVCFV27dfUA6UGg4S` (tx_idx=7, ev_idx=0, amount_in=122784121242)
  - **victim_sell** cp=285809714 `8GnU6V4VniQaoNBZPArWxMePJAvwZRyE93qjBAMJHfR2` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=4, ev_idx=1, amount_in=1222658553129)

### Pair 57

- **pool** `0x3f77391f6b33ca2967430490c68dab38596608d05fc19d1ac9c3797595a8fddb` (+4063 checkpoints)
  - **bot_buy** cp=285811695 `2zp4tfNeh6xRSMFbbx1HkGNLoGuuYS25kRLbWsctSBdD` (tx_idx=15, ev_idx=0, amount_in=2298384862)
  - **victim_sell** cp=285815758 `9Fx5NE6rKU2grYVmvqMAP5JhKM8VHUM1CcPwxiD4PxK7` (sender=`0x897180eeaeb0198f01fd4d25159c89828ad21edeb4abd2eedd57b2b5f0e69a54`, tx_idx=3, ev_idx=0, amount_in=467097612636)

### Pair 58

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=285817140 `DkiqhF6M4FeRmreW4C1i5bNxNovvxwGDDkqmXR3WMQho` (tx_idx=15, ev_idx=0, amount_in=9236862905)
  - **victim_sell** cp=285817143 `3cxHYFopGz4JQfeDBr1VybxjvTm1e8rdPEiHKqwrGnxK` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=16, ev_idx=7, amount_in=9956856558597)

### Pair 59

- **pool** `0xc23e7e8a74f0b18af4dfb7c3280e2a56916ec4d41e14416f85184a8aab6b7789` (+2 checkpoints)
  - **bot_buy** cp=285818228 `7utVL3aT5CPnYbF4hh4dLJoUrBS2XghzoJfYiFPiHEr5` (tx_idx=5, ev_idx=0, amount_in=40217279093)
  - **victim_sell** cp=285818230 `C2TMZ7FzRqqej2G3KSAp7mLTFa388bAG8Tyh47s8xgbT` (sender=`0xdef166e88048b9a44048b71528529bfa7a956db14b68c04fc8db1e66cf1bd32c`, tx_idx=6, ev_idx=7, amount_in=13358982109190)

### Pair 60

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=285818324 `A8k2bHrrUXXcAM2PPrB9Vj1LCfook81JVPmGAhaYzQSn` (tx_idx=23, ev_idx=0, amount_in=19086710949)
  - **victim_sell** cp=285818327 `6sM7YCHCXNXn4bd7yeAaEW7BawU3JwnBo59aWkjfqws7` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=18, ev_idx=7, amount_in=29266333639981)

### Pair 61

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=285818745 `AheMAQp1EtDQikZhhZLyZfMnGZpDvvG513S5VSuiFhcj` (tx_idx=1, ev_idx=0, amount_in=18890194960)
  - **victim_sell** cp=285818748 `6KqcGCLG9zaTwvARg3pvhdU3pcVDRNmCVYy45fTc5h5o` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=15, ev_idx=7, amount_in=46936475772956)

### Pair 62

- **pool** `0x76b7709caa7d74649d9bf1abb5f38ef452564b61496c1876486e6f500abb6b5b` (+61 checkpoints)
  - **bot_buy** cp=285819928 `8zs4jUVzp4s9ZNZW9pFTAXfdfUa8AHGgjJ1LbWPyRnKW` (tx_idx=6, ev_idx=0, amount_in=60658676358)
  - **victim_sell** cp=285819989 `DFCguEx5biiFQjwCmamRjgYNPbtcCd6xeoLYGdp7hBF7` (sender=`0x54845cc51a9a3328db6009af2394e00dc5947ba81840a239f418e4d103d76664`, tx_idx=30, ev_idx=0, amount_in=159425443978248)

### Pair 63

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=285820122 `6KbFT2aCFUxN1FD5xxU6nPBrQz2h1BSbLvN5kVoM8izg` (tx_idx=1, ev_idx=0, amount_in=9546083437)
  - **victim_sell** cp=285820125 `uwXhWd8yJMyh8uwyjhaGuRKzGZ4Wb64aRSeFTZjvs7c` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=3, ev_idx=7, amount_in=47118458145413)

### Pair 64

- **pool** `0x76b7709caa7d74649d9bf1abb5f38ef452564b61496c1876486e6f500abb6b5b` (+102 checkpoints)
  - **bot_buy** cp=285820195 `2Wvn185bjEDHZTBxT3xNdpcA3fEAWR9g5ZAriVT5Kfvh` (tx_idx=7, ev_idx=0, amount_in=60742810061)
  - **victim_sell** cp=285820297 `C9cvqFwM3aSFkzzPy9TY7M2Ufry2NWiYwMUi9GFxyKUQ` (sender=`0x51b00f889064537ec4d26299a6068fc8e92c32c71ebcbe23350232c48f0433b8`, tx_idx=23, ev_idx=0, amount_in=351813797619047)

### Pair 65

- **pool** `0x76b7709caa7d74649d9bf1abb5f38ef452564b61496c1876486e6f500abb6b5b` (+90 checkpoints)
  - **bot_buy** cp=285820744 `MsjnY79NouyPJ1JQvbt6syEV4YS8uRxdZCN99rHBQqK` (tx_idx=11, ev_idx=0, amount_in=60765942282)
  - **victim_sell** cp=285820834 `3XfiWu8uoEyLmH1eaMDVPcimiyY3spMeqnZES4iuurVa` (sender=`0xa90b0999a8ed523c2597d45a1390ea1161c658c4fd8a32db45ea26b41026ff5e`, tx_idx=10, ev_idx=0, amount_in=18762038212769)

### Pair 66

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=285821263 `3KXUNbyJ45VPZvC8zKmaNSBTCtqEvAHeuoRuYzTo7nom` (tx_idx=10, ev_idx=0, amount_in=9065752588)
  - **victim_sell** cp=285821266 `33CRK46bnDrkFMRTtXZ98yhL4xcLY8T7WQAybpz9trVw` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=15, ev_idx=7, amount_in=14465863839183)

### Pair 67

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+4 checkpoints)
  - **bot_buy** cp=285821869 `D939MjTUMhmVQn5K9gn16bTrL8W5aXRN7hBzsjTSUdyF` (tx_idx=4, ev_idx=0, amount_in=138618113772)
  - **victim_sell** cp=285821873 `J8LTphiY9hnfeVMQDjaZtN6Q4Zk1mktFGdj1vFShnjei` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=1, ev_idx=1, amount_in=17481851831)

### Pair 68

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+3 checkpoints)
  - **bot_buy** cp=285822012 `Hj3A4KbHC23TCCGnf9h8o5jW9hCVHBxucRdj42C2xJvg` (tx_idx=13, ev_idx=0, amount_in=138479553812)
  - **victim_sell** cp=285822015 `5DjHYUAxfGXgmgjGCbX7xQBsNDiiPKL7ZY1sKyXzBPiF` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=20, ev_idx=1, amount_in=10504789314)

### Pair 69

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+10 checkpoints)
  - **bot_buy** cp=285825766 `EjkrofAbfNMNrym1QGmXT11HWY9hAD8a9vk5iZTgkVQR` (tx_idx=20, ev_idx=0, amount_in=138615199745)
  - **victim_sell** cp=285825776 `DUFLK5S58EjUg2ECBXDyKz44rhWgarHQERP2S9rAdkDz` (sender=`0x609b7f187082ccc9a1af38d060dc85cadf76be9fa11e1cc52ac4964a43a377bb`, tx_idx=6, ev_idx=4, amount_in=293798600)

### Pair 70

- **pool** `0xba2e118975948c935ed73b66219970604a665ff5a19795f536ce4b9d9f5956d4` (+1734 checkpoints)
  - **bot_buy** cp=285827276 `6WRVW93XDv7MEcR9yGNYfbdm3rUpadRKr5xSWvacHkcm` (tx_idx=13, ev_idx=0, amount_in=1644033849)
  - **victim_sell** cp=285829010 `4vrL2mR6tNgHmxbdanVyW8jw75ua8UT2cVA8hm1bzEHR` (sender=`0x6eb263bcd8266ad5a5fdb64b9ab65f5f7114cf5d5f634989f6e032cd9b97c32a`, tx_idx=1, ev_idx=0, amount_in=9701353085)

### Pair 71

- **pool** `0x03d135b439d55511a6d7d98fafe5b92093f78b14c522d9d4a8cb004df5aead4f` (+1277 checkpoints)
  - **bot_buy** cp=285833171 `CqaWPhJg2MyKnhQPUBxqTZGSFPwsMN9M225PUjckd4kH` (tx_idx=5, ev_idx=0, amount_in=32099933646)
  - **victim_sell** cp=285834448 `QN9iiFMc22ziSr2vWaMNRNQ434bjyvcP3umSzuQkB3m` (sender=`0x7ba845f41f4fc8d0ad9f637887587b9f4b6e1e0e2f087e8db3bd16cd92fde971`, tx_idx=1, ev_idx=0, amount_in=132406011138)

### Pair 72

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=285837299 `A3yxVXut8Jcm2fQTCAjSdQhH7TmzhUyWkVvQKkDfMeEK` (tx_idx=16, ev_idx=0, amount_in=9191068763)
  - **victim_sell** cp=285837302 `D679dX8yd63xRTZP3xBydWRL6sNgfFsB7iKLNPmiRPBG` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=9, ev_idx=7, amount_in=6179125974833)

### Pair 73

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+22 checkpoints)
  - **bot_buy** cp=285839582 `9MPxUiV1eoJuzDDHqVbxauASaePvL4zBHZLc9YcJ5Ten` (tx_idx=34, ev_idx=0, amount_in=1873182586)
  - **victim_sell** cp=285839604 `AbBHYqthy2Wbwz6sp6CFKyz7WWf5qRZcEsoaLqgxgqrG` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=21, ev_idx=1, amount_in=15248345448027)

### Pair 74

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=285839601 `52aatMCmFZpuHwMnuZ1kRP6xCdCyANPJENbNmkNTBt5L` (tx_idx=25, ev_idx=0, amount_in=9302452845)
  - **victim_sell** cp=285839604 `AbBHYqthy2Wbwz6sp6CFKyz7WWf5qRZcEsoaLqgxgqrG` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=21, ev_idx=1, amount_in=15248345448027)

### Pair 75

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=285839636 `5RDBffEZsj3g2jFw3j7j7LGvV1b9Y5FPYdPBA6iiNZw5` (tx_idx=1, ev_idx=0, amount_in=9448875299)
  - **victim_sell** cp=285839639 `7eXLnJKfCdXAKCixnxrsevsV2e7hTpDPe2ybDT3zziEx` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=3, ev_idx=7, amount_in=33865745349098)

### Pair 76

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+430 checkpoints)
  - **bot_buy** cp=285839673 `3oac3VPRb89nWMf5zr4vXaxTooN3zysmfNFQfo8XYqHj` (tx_idx=9, ev_idx=0, amount_in=9426927867)
  - **victim_sell** cp=285840103 `6qpuszdNzHSytHDc11A3MECXsttnCqQbXM6KVv5eU8nb` (sender=`0xf4fed53ed5258ba318c5dcb3ff9054b1238e0aa61453410149a6cd39f3ca0755`, tx_idx=5, ev_idx=8, amount_in=3811180665519)

### Pair 77

- **pool** `0xba2e118975948c935ed73b66219970604a665ff5a19795f536ce4b9d9f5956d4` (+336 checkpoints)
  - **bot_buy** cp=285842649 `3bCK9FcsuPEW8SDuKKJ36gJvrx4oGYEKo883ZqhKTnBe` (tx_idx=17, ev_idx=0, amount_in=1644814182)
  - **victim_sell** cp=285842985 `adFgMddpT5iLshoFsvyW5TQZyd7LdNgrwPy4JMSCwBX` (sender=`0x6eb263bcd8266ad5a5fdb64b9ab65f5f7114cf5d5f634989f6e032cd9b97c32a`, tx_idx=4, ev_idx=0, amount_in=6000000000)

### Pair 78

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+4 checkpoints)
  - **bot_buy** cp=285843632 `Hs3rW8ZSTy6d3GWwvbGKfG8WJQCxxsqT6eRbEYtfMKLy` (tx_idx=25, ev_idx=0, amount_in=19304451629)
  - **victim_sell** cp=285843636 `GnetWLqLBF9E86vjzuLyM6Hsaa5a8USQqERR39RvWfWq` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=3, ev_idx=7, amount_in=57737742507487)

### Pair 79

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=285848475 `9Dr38gT4WEbm9aySkxkZo9dKmQ1mDQnY2S4WfyTf8Z3u` (tx_idx=8, ev_idx=0, amount_in=9708100977)
  - **victim_sell** cp=285848478 `CcUDqQCmNtSpXjUbLfUrijxzZXjCbV9BuCFRBcooi2Qm` (sender=`0x89a1c807393670de16b055f0316232a5627b94bf74dfaa7ac34d3124109acf19`, tx_idx=15, ev_idx=3, amount_in=1754071739620)

### Pair 80

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=285848735 `3WucrFRpQ5Av4H4Svv7f7F3nXK1CWF8M69AvzWnjBq7k` (tx_idx=10, ev_idx=0, amount_in=9754213516)
  - **victim_sell** cp=285848738 `87VNnTmme5TS8rL59AeK7GftH57kduuUkrXS8Lr38KKq` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=18, ev_idx=7, amount_in=5918511434888)

### Pair 81

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+568 checkpoints)
  - **bot_buy** cp=285851843 `5K3KviLggisPukBB1yk6u3bLdsiwJRhmod6M8WnPQnuG` (tx_idx=9, ev_idx=0, amount_in=138905671712)
  - **victim_sell** cp=285852411 `2RVuXfWTJ9cTsLvX4DTBuS2iT5X9Fa3beQ2dsmvVc4CG` (sender=`0x52d1fc6f2e98c7d3e1e41afdcbc95745db051bff6269073bb2ecc1bc5f2c91f5`, tx_idx=3, ev_idx=9, amount_in=227897885)

### Pair 82

- **pool** `0x2d3230025b4615087656952bf5ddb49d7a9b6712ac9aa14977a877f02a16f165` (+2 checkpoints)
  - **bot_buy** cp=285853160 `C4yQMFKJBRujL2zwHCpFzYu832ddEqeA8BDM4cFEJUMz` (tx_idx=6, ev_idx=0, amount_in=4271090510)
  - **victim_sell** cp=285853162 `ucTxtPm4pkRHxkLDjtcBG56mYYv823PG2Tnyj824rUY` (sender=`0x00006f748f809057fd1ca9ff8d02d89947f9079c26029ea2348657d8467b0000`, tx_idx=10, ev_idx=2, amount_in=151445158687)

### Pair 83

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=285856171 `48S8UNvqNpwdKs75dZSvRAjHHbRhYLwVN8hqwefQrSV2` (tx_idx=1, ev_idx=0, amount_in=21593247073)
  - **victim_sell** cp=285856174 `DoUab7WHQV2qwY4oHY1vfQNhtxkPmwrQsPqjqGgk6gW1` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=2, ev_idx=7, amount_in=139844913950844)

### Pair 84

- **pool** `0x4edb54baafdf02a2219b6c327c9acacd79280b9c98cdc1b2b23230de906fa421` (+62 checkpoints)
  - **bot_buy** cp=285857124 `C5cNMhbyuk6P6mUZkoFyfpMQCZtRkLrA5AVAa74u5VAN` (tx_idx=4, ev_idx=0, amount_in=23490457057)
  - **victim_sell** cp=285857186 `CBBhwsdfeyS7G29J5G467Sd1Js5cXBPdbMdCDxcU5m3r` (sender=`0x9295f76114012e214e5c75c25700a813edf5ee9c9d5e321d4e473f7d2c3d3403`, tx_idx=6, ev_idx=0, amount_in=1184332681197)

### Pair 85

- **pool** `0xf9107158e4945d6bbc321c7471e0b7c9854c2d3a1b04aaff6acaa50b8ea203d2` (+6373 checkpoints)
  - **bot_buy** cp=285868465 `9vV9fHRVySZQheuKNHmgv2PneYNFS9DnHN56Y59jbzu2` (tx_idx=5, ev_idx=0, amount_in=1072023430)
  - **victim_sell** cp=285874838 `4mnafnG1n69op3wHFxJkHeB825j6XgMvyGSZBFJQcwcY` (sender=`0xabf6e308865254d130ef8b3ba6b8e30396649da726e2b6e3744517923364b82e`, tx_idx=1, ev_idx=0, amount_in=230728902031721)

### Pair 86

- **pool** `0x4edb54baafdf02a2219b6c327c9acacd79280b9c98cdc1b2b23230de906fa421` (+67 checkpoints)
  - **bot_buy** cp=285868773 `8JbMF6cSZe7zFebDgXsXWstZbzLNNN2c9Sh897LMA6Y2` (tx_idx=11, ev_idx=0, amount_in=23368568688)
  - **victim_sell** cp=285868840 `C7Xsxz9GSpht8tLKqxZ7V8LHimJo351CdeWMzG1nNHmS` (sender=`0x9295f76114012e214e5c75c25700a813edf5ee9c9d5e321d4e473f7d2c3d3403`, tx_idx=1, ev_idx=0, amount_in=1286396034574)

### Pair 87

- **pool** `0x29333c096043846d42356e250647f113a7a99d0470bcd584f98e9edeab69100e` (+6 checkpoints)
  - **bot_buy** cp=285869836 `2n1LoiwQdMcxvNbNBEvdZg5XVTnL4Rt8YUEgp5myYoYY` (tx_idx=1, ev_idx=0, amount_in=138917213684)
  - **victim_sell** cp=285869842 `3t3vMT5ngvfhjAYdwVi7fqr7texaG7afdxjzxpwV3HfS` (sender=`0x98391d8f43736152b61caaa38b615621095d7595f34ae6de18d507996da8748e`, tx_idx=29, ev_idx=1, amount_in=46083086927)

### Pair 88

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (same checkpoint)
  - **bot_buy** cp=285871342 `31RFpKzWLAVSCv33uxfCtNJSSEeQz76FqeWDB5cyA7bs` (tx_idx=13, ev_idx=0, amount_in=112257333382)
  - **victim_sell** cp=285871342 `7WahFQnVpsDxUaJaSzYFekhAevoEBWh4FM8Bq8mWeukr` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=14, ev_idx=7, amount_in=26435147599278)

### Pair 89

- **pool** `0xfc6a11998f1acf1dd55acb58acd7716564049cfd5fd95e754b0b4fe9444f4c9d` (+2 checkpoints)
  - **bot_buy** cp=285873702 `w57ZWavU3j37a5Jpxi8nczqKesvFWRrQoDUc5uRkKck` (tx_idx=5, ev_idx=0, amount_in=11279134157)
  - **victim_sell** cp=285873704 `CsuxzM8icifN2B3PHN1x6Qe1oLa33uShXRq6Q6a66JGy` (sender=`0x00006f748f809057fd1ca9ff8d02d89947f9079c26029ea2348657d8467b0000`, tx_idx=12, ev_idx=2, amount_in=2627328711877)

### Pair 90

- **pool** `0x4edb54baafdf02a2219b6c327c9acacd79280b9c98cdc1b2b23230de906fa421` (+73 checkpoints)
  - **bot_buy** cp=285878942 `8DqY5rXK1rDKbe6jEkf6Gs56rBKenqe2SxGCvqVLcJ7` (tx_idx=5, ev_idx=0, amount_in=11679560557)
  - **victim_sell** cp=285879015 `DzvzuaHQKmnHBLb1cdkq2fy77sTpfNvpBdoB6Vqootw` (sender=`0x9295f76114012e214e5c75c25700a813edf5ee9c9d5e321d4e473f7d2c3d3403`, tx_idx=1, ev_idx=0, amount_in=1224769662097)

### Pair 91

- **pool** `0xfc6a11998f1acf1dd55acb58acd7716564049cfd5fd95e754b0b4fe9444f4c9d` (+3 checkpoints)
  - **bot_buy** cp=285883866 `BQawRFy32jQWWffjgiphkyQRpabQzKdL1teau382wNJv` (tx_idx=6, ev_idx=0, amount_in=11208196024)
  - **victim_sell** cp=285883869 `Br3mkYNmdvy6GMikFJ6CspcsfrvpxkJUtYeq2hyRM7AV` (sender=`0x89a1c807393670de16b055f0316232a5627b94bf74dfaa7ac34d3124109acf19`, tx_idx=20, ev_idx=2, amount_in=44566720048427)

### Pair 92

- **pool** `0x31063a1f5775edae48d137f96a30c500733a54f64646e97f71561ba3abe49bd6` (+2 checkpoints)
  - **bot_buy** cp=285889276 `3JrTMo35wQtDs99oq8sqcDE7dNoqe4AT4uvA1DSnWXbm` (tx_idx=9, ev_idx=0, amount_in=20595190593)
  - **victim_sell** cp=285889278 `CaZ3M6ahsW9JUZ29ShX1M4FMFynWF9itMRGG7Ec1bTho` (sender=`0xa6c8aa8ddcc3ffebb59c0a38d8019466d6e5b7bf51c230e7b76c853082c499b2`, tx_idx=7, ev_idx=2, amount_in=7999918)

### Pair 93

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+3 checkpoints)
  - **bot_buy** cp=285898275 `AVecJrJ73cYiBBCb1MDZHWmmdupYxFwpadmn9JUXUhVY` (tx_idx=2, ev_idx=0, amount_in=130257013786)
  - **victim_sell** cp=285898278 `4qtrShexC2THNCuhsMjMLhU7tfqqvcgSKE3bxQmMSJ2m` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=2, ev_idx=12, amount_in=18839000000)

### Pair 94

- **pool** `0xf9107158e4945d6bbc321c7471e0b7c9854c2d3a1b04aaff6acaa50b8ea203d2` (+1841 checkpoints)
  - **bot_buy** cp=285898638 `6kcbHck36F8G9vGoCJ94Pr8v3v2LXwqdop8eC3iUDQNX` (tx_idx=5, ev_idx=0, amount_in=1124472772)
  - **victim_sell** cp=285900479 `7ov7suLBB2iazssJAFSwhqb2mtyExstfZtBLjJ2zQbyH` (sender=`0x8cc479ea2741742a574565eed9dc5b43dca3fd576a5c376770ff93f134bee918`, tx_idx=9, ev_idx=21, amount_in=12077506164)

### Pair 95

- **pool** `0x4edb54baafdf02a2219b6c327c9acacd79280b9c98cdc1b2b23230de906fa421` (+68 checkpoints)
  - **bot_buy** cp=285902737 `G7gLUYs7EBgUDGerdd9ZcGuj4TsqUhvTA124Bvc2gSVS` (tx_idx=3, ev_idx=0, amount_in=23377119370)
  - **victim_sell** cp=285902805 `3Agz9WsyXVCeV86nAZiGWAhsLc9894RPLXHnkUUds2X4` (sender=`0xd7c6cc85a7794c1db4ee3186804b15810c949202ea95c7b8e681ac118cd3ed90`, tx_idx=1, ev_idx=0, amount_in=1173020303178)

### Pair 96

- **pool** `0xd6918afa64d432b84b48088d165b0dda0b7459463a7d66365f7ff890cae22d2d` (+1313 checkpoints)
  - **bot_buy** cp=285904402 `647MmqRCa2937BgEjuvGPBv2tUjdkGejF6bi3K21diFA` (tx_idx=1, ev_idx=0, amount_in=33373442772)
  - **victim_sell** cp=285905715 `Gtv6vD3Hv5j355AcjuU26EmCmaXPa2aMMVsbWDvvLXgK` (sender=`0x1f4eadfcaa9828e2e216e4b0111ed292d68716a8cf819784c603404aa903b6ed`, tx_idx=4, ev_idx=0, amount_in=258764386983)

### Pair 97

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+5 checkpoints)
  - **bot_buy** cp=285907809 `AmZXds5keYhmGqU1HrXghWNouydL7N6Vc6h36jze8mdR` (tx_idx=15, ev_idx=0, amount_in=130321524856)
  - **victim_sell** cp=285907814 `DiLB1nMHYLRecM7ved9H2tKbjU6M9HYxPR5BQXwtoquG` (sender=`0xccf42fb2378a85ee46cf4de8fd838d6fe2113a623c0d8a61bdfd8826e26dc9fa`, tx_idx=8, ev_idx=5, amount_in=100078823)

### Pair 98

- **pool** `0x29333c096043846d42356e250647f113a7a99d0470bcd584f98e9edeab69100e` (+2763 checkpoints)
  - **bot_buy** cp=285914914 `DowELvJxrZ4yDTwDRPmz6eV63HeKizaCszdLFa4RAMz3` (tx_idx=3, ev_idx=0, amount_in=130236590908)
  - **victim_sell** cp=285917677 `FDNJqdmJ9RyZZE7ZW4A761SUjv21NM3byBnk6avdLJN2` (sender=`0x40b4c999c31b31ee327f830b0696ef4c402a557cf65750fd07c7dd03640c34f9`, tx_idx=1, ev_idx=0, amount_in=115805612944)

### Pair 99

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+57 checkpoints)
  - **bot_buy** cp=285947534 `GWc7FypT1BcfVcGj7HCy4PRgV7gWx9MjXNZJw9DeKtCH` (tx_idx=1, ev_idx=0, amount_in=130445658621)
  - **victim_sell** cp=285947591 `uvkAxkr5JKYShzi9X2bXst1hoQ78QUZbiCbttUsShY9` (sender=`0x1eb59ebed1febea954bdf8b1a17f4ea388a326f32ad3a4ae357015216092e834`, tx_idx=6, ev_idx=0, amount_in=3030000000000)

### Pair 100

- **pool** `0x68d16416770f9b73b0b1b45e118f6ea3a2910f548f942fe335824fd515cdff08` (+12566 checkpoints)
  - **bot_buy** cp=285952545 `H6zGCWyotnpezg2xt84dfSzPgoWeKFjSA35auP2LBTJ9` (tx_idx=3, ev_idx=0, amount_in=49231739893)
  - **victim_sell** cp=285965111 `H4hxJ5hEAUjw94ghWjxT39Brn6VYCvK9yz9WJuCMc4Wi` (sender=`0xc70f3cb7b384def8dbc53baac3c991de20c138d70503438ad20070e40e79acd8`, tx_idx=8, ev_idx=1, amount_in=1705533057823)

### Pair 101

- **pool** `0x8049d009116269ac04ee14206b7afd8b64b5801279f85401ee4b39779f809134` (+2 checkpoints)
  - **bot_buy** cp=285960105 `2sxyRrL83oJ4ptrtzMrwnXXnW2Pc6faZuUfoKfv6RmAB` (tx_idx=8, ev_idx=0, amount_in=254302058)
  - **victim_sell** cp=285960107 `DCUBbeWFkuKZthNHYbKaF4Jp57oxXC5kYmdjtsiNySGg` (sender=`0xea54891662533a597e11ac2d26ee391e1057e35b18e24261a5221fab77f54591`, tx_idx=20, ev_idx=2, amount_in=356528977576)

### Pair 102

- **pool** `0x76cab5e864ddce96a2db3e51b362b402388126f4379239dcd0b0e463d9ed6aeb` (+3 checkpoints)
  - **bot_buy** cp=285962025 `3AkbBKYYTKUQJYRe2asg2JGTEYPw3dEfFYmw4WK55env` (tx_idx=1, ev_idx=0, amount_in=2261057748)
  - **victim_sell** cp=285962028 `6FJE131CtrEGJVNQGwoSEpcrMsNXTJNkRGGToYiGPEGF` (sender=`0xd5033d5f4b7e48c326db1c8c664d6103db1341f815016684da1b4687a05c098c`, tx_idx=15, ev_idx=2, amount_in=16977107568151)

### Pair 103

- **pool** `0xa809b51ec650e4ae45224107e62787be5e58f9caf8d3f74542f8edd73dc37a50` (+6 checkpoints)
  - **bot_buy** cp=285970797 `8kPHRAaTCw4idojjeSf3BqAmhAwFsyn8UajvLkF2FDDZ` (tx_idx=5, ev_idx=0, amount_in=1672561286)
  - **victim_sell** cp=285970803 `DSpDf9V52L3nuTs9h13szfGN72D6sNQU2RzPYobAGGRd` (sender=`0x609b7f187082ccc9a1af38d060dc85cadf76be9fa11e1cc52ac4964a43a377bb`, tx_idx=16, ev_idx=2, amount_in=12239379495146)

### Pair 104

- **pool** `0x4edb54baafdf02a2219b6c327c9acacd79280b9c98cdc1b2b23230de906fa421` (+64 checkpoints)
  - **bot_buy** cp=285975848 `Ghp8dFQ4dn8BD8AdpLmKKm19U8FYFoTXesf8Actn7sJH` (tx_idx=1, ev_idx=0, amount_in=23537822608)
  - **victim_sell** cp=285975912 `2FTKzAAKYynB6XfnMTP62j6QYjGGwG57R6VXBXNeEbGc` (sender=`0x5b74ac99f7e9aa9a1ab141db0ce55a94fc3fef2a9abe017c1bac3d48ef5f8f1c`, tx_idx=1, ev_idx=0, amount_in=121444992473)

### Pair 105

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+3 checkpoints)
  - **bot_buy** cp=285977823 `9qDgK82kNi1txabG5R46ApC44Tda5urmEKeuMyGZiS13` (tx_idx=23, ev_idx=0, amount_in=124583824779)
  - **victim_sell** cp=285977826 `EZ5hoShDu85xPrYyu7doQjEhTgp1CGyyZyA2gjfc6fSt` (sender=`0x5f60ad4dc8117ba3d6f71a19a52dfd85d0691be1e3915ca6054d1d7e35109aa5`, tx_idx=35, ev_idx=2, amount_in=88823579141)

### Pair 106

- **pool** `0x0254747f5ca059a1972cd7f6016485d51392a3fde608107b93bbaebea550f703` (+3 checkpoints)
  - **bot_buy** cp=285989141 `5MzJPJ1P4jhv91B9PTzU3E6bPMYMyBfcMTyVJa776nER` (tx_idx=11, ev_idx=0, amount_in=124655130831)
  - **victim_sell** cp=285989144 `HuoEckUQXGdUfLeC3JzaVxXxxabNpmNMvDXYs1ddmWjb` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=20, ev_idx=2, amount_in=2167973126667)

### Pair 107

- **pool** `0x51e883ba7c0b566a26cbc8a94cd33eb0abd418a77cc1e60ad22fd9b1f29cd2ab` (+14 checkpoints)
  - **bot_buy** cp=285996696 `Eqp4d4uD5JSvHm5oPbVBSAAFVyGsSR8NM2eYDsm6trS5` (tx_idx=11, ev_idx=0, amount_in=124033360603)
  - **victim_sell** cp=285996710 `B5YHkzkkXeoYhx3XTMbYPPzMd7nm5BrecqT89kX1hSN5` (sender=`0x3a60dbe51d6e448968e2e3d6fd8f902c2657cca50cb201eb0d5576bdff24f2ee`, tx_idx=5, ev_idx=2, amount_in=104708799)

### Pair 108

- **pool** `0xb785e6eed355c1f8367c06d2b0cb9303ab167f8359a129bb003891ee54c6fce0` (+3 checkpoints)
  - **bot_buy** cp=286002203 `EehKecvjayE1tJwg3pdogzxn2LxAC9aBY4aEcqgHXKsz` (tx_idx=4, ev_idx=0, amount_in=28053425235)
  - **victim_sell** cp=286002206 `FBg81kZh8qA23c8ZTcRCLaJ8G642RtACPqbavm7hAzkc` (sender=`0xd5033d5f4b7e48c326db1c8c664d6103db1341f815016684da1b4687a05c098c`, tx_idx=17, ev_idx=2, amount_in=121271136374254)

### Pair 109

- **pool** `0x4edb54baafdf02a2219b6c327c9acacd79280b9c98cdc1b2b23230de906fa421` (+66 checkpoints)
  - **bot_buy** cp=286004778 `HLst7giaRP4AiVNFVWCYKHPBnDoeguNdYDFMGWAxv6Xa` (tx_idx=10, ev_idx=0, amount_in=23651622603)
  - **victim_sell** cp=286004844 `57oqQUeWNdNoLHnusCdvDbzkjaAJxuP1siTU8rxSMbyu` (sender=`0x283adcb1b65b85c3cf856b7ef2b3fbfd328377cd125c3177c729990b0cdbd701`, tx_idx=2, ev_idx=0, amount_in=2010443645254)

### Pair 110

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+3 checkpoints)
  - **bot_buy** cp=286030544 `HZf1dzF7hr6QkdooMmsgNHSphB798ti8RMjNaC3C7ToZ` (tx_idx=11, ev_idx=0, amount_in=124205512925)
  - **victim_sell** cp=286030547 `RvrtsAxhQsm9x43meTT5sGB1pWgEMokRw7t9DBZuHFY` (sender=`0x7e17ed25424ca1e811d2627583000734c57636362382d8ca6e43f7b203171bbd`, tx_idx=1, ev_idx=18, amount_in=7339900000000)

### Pair 111

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+4 checkpoints)
  - **bot_buy** cp=286034918 `6w5oVE8fJW2mqTcamNE2baTtT6Vnt5kFFNtunwC1PyTi` (tx_idx=24, ev_idx=0, amount_in=9711518868)
  - **victim_sell** cp=286034922 `Dt7MX1bEJUeB6nJpZvNTXRNj7UZQhgKRJxYBq5JQWhAb` (sender=`0x00000cfa6d94cff09b37a1a4dcfc92a993b61fffb71ce95ad9949e2f4cfdad26`, tx_idx=2, ev_idx=7, amount_in=4289586968553)

### Pair 112

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286039163 `29yFhQZroHfKKQxpiKH15aMuSsGiSdbtX1sSCjMgwRui` (tx_idx=1, ev_idx=0, amount_in=9771734336)
  - **victim_sell** cp=286039166 `7CBFUKCGdsj3Zx3q9QzV143T8DpA11fyyZS9yCif8V8a` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=4, ev_idx=7, amount_in=5930328889326)

### Pair 113

- **pool** `0xe01243f37f712ef87e556afb9b1d03d0fae13f96d324ec912daffc339dfdcbd2` (+82 checkpoints)
  - **bot_buy** cp=286047127 `G7mcus7gatNEVBA9rwCtygyJbQbMYKHfttV6NERvwe6n` (tx_idx=10, ev_idx=0, amount_in=124751347169)
  - **victim_sell** cp=286047209 `5vBoWBDyyypSmLuVoFDiQBruuGUJ6BpTqMbncPrD4ywb` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=27, ev_idx=15, amount_in=588651000000)

### Pair 114

- **pool** `0x9620c1c5e8d0a26ff358c25bbbb6d59b2aeb88807c9e5bf60af1fcf7aaafd775` (+34239 checkpoints)
  - **bot_buy** cp=286053717 `DLtT7k5guYmcpxvYqQZdLNCeN7M2hFxVADrnbb6U6sST` (tx_idx=8, ev_idx=0, amount_in=7323633136)
  - **victim_sell** cp=286087956 `2nN9JQeN1onEswopEm2Sb5Fa3nNHBmqKE98nXivkShvx` (sender=`0x5391c49b612eb44b3350946ecb82a8e359c60f7f67eece5f6cc42c8d3a61ca1d`, tx_idx=9, ev_idx=1, amount_in=914561107)

### Pair 115

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+9 checkpoints)
  - **bot_buy** cp=286053814 `7QLUFETFmN1AKnKAPRyctm1MJy4NUm5NRngiQ2zNThbF` (tx_idx=5, ev_idx=0, amount_in=10041707706)
  - **victim_sell** cp=286053823 `BSzSbCdymBTSiPZwMF4NkKS4zM8J4x7u4UPBUohcEcoH` (sender=`0xf4fed53ed5258ba318c5dcb3ff9054b1238e0aa61453410149a6cd39f3ca0755`, tx_idx=16, ev_idx=8, amount_in=3005601385126)

### Pair 116

- **pool** `0x9661cca01a5b9b3536883568fa967a2943e237de11a97976795f5adb293892e9` (+3 checkpoints)
  - **bot_buy** cp=286056859 `6fKiHW1hF8z4BLqGiX4imza1tyX7PnafEgKvK1RDWLUR` (tx_idx=2, ev_idx=0, amount_in=12349604034)
  - **victim_sell** cp=286056862 `FgiBDThnPSLmSSFSttTjJN6E52edncndAKdaHEoz7etY` (sender=`0x4144ed30f708bc9291c59c9f6490cf09aa995b51fdc877e59b9674431051f795`, tx_idx=14, ev_idx=1, amount_in=540747366576)

### Pair 117

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286057611 `EumZTR1siWC8skLZjQ3aR7kfNELZfE1QHwAq4Z6R5AD2` (tx_idx=5, ev_idx=0, amount_in=10148956289)
  - **victim_sell** cp=286057614 `ACcuihABrK3F3eCwdK9zmctVCo3MgRyHQJft69UesxiV` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=4, ev_idx=7, amount_in=6616152778598)

### Pair 118

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+4 checkpoints)
  - **bot_buy** cp=286063994 `G1vW2Bp7L8SUcNqqnHg2dLNNncpgcah2jdxYhi3Z6BAa` (tx_idx=3, ev_idx=0, amount_in=125655488427)
  - **victim_sell** cp=286063998 `8XXfWGph1tEQsrgR3R7ToryWkWPJyQn3cfFXNfSreKfg` (sender=`0x1eb59ebed1febea954bdf8b1a17f4ea388a326f32ad3a4ae357015216092e834`, tx_idx=13, ev_idx=0, amount_in=5657417784620)

### Pair 119

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+3 checkpoints)
  - **bot_buy** cp=286064920 `GdmtYatQw8nKJX5x3uWc9eXfBrksfUJ4EfjN85aMLZED` (tx_idx=8, ev_idx=0, amount_in=126027191526)
  - **victim_sell** cp=286064923 `DsLzZC4skgKT9LMEb9YbF7dD6jYSVahYU1GsAZGgTC5t` (sender=`0x89a1c807393670de16b055f0316232a5627b94bf74dfaa7ac34d3124109acf19`, tx_idx=13, ev_idx=4, amount_in=411932339399)

### Pair 120

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+3 checkpoints)
  - **bot_buy** cp=286065546 `BfPNhEXwrSh8GiFC5DePxef1aCxhQNzcfLLfXDSNxjyL` (tx_idx=4, ev_idx=0, amount_in=126109011817)
  - **victim_sell** cp=286065549 `BuCjtpC2YcUuZiaD6EEZehCYL7JdvTNbqpzzB9MRVptk` (sender=`0x1eb59ebed1febea954bdf8b1a17f4ea388a326f32ad3a4ae357015216092e834`, tx_idx=4, ev_idx=0, amount_in=4049291336237)

### Pair 121

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+2 checkpoints)
  - **bot_buy** cp=286065767 `FBxpUumJA11dGqJb3NGyXR55DLqDDY69dTfvKbapmx6e` (tx_idx=14, ev_idx=0, amount_in=126135432196)
  - **victim_sell** cp=286065769 `BtifeATsTaWxsrXtP2oAQ9X4ykv2Z8KCigfvfUgnTiid` (sender=`0x1eb59ebed1febea954bdf8b1a17f4ea388a326f32ad3a4ae357015216092e834`, tx_idx=13, ev_idx=0, amount_in=5176031944127)

### Pair 122

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+3 checkpoints)
  - **bot_buy** cp=286070902 `FK9NSMqAKrFcj5Y1yNG7B2msFcxABuJSU6ihkXuEuJch` (tx_idx=13, ev_idx=0, amount_in=125851975934)
  - **victim_sell** cp=286070905 `6LTNHmVJzSVY5aVJB4zXS6h3yVa7JQ1Zy7b4yf6oLU2w` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=23, ev_idx=1, amount_in=5348148402301)

### Pair 123

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+9 checkpoints)
  - **bot_buy** cp=286071517 `2Rq75j4VfmjH3L8UJCM2FSvpS4V1KRfXmKML3wg3kser` (tx_idx=14, ev_idx=0, amount_in=125946522133)
  - **victim_sell** cp=286071526 `EYDo6BN2NzMWmfoP8N4DNtHCzbnXwfCm39yrvrCuSLkB` (sender=`0x037ff921b39f0ebf9a732a40f7bf9dbe09e6e53292456f7ca27c29a1fbf226db`, tx_idx=11, ev_idx=7, amount_in=45723056049)

### Pair 124

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+3 checkpoints)
  - **bot_buy** cp=286072142 `4uxtVLWBH6rTX8tgXagxcuP5KdiTxBtpgf18jKwAWiCp` (tx_idx=3, ev_idx=0, amount_in=126175808442)
  - **victim_sell** cp=286072145 `3jvgbS7UM3hcccaD4pBqjQPWaTWFDyDxpxZsiJFHZLYp` (sender=`0x1eb59ebed1febea954bdf8b1a17f4ea388a326f32ad3a4ae357015216092e834`, tx_idx=1, ev_idx=0, amount_in=6380760605184)

### Pair 125

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+2 checkpoints)
  - **bot_buy** cp=286072249 `HzFcDKkfcvVmUd1ks772YUEbxfHhxw4Xchz9VGbhJ6Nd` (tx_idx=4, ev_idx=0, amount_in=126175808442)
  - **victim_sell** cp=286072251 `B5FAG2x7XbDsmUZ96nQGvjZVJ7b5WKp14w7AXEYDJmdp` (sender=`0x1eb59ebed1febea954bdf8b1a17f4ea388a326f32ad3a4ae357015216092e834`, tx_idx=6, ev_idx=0, amount_in=11051125889120)

### Pair 126

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+3 checkpoints)
  - **bot_buy** cp=286072290 `9WicXtytE99JZT3wMEyG9GCNejbS6Wqx8zXegfPtcs7k` (tx_idx=7, ev_idx=0, amount_in=126175808442)
  - **victim_sell** cp=286072293 `6GH3HHAFdGVd3bYEvZXiEL7xMV6hfG76wuLH1m7gGGpB` (sender=`0x1eb59ebed1febea954bdf8b1a17f4ea388a326f32ad3a4ae357015216092e834`, tx_idx=1, ev_idx=0, amount_in=5364790578489)

### Pair 127

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286073206 `5tJ9yV7qiiGqA3UVTrSfSvJLz1sqdY6BKrhFX6fB7tD1` (tx_idx=7, ev_idx=0, amount_in=21040343363)
  - **victim_sell** cp=286073209 `3mcKWaX7fGmmEVWkC2tU9LHFfxKrPhhHi1yYg9GYcU8q` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=23, ev_idx=7, amount_in=19287343498375)

### Pair 128

- **pool** `0x3b982ac4be6f654c2e11ce2d70639730a0c10a97abddfb362a99eaf181837ad0` (+3 checkpoints)
  - **bot_buy** cp=286074845 `5QDTk6TjTNZYaxzMJVkN3sW95GXL5GDEscBtSn9UrnDf` (tx_idx=16, ev_idx=0, amount_in=217690816)
  - **victim_sell** cp=286074848 `EM7fc3qWKKVwuMhYpWMTGwAPaPSgGvur9MYkPKQqFGGJ` (sender=`0xa8a6670d32e66762b8ee6d66f57aa847f718551099752a87cfa4ee7058e9b392`, tx_idx=1, ev_idx=1, amount_in=4803633982222)

### Pair 129

- **pool** `0x3b982ac4be6f654c2e11ce2d70639730a0c10a97abddfb362a99eaf181837ad0` (+2 checkpoints)
  - **bot_buy** cp=286076320 `DQzzYfxanVGfNxVkkaQEwnWKmJTeGTbt2EzzThpD1LBQ` (tx_idx=1, ev_idx=0, amount_in=224280338)
  - **victim_sell** cp=286076322 `GNgPu2No5D3MhGsxTpGBKeq5HZ7rLhXRDYVko1CZFTZY` (sender=`0x89a1c807393670de16b055f0316232a5627b94bf74dfaa7ac34d3124109acf19`, tx_idx=9, ev_idx=4, amount_in=5063022232930)

### Pair 130

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+1 checkpoints)
  - **bot_buy** cp=286078070 `CYJj2cdWY7Jw45hjfsdmXAZcG4q4UZ6qZYSKTgDq6FEE` (tx_idx=11, ev_idx=0, amount_in=10555105536)
  - **victim_sell** cp=286078071 `4D6Gjw6z19QTWhZW7TWLbyv314RquBNGCDYppQgVXVcA` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=1, ev_idx=1, amount_in=5853455488512)

### Pair 131

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286079146 `FxN63CnL34jfU3k6T9YXuaGfxFFZFgMVwi2cStbcqs6a` (tx_idx=13, ev_idx=0, amount_in=10594204298)
  - **victim_sell** cp=286079149 `7tCja1SYN3Rp4mCjv3cCBdTkGApyNV8iEnfShoZpoYQC` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=5, ev_idx=1, amount_in=4072943944522)

### Pair 132

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+17 checkpoints)
  - **bot_buy** cp=286079510 `3XbnXhTJjbESkTcuqmD2TZMqPEWkhKQXYYmziW2zKAat` (tx_idx=1, ev_idx=0, amount_in=131098186155)
  - **victim_sell** cp=286079527 `DvQ62BQ4GPgrw9XjCHYmnLe51fuD8HqE6vmtm6SASW2v` (sender=`0x9cafda78c65e3e698325f34ebebf8d55df8a9f280e737cb2ca09ad55192f851e`, tx_idx=10, ev_idx=1, amount_in=169147415882)

### Pair 133

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+256 checkpoints)
  - **bot_buy** cp=286084942 `5kyGbWcNmAN3iBnn9fJtnt85f5GEsGiQbR4eNYpE6weF` (tx_idx=11, ev_idx=0, amount_in=10616686265)
  - **victim_sell** cp=286085198 `GVqg4XEuBVtJEYUVv9TH7g7uXz1wgfqyFNyQXgSSCjXv` (sender=`0x00006f748f809057fd1ca9ff8d02d89947f9079c26029ea2348657d8467b0000`, tx_idx=15, ev_idx=4, amount_in=6600715940825)

### Pair 134

- **pool** `0x7852612f5bf73613021f17353985fc186b3b224139c6a2576239132ba5a33b66` (+4 checkpoints)
  - **bot_buy** cp=286086669 `G68VJzK8Fw87GKQfdVF4QYWWphFcFpjT7McReRMejydN` (tx_idx=1, ev_idx=0, amount_in=37882251631)
  - **victim_sell** cp=286086673 `7XPcEHQWYJXQBJEKs6LoLcfPCi2jYX34Drd2r3YKN3UA` (sender=`0x84918d81f13cb9c2e6eb0939e5274cc2543a4b4fcd2a0e4f725994ef995776cc`, tx_idx=22, ev_idx=1, amount_in=3937453359)

### Pair 135

- **pool** `0x7852612f5bf73613021f17353985fc186b3b224139c6a2576239132ba5a33b66` (+3 checkpoints)
  - **bot_buy** cp=286087732 `2WMUxitz1AFUToejSKd2PTPCsDUR8858zgXHAPcWioQH` (tx_idx=4, ev_idx=0, amount_in=38368227442)
  - **victim_sell** cp=286087735 `2h5fNPNUMhELDrdCaGzSeCeznsJSVHkNTvQd3UpSSthq` (sender=`0x235d10e7e362a91be4cf51026bd4160aaa5bae742069aeba4d11d850d755aba6`, tx_idx=15, ev_idx=1, amount_in=6005729141)

### Pair 136

- **pool** `0x7852612f5bf73613021f17353985fc186b3b224139c6a2576239132ba5a33b66` (+6 checkpoints)
  - **bot_buy** cp=286088056 `H1mdJLspwT4XJvMjXs6u2myfs7qFvDQ7zzP9Aexqirwi` (tx_idx=26, ev_idx=0, amount_in=38468125035)
  - **victim_sell** cp=286088062 `6HxfUxyhuoyUqEye9oDRXTYt4N9AR6wsqQvcL8Ldc2sY` (sender=`0x609b7f187082ccc9a1af38d060dc85cadf76be9fa11e1cc52ac4964a43a377bb`, tx_idx=22, ev_idx=4, amount_in=942904617)

### Pair 137

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+3 checkpoints)
  - **bot_buy** cp=286093460 `SQ4ZXYfYLFSVLoxCieLYZCifoYgB7oq7Uhw4etTrKU3` (tx_idx=5, ev_idx=0, amount_in=133109328522)
  - **victim_sell** cp=286093463 `DSrZHRw4EaWxo4Rfj6U72Vc15TAPG6ftUCLaXAx5a3Bp` (sender=`0x52cbfdcdcc5e56b5cfee807c2b2360909dbd2d6f057b0fb5f00599bfc8581090`, tx_idx=4, ev_idx=2, amount_in=347861121)

### Pair 138

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+3 checkpoints)
  - **bot_buy** cp=286095727 `Hzk4Sy3XUhewRXXdsr99HqBdR8A7JkCkqABRrsNhGjgY` (tx_idx=10, ev_idx=0, amount_in=133178982784)
  - **victim_sell** cp=286095730 `HWYDDZctnVUFFJstTbB69cVeZQmHrbYTtW553H3LT982` (sender=`0xb5d3008714425fdcf228c2cbe05e4ec1ab1c3d8e5b8d4d078385793996238f1c`, tx_idx=10, ev_idx=3, amount_in=165076401)

### Pair 139

- **pool** `0x2f47d887c4ca1640c48946676dc3ccb40025cdb0aa52f21d6b043568a7c39ffe` (+3 checkpoints)
  - **bot_buy** cp=286096613 `BAZAfbxvd7wzjhBnvC1GNHuoemyeh2Ws4T1mpWCyu8e9` (tx_idx=9, ev_idx=0, amount_in=9001611159)
  - **victim_sell** cp=286096616 `AEQRMUniGC7sDvU65zHFX9tcm2bNUDe7HRVCUoeNfsd5` (sender=`0x7a6f34429afaf09469793b369fc9fad46bc0fc33036827ae955e424f37477e0a`, tx_idx=28, ev_idx=1, amount_in=32139193731845)

### Pair 140

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (same checkpoint)
  - **bot_buy** cp=286096770 `73SpKqvZgCCY4m8RvNV1w7Q5maxyMyVWcFZAjFB3janX` (tx_idx=3, ev_idx=0, amount_in=10758869533)
  - **victim_sell** cp=286096770 `GaiPpDtu5FZJ53CrRfFk4B4HvrDCnr13spixbfD7HMEX` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=13, ev_idx=1, amount_in=5786448320917)

### Pair 141

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+4 checkpoints)
  - **bot_buy** cp=286096824 `4VEK6Y1kTuigrxnJojDnGGaVrXYfitE3VRCiuMnWdGmM` (tx_idx=20, ev_idx=0, amount_in=10804465544)
  - **victim_sell** cp=286096828 `AVhwEFDaKtDEGhsVJG9gwvkBpciNkrapmpBvGJtTyqbd` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=3, ev_idx=7, amount_in=5123378756780)

### Pair 142

- **pool** `0x0254747f5ca059a1972cd7f6016485d51392a3fde608107b93bbaebea550f703` (+5 checkpoints)
  - **bot_buy** cp=286103559 `GyspzTCMsqNLZkSvcWP5qs1n9AZqm8Ktv5reXbDpnTxc` (tx_idx=38, ev_idx=0, amount_in=25805579837)
  - **victim_sell** cp=286103564 `E3a31HER6CkVaqKHRCJAsLMdNKZrtAjeTo2vhnZDp11Y` (sender=`0xc8c8b762ef5c2939bf6e84ddecb5275afea78fdbc1ab55bd50e0ef4d5b9aca5e`, tx_idx=34, ev_idx=3, amount_in=341789731668)

### Pair 143

- **pool** `0x9661cca01a5b9b3536883568fa967a2943e237de11a97976795f5adb293892e9` (+50 checkpoints)
  - **bot_buy** cp=286106288 `2vjCGJx9MtyT9G8B8W9hZZwCbyajReYUHnzQpk6edXnE` (tx_idx=2, ev_idx=0, amount_in=12304792042)
  - **victim_sell** cp=286106338 `9NeKq8kUvNouftgh4qaHT9Gkyc33HNFAFubZ5vFoGVGK` (sender=`0xb5b57ae7edf17cb839d39f8035655b1e48239ffd3632ccb527f61001cbc311eb`, tx_idx=19, ev_idx=0, amount_in=45113220317)

### Pair 144

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286107116 `AWJwrS4gaPsxe6e5Q2RTeQRitjk2wfMX3SPKp4SvtYmG` (tx_idx=2, ev_idx=0, amount_in=11272230935)
  - **victim_sell** cp=286107119 `Djg37CtMbDzwscoc1omDTRvrPLXgH5sWBXF7QVHzk7kz` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=2, ev_idx=7, amount_in=5585475420286)

### Pair 145

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286107241 `8DfuHWAmvbGP6CqjMDAHvHdxkDTiVc44x8Qu7zCS2f3j` (tx_idx=9, ev_idx=0, amount_in=11316762654)
  - **victim_sell** cp=286107244 `BXJNndBfoQvwFJgzaU3g4gwcMbTh4MXbJkaLJ6bCWrbE` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=10, ev_idx=7, amount_in=4589585963704)

### Pair 146

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286109963 `3B8UFJD9Yb4irxP1C6fmofmtytMNjVgs898CMYU1MKUa` (tx_idx=5, ev_idx=0, amount_in=11261305337)
  - **victim_sell** cp=286109966 `DkVeheBC1TqHNWJw7ECTnGz4vc299zmzq5bEyGUJu6QQ` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=14, ev_idx=7, amount_in=5033041742438)

### Pair 147

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286110070 `Cpz3VL2WwtJVkfwyQ9o3Mcsj4S3Ef1uVvk7w1rRsgUb2` (tx_idx=7, ev_idx=0, amount_in=11306328262)
  - **victim_sell** cp=286110073 `HRqthZmWuNALEDHBd6tnH1uv2tchRk4Low6QpZTkzarz` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=8, ev_idx=7, amount_in=5834248480867)

### Pair 148

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+2 checkpoints)
  - **bot_buy** cp=286110322 `2BsRiPqfYbJzdd6MdavDFP698r3yPAhFKqaodSXxUPi2` (tx_idx=9, ev_idx=0, amount_in=11336830591)
  - **victim_sell** cp=286110324 `BtwF7o9ogaufRsgPBRyi511SRm2CT78C7Uw3cj1sCqUm` (sender=`0xefe66cb8f823a99e09da9f59dc4e8c77a6c2a1f408876e903931fc6c393811dd`, tx_idx=6, ev_idx=10, amount_in=8993300000000)

### Pair 149

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (same checkpoint)
  - **bot_buy** cp=286110378 `6U84H718JUG5fdv5b6QBXURtz9qHK6UF4DjowVFrpRYk` (tx_idx=20, ev_idx=0, amount_in=11353872425)
  - **victim_sell** cp=286110378 `8Nhq8KCuiJrPjF8TegrUWn6iCCvhxzi5bxjgJPDU8zGZ` (sender=`0x7a6f34429afaf09469793b369fc9fad46bc0fc33036827ae955e424f37477e0a`, tx_idx=22, ev_idx=11, amount_in=6269400000000)

### Pair 150

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286110454 `GPHZa8sEqzpe4amrWkLL86XE8rpz2HPiPeJQXmHZsVik` (tx_idx=2, ev_idx=0, amount_in=2260129012)
  - **victim_sell** cp=286110457 `AZ7e4yjXN4WFvj5ST1u7aNAWx1s5JksrXDwofUoXwbtW` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=4, ev_idx=7, amount_in=6694396470030)

### Pair 151

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286113631 `3ps5sPa2wvZ5gEPLpXqRDsKF4qdw6PDM51aKwUm2xnr4` (tx_idx=4, ev_idx=0, amount_in=11315889403)
  - **victim_sell** cp=286113634 `Cbp6hART1YycytKtzX1xPWzjyT9KmtA5aYzykBDwXvSV` (sender=`0xa6c8aa8ddcc3ffebb59c0a38d8019466d6e5b7bf51c230e7b76c853082c499b2`, tx_idx=20, ev_idx=2, amount_in=3683855915527)

### Pair 152

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286114609 `8ZEfEZXHkP2neh3CL8Q3R5PqPGsDHVdLaQkLRkt2zUoB` (tx_idx=1, ev_idx=0, amount_in=11377994151)
  - **victim_sell** cp=286114612 `3zp6XVSGWk3HP1FcWDzBjdYyYcE7XsLthFaShchZrozQ` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=4, ev_idx=7, amount_in=3936201287302)

### Pair 153

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286114642 `2u9yTcvm5xAVq23gquvawe8deXHbPJD3zdj7YTvNjvGs` (tx_idx=3, ev_idx=0, amount_in=11469110220)
  - **victim_sell** cp=286114645 `HL3rR5A3V4tinrokjMSxVWVHWNzaDZai5Sd1AUj2sK3t` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=2, ev_idx=7, amount_in=10410787000376)

### Pair 154

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286114678 `729eD5jxLebVEdYLwqK8RU8MEBGXdHHpUNhVSnMgvsLs` (tx_idx=12, ev_idx=0, amount_in=11473539990)
  - **victim_sell** cp=286114681 `39pSbgJE1ZgSNwzAY5bFeGAyUCLYEtNDvE4f946FXdLC` (sender=`0xefdd8cd1016e4523133bd178b688b31aee69a8fc555b914d8d7a9b3d157554eb`, tx_idx=3, ev_idx=7, amount_in=4631523138345)

### Pair 155

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286115413 `HjSYsoqMgVFSnQUYtuKR319bvpyUHVFNCcUtzabuJcPC` (tx_idx=8, ev_idx=0, amount_in=11565912227)
  - **victim_sell** cp=286115416 `3eHgK9QaApe4Unam2V1Q3V5du9DjTDdrkgqnqXaSBq3A` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=12, ev_idx=7, amount_in=11983849815318)

### Pair 156

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286116181 `AtMeFnKeqEyCnfLbeSoBk6wfSeLAvmmpsfNupJKQ8AJC` (tx_idx=4, ev_idx=0, amount_in=11573081238)
  - **victim_sell** cp=286116184 `B1YDStFg6F4ZiYVbpwhUWwz9jxbRX7fWDB112QfSTszv` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=2, ev_idx=1, amount_in=4381032676838)

### Pair 157

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+4 checkpoints)
  - **bot_buy** cp=286116223 `9txoT5ibdu3Bi9bkpjSbPLHb9TizSSH9opQqi98fBK2c` (tx_idx=9, ev_idx=0, amount_in=133316917830)
  - **victim_sell** cp=286116227 `jFrWgAV76VpD3iec3TdnViL3Afku5oYpnPtyWRSoaV3` (sender=`0xeadb75d546fe2861d63f3bcb2296d3a3c1796ab5511553ba425b1886926f7f9b`, tx_idx=5, ev_idx=2, amount_in=275525290)

### Pair 158

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286116753 `HSSbjPPAxXjoyPs1Yv6SiKtryAPBkTFCb6PDJP6RiCEA` (tx_idx=11, ev_idx=0, amount_in=11783080957)
  - **victim_sell** cp=286116756 `5j93RuHyL18Nn3cWCKTaWjAQ4td6QRojx94GziT611M7` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=2, ev_idx=7, amount_in=9710424591342)

### Pair 159

- **pool** `0x155b01dc5dbf6eb319ee5df50d201ae49fdc7a0a074acfe4fe1201acbf181a56` (+455 checkpoints)
  - **bot_buy** cp=286116854 `5psjRfASZcEebeRWWcLt6CXALqZvjzgZp2rPshCz6GEv` (tx_idx=1, ev_idx=0, amount_in=36386702790)
  - **victim_sell** cp=286117309 `B9ki4QD97kGJ7o16HFmLESHFeHTtJeC9L5GKDGuANdY4` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=7, ev_idx=3, amount_in=15995057958525)

### Pair 160

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286117788 `6atQGDDWz9tp5qDnoq6bNRvYoyDWEHZgSaT3r8pvLALt` (tx_idx=19, ev_idx=0, amount_in=11770586789)
  - **victim_sell** cp=286117791 `85nt2phi87bpUimvWHfEkXvNvgZEi6JitvUZenfNuuP5` (sender=`0xa6c8aa8ddcc3ffebb59c0a38d8019466d6e5b7bf51c230e7b76c853082c499b2`, tx_idx=17, ev_idx=2, amount_in=4533383682850)

### Pair 161

- **pool** `0x4ba47580ade3fa6d64f699d746aeb2dcad986589fcd2cbc1b11923e0ce94c4af` (+4 checkpoints)
  - **bot_buy** cp=286118988 `86R4XAM1nNbWK7DB1THAqjiKaW61TEWKSnxGgFrBGyoc` (tx_idx=3, ev_idx=0, amount_in=4878791808)
  - **victim_sell** cp=286118992 `Dsw7fikao77KPLgkd14qESfSCzsSCRLXHKTF8XrRj5fA` (sender=`0xa6c8aa8ddcc3ffebb59c0a38d8019466d6e5b7bf51c230e7b76c853082c499b2`, tx_idx=2, ev_idx=4, amount_in=5835470807741)

### Pair 162

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286120016 `GK5FGAdY5TFXT2WvAf284kbv8yM77Qgpv7Rmr3vFaann` (tx_idx=8, ev_idx=0, amount_in=11760041787)
  - **victim_sell** cp=286120019 `5sBGvKaTTaqi3aoN3bJZJBAi8bdep7QBEc7rhKbkHpdb` (sender=`0x5347b918a9cc46358da35e787758707a459929f0c0ff921810f0f64c2790e117`, tx_idx=18, ev_idx=7, amount_in=1393973770661)

### Pair 163

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286120158 `DiNMEfgdmLHNX9NkQarccjb4ESnkwpHVyviuXMLEGMPJ` (tx_idx=6, ev_idx=0, amount_in=11783791758)
  - **victim_sell** cp=286120161 `3p2sYuM7N58cFWzAErgAYNWVHXcjrGt7wqR5V6T3Qzn3` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=8, ev_idx=7, amount_in=4923518351370)

### Pair 164

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+5 checkpoints)
  - **bot_buy** cp=286120712 `BKkfGq2CacQK9yuftyjtjfA59kXQszW29RKKTicmjHdD` (tx_idx=3, ev_idx=0, amount_in=12072354400)
  - **victim_sell** cp=286120717 `9SaeTZfjPkPZgGcbAkXBpPpWCqvV7jYFXPKPrks3EQWu` (sender=`0x5347b918a9cc46358da35e787758707a459929f0c0ff921810f0f64c2790e117`, tx_idx=10, ev_idx=7, amount_in=1568924820505)

### Pair 165

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3` (+36 checkpoints)
  - **bot_buy** cp=286122182 `5hMJMDXGnX1iPAjvYVvGKAEM6n4M7PuQ8CyYDMu42Nud` (tx_idx=8, ev_idx=0, amount_in=137967466)
  - **victim_sell** cp=286122218 `28nXBird5cKphdYUQ6Lpt68JVBGAF5Btop6LSotmuZBn` (sender=`0x33a6dcc43399a6ee9d0540059854a0842283a75d92178e6d5076125d1763bd40`, tx_idx=3, ev_idx=0, amount_in=85678671)

### Pair 166

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+3 checkpoints)
  - **bot_buy** cp=286122582 `GifrUJqDSM2ZHi2727CviWYEgQ5w8hLV49ZfsgeGUiqb` (tx_idx=8, ev_idx=0, amount_in=134790842513)
  - **victim_sell** cp=286122585 `A6WPYAfdJZSSSRsuxMXtYRjJjFQgPPA2oYkdMj7FbvxL` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=15, ev_idx=14, amount_in=13860000000)

### Pair 167

- **pool** `0xb2d10aba1311b6b50c419c2310a19133200468b1cc543ab117f3b9550a65227a` (+91 checkpoints)
  - **bot_buy** cp=286122762 `6YcDoMUDXUBTrnGZCTnMK9h9GFmCTDQrpJsbiBfU4gG5` (tx_idx=7, ev_idx=0, amount_in=94054875904)
  - **victim_sell** cp=286122853 `7o5vp59jPuyzm5dRFEsg91o8zYsB1Vx3wgkgDPGhG7Mh` (sender=`0xf8d155c1a66432631e8f9f4accbbf716337df22d187d44f5f86337aded61f1c8`, tx_idx=9, ev_idx=0, amount_in=1625005504829210)

### Pair 168

- **pool** `0xb2d10aba1311b6b50c419c2310a19133200468b1cc543ab117f3b9550a65227a` (+27 checkpoints)
  - **bot_buy** cp=286122902 `HkV9yP4xZ1NA3azmYsfzQtnAgQZXrBgJjg8vWant1w2d` (tx_idx=5, ev_idx=0, amount_in=94470955083)
  - **victim_sell** cp=286122929 `3Qh1RLsqDgXymsy6pyJVQzCJqTVxfgn2UTesCWwamK6f` (sender=`0x180842a600a447061a3da984310ac601a5438b6685c40764d36c975afcc754c4`, tx_idx=6, ev_idx=0, amount_in=244823962570621)

### Pair 169

- **pool** `0xb2d10aba1311b6b50c419c2310a19133200468b1cc543ab117f3b9550a65227a` (+13 checkpoints)
  - **bot_buy** cp=286123111 `D39u9je7xxaoNsT7kfJn29NpV48E8L6fa8aXojYv4MeF` (tx_idx=9, ev_idx=0, amount_in=94760221173)
  - **victim_sell** cp=286123124 `8mH8XzQzDGPmPLJVLmtvxSFy2zE4rkv2FSUHuauYHXPP` (sender=`0x6002c8fc54252b2ff6a73543714423daf0c091285417dfaf6fb5e4ebac33d2e6`, tx_idx=15, ev_idx=0, amount_in=2155706222873213)

### Pair 170

- **pool** `0xb2d10aba1311b6b50c419c2310a19133200468b1cc543ab117f3b9550a65227a` (+85 checkpoints)
  - **bot_buy** cp=286123148 `Fj5HpbgXMDHkikwA4RZRGY6XEHxBBLh8pNj3hjNmocp` (tx_idx=1, ev_idx=0, amount_in=94554178539)
  - **victim_sell** cp=286123233 `25i2D253KNk6KtNc2RuACREtCTmKYY6HQ3dcnbuN2qFF` (sender=`0x56c66e77c243f160384c24550e5a6b6098baa91ec447735b85e1e5f7bfd739d3`, tx_idx=8, ev_idx=0, amount_in=215695347034096)

### Pair 171

- **pool** `0xb2d10aba1311b6b50c419c2310a19133200468b1cc543ab117f3b9550a65227a` (+59 checkpoints)
  - **bot_buy** cp=286123174 `4x5cWGQ4UFvdJuAQmAacuybPa24YKKar4WjVgwAjGRG2` (tx_idx=3, ev_idx=0, amount_in=94762562974)
  - **victim_sell** cp=286123233 `25i2D253KNk6KtNc2RuACREtCTmKYY6HQ3dcnbuN2qFF` (sender=`0x56c66e77c243f160384c24550e5a6b6098baa91ec447735b85e1e5f7bfd739d3`, tx_idx=8, ev_idx=0, amount_in=215695347034096)

### Pair 172

- **pool** `0xb2d10aba1311b6b50c419c2310a19133200468b1cc543ab117f3b9550a65227a` (+39 checkpoints)
  - **bot_buy** cp=286123194 `91t7fkhpZqqEu8GdjWZKaZWNavAL3ApCEkmbs2ciDM4E` (tx_idx=8, ev_idx=0, amount_in=94859096728)
  - **victim_sell** cp=286123233 `25i2D253KNk6KtNc2RuACREtCTmKYY6HQ3dcnbuN2qFF` (sender=`0x56c66e77c243f160384c24550e5a6b6098baa91ec447735b85e1e5f7bfd739d3`, tx_idx=8, ev_idx=0, amount_in=215695347034096)

### Pair 173

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286123414 `EnAAq5L9fuCRbmNdbfmeTNMia75h1F7DjihbBVPH677n` (tx_idx=11, ev_idx=0, amount_in=24962266117)
  - **victim_sell** cp=286123417 `57PxJQhHhTFHz22LGbEZbyM6PP1pWGfV6JTbssd5cTnw` (sender=`0x5347b918a9cc46358da35e787758707a459929f0c0ff921810f0f64c2790e117`, tx_idx=15, ev_idx=7, amount_in=2087487050313)

### Pair 174

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286123690 `2NR4sM6hSeUn2wy8KWKHGmYPrxFr9fg79dEDKStwNs2L` (tx_idx=7, ev_idx=0, amount_in=25506255617)
  - **victim_sell** cp=286123693 `ACEAXDpMDSUvrNNH5U7nxhym9UxwCsJBjxYKBWnB2soD` (sender=`0x89a1c807393670de16b055f0316232a5627b94bf74dfaa7ac34d3124109acf19`, tx_idx=9, ev_idx=3, amount_in=2575407155228)

### Pair 175

- **pool** `0x4edb54baafdf02a2219b6c327c9acacd79280b9c98cdc1b2b23230de906fa421` (+74 checkpoints)
  - **bot_buy** cp=286124532 `4PavLyFcnuDnRR7rkcfuzsrw6Ccn1Fv8jp8d3DQVhddd` (tx_idx=5, ev_idx=0, amount_in=23536154447)
  - **victim_sell** cp=286124606 `3n5W5WZaxEoHW5WmSE8zQPtneCczveXNAarZU6PhknJa` (sender=`0xd7c6cc85a7794c1db4ee3186804b15810c949202ea95c7b8e681ac118cd3ed90`, tx_idx=8, ev_idx=0, amount_in=1708685278177)

### Pair 176

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (same checkpoint)
  - **bot_buy** cp=286128762 `4AfKtt8kjLZvf4RiEbt2ESpLrXsdrqyZgnE9nkggcTDJ` (tx_idx=9, ev_idx=0, amount_in=48067074402)
  - **victim_sell** cp=286128762 `FAQtatckwDhnHPz2QQ3dN6S23JUgU5itZX3X5qqxCZbC` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=10, ev_idx=1, amount_in=5697056151476)

### Pair 177

- **pool** `0xbba38df125bfe2267af5ebb05d741b2a2364f5893d9ec2f8c856dba0f0365e32` (+179 checkpoints)
  - **bot_buy** cp=286129103 `4PmhriuankbMdfCMvmkUfZ7vockiKXz33U36cvXmwsHZ` (tx_idx=1, ev_idx=0, amount_in=1042058727)
  - **victim_sell** cp=286129282 `EKWpa41Bd3sE5xuUYQioP6DEEu2HDZoRQygZpuLsvsKA` (sender=`0xf3e2d7f83a3e4fe2e006face98aa2068b3d7dd2a3427cf33fb569757362b8018`, tx_idx=53, ev_idx=0, amount_in=215300000000)

### Pair 178

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+4 checkpoints)
  - **bot_buy** cp=286129981 `Dhhp4aBMbFM2e1Cx7vSXhfExGtffTnYCJUSjj2NrGroo` (tx_idx=4, ev_idx=0, amount_in=137608608648)
  - **victim_sell** cp=286129985 `8taYfVh7HeRZuHJTdLtbngiDwaN4r2mEmFvxoBgf7uku` (sender=`0xe70a9ac2b885f879704cccb13f603404d673fe5a1a7f7af59fb431ce25ccb9ec`, tx_idx=1, ev_idx=2, amount_in=222711871)

### Pair 179

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286130190 `69ghoJBTRnJ2pFfrPYgqV3m5W7JJ7EVq3bL8BQjZUhki` (tx_idx=1, ev_idx=0, amount_in=12316688620)
  - **victim_sell** cp=286130193 `KmRwXtnKbnwb596GRh4QdHZWG95DQ47obqBGPNziVeF` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=2, ev_idx=7, amount_in=15740415143255)

### Pair 180

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+4 checkpoints)
  - **bot_buy** cp=286130311 `HrNyvdRaexWsCpCpjTTgQs89chF2ZD78M8gXWiadUysd` (tx_idx=20, ev_idx=0, amount_in=12430325440)
  - **victim_sell** cp=286130315 `HGbx7hyTu646YBnRQXtWuE6HsVtJCPtbJyW3JdRknchN` (sender=`0xf4fed53ed5258ba318c5dcb3ff9054b1238e0aa61453410149a6cd39f3ca0755`, tx_idx=20, ev_idx=8, amount_in=2652414831497)

### Pair 181

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286130474 `DAwTMJqdkYjMmGq1DxYFhhSsgp8n1SdGEJw1N6NJnJZV` (tx_idx=4, ev_idx=0, amount_in=12430886244)
  - **victim_sell** cp=286130477 `BjaJ8y3j8SSWsVfnttfFPxo2b8sqTXsBXWU5ZV1xhTbu` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=4, ev_idx=7, amount_in=3450459547836)

### Pair 182

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286131525 `Ec8H6BqXsLqiZpH5viCbyu1kxxMEg5eVsYppenft3zcv` (tx_idx=38, ev_idx=0, amount_in=12344890351)
  - **victim_sell** cp=286131528 `9soho5EMur5kV2EETaB5niDJM1AtP5bb68dRk8goiNhz` (sender=`0x89a1c807393670de16b055f0316232a5627b94bf74dfaa7ac34d3124109acf19`, tx_idx=3, ev_idx=3, amount_in=681384039090)

### Pair 183

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+2 checkpoints)
  - **bot_buy** cp=286131615 `29izRafc1Z8TiBfB5Fy5pmEvGL3piBNTLzrgX9XTbvSn` (tx_idx=5, ev_idx=0, amount_in=12318646193)
  - **victim_sell** cp=286131617 `EzMW9aQi56T5Aaz2ozpEe8FWHTUGp2RAKhtEJjvUXn6f` (sender=`0x89a1c807393670de16b055f0316232a5627b94bf74dfaa7ac34d3124109acf19`, tx_idx=12, ev_idx=3, amount_in=673022556758)

### Pair 184

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+4 checkpoints)
  - **bot_buy** cp=286137924 `Fcfmp4LLFLuiqkWBiTd9tbEoPDVNRfeduaN54r1g7nkQ` (tx_idx=8, ev_idx=0, amount_in=11731118120)
  - **victim_sell** cp=286137928 `5NPfNPoQTBXcF9kW9jv9dTUxiRjpkvYWxfJyNK4dahd1` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=3, ev_idx=1, amount_in=8092578184613)

### Pair 185

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286138546 `BhUUBHPk3vKDbEc8chaNvTSF1mXYqS2wkWvZZ6sjMXsm` (tx_idx=1, ev_idx=0, amount_in=11949259972)
  - **victim_sell** cp=286138549 `6yuL9tNL3QgTk3LWtvJ4jdfEakZjJZk1JthEBpyS3PzD` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=2, ev_idx=7, amount_in=20351521125389)

### Pair 186

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286138630 `JWJi1TetvqzW8tGBtbrmXQQPA95fST9YQXvbZRe4yaJ` (tx_idx=1, ev_idx=0, amount_in=11956477940)
  - **victim_sell** cp=286138633 `Dvur8JJokxcSBBA7nvNuwurkbPbJHRXpgQQ4foyxq75Z` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=4, ev_idx=12, amount_in=21220500000000)

### Pair 187

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286138661 `D4opTVxGKhFv89218panudeps8QehzBvhPGaqhpcxsyy` (tx_idx=9, ev_idx=0, amount_in=11969982231)
  - **victim_sell** cp=286138664 `H1MD6Zs2Rn3E6Eu6haYwVPU4rfSs5grSVSZnHZzqXU72` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=11, ev_idx=1, amount_in=8354524056099)

### Pair 188

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286138692 `DD3aaFSDnnNeDjTMN46fqFqKDH65zDBnJxnQs83rcm6h` (tx_idx=7, ev_idx=0, amount_in=12052806961)
  - **victim_sell** cp=286138695 `DHJdZVW7aUXn761tLKWdnD2q3nRBEFVuQXfUnPgnuQ4G` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=11, ev_idx=7, amount_in=11355292922125)

### Pair 189

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286138755 `C7ow6ozAKRNLxkYhrkh65QeapH1eS6i4C17DFgxS8oaC` (tx_idx=4, ev_idx=0, amount_in=12095333620)
  - **victim_sell** cp=286138758 `6ci8C3DH76VfWq9cuWiYj1r3usxppoUGeD9wGkuzBpxw` (sender=`0x7e17ed25424ca1e811d2627583000734c57636362382d8ca6e43f7b203171bbd`, tx_idx=11, ev_idx=3, amount_in=3740268039726)

### Pair 190

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286139018 `4Eh1pxt3ZskWd8aK4BNv2JhWSfd58V4EG1weh2JzPwix` (tx_idx=19, ev_idx=0, amount_in=12134452966)
  - **victim_sell** cp=286139021 `BcYHnopVwjQ2UvdkSaUL7fFXEFE96BJLhA2v7NB51Uf4` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=14, ev_idx=7, amount_in=4516031403392)

### Pair 191

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+1 checkpoints)
  - **bot_buy** cp=286139389 `2PCHFbijDNN4LXBzebSNZ8dxJHPZXMCtMjWxyqiSGbYw` (tx_idx=6, ev_idx=0, amount_in=139198645480)
  - **victim_sell** cp=286139390 `CVBg7WuiNdY7epEqjFw1JQwC4H1GaNDk44E5HMgqdTX3` (sender=`0xdef166e88048b9a44048b71528529bfa7a956db14b68c04fc8db1e66cf1bd32c`, tx_idx=8, ev_idx=18, amount_in=2915278381)

### Pair 192

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286140541 `FGUhrXcFFaNyPuQmXg5cvBdrnhvdRBepSzKr2k85hy6p` (tx_idx=9, ev_idx=0, amount_in=12256982373)
  - **victim_sell** cp=286140544 `8GU8duqikF2jQvnPMDPd37M2nnc95er1zpxSkM6FCCkG` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=25, ev_idx=7, amount_in=9221943965935)

### Pair 193

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+2 checkpoints)
  - **bot_buy** cp=286140742 `2PGfnagDFWe46BWcfM4QEW8XPgyZqyipUfBp6biC5Tjx` (tx_idx=11, ev_idx=0, amount_in=12402247314)
  - **victim_sell** cp=286140744 `2W2AzY45wEc2u9pCNm5Lo4wwjvxLvUGog8QjabPcGWRi` (sender=`0x89a1c807393670de16b055f0316232a5627b94bf74dfaa7ac34d3124109acf19`, tx_idx=15, ev_idx=11, amount_in=3851600000000)

### Pair 194

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+2 checkpoints)
  - **bot_buy** cp=286141194 `7L76dsEKsf64kk6GwwQMQzyDjK1G4Ce3ixQDCtUeRgpo` (tx_idx=1, ev_idx=0, amount_in=12379990979)
  - **victim_sell** cp=286141196 `Ev7QpdunnWBNeDVm9XnaMxjzyjGMq6HJ1cKeFRKSszyk` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=18, ev_idx=12, amount_in=7558200000000)

### Pair 195

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286142437 `Hm7cxBM3LkX2qKioRDCi6U7MNQiahLXmT6EeHXg168Z9` (tx_idx=7, ev_idx=0, amount_in=12447080888)
  - **victim_sell** cp=286142440 `5DQhNybEMqpJKaMHaTrcJCU1ywp2tkLM3sJwCJtweGGs` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=18, ev_idx=1, amount_in=5535938957843)

### Pair 196

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+4 checkpoints)
  - **bot_buy** cp=286142468 `FgcFgEPeceiqShZARzsWKzXNfGTVoxpSut9FKoCGErBg` (tx_idx=19, ev_idx=0, amount_in=12518584099)
  - **victim_sell** cp=286142472 `J3HhrHXnPj14C36M9SsmfBVHPQob7S9rBWCsRYJZV4QY` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=4, ev_idx=7, amount_in=7149917692665)

### Pair 197

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+2 checkpoints)
  - **bot_buy** cp=286154610 `6jWXjRo4Lcvc7fw64YKfuP1TfuwBr2fiDD9okf3UexjK` (tx_idx=2, ev_idx=0, amount_in=11801616241)
  - **victim_sell** cp=286154612 `CQNqfhEuAr55CtzWBn3sTxvH6WWzP8dikeMKe6JDDnZm` (sender=`0x00006f748f809057fd1ca9ff8d02d89947f9079c26029ea2348657d8467b0000`, tx_idx=11, ev_idx=13, amount_in=2169000000000)

### Pair 198

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286156045 `2chm6DPeqeSZqbTaNfWcRKkjXeGpxgpPLcaPPXGcakeH` (tx_idx=6, ev_idx=0, amount_in=11872684216)
  - **victim_sell** cp=286156048 `8CaCP1iqabgoww5ndbrrFyVudU17QewC5PupmqZXBzfc` (sender=`0x89a1c807393670de16b055f0316232a5627b94bf74dfaa7ac34d3124109acf19`, tx_idx=22, ev_idx=3, amount_in=1390075189307)

### Pair 199

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+585 checkpoints)
  - **bot_buy** cp=286156172 `3za4G5d3FC6LGu94aYRPs4YwY67Ngmc62ixaV2S7yhKL` (tx_idx=18, ev_idx=0, amount_in=11872930115)
  - **victim_sell** cp=286156757 `78ES8T6GHwPiFJrLTQw24xmTP2Xzn1c1otbcRe5SWbn2` (sender=`0x725004a49296de37f77aa3d4a70bb14269d41aebaed4362ab6b8621e3b55d085`, tx_idx=2, ev_idx=0, amount_in=2849645339999)

### Pair 200

- **pool** `0xde265ef8645c680c71b33805de77ce5261a20c58397d83b3915bdbb3a7209d7e` (+2064 checkpoints)
  - **bot_buy** cp=286156791 `5o1dhyEZ5by8r8uhec5mLEdBCcq7p3XybdbpWz6tMkzh` (tx_idx=15, ev_idx=0, amount_in=28050165779)
  - **victim_sell** cp=286158855 `DTebiTyn3Nk28GLCEEzuix4xp4nLC6wxGkmc8V8BNcYq` (sender=`0xc7e16c8399a3218468cdda2ebedb3a038c3bd04590b8110c37756a3592b0c4c2`, tx_idx=18, ev_idx=0, amount_in=7914773716619)

### Pair 201

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+3 checkpoints)
  - **bot_buy** cp=286159492 `Ar2c9nsU29qV5BidDwVdGj6QombdQp9tSn168fbB29tT` (tx_idx=18, ev_idx=0, amount_in=25142648498)
  - **victim_sell** cp=286159495 `FnVJW35vrotc84Unq7y74qjVp88RrmhTEfZYDUGLjt9z` (sender=`0x5ca5872f9743c6624e8da0bece1b5da905bb4959ab8999082e239b2c833942d7`, tx_idx=17, ev_idx=1, amount_in=58755103251)

### Pair 202

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+40 checkpoints)
  - **bot_buy** cp=286160643 `5AkBfFFsqidP3K11hb5H6evjQofWQgicPy7i2CBRCpHo` (tx_idx=4, ev_idx=0, amount_in=12137375941)
  - **victim_sell** cp=286160683 `2uVqpqfNcbzLwPK7AjerwLSVMn47MGVC555mTVx5451J` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=27, ev_idx=7, amount_in=5025564589431)

### Pair 203

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286160680 `FGE9QNxRkMPt4fqhTWK2iTmtLsHy9WrixhffW8LwP3jR` (tx_idx=19, ev_idx=0, amount_in=2428086810)
  - **victim_sell** cp=286160683 `2uVqpqfNcbzLwPK7AjerwLSVMn47MGVC555mTVx5451J` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=27, ev_idx=7, amount_in=5025564589431)

### Pair 204

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286160765 `6nrnmcU8ZghyZHFHkP8H9VtXuajt2Uh9HCqiScbf4yoF` (tx_idx=17, ev_idx=0, amount_in=2433477983)
  - **victim_sell** cp=286160768 `56KfVh9c8aZ6AqwFyjUybr9XUAXHuWG2BxY7dVVooFMW` (sender=`0x89a1c807393670de16b055f0316232a5627b94bf74dfaa7ac34d3124109acf19`, tx_idx=17, ev_idx=3, amount_in=1436751146257)

### Pair 205

- **pool** `0xe79efa7b95f6920dfc46ab38d0fae7419113d19e40e84b41abf8ddf3fd287ae1` (+7235 checkpoints)
  - **bot_buy** cp=286161801 `EMEgTdxoM8fBw2eP3TaRsJH9vtTAkcKd7LV8KcdZ6qP6` (tx_idx=10, ev_idx=0, amount_in=12353309571)
  - **victim_sell** cp=286169036 `Akq1Jrm6obkv5UzF91TzSAnVXSaePnpdTLFba5wiYsWV` (sender=`0x7a6f34429afaf09469793b369fc9fad46bc0fc33036827ae955e424f37477e0a`, tx_idx=2, ev_idx=2, amount_in=1009412633484)

### Pair 206

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286163531 `W2dvYa1rwouBot67v2DoMPpAJAxFcyNfPQ3y98KHqwh` (tx_idx=8, ev_idx=0, amount_in=11986494683)
  - **victim_sell** cp=286163534 `HbdacRfXHnCnDstfZtSaCFZ36RWaj5eZbvK3vrsVfZrX` (sender=`0x457d5ecda9f125053adf94fe357549e11e6fa688679cf94ababb5793ed3916ce`, tx_idx=11, ev_idx=14, amount_in=9695100000000)

### Pair 207

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286168856 `9mUWj3oFRakE6xTa8G6m7PgEYkei1k2qW5UkWf4njVR8` (tx_idx=8, ev_idx=0, amount_in=11789003739)
  - **victim_sell** cp=286168859 `F9Dh6S7LD1oQMfqsq2CmnZK2ortDmiE11r3gdoya5kjA` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=20, ev_idx=7, amount_in=4504596141364)

### Pair 208

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286172226 `EmSnqpQpGEABr8TfdYGytvbuetbpf25y6SSjaeG37vNF` (tx_idx=23, ev_idx=0, amount_in=11873271568)
  - **victim_sell** cp=286172229 `77HJYYwAUPsbCU7Q15QwMokVrbTaVDi6ZNGRa7hN6Eaf` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=13, ev_idx=7, amount_in=3711974742414)

### Pair 209

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286173063 `Gs75RuHiBAJdZefsg6JkDhJtB9CGi7AMGow82Tufx3n8` (tx_idx=19, ev_idx=0, amount_in=25194154419)
  - **victim_sell** cp=286173066 `PS3oRw7oGcVm7JmBaseaJUd645sBFYbSzV82s5Bj3AF` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=28, ev_idx=7, amount_in=45670749288742)

### Pair 210

- **pool** `0x51e883ba7c0b566a26cbc8a94cd33eb0abd418a77cc1e60ad22fd9b1f29cd2ab` (+1 checkpoints)
  - **bot_buy** cp=286173183 `7SMHQVqMZihc3JNSDtAENNLQGuwZ4XBVAUXrqJnutxKs` (tx_idx=14, ev_idx=0, amount_in=140628009824)
  - **victim_sell** cp=286173184 `4iQ11otJRS3oCGzA3YuBWJeCYGbM19gu4ZDVE4LCieZo` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=1, ev_idx=14, amount_in=325545260)

### Pair 211

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286173188 `927amqTkYZNZkTHBgsrgoy5yTCtwtLs6MPPqVLMRogWn` (tx_idx=17, ev_idx=0, amount_in=25452464002)
  - **victim_sell** cp=286173191 `HpDwiqHRzujLgVthccscHvZAMNQiSvdsbegdQUbf11A2` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=6, ev_idx=7, amount_in=22298123907347)

### Pair 212

- **pool** `0x4edb54baafdf02a2219b6c327c9acacd79280b9c98cdc1b2b23230de906fa421` (+71 checkpoints)
  - **bot_buy** cp=286174439 `42ovVGn4ff2SnkFRrqU5XF38tWqZttKgg34oM8NHX6ur` (tx_idx=4, ev_idx=0, amount_in=23532204769)
  - **victim_sell** cp=286174510 `73ehX3akPWCfx2bkLF35mDyA1REnyYWptiXRsZaWx1J8` (sender=`0x283adcb1b65b85c3cf856b7ef2b3fbfd328377cd125c3177c729990b0cdbd701`, tx_idx=13, ev_idx=0, amount_in=1481673795079)

### Pair 213

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+7 checkpoints)
  - **bot_buy** cp=286176192 `651qbkkhX3ed7J6itEf1T4aACoJWpDV4qguMuLW2BDAr` (tx_idx=7, ev_idx=0, amount_in=143831379150)
  - **victim_sell** cp=286176199 `G2BUQYNwXBHoZmihKL3k2sCzh7Fc8bXtY9MPusb2HQJS` (sender=`0x00006f748f809057fd1ca9ff8d02d89947f9079c26029ea2348657d8467b0000`, tx_idx=13, ev_idx=13, amount_in=284000000)

### Pair 214

- **pool** `0x4edb54baafdf02a2219b6c327c9acacd79280b9c98cdc1b2b23230de906fa421` (+63 checkpoints)
  - **bot_buy** cp=286178367 `BC1eK8uK1eBFci6LaM2DQwrfRa3d8q2auc8ZyqWF8kZs` (tx_idx=14, ev_idx=0, amount_in=23463376263)
  - **victim_sell** cp=286178430 `4eYMswrDvXNRndKJMGdexza5Q6NbLmkN4TPxfhWmY9mG` (sender=`0x283adcb1b65b85c3cf856b7ef2b3fbfd328377cd125c3177c729990b0cdbd701`, tx_idx=6, ev_idx=0, amount_in=1610303681867)

### Pair 215

- **pool** `0x4edb54baafdf02a2219b6c327c9acacd79280b9c98cdc1b2b23230de906fa421` (+12 checkpoints)
  - **bot_buy** cp=286179363 `gQqH9M6rvHD55xfBDFYWnLZDanL2g6qbtnuNJ8aM4QJ` (tx_idx=12, ev_idx=0, amount_in=23430584261)
  - **victim_sell** cp=286179375 `Ds9tkc94bph4YLmHbGjMyGKjBbLoKbD6VQBRs7aiVdAV` (sender=`0x6893b30f86bf628824525d482d13ffd12c32af6163b20fa39486a81d3f7b8ea6`, tx_idx=13, ev_idx=0, amount_in=64442117547)

### Pair 216

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286184457 `6mdkBTFExJEiV8wXZMYpRLJGP8LSRUaej35UVN4C6uU5` (tx_idx=19, ev_idx=0, amount_in=11687432349)
  - **victim_sell** cp=286184460 `BiPm9nLoaGXhCkPeCoZsw2ndxZnivW8ZgNAtGTPv5u46` (sender=`0x8af2133a24d1097119305ec4262319ebd54e0e6473976a13e94bfe8f3341716f`, tx_idx=7, ev_idx=3, amount_in=1100497374710)

### Pair 217

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286184513 `2feKNcmeKLkdCj7SHq7icXp7zGyerE39xrX8hZ2Yhpep` (tx_idx=19, ev_idx=0, amount_in=11734615105)
  - **victim_sell** cp=286184516 `FWdewUyj2uZeHKjt5S4YypMphiXzT7gxMAjYtqAXLtwL` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=13, ev_idx=7, amount_in=9942093429199)

### Pair 218

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286184562 `2NNuchXJXmnztX7SrU8nN5VHv72XURETCah3nHt5Sr7P` (tx_idx=14, ev_idx=0, amount_in=11821711635)
  - **victim_sell** cp=286184565 `D7HCmCfY6zfDwe5WCnKqHnxD7vkfCc7icC4wWQdwmHVL` (sender=`0x5347b918a9cc46358da35e787758707a459929f0c0ff921810f0f64c2790e117`, tx_idx=10, ev_idx=7, amount_in=1951506043590)

### Pair 219

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286184762 `4gfLaK6cqkBLfqUb7PqGYJ4ZZD3eGxNdiwkYYEdCTQP5` (tx_idx=1, ev_idx=0, amount_in=11916955211)
  - **victim_sell** cp=286184765 `4GBPrd2aE8CF6yXt9iVirodh7nwRFsuFqExPFRz9UXCx` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=6, ev_idx=7, amount_in=5547458269893)

### Pair 220

- **pool** `0xf9107158e4945d6bbc321c7471e0b7c9854c2d3a1b04aaff6acaa50b8ea203d2` (+3036 checkpoints)
  - **bot_buy** cp=286191542 `Fq9RtYLj86skn2W8Z3RPxFw88NRXugN3YJw3LDBLtoob` (tx_idx=24, ev_idx=0, amount_in=1236510479)
  - **victim_sell** cp=286194578 `42k4BdB2zcRtenuf8rjyFR7FKRCzTswngACx4Zednj89` (sender=`0xd01d8a0692fecae09fbbadd4a61e72eb6f8d14ba305764657df809a063d0e182`, tx_idx=6, ev_idx=15, amount_in=7491614555)

### Pair 221

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286193725 `AXCoEhHf27a8gyxq9TjdVVxF6X9V4yZr5ZK21ekGPV6v` (tx_idx=17, ev_idx=0, amount_in=2284480741)
  - **victim_sell** cp=286193728 `8wJB7tMdVbcosUXt5gtMmYw9QktP6B7xQJGvZPPNPXKV` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=12, ev_idx=7, amount_in=5002717872916)

### Pair 222

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+2 checkpoints)
  - **bot_buy** cp=286194278 `7vdrQBhdtD2pMFihibE27HojFqK8xmy44gkWkSazow2m` (tx_idx=2, ev_idx=0, amount_in=11635112205)
  - **victim_sell** cp=286194280 `GNYu7EcGzBzRK88BUdXjRcF3qLHBzJkpjxh2s68zY2mD` (sender=`0xc15abe9518026b1c8b47357d0901690f5ee9e9fbb92b7fa69581e943d8d065cd`, tx_idx=15, ev_idx=1, amount_in=1059582106672)

### Pair 223

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286194311 `G6gFXqvyQFytQE8DRdAmQ7RLZMs4M3UWoK72ACfMzb7c` (tx_idx=5, ev_idx=0, amount_in=11687632142)
  - **victim_sell** cp=286194314 `Dv713QvU35syiibJ7hfFpZzNPesLpri1dn9vgmka4h5R` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=3, ev_idx=1, amount_in=9229250409555)

### Pair 224

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286194338 `3v6VD8t448ESmRxNGAc5oXXoB1BwHEPxZimZaGmtsjMS` (tx_idx=16, ev_idx=0, amount_in=11718467724)
  - **victim_sell** cp=286194341 `HxxcCEJ8gfBEWbtKBnF2pyupzEHuVppduaGCx46jc6X6` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=3, ev_idx=7, amount_in=5443642014188)

### Pair 225

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286194393 `EDDzTbk5hXDhNRYGJHYx1qAqQJKTWDomECArb5KeBzL5` (tx_idx=1, ev_idx=0, amount_in=11756334352)
  - **victim_sell** cp=286194396 `BW9sneU6BTzRPP3sNNWo4h9u8ovLivd5fyHo9JcGZwu2` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=2, ev_idx=7, amount_in=4677156119697)

### Pair 226

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286196752 `Aj2hoChWrWRWuv29kCdXCM1ibb1sDoiBmRKzsqsAUaww` (tx_idx=8, ev_idx=0, amount_in=2352711403)
  - **victim_sell** cp=286196755 `5HkLrKEwtmK4LWJnZwShYWUH5kL8fPng7fZJ9MaBx5c7` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=11, ev_idx=7, amount_in=3629950612205)

### Pair 227

- **pool** `0xf45b01f23e9951e37733b76c8cc7d22dcd23141aa246a86e17595a7aca610e1d` (+1 checkpoints)
  - **bot_buy** cp=286206688 `DSwgaDFUWe29YstwfHSGWsNXLq52rczoCmLFP7CuXtbh` (tx_idx=3, ev_idx=0, amount_in=48629247223)
  - **victim_sell** cp=286206689 `HtRgtcnb2fa6EewEpX97LraQwfXcKY8v6HQfFJW2vUVo` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=3, ev_idx=8, amount_in=23233514)

### Pair 228

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286210014 `BfWamCdbuk8muB4eudDYCLxeAR4R8PtEvHTbsrrH9kqo` (tx_idx=1, ev_idx=0, amount_in=10735892678)
  - **victim_sell** cp=286210017 `8qucG5Tib4x8YjmGLLXUA4EM94MCVMmrYhLLZFKHtq7Q` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=8, ev_idx=7, amount_in=35846190173078)

### Pair 229

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286210090 `CxQujqbhw4hvHPYFfTrVDovDqqCY77Jp4pRN22Pw86RQ` (tx_idx=12, ev_idx=0, amount_in=10779707627)
  - **victim_sell** cp=286210093 `9Dsv6UcAko71RUjamg3Kc3A2tZymrV3BWypTE7AKNhrx` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=9, ev_idx=7, amount_in=44079723459194)

### Pair 230

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286210183 `BC2zPym5kJvYey2LFZXdrgvzwApaSD7uY7RSuezHjNyp` (tx_idx=4, ev_idx=0, amount_in=11186663806)
  - **victim_sell** cp=286210186 `EhUGV1bKThKJw45332EMLYoneEuCkvU8gWKUvSiRzgWC` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=15, ev_idx=1, amount_in=16252445232749)

### Pair 231

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+25 checkpoints)
  - **bot_buy** cp=286210973 `Ex734rgTCWP3rQS1FzVYmekRTJFkFaeD2TiRrcdTHFNu` (tx_idx=11, ev_idx=0, amount_in=146114433880)
  - **victim_sell** cp=286210998 `8xrkmft7at8MREMFxNDJMr9JkVL6joHuW6H16h2QHyC9` (sender=`0x609b7f187082ccc9a1af38d060dc85cadf76be9fa11e1cc52ac4964a43a377bb`, tx_idx=2, ev_idx=4, amount_in=35350639)

### Pair 232

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286211530 `3BRaSjK62c7rCHf37rsRkMvvChmQzVqjWELJJrnaJ848` (tx_idx=7, ev_idx=0, amount_in=11495817034)
  - **victim_sell** cp=286211533 `35ZrPbKc1cSjaP562sFLtfxpHvd6KcWFEUeUkaN3Mfdc` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=7, ev_idx=7, amount_in=3776694755104)

### Pair 233

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286212873 `D37bwGy55WkUFfpaTjEfGKEtx1EDoJ3NvVobEQDKNjwZ` (tx_idx=11, ev_idx=0, amount_in=11628962839)
  - **victim_sell** cp=286212876 `BVphSbNfHzUogSZNdzYdGhkRFh8JrVMEeuswZ2Z8fJSr` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=8, ev_idx=7, amount_in=8422064996195)

### Pair 234

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286212907 `Eq8Z1ZSosftNt6Czu6ddsTQFSwUk3cZAvzcoqqFCgWzr` (tx_idx=12, ev_idx=0, amount_in=11645778901)
  - **victim_sell** cp=286212910 `BnX4emcRXPs5mcc46TLrQvcwvHVzduVvpehKbj4MEd2K` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=14, ev_idx=7, amount_in=7299882632466)

### Pair 235

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286212941 `GYzH5UbfWaDCm6NhFe3AP61Mn9DQUGnUYbK7uBbTj6Fq` (tx_idx=24, ev_idx=0, amount_in=11670738030)
  - **victim_sell** cp=286212944 `8PMWxK31RnNVAYsJ5LG9Ha7nEcCqygyxPCV7nBFVsHEF` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=19, ev_idx=1, amount_in=6147232660373)

### Pair 236

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+1 checkpoints)
  - **bot_buy** cp=286212976 `7AGqAzKTwbrjCJC1eeppARETVc4EuhUwo7fFJbqMrFj6` (tx_idx=4, ev_idx=0, amount_in=11670167209)
  - **victim_sell** cp=286212977 `G2iD6S8F1grd7s3qZZjKJJiTMySNo1TQnAtRBZ4Dy11V` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=2, ev_idx=1, amount_in=6742451316322)

### Pair 237

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (same checkpoint)
  - **bot_buy** cp=286213390 `ErZLeLgMe5Q4aWH5KTz2PC5FDvwhCXY2HZBDiNj9oyEY` (tx_idx=3, ev_idx=0, amount_in=146091137840)
  - **victim_sell** cp=286213390 `CR72adwc8oh8HJoQryuUV31EQFcBrKM5CcmpQeNwa7s2` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=7, ev_idx=7, amount_in=20829528190562)

### Pair 238

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+2 checkpoints)
  - **bot_buy** cp=286213465 `BWcBY4X9xBX2ZbD58XMSCTcAzX1yqZawpc53uFWRfuBV` (tx_idx=5, ev_idx=0, amount_in=2318143160)
  - **victim_sell** cp=286213467 `3kj4i7RkgPg3XKkkYJcBM6mK1FHKp8PjmsjsPw4DcTuL` (sender=`0x00000cfa6d94cff09b37a1a4dcfc92a993b61fffb71ce95ad9949e2f4cfdad26`, tx_idx=12, ev_idx=7, amount_in=3457564812934)

### Pair 239

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+4 checkpoints)
  - **bot_buy** cp=286214477 `ETSnRPjLBxy4fjMPTHWzXs1rscs8aBLWi6pxvzu4mEH3` (tx_idx=19, ev_idx=0, amount_in=2265103810)
  - **victim_sell** cp=286214481 `Bp4qk9XFdXVyneNVREdvnKc5Hu8MFY87fVVXnNMEtTw` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=2, ev_idx=7, amount_in=9740567126842)

### Pair 240

- **pool** `0x2d3230025b4615087656952bf5ddb49d7a9b6712ac9aa14977a877f02a16f165` (+4 checkpoints)
  - **bot_buy** cp=286215467 `5RN4YWjW6QeyHsKbr2T6QeQPKb9pNjSTuRR4tQrVNQNf` (tx_idx=4, ev_idx=0, amount_in=4317535893)
  - **victim_sell** cp=286215471 `94m7jeBw77QKVpL6zTMPNy9zzPxsZVQinsavE5LqNVR1` (sender=`0x609b7f187082ccc9a1af38d060dc85cadf76be9fa11e1cc52ac4964a43a377bb`, tx_idx=8, ev_idx=4, amount_in=6718338902)

### Pair 241

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+10 checkpoints)
  - **bot_buy** cp=286216205 `F7QygTx4w6zYDKm4nRJnGBZfsEuXUpG5UGD9QVUheG6m` (tx_idx=17, ev_idx=0, amount_in=138847535438)
  - **victim_sell** cp=286216215 `3FodxbDESp7rqoiq6Nt1z8icQ4tDviEdkvU8AdpFjabH` (sender=`0x827e8052c08056ae1a7ba7be78cca0abe56f076ead9c33f9bbf1aec9e37c6988`, tx_idx=93, ev_idx=3, amount_in=2241844992)

### Pair 242

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+3 checkpoints)
  - **bot_buy** cp=286216997 `DCqkcCbkQKETaYSjJ5U3xGa6PE1EsE9jBWJXS8njjBtq` (tx_idx=6, ev_idx=0, amount_in=139145694709)
  - **victim_sell** cp=286217000 `LPdftuVG6LvrGcbqSXthWQj6H7oreGoW5BzQo1qnGE5` (sender=`0x20c03434a59947780aa089a8aa1a2a71b5685f9e65f8a90902132f4008c93f0b`, tx_idx=6, ev_idx=12, amount_in=343000000)

### Pair 243

- **pool** `0x1de5cc16141c21923bfca33db9bb6c604de5760e4498e75ecdfcf80d62fb5818` (+7 checkpoints)
  - **bot_buy** cp=286217258 `CujPn1zaMZ1pdbwu7pB9YA61iiZkaPbRLepGqDCdWeqv` (tx_idx=13, ev_idx=0, amount_in=15353557442)
  - **victim_sell** cp=286217265 `6YDuaghka2ha4Lxedqvc5F6Rcd4NbFT92tujamhd6R9x` (sender=`0x609b7f187082ccc9a1af38d060dc85cadf76be9fa11e1cc52ac4964a43a377bb`, tx_idx=1, ev_idx=1, amount_in=926538237145)

### Pair 244

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+7 checkpoints)
  - **bot_buy** cp=286224369 `B1jJEthEjd7zjDon1V6z3bW5ZA5qobBaFsYEFgFXoJpe` (tx_idx=5, ev_idx=0, amount_in=141157972718)
  - **victim_sell** cp=286224376 `5JWu7eVC2z2v5y16erT4uwTm7ULthzyfdbxevBzuQ1wL` (sender=`0x00006f748f809057fd1ca9ff8d02d89947f9079c26029ea2348657d8467b0000`, tx_idx=45, ev_idx=17, amount_in=408000000)

### Pair 245

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+1 checkpoints)
  - **bot_buy** cp=286224524 `BmoNitG6pYsAuNNrYzjkVVxgsqjnjnPHht4vKhWrmsmT` (tx_idx=10, ev_idx=0, amount_in=141540613248)
  - **victim_sell** cp=286224525 `5AmhDWwBu1fFog1eew3yxZ2G4kYzkG7cdF4nhnyLUnsN` (sender=`0xfcd96a53f698f63541f1cfdfc84b5ad637ec6a03f7bb4877b7c9cf7dbc171905`, tx_idx=8, ev_idx=12, amount_in=25461683)

### Pair 246

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286228138 `5rwMDrhpMgMBGqPmuMiyy9JEiuqDyjrQCKsjjhy3zCJv` (tx_idx=10, ev_idx=0, amount_in=23052586916)
  - **victim_sell** cp=286228141 `HQWDgwVnUPBAzpuUogCyaMh4CAx3E9Dn7KX2KkkEccwK` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=11, ev_idx=1, amount_in=10949884569204)

### Pair 247

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+3 checkpoints)
  - **bot_buy** cp=286232250 `9DaoidxmPdPXVSUxpoaiG9KBLy1YYpcBRUy5ztSLA8Lk` (tx_idx=9, ev_idx=0, amount_in=147795376610)
  - **victim_sell** cp=286232253 `DLCSywr1pmV1mCbAfkQQ2q26RSbo4R6HbxyTUN1qFFkT` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=10, ev_idx=1, amount_in=12607600361287)

### Pair 248

- **pool** `0xaa2347159a55adaf1d76745e13c2bc91449570d998f6ba8ecbf5129a5d4a0bbf` (+31627 checkpoints)
  - **bot_buy** cp=286245563 `2cWymhoP2fekLsvmm3z6ZUiZ6Jpah5vwocQ1RSRwrUzR` (tx_idx=19, ev_idx=0, amount_in=857301236)
  - **victim_sell** cp=286277190 `GCc96ebxuQUUanzDusVnuTMqG7LAHjdGit7tgkkhGUm4` (sender=`0xd0f78fa93b1e39ab523eb41591c9e631672a0ad9dfd27813f8747de5b31461da`, tx_idx=8, ev_idx=0, amount_in=22093508688305531)

### Pair 249

- **pool** `0xaa2347159a55adaf1d76745e13c2bc91449570d998f6ba8ecbf5129a5d4a0bbf` (+30138 checkpoints)
  - **bot_buy** cp=286247052 `HtQtNpR4Q2SWq8zkp2CxeG9faoNUahkLz16BJFi5ze8e` (tx_idx=8, ev_idx=0, amount_in=474199933)
  - **victim_sell** cp=286277190 `GCc96ebxuQUUanzDusVnuTMqG7LAHjdGit7tgkkhGUm4` (sender=`0xd0f78fa93b1e39ab523eb41591c9e631672a0ad9dfd27813f8747de5b31461da`, tx_idx=8, ev_idx=0, amount_in=22093508688305531)

### Pair 250

- **pool** `0xaf414b3d3bc14b8c92d79947e84dac88db214f60e5e732165f9f25a13843996a` (+4480 checkpoints)
  - **bot_buy** cp=286253193 `4PiKUMLCAb4sXD9QwjcWi46Bz2Zz72HUMp6CnqFBgBew` (tx_idx=12, ev_idx=0, amount_in=5893209448)
  - **victim_sell** cp=286257673 `8ujLqSQvuDzzBJh4BF11q8xkdmYDzAR7nqhrsRazVYBR` (sender=`0x601a43f172b17ae92aaf08572e9c5087b92421919498268612e8a6a1b498a3e3`, tx_idx=2, ev_idx=2, amount_in=272037660)

### Pair 251

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+3 checkpoints)
  - **bot_buy** cp=286289352 `BgygMU3NoxhXB5u36HYoPY5QP2gnW6zhbH9U21LVQfym` (tx_idx=23, ev_idx=0, amount_in=150254922211)
  - **victim_sell** cp=286289355 `9Csj3ARf7h7TJxRQJQVc5f1YHPWFzF6wvy5N8AfaXx3T` (sender=`0x788a9ada3f7ee01cb93352878d84e68dce92a3ebcdd418f7dde34ccba262db6b`, tx_idx=19, ev_idx=4, amount_in=1455224434)

### Pair 252

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+4 checkpoints)
  - **bot_buy** cp=286289588 `5Hw3pbJKHxEgvTAUkiJEvEwwsCHrK2KeDYzuwHxeHw1w` (tx_idx=10, ev_idx=0, amount_in=150254922211)
  - **victim_sell** cp=286289592 `Ay5SNTdgx8WtYt15JExtpma97qSf6ugNfkLYDVDVssF5` (sender=`0x6cae00a08b04f6a4ca7157628ccf60f40078616deab20d2b626bd1de7c8a16c9`, tx_idx=30, ev_idx=7, amount_in=1973754296)

### Pair 253

- **pool** `0x008c0a882b65d966862a47a2b1de308c42be621080cb623543638b6920fd505d` (+49720 checkpoints)
  - **bot_buy** cp=286297061 `EiHfSqAZg7pf4XurAu6JPjpu87ncwLwCN2WjzaWsfCka` (tx_idx=12, ev_idx=0, amount_in=19879523693)
  - **victim_sell** cp=286346781 `25TyBfqN9pttcKnsXaVg5iaA6hWTCxZJY7VweoDYHYqW` (sender=`0xc389831335a36515f56c4de366c202730eee6062f0cacb30146431f1e0712894`, tx_idx=7, ev_idx=0, amount_in=1798699298732098)

### Pair 254

- **pool** `0x2f47d887c4ca1640c48946676dc3ccb40025cdb0aa52f21d6b043568a7c39ffe` (+3 checkpoints)
  - **bot_buy** cp=286302499 `EGYS8yprmjagCCxpiZLbfndwwksG437VVkBSkBQoskVF` (tx_idx=7, ev_idx=0, amount_in=9752294652)
  - **victim_sell** cp=286302502 `CFEd1wx8ZRzNdo6AchoaYUMHKxCaq1AAz8mj8SD9NnEp` (sender=`0x5347b918a9cc46358da35e787758707a459929f0c0ff921810f0f64c2790e117`, tx_idx=4, ev_idx=2, amount_in=26160740284882)

### Pair 255

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+3 checkpoints)
  - **bot_buy** cp=286302896 `G1GfuWrmCMhSB6Aqq4xfgP1JVQABuXsUvjKRNgX9G23F` (tx_idx=21, ev_idx=0, amount_in=36362451575)
  - **victim_sell** cp=286302899 `5JPLaawHYM6EgiE94ND7j6G3QiLrM5Son1JMYSGQaPrz` (sender=`0x2896eec01861e1682730c37457daf4a222217dec9d9ab545f0ccccb643b88e0c`, tx_idx=6, ev_idx=2, amount_in=70453343739)

### Pair 256

- **pool** `0x2d3230025b4615087656952bf5ddb49d7a9b6712ac9aa14977a877f02a16f165` (+3 checkpoints)
  - **bot_buy** cp=286311842 `CQ7SoqP42bUrczYbvrrJXE8dVzVL5BRXU1hKxMAzF5Rq` (tx_idx=1, ev_idx=0, amount_in=4295672261)
  - **victim_sell** cp=286311845 `CNgjstdn4e5jq7x6vCSpNMNg25tKcE4oM49eV1JdA2hN` (sender=`0x89a1c807393670de16b055f0316232a5627b94bf74dfaa7ac34d3124109acf19`, tx_idx=40, ev_idx=2, amount_in=19449318912)

### Pair 257

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+155 checkpoints)
  - **bot_buy** cp=286314335 `8BkA8X2FJibxJnCHd2vgBUHySfxQWwAsxCMLDBU6zfw9` (tx_idx=19, ev_idx=0, amount_in=14378022914)
  - **victim_sell** cp=286314490 `8KtVX5W33StX3hhEEzTLsy3ifi7wFiGja6iFx7mx8fKu` (sender=`0x9cad98bde3e40d10fec68a6d6de179f53b2fcdce339519db9599fb8fe2b7f6c2`, tx_idx=19, ev_idx=8, amount_in=196724143093)

### Pair 258

- **pool** `0x9661cca01a5b9b3536883568fa967a2943e237de11a97976795f5adb293892e9` (+520 checkpoints)
  - **bot_buy** cp=286324352 `32Sh3gsom43QFg3uvVd38GCx5yTC9nqX4eiK7yPwYnDx` (tx_idx=1, ev_idx=0, amount_in=12244803219)
  - **victim_sell** cp=286324872 `82UTYPieZmqGLHtAzaaeKZNbFwCy5TgtUfx3jxEUuyqU` (sender=`0xd6ed19aac25ee4986feb1bd0c1ee988b7226d39634092b6b898f2f87a015d216`, tx_idx=3, ev_idx=1, amount_in=101622377010)

### Pair 259

- **pool** `0x0254747f5ca059a1972cd7f6016485d51392a3fde608107b93bbaebea550f703` (+3 checkpoints)
  - **bot_buy** cp=286325409 `8pJLW2eowiLQiQV6odBFFKirmoSzfLjJV8AHXhoYgZo9` (tx_idx=1, ev_idx=0, amount_in=152890288712)
  - **victim_sell** cp=286325412 `EsZFcxwjwGrK6XYYZnJ3eKY7tDvy1HbS85CCAcGhCNhr` (sender=`0xa8a6670d32e66762b8ee6d66f57aa847f718551099752a87cfa4ee7058e9b392`, tx_idx=1, ev_idx=1, amount_in=668072992364)

### Pair 260

- **pool** `0x51e883ba7c0b566a26cbc8a94cd33eb0abd418a77cc1e60ad22fd9b1f29cd2ab` (+4 checkpoints)
  - **bot_buy** cp=286327294 `9GCqLXusc25JDcUnruh71wKcx4Safqvc5nTs1dh5KxKE` (tx_idx=9, ev_idx=0, amount_in=152941668740)
  - **victim_sell** cp=286327298 `CNoPJahM3h5VRrFy87zC9ad25WHs2Jo5bc937j1jyk4y` (sender=`0xd265672730b0540ffd3569530682a0f02ef984b703457790554eb0e19329663a`, tx_idx=3, ev_idx=16, amount_in=461840821)

### Pair 261

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+4 checkpoints)
  - **bot_buy** cp=286335037 `G5wq3bsni45Uemq8ZdEFBuV5g67Yv4khH6qHEgKpvKe9` (tx_idx=4, ev_idx=0, amount_in=152931694104)
  - **victim_sell** cp=286335041 `3dFfPchKmaxbniQNg1aPs3DiPY5NK3vUeuSq2WzpqXfk` (sender=`0xa8a6670d32e66762b8ee6d66f57aa847f718551099752a87cfa4ee7058e9b392`, tx_idx=2, ev_idx=1, amount_in=268487998)

### Pair 262

- **pool** `0x7852612f5bf73613021f17353985fc186b3b224139c6a2576239132ba5a33b66` (+7255 checkpoints)
  - **bot_buy** cp=286344779 `3nQUpPBdPoZUGhTGLjn9dypzgdTFTYPCn12S7CXCvJQ5` (tx_idx=1, ev_idx=0, amount_in=37921346993)
  - **victim_sell** cp=286352034 `BzMxJLNQuAYRK18WY7wMRh5oFqqQWQuStyKB45fUCvjf` (sender=`0x609b7f187082ccc9a1af38d060dc85cadf76be9fa11e1cc52ac4964a43a377bb`, tx_idx=3, ev_idx=4, amount_in=381928001)

### Pair 263

- **pool** `0x7852612f5bf73613021f17353985fc186b3b224139c6a2576239132ba5a33b66` (+5 checkpoints)
  - **bot_buy** cp=286352029 `4tNeamQA5dVwDKHwR4oVnHaAAJCQNZyvuGKFHHjrKcbh` (tx_idx=5, ev_idx=0, amount_in=37962629313)
  - **victim_sell** cp=286352034 `BzMxJLNQuAYRK18WY7wMRh5oFqqQWQuStyKB45fUCvjf` (sender=`0x609b7f187082ccc9a1af38d060dc85cadf76be9fa11e1cc52ac4964a43a377bb`, tx_idx=3, ev_idx=4, amount_in=381928001)

## Full sandwiches

### Checkpoint 285709468 (1 sandwich)

#### Sandwich 1

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126`
  - **bot_buy** `FZzzShZFz3cPMbF8vxA1ePtD1LZ1FeBLtNDFrYbn88RW` (tx_idx=2, ev_idx=0, amount_in=104193856784)
  - **victim** `55EMgBGYcn6wzneec7ZGwEieK1aQR3zsz2jJ3XQ3KvLx` (sender=`0x530d1f04126b6a1207b3050f402e057212ef7956662be803e32b576e5de2ff30`, tx_idx=3, ev_idx=0, amount_in=129187812542)
  - **bot_sell** `HvLRS5KxGAMRqydAtW3qAYmQ5BoDeKu7ELXB4akmzeBY` (tx_idx=11, ev_idx=0, amount_in=4724809530)

### Checkpoint 285711508 (1 sandwich)

#### Sandwich 1

- **pool** `0x3227fe6ef46d38c05896a65e8365d5812d03b198a51b323ddd4ec13817661442`
  - **bot_buy** `CPCbtBoT9YPumbqX15ui6jiJ2mBwbPo5iPtyYW84S3h9` (tx_idx=8, ev_idx=0, amount_in=133774004)
  - **victim** `Bf7amYTVMHs6iU57wmDMMG2erjfqgkoy1qvoeXxHGP9z` (sender=`0x13cb9c7d3fd61a869dccb02695263f13f236dc639c8189e3ff3db68cebf59d01`, tx_idx=9, ev_idx=0, amount_in=600000000)
  - **bot_sell** `Hjzk1TLMP9SD2Uj5DzC6eG8WZ4oTN6qotU2qxiyYf8Sn` (tx_idx=11, ev_idx=0, amount_in=73839184)

### Checkpoint 285712427 (1 sandwich)

#### Sandwich 1

- **pool** `0x3227fe6ef46d38c05896a65e8365d5812d03b198a51b323ddd4ec13817661442`
  - **bot_buy** `CyofY9ENxEi6KGoTDN9reZw2qV9ek7HkHthRC6aAtGD8` (tx_idx=2, ev_idx=0, amount_in=217016408)
  - **victim** `23wZmmp2NjjYkWYrGw6ZevXscnxcUew3az5kKSwW9QgU` (sender=`0xae812fce582dad7ec7dde6f2154038051e492bcb4d0c50e47f801d96628f33d1`, tx_idx=3, ev_idx=0, amount_in=800000000)
  - **bot_sell** `9n4fhdw187acFezR8Rew2BAjPs5kbhsQBcXh2vo8dhQH` (tx_idx=16, ev_idx=0, amount_in=44667721)

### Checkpoint 285750290 (1 sandwich)

#### Sandwich 1

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c`
  - **bot_buy** `7o21YHnQFGFx7ffq25tmmCiQe3pKWSqrwoAASityFJ83` (tx_idx=5, ev_idx=0, amount_in=104484439699)
  - **victim** `BvZbKmgUBgmZRSbTn3F9ig2d27YtkTjTYn9ybtV7udPa` (sender=`0x17e47231137803aa4542f0605abd3788e380f5467dcc7c8889d8829fd624395c`, tx_idx=6, ev_idx=6, amount_in=335203788746)
  - **bot_sell** `Ax37WJXng7azCr9ssQ1h3m5uyL2YHUjec6auoSyrxiLB` (tx_idx=11, ev_idx=0, amount_in=2418194947576)

### Checkpoint 285768408 (1 sandwich)

#### Sandwich 1

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c`
  - **bot_buy** `qraeFtmnbyMXnjNx76LwegJZFCeMkBFsBcaUib36XU4` (tx_idx=6, ev_idx=0, amount_in=105112663172)
  - **victim** `BdZsN6wvDPwLMhoR4kjP7AahReL8v7iCzZSiU4N89itF` (sender=`0xa82534a5f024715faf85a11721d99bdd07e2f3aa8b7a64cf182b7f9998272c27`, tx_idx=21, ev_idx=0, amount_in=254140244931)
  - **bot_sell** `HKTCP8EC8AFYZH1QGdFXL4N7Ng9W9WWrZg95Met2cLhM` (tx_idx=24, ev_idx=0, amount_in=2437527921428)

### Checkpoint 285770942 (1 sandwich)

#### Sandwich 1

- **pool** `0xd6918afa64d432b84b48088d165b0dda0b7459463a7d66365f7ff890cae22d2d`
  - **bot_buy** `CnisAAHQ6L6vJBrYDYGJR5A6BwBURHizHtRXY27deuSx` (tx_idx=5, ev_idx=0, amount_in=105237587143)
  - **victim** `AvWamcPMitUYdduET3fZv8YJ8PmBJAzQ7kUyEYx9a6PH` (sender=`0xb5eb3d29c48566b7aa81311828f1d4dd48852814d99c9fe62cde12726344a1f0`, tx_idx=6, ev_idx=0, amount_in=140000000000)
  - **bot_sell** `4uBHp25wNAptAvix74wRa46TYs9VeKL7WiKrrGvFSczf` (tx_idx=16, ev_idx=0, amount_in=20068065892532)

### Checkpoint 285774668 (1 sandwich)

#### Sandwich 1

- **pool** `0x3f77391f6b33ca2967430490c68dab38596608d05fc19d1ac9c3797595a8fddb`
  - **bot_buy** `7JYkttQJUDWYW8E73GRFGvUUxFUNduSFH9CgEKts4PTU` (tx_idx=1, ev_idx=0, amount_in=2313565508)
  - **victim** `sxm7pQQP6EabT4yRxZEN9DkqFcai73WPxJvj4H48M16` (sender=`0x9d72d697a15f6859dc80d975a8cec24f6cce6d0f42d56ddccbee67830e48622f`, tx_idx=2, ev_idx=2, amount_in=22612000000)
  - **bot_sell** `6V5xLMXqrFEMmJBb5uqDpeh7kEGMGgkyuumA9RSfsNto` (tx_idx=44, ev_idx=0, amount_in=92894121703)

### Checkpoint 285778003 (1 sandwich)

#### Sandwich 1

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c`
  - **bot_buy** `Akujx6zWr1PiDYGP2zswDpUEsdFpnuEiysmVsivbt2uK` (tx_idx=7, ev_idx=0, amount_in=105800762067)
  - **victim** `381kBqTEicKjbismM2fZ3KV8xn2YzWNqTSARSBKXeTh8` (sender=`0x885776852ea04c28c668bc95e2ebf68ab874565dd9351c2f17d3d0321516c1d4`, tx_idx=8, ev_idx=4, amount_in=266432640407)
  - **bot_sell** `APRQ5xCgzmYEeRNfFHsQyozmtYTj1CARrvdnrsBijyFP` (tx_idx=15, ev_idx=0, amount_in=2453191881492)

### Checkpoint 285794980 (1 sandwich)

#### Sandwich 1

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c`
  - **bot_buy** `GSQsXvv8PnE4wd9aT2Rcf46Z1j5U9WqKFx9H14L8dq2W` (tx_idx=37, ev_idx=0, amount_in=106039606165)
  - **victim** `3iQWk93AcDyFg9CAr4q9uZrqqUDMTbeAoV68dQqs1kSX` (sender=`0x1eb59ebed1febea954bdf8b1a17f4ea388a326f32ad3a4ae357015216092e834`, tx_idx=40, ev_idx=0, amount_in=212922897218)
  - **bot_sell** `6cE7EcQhiUyWy74D2acqwPCwNXizd2kd2gan4A1aBvEt` (tx_idx=41, ev_idx=0, amount_in=2444177202300)

### Checkpoint 285798420 (1 sandwich)

#### Sandwich 1

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3`
  - **bot_buy** `Gbt8Yvi7jNbkz8VPR3dQsHdJGj6NaaL7QoW6GeZfP4SJ` (tx_idx=9, ev_idx=0, amount_in=23981035955)
  - **victim** `CpqstWZZeEEAcnFQYGXXESRTdprjyDn1CkN3Pie2CjaK` (sender=`0x2b38f1f60666c00a33edd8bc9cd05fa66b2d1df0c1b0b555cd023d9c626cea2d`, tx_idx=10, ev_idx=0, amount_in=95288963495)
  - **bot_sell** `C1FfmeubFQceobPumE8h4SZGUZTzrcNQxvvQk1v6MXgX` (tx_idx=11, ev_idx=0, amount_in=10868773630367571)

### Checkpoint 285798934 (1 sandwich)

#### Sandwich 1

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3`
  - **bot_buy** `9N5wT1m59RMiaSkouvJ3HkvNzfhoLsBQiLQg2Z4MTDb5` (tx_idx=9, ev_idx=0, amount_in=27822074128)
  - **victim** `DxgyV5ZQVGUnXhWpcKRzkYt1RUwjFL7v8ZLV2ai9pFah` (sender=`0x092d21d8796de90b8b10c15e016df7263bb9e7c58f924dbee7fe76a991986e24`, tx_idx=10, ev_idx=0, amount_in=1874038201)
  - **bot_sell** `FSNKuu26EiwPiBV4xEBTM19yDVqgwcu8ShHWkrgK8n8L` (tx_idx=19, ev_idx=0, amount_in=8520821298604338)

### Checkpoint 285799353 (1 sandwich)

#### Sandwich 1

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3`
  - **bot_buy** `4Gg6X6UBB8jpfxxXuHoRdJHqvj67wixRytLeN7jau3wp` (tx_idx=3, ev_idx=0, amount_in=27778520234)
  - **victim** `7RiSs2LcsXnNF1VmtJAyaeKWrK14vVPvVrWaSG95n6oF` (sender=`0x2b4c1e68417bd2288b59e0875d93264ba3aeedc8bed17bdf5857d62b0d2ce81b`, tx_idx=4, ev_idx=0, amount_in=1328483524)
  - **bot_sell** `CLSrEbE6WU4WSN79ZP3uCnq2UNQcoMFib6kzSFdG4Mk1` (tx_idx=11, ev_idx=0, amount_in=8530027578807130)

### Checkpoint 285799416 (1 sandwich)

#### Sandwich 1

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3`
  - **bot_buy** `AY2NGKvjeF5TFbWurSoPF225AkZVSdU74AkuVM1QMNhv` (tx_idx=1, ev_idx=0, amount_in=27809860002)
  - **victim** `48TGkjwDfM3VrxgooooqVWbP3bLTJ19QoUSTSTS73Zyy` (sender=`0x2f370aa1631b61f7e9d6e250fdaaa0f022fa2fa1b163a485943d3d2363ae919c`, tx_idx=2, ev_idx=0, amount_in=1277729983)
  - **bot_sell** `8N1WoQMJTPrVXJdKstjXpDtKmzU17v5GQh8YcySLFSGQ` (tx_idx=11, ev_idx=0, amount_in=8520015647323063)

### Checkpoint 285799980 (1 sandwich)

#### Sandwich 1

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3`
  - **bot_buy** `3dt5JjJQTkqon8dieWnCjGonpAB88fJTYBcyopm2zwgB` (tx_idx=1, ev_idx=0, amount_in=27945042126)
  - **victim** `Ai2QV8Thkj7vTv7Gor9r6caPRsmWrH5YqGkkqYLCFFxE` (sender=`0x474a674874fef97922e33bd4a507a86de5c30b2033f78f680890fc67a28bc540`, tx_idx=4, ev_idx=0, amount_in=2837366230)
  - **bot_sell** `CMa2xQU1581C2KM2J5P5NPz1BVZ92r2DJ2BNuZUBkt3n` (tx_idx=9, ev_idx=0, amount_in=8490552882160481)

### Checkpoint 285800286 (1 sandwich)

#### Sandwich 1

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3`
  - **bot_buy** `9bRPiuQamtFtDx7RvNZa9R5mYMWpmr2dneWAPowoG7Dk` (tx_idx=9, ev_idx=0, amount_in=28007130258)
  - **victim** `HEm7XTRUjftP3ayaDSV7SM3VqsrbCYTkbWtC2hcpB4gd` (sender=`0x8a167b71445110865124a3600ba6f7d1c0426697f4d53218d04e34701446a8e5`, tx_idx=12, ev_idx=0, amount_in=1542662200)
  - **bot_sell** `y7V1VUz7x8Y1AjiMpxPNofT36JsvpDeijdRJMXdcZ4D` (tx_idx=16, ev_idx=0, amount_in=8461931062812871)

### Checkpoint 285800569 (1 sandwich)

#### Sandwich 1

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3`
  - **bot_buy** `t95dWUynYv8hKnyCFyrbGErNTuXgf5cKJoQ8fMZTXCo` (tx_idx=1, ev_idx=0, amount_in=28019982090)
  - **victim** `hwQ4EKkDSpP31zRSZXSWKpw6K2WFv4sd8wviP5qygM2` (sender=`0xb5f5233ae3cab85659bcd23cc571d09ee0c028b0f613a07dded941e33683595c`, tx_idx=2, ev_idx=0, amount_in=1669890791)
  - **bot_sell** `BMa72VRSHy9H2G8GSifAYJQAApAc3HsvTtwPnai5mrhG` (tx_idx=8, ev_idx=0, amount_in=8459001734629153)

### Checkpoint 285801080 (1 sandwich)

#### Sandwich 1

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3`
  - **bot_buy** `BaRFNxYyVSCvtNdNwc4RrhQZcRRwUJ53mP4F7mtgkYRP` (tx_idx=4, ev_idx=0, amount_in=28072238785)
  - **victim** `BLov5MagTJiUu1GKsJDkWtjXcUk3nMAv1YJDWkBs42FR` (sender=`0xd91b44c0d0d3b2a5440e8b203f9e5cac6ed92159da269a5a838cd3a8e8721ed2`, tx_idx=5, ev_idx=0, amount_in=1609368610)
  - **bot_sell** `4hTpF8fxVue2wRq7ycWSUJykPWCBhS4qAYpuTvgAzr93` (tx_idx=17, ev_idx=0, amount_in=8442778096261411)

### Checkpoint 285801346 (1 sandwich)

#### Sandwich 1

- **pool** `0x2d3230025b4615087656952bf5ddb49d7a9b6712ac9aa14977a877f02a16f165`
  - **bot_buy** `22cSfx2BRUqnDWZL1gqiB3oC1okyeDE5Y2GkdWbyAMs5` (tx_idx=1, ev_idx=0, amount_in=4240495021)
  - **victim** `G1TthkfLSLGfYnaXLjwA2MJgtMt6iJAL9E5T5LX2WBet` (sender=`0x1f4eadfcaa9828e2e216e4b0111ed292d68716a8cf819784c603404aa903b6ed`, tx_idx=2, ev_idx=5, amount_in=5371302617)
  - **bot_sell** `A8CnYaqQ23RKiJr6qkUsSnoNLm4UTUYfWFRRQSKrBrN5` (tx_idx=8, ev_idx=0, amount_in=101702948359)

### Checkpoint 285801353 (1 sandwich)

#### Sandwich 1

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3`
  - **bot_buy** `6Fk4thks9WtLx413wJk5vzPtU1sMaxNDJjoZD1q4UTic` (tx_idx=2, ev_idx=0, amount_in=28098121706)
  - **victim** `GxgMnwbhWrvh5d2Mkark7QMrjvQ4i5paqJxDhkaYk9C8` (sender=`0xfa3cb042b61fd02a05eaa057fcc0648b65247396cc935b95b3ab7a33c9bfa2f0`, tx_idx=3, ev_idx=0, amount_in=1409344810)
  - **bot_sell** `A2eSujMyVw271jkjJHBsnY2wqmpMHyS113kiBzS2Ugqt` (tx_idx=7, ev_idx=0, amount_in=8433493493110995)

### Checkpoint 285801501 (1 sandwich)

#### Sandwich 1

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3`
  - **bot_buy** `3hHt9u38Gphq9s9MKE7QVMiHsxKkoEiuyE7pVvv4J87x` (tx_idx=1, ev_idx=0, amount_in=28109994784)
  - **victim** `6Rs4jCEBsQsU8ku154qZdhM25RvFskVqAGP54pohyAPy` (sender=`0x0ea94a26e3d941f572d4332b3e13633dd34e25dab705c90eb9a81600a0bd162b`, tx_idx=3, ev_idx=0, amount_in=1788879115)
  - **bot_sell** `DTvFCcg7bY6XkoL8QqqDLUTNKRHm8URyAgSeF9fXUsai` (tx_idx=8, ev_idx=0, amount_in=8432763908933095)

### Checkpoint 285801627 (1 sandwich)

#### Sandwich 1

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3`
  - **bot_buy** `CoQibSgbarAvNiJKAMYU9wjN1k2wtDRuGYAcVz142jCs` (tx_idx=5, ev_idx=0, amount_in=28120409480)
  - **victim** `C51iawX3qGjWLWhj2n2VH9GnbAwFmWgTU3UywwzbrTko` (sender=`0x34b17ac27c2cac2ee8c16810a955895a095f824c613abcd05fa26245657a5e1c`, tx_idx=6, ev_idx=0, amount_in=2150488015)
  - **bot_sell** `ENLzcACc1Uxki7jMnzFRy9u9JYsBTX9y4xtHdtYVUYZu` (tx_idx=11, ev_idx=0, amount_in=8432337354549804)

### Checkpoint 285804024 (1 sandwich)

#### Sandwich 1

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3`
  - **bot_buy** `4pbsWadv4MP4QKNYpjVaRfhcGuLKCgqFjr2AzvAmgHTD` (tx_idx=1, ev_idx=0, amount_in=28506488196)
  - **victim** `3eiQyhfQT33ycXZvsxbj2sgxBjMS7tzkk6Th1do63GWo` (sender=`0x4ecc70fadf372d6d2c246f10ae828b6aa6d21765914960e4b0cff13086d07e9d`, tx_idx=2, ev_idx=0, amount_in=20000000000)
  - **bot_sell** `FnNPSYvheWvWtytUxZAYkwfKgTguJM4AbunXt3pM7NAS` (tx_idx=8, ev_idx=0, amount_in=8448187842051742)

### Checkpoint 285804946 (1 sandwich)

#### Sandwich 1

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3`
  - **bot_buy** `CFz6ddg3bD5Gr5ePfxNo7XX5GVZ4Hbh2P2sVuNXyhK4e` (tx_idx=4, ev_idx=0, amount_in=29071069360)
  - **victim** `Em49crAsguwZ2JoYVR3mBKG9eFD3gwe2ysCJ9XVwZrbq` (sender=`0x9fbbdf9b85b1bbef04dfaff5dda8a593d84fa1139d576e9c3f9cc81746106d83`, tx_idx=5, ev_idx=0, amount_in=10000000000)
  - **bot_sell** `5wC5C2ebXGZBCBkMZwmjDsETj1cBrLSbwKNW8sAX97fG` (tx_idx=10, ev_idx=0, amount_in=8211045458045788)

### Checkpoint 285805047 (1 sandwich)

#### Sandwich 1

- **pool** `0xd4b0ff77b15f977f68ab25554399aafeee79ad0359ab0e4c31b679fc3f10a8d3`
  - **bot_buy** `BxwtBmzAWZLsVTqGPSCRQqqDJcG2jDDPA2dKQJbYuFrn` (tx_idx=1, ev_idx=0, amount_in=29242866687)
  - **victim** `4WcZsnuiA2D6PQ6HPjq7uxwXPqXxATEmRmNAV33ZFYW6` (sender=`0x00bcc4ec7d574856e96e971b127c92a3602e114bcc02532a8d152a138e822ec4`, tx_idx=2, ev_idx=0, amount_in=1659759315)
  - **bot_sell** `DtEgRGuhTBeuK5HDsNXdoCNYaEoF36d1opeF4yjHbfJx` (tx_idx=19, ev_idx=0, amount_in=8104687799903571)

### Checkpoint 285808268 (1 sandwich)

#### Sandwich 1

- **pool** `0x9661cca01a5b9b3536883568fa967a2943e237de11a97976795f5adb293892e9`
  - **bot_buy** `GSq3bG6kNUGiiuYCfxCodmoJ1g82L3rtLYQReqNiWhUY` (tx_idx=9, ev_idx=0, amount_in=12253496126)
  - **victim** `7UQFmT2fiCEz3RjsW1zWbxP4aKfiLc1snhDAC5KpB2ZD` (sender=`0xd197f6829946425e3fa64c309a25b22cfddd916403fca1c0fdabf8cf42d23f99`, tx_idx=11, ev_idx=2, amount_in=55952101661)
  - **bot_sell** `6HiRFT1RimdxuCyiRsHQaupkFm2VYyjp7qRv5MdL4erE` (tx_idx=16, ev_idx=0, amount_in=731580027568)

### Checkpoint 285812439 (1 sandwich)

#### Sandwich 1

- **pool** `0x7d7f4f0e7fcaa675fe8505f06d649dfc320d5340d8b9b6aced5b5a032e77cc50`
  - **bot_buy** `8bsALXd4vVuaCz656b3S8xvXHmmpQYPj2Q44DWUTDxfy` (tx_idx=18, ev_idx=0, amount_in=50950256874)
  - **victim** `bLEsPdfHtWWzTQxC1grqJXQXfxceRncMcaGqaxGaXfw` (sender=`0x4b7c872a5bf9310892b936890532264f62adce926456278e0df88b824801191b`, tx_idx=19, ev_idx=0, amount_in=250426207217)
  - **bot_sell** `8zEzMmq7baJwrxMCjgCGivJHG6pKyW5HqG1x9sJdAknG` (tx_idx=20, ev_idx=0, amount_in=17811860840507903)

### Checkpoint 285812444 (1 sandwich)

#### Sandwich 1

- **pool** `0x7d7f4f0e7fcaa675fe8505f06d649dfc320d5340d8b9b6aced5b5a032e77cc50`
  - **bot_buy** `H3FyZ67yokCFfs3zSQuWqXKpVpLV3j9D2mhUhBpYEJGM` (tx_idx=15, ev_idx=0, amount_in=55263126670)
  - **victim** `AggvHo6bDBgC3oLrmbXwqFrFpSioP3sQzQgZUjYTqCrp` (sender=`0x54cc76a4f23c82d354905e5a79755557bd2b193dd7cc156c112bbbb8dd090b01`, tx_idx=17, ev_idx=0, amount_in=105803036670)
  - **bot_sell** `2wjzqXVDfiLunSisjsn9j5DzKZYGfAWrQxJgN8C9oyhE` (tx_idx=18, ev_idx=0, amount_in=15290383659040132)

### Checkpoint 285819928 (1 sandwich)

#### Sandwich 1

- **pool** `0x76b7709caa7d74649d9bf1abb5f38ef452564b61496c1876486e6f500abb6b5b`
  - **bot_buy** `8zs4jUVzp4s9ZNZW9pFTAXfdfUa8AHGgjJ1LbWPyRnKW` (tx_idx=6, ev_idx=0, amount_in=60658676358)
  - **victim** `2cTNkeiQB8Ev1yXEVEESr5JyzZa7H7uMKF6NtVguFgKZ` (sender=`0x14333bee793ed50d771b79ae6aac7d31676cc288795c8bdb5b06745d883e7a76`, tx_idx=7, ev_idx=0, amount_in=2885113516)
  - **bot_sell** `CZhBce8Rfswe1vLu6XcbiYQGxEnmeUcs37mwB1qqibbk` (tx_idx=15, ev_idx=0, amount_in=17249268150941405)

### Checkpoint 285820195 (1 sandwich)

#### Sandwich 1

- **pool** `0x76b7709caa7d74649d9bf1abb5f38ef452564b61496c1876486e6f500abb6b5b`
  - **bot_buy** `2Wvn185bjEDHZTBxT3xNdpcA3fEAWR9g5ZAriVT5Kfvh` (tx_idx=7, ev_idx=0, amount_in=60742810061)
  - **victim** `8xdc3nri5aX6HT5JmUwJJ1RrfXSA1fYsijFip3o14L82` (sender=`0x624502f005d7d8f5f9a4b4a49a021a72c647dcf00e7e578e1f181226e3eb987f`, tx_idx=8, ev_idx=0, amount_in=1030841262)
  - **bot_sell** `EUTRgYLpYGP4QEJ6nSmNbMNZYj4dSRKEWMeLuQ9GYKFe` (tx_idx=9, ev_idx=0, amount_in=17212244671265600)

### Checkpoint 285820744 (1 sandwich)

#### Sandwich 1

- **pool** `0x76b7709caa7d74649d9bf1abb5f38ef452564b61496c1876486e6f500abb6b5b`
  - **bot_buy** `MsjnY79NouyPJ1JQvbt6syEV4YS8uRxdZCN99rHBQqK` (tx_idx=11, ev_idx=0, amount_in=60765942282)
  - **victim** `26AAi3UxRXK4ujNRsoxcyB5uTDPzm68qbHrkafoMSgMK` (sender=`0xfa511cbf6cf29c3e0207e35a77504aced1b52ce6c53fd3abe88382db31288f31`, tx_idx=12, ev_idx=0, amount_in=1289906856)
  - **bot_sell** `HAELjuFUHtbBT9rAtpRzpD1FCR8hv3dGcBAcpaLbsAzS` (tx_idx=13, ev_idx=0, amount_in=17207518283130456)

### Checkpoint 285833171 (1 sandwich)

#### Sandwich 1

- **pool** `0x03d135b439d55511a6d7d98fafe5b92093f78b14c522d9d4a8cb004df5aead4f`
  - **bot_buy** `CqaWPhJg2MyKnhQPUBxqTZGSFPwsMN9M225PUjckd4kH` (tx_idx=5, ev_idx=0, amount_in=32099933646)
  - **victim** `EMLu7UQH2BvC32jGBMC5P4yKmDfjJXR3nz9q1wFcEUUu` (sender=`0x2eee8d68306fda9dc27a23caf6487169fa796e1dce38697744f923fff9fe607a`, tx_idx=7, ev_idx=0, amount_in=92810000000)
  - **bot_sell** `Hu3kLwKdJdXqsPJ14Khx9KHAMa4aFU4RwhN7jENRoL89` (tx_idx=12, ev_idx=0, amount_in=304567963018)

### Checkpoint 285857124 (1 sandwich)

#### Sandwich 1

- **pool** `0x4edb54baafdf02a2219b6c327c9acacd79280b9c98cdc1b2b23230de906fa421`
  - **bot_buy** `C5cNMhbyuk6P6mUZkoFyfpMQCZtRkLrA5AVAa74u5VAN` (tx_idx=4, ev_idx=0, amount_in=23490457057)
  - **victim** `BeHfisN8XQjhQci58xysRUag4JWUJKDAbqrqGxApFaAn` (sender=`0x9295f76114012e214e5c75c25700a813edf5ee9c9d5e321d4e473f7d2c3d3403`, tx_idx=5, ev_idx=0, amount_in=19972224015)
  - **bot_sell** `C6n1D6vUuGAM82eDQJnMTvg4SMu5raxqkQKb2D2xEBpW` (tx_idx=11, ev_idx=0, amount_in=7791034380048)

### Checkpoint 285878942 (1 sandwich)

#### Sandwich 1

- **pool** `0x4edb54baafdf02a2219b6c327c9acacd79280b9c98cdc1b2b23230de906fa421`
  - **bot_buy** `8DqY5rXK1rDKbe6jEkf6Gs56rBKenqe2SxGCvqVLcJ7` (tx_idx=5, ev_idx=0, amount_in=11679560557)
  - **victim** `46MpgEAGyfo4rWuXXLXF71sD5qbc4nsW2tGdWjfD5WRT` (sender=`0x1f4eadfcaa9828e2e216e4b0111ed292d68716a8cf819784c603404aa903b6ed`, tx_idx=6, ev_idx=2, amount_in=16768418876)
  - **bot_sell** `HRzExUk34rgeQHa8J6Cz2mxEFkKpDggpUvRoEKttJkn6` (tx_idx=11, ev_idx=0, amount_in=3895135810918)

### Checkpoint 285902737 (1 sandwich)

#### Sandwich 1

- **pool** `0x4edb54baafdf02a2219b6c327c9acacd79280b9c98cdc1b2b23230de906fa421`
  - **bot_buy** `G7gLUYs7EBgUDGerdd9ZcGuj4TsqUhvTA124Bvc2gSVS` (tx_idx=3, ev_idx=0, amount_in=23377119370)
  - **victim** `HkNe8Vt91vHk1ZgDA2NRXhwMPi7P3s3HDdQmFKFytiU8` (sender=`0xd7c6cc85a7794c1db4ee3186804b15810c949202ea95c7b8e681ac118cd3ed90`, tx_idx=4, ev_idx=0, amount_in=19910669581)
  - **bot_sell** `3YwduhSVGCxnU2qptaB5gZRkDfBLifBn8c1HqNrcK11V` (tx_idx=8, ev_idx=0, amount_in=7828863434213)

### Checkpoint 285904402 (1 sandwich)

#### Sandwich 1

- **pool** `0xd6918afa64d432b84b48088d165b0dda0b7459463a7d66365f7ff890cae22d2d`
  - **bot_buy** `647MmqRCa2937BgEjuvGPBv2tUjdkGejF6bi3K21diFA` (tx_idx=1, ev_idx=0, amount_in=33373442772)
  - **victim** `4dLQ1HyTBF5cpy8TGD4G7M3LWYwXdejGk943VQmGvKit` (sender=`0x1f4eadfcaa9828e2e216e4b0111ed292d68716a8cf819784c603404aa903b6ed`, tx_idx=2, ev_idx=10, amount_in=39171120710)
  - **bot_sell** `4gtBu9Te7eHPfaYR5kYbtZXsTtqYSieYXGSXwD1sJWAx` (tx_idx=11, ev_idx=0, amount_in=6299455983332)

### Checkpoint 285914914 (1 sandwich)

#### Sandwich 1

- **pool** `0x29333c096043846d42356e250647f113a7a99d0470bcd584f98e9edeab69100e`
  - **bot_buy** `DowELvJxrZ4yDTwDRPmz6eV63HeKizaCszdLFa4RAMz3` (tx_idx=3, ev_idx=0, amount_in=130236590908)
  - **victim** `27Ki222qgn6AFsvUuU9vwhBVDZx6dDmzVv8TBsZtgrC7` (sender=`0x2d7889362e643364ce39504d902613bd504fd071ab6fde38b81e1eb8215aab47`, tx_idx=4, ev_idx=0, amount_in=134376617720)
  - **bot_sell** `2A3borjxUvC5WSZiqaEdphXpVtZi6nfCRFzQwRxj4Grz` (tx_idx=9, ev_idx=0, amount_in=2999820076004)

### Checkpoint 285947534 (1 sandwich)

#### Sandwich 1

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c`
  - **bot_buy** `GWc7FypT1BcfVcGj7HCy4PRgV7gWx9MjXNZJw9DeKtCH` (tx_idx=1, ev_idx=0, amount_in=130445658621)
  - **victim** `EBNEBk82EfPbL6N66Zxxg2mU32SjHRYXFDu4ziJg3tyX` (sender=`0x507919491904f1082be6365c484aebe151a3ed113414671e2ba080e414970a6e`, tx_idx=2, ev_idx=5, amount_in=210001501700)
  - **bot_sell** `HXKwHUaQ2mprtNWajdNwy6mrSJ7ruszDWHaUrGBmVnKq` (tx_idx=7, ev_idx=0, amount_in=2985151847885)

### Checkpoint 285952545 (1 sandwich)

#### Sandwich 1

- **pool** `0x68d16416770f9b73b0b1b45e118f6ea3a2910f548f942fe335824fd515cdff08`
  - **bot_buy** `H6zGCWyotnpezg2xt84dfSzPgoWeKFjSA35auP2LBTJ9` (tx_idx=3, ev_idx=0, amount_in=49231739893)
  - **victim** `47PCMEoSKAYh9irpEsAV5CBSF4ypFVztvDKPhG25mUbR` (sender=`0xd04713d2ad4be7fbb158eaede761fb9a262cb77cd760dc0256b9205408cea1b2`, tx_idx=4, ev_idx=3, amount_in=26989050225)
  - **bot_sell** `FymfucNYK8LvuvtjF1XdeUp8ckNrkZCR2wGtXx3EiW81` (tx_idx=9, ev_idx=0, amount_in=36882240748151)

### Checkpoint 285970797 (1 sandwich)

#### Sandwich 1

- **pool** `0xa809b51ec650e4ae45224107e62787be5e58f9caf8d3f74542f8edd73dc37a50`
  - **bot_buy** `8kPHRAaTCw4idojjeSf3BqAmhAwFsyn8UajvLkF2FDDZ` (tx_idx=5, ev_idx=0, amount_in=1672561286)
  - **victim** `AG9eH5b61G1uGzeTH78pjeqD7szLsfFfh3hhrk3chZQC` (sender=`0x1f8aedfbe4fbcca0b4109a01041a69f13e5b9626db7310a418b1ede39e46d343`, tx_idx=6, ev_idx=0, amount_in=15000000000)
  - **bot_sell** `8cnFj9p9qDqTiJc4UgJGyzW24xApiV4c3GHrp654wAtH` (tx_idx=10, ev_idx=0, amount_in=764496925290721)

### Checkpoint 285975848 (1 sandwich)

#### Sandwich 1

- **pool** `0x4edb54baafdf02a2219b6c327c9acacd79280b9c98cdc1b2b23230de906fa421`
  - **bot_buy** `Ghp8dFQ4dn8BD8AdpLmKKm19U8FYFoTXesf8Actn7sJH` (tx_idx=1, ev_idx=0, amount_in=23537822608)
  - **victim** `jJYuY6oqVQjZDsJXLwthEVkhD1UaxtZCUWbDKBu9Rey` (sender=`0x283adcb1b65b85c3cf856b7ef2b3fbfd328377cd125c3177c729990b0cdbd701`, tx_idx=2, ev_idx=0, amount_in=20140998849)
  - **bot_sell** `EDXCo2NgGyY5rzUvxeXa9fXXe9aSZaqpW1E8TFdneasV` (tx_idx=9, ev_idx=0, amount_in=7775561264989)

### Checkpoint 286004778 (1 sandwich)

#### Sandwich 1

- **pool** `0x4edb54baafdf02a2219b6c327c9acacd79280b9c98cdc1b2b23230de906fa421`
  - **bot_buy** `HLst7giaRP4AiVNFVWCYKHPBnDoeguNdYDFMGWAxv6Xa` (tx_idx=10, ev_idx=0, amount_in=23651622603)
  - **victim** `5YJe13o9sDYFqoCAR32hXHfukBwoBanriZ8sGScadKXx` (sender=`0x283adcb1b65b85c3cf856b7ef2b3fbfd328377cd125c3177c729990b0cdbd701`, tx_idx=11, ev_idx=0, amount_in=20336228638)
  - **bot_sell** `98PZ4YurixegM2ie8WqtQ1vJX6jHDzL5TY47pJVkUhP7` (tx_idx=13, ev_idx=0, amount_in=7738303653942)

### Checkpoint 286034918 (1 sandwich)

#### Sandwich 1

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18`
  - **bot_buy** `6w5oVE8fJW2mqTcamNE2baTtT6Vnt5kFFNtunwC1PyTi` (tx_idx=24, ev_idx=0, amount_in=9711518868)
  - **victim** `EekRFEuaL3txZUzkDjfUx9b2xEyV9UDUPdFnaW3BgEY2` (sender=`0xdcdef8a4a0dd417d7c4218e258519b37e7b1468b95fade5bcc815382760c4660`, tx_idx=25, ev_idx=1, amount_in=30462765670)
  - **bot_sell** `3ExPZeWVqHszkB6G1xzxg8PeSepM8ae1SEepGXSQtqYC` (tx_idx=26, ev_idx=0, amount_in=4884249869690)

### Checkpoint 286065546 (1 sandwich)

#### Sandwich 1

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c`
  - **bot_buy** `BfPNhEXwrSh8GiFC5DePxef1aCxhQNzcfLLfXDSNxjyL` (tx_idx=4, ev_idx=0, amount_in=126109011817)
  - **victim** `9Vr9AkNw2dLznaGGFh7nkNkoJzLRhQY7TpXwa5dBR4Hu` (sender=`0x885776852ea04c28c668bc95e2ebf68ab874565dd9351c2f17d3d0321516c1d4`, tx_idx=9, ev_idx=16, amount_in=195531017370)
  - **bot_sell** `DNN3xj71dHW69wCCuZEnrNK1eppsy2PFfuVfpQtUtGCB` (tx_idx=10, ev_idx=0, amount_in=2924618888398)

### Checkpoint 286072142 (1 sandwich)

#### Sandwich 1

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c`
  - **bot_buy** `4uxtVLWBH6rTX8tgXagxcuP5KdiTxBtpgf18jKwAWiCp` (tx_idx=3, ev_idx=0, amount_in=126175808442)
  - **victim** `CPRTD3YwsKMutLjEgdQzHHbpKj5vrVxqFM8zV7WVRrMg` (sender=`0x885776852ea04c28c668bc95e2ebf68ab874565dd9351c2f17d3d0321516c1d4`, tx_idx=4, ev_idx=22, amount_in=289383873982)
  - **bot_sell** `9Pvz4VZpgtuVeZUPeRiBzTJZnQvXrZBiHTCKkkeWC5HH` (tx_idx=8, ev_idx=0, amount_in=2918987354711)

### Checkpoint 286072249 (1 sandwich)

#### Sandwich 1

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c`
  - **bot_buy** `HzFcDKkfcvVmUd1ks772YUEbxfHhxw4Xchz9VGbhJ6Nd` (tx_idx=4, ev_idx=0, amount_in=126175808442)
  - **victim** `9ogyM7BZhfHARazpWuAFgYDNbM3NCimqHXaE9GYq6dUQ` (sender=`0xfcc5671ec7798cfdb82e05a2aa51cd0013399819b8409d6779e9c6f293dda62f`, tx_idx=5, ev_idx=2, amount_in=460273136954)
  - **bot_sell** `91YxkfdTenRUiSkWPV4VT6YPpKfHdJcNHsheYLHrsgFp` (tx_idx=13, ev_idx=0, amount_in=2919775941661)

### Checkpoint 286076320 (1 sandwich)

#### Sandwich 1

- **pool** `0x3b982ac4be6f654c2e11ce2d70639730a0c10a97abddfb362a99eaf181837ad0`
  - **bot_buy** `DQzzYfxanVGfNxVkkaQEwnWKmJTeGTbt2EzzThpD1LBQ` (tx_idx=1, ev_idx=0, amount_in=224280338)
  - **victim** `9KGJqQm7fv66jcaA9XxtuaB4PpEERgoxuNjjJk1UvZgT` (sender=`0x357b6591a9451944e6099fc669123e229e89a5a65a82c3f2fd4efdc56ae314e3`, tx_idx=2, ev_idx=0, amount_in=2000000000)
  - **bot_sell** `APXNU43Aro9FMZVWJeqmzbPJ168HGvqtDuGShMV8XWUg` (tx_idx=7, ev_idx=0, amount_in=4656707618164)

### Checkpoint 286079510 (1 sandwich)

#### Sandwich 1

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c`
  - **bot_buy** `3XbnXhTJjbESkTcuqmD2TZMqPEWkhKQXYYmziW2zKAat` (tx_idx=1, ev_idx=0, amount_in=131098186155)
  - **victim** `GjTUfac5dM8qPMj4VkuD62aJBKvPZc9KtkAmtqZMmnCW` (sender=`0x6614da503f70518aeeeb65e3848929dcd4e6f078213b6e9965ef2a77651b5611`, tx_idx=2, ev_idx=13, amount_in=228007025000)
  - **bot_sell** `7ndE4GDfobVbKcpSHBKB5PapaGXruV498DbHKHki3Nah` (tx_idx=7, ev_idx=0, amount_in=3031864810752)

### Checkpoint 286086669 (1 sandwich)

#### Sandwich 1

- **pool** `0x7852612f5bf73613021f17353985fc186b3b224139c6a2576239132ba5a33b66`
  - **bot_buy** `G68VJzK8Fw87GKQfdVF4QYWWphFcFpjT7McReRMejydN` (tx_idx=1, ev_idx=0, amount_in=37882251631)
  - **victim** `CyiFUFtQbL8esebTV3nRMFBzUcFHMY1WNoDd3h2VPU6u` (sender=`0x866c9dad1035d2e5cb6b3377f803756c438558a7c8c6eb725a68f511d99e2cc2`, tx_idx=2, ev_idx=4, amount_in=60123242529)
  - **bot_sell** `2rddfEAS7KvyUtXx8TUQQuqUHpzSivsbVgkxtmbJckh2` (tx_idx=10, ev_idx=0, amount_in=137092994068)

### Checkpoint 286087732 (1 sandwich)

#### Sandwich 1

- **pool** `0x7852612f5bf73613021f17353985fc186b3b224139c6a2576239132ba5a33b66`
  - **bot_buy** `2WMUxitz1AFUToejSKd2PTPCsDUR8858zgXHAPcWioQH` (tx_idx=4, ev_idx=0, amount_in=38368227442)
  - **victim** `21xtJv7EiYG8tCmC4joKUFuVCx89M1bYWujpgqVMobEr` (sender=`0x866c9dad1035d2e5cb6b3377f803756c438558a7c8c6eb725a68f511d99e2cc2`, tx_idx=5, ev_idx=2, amount_in=81234006064)
  - **bot_sell** `EodhSFqMHS2q8xQ6obasCuY8WqUnJUQXZEhES8PbeicL` (tx_idx=13, ev_idx=0, amount_in=135531082866)

### Checkpoint 286093460 (1 sandwich)

#### Sandwich 1

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126`
  - **bot_buy** `SQ4ZXYfYLFSVLoxCieLYZCifoYgB7oq7Uhw4etTrKU3` (tx_idx=5, ev_idx=0, amount_in=133109328522)
  - **victim** `34yKXafehifT44gsijAF1upirbA7EVftALv9YDRoBmXt` (sender=`0x24c90a0d6bdad5ab6cef23352d1206dd9df122915c530185dfe7bb45be62620d`, tx_idx=6, ev_idx=0, amount_in=226756446969)
  - **bot_sell** `52Kjb1hh5c4n11ZFuoTP7TxgKskEhsNp3TQXy84c6TRr` (tx_idx=15, ev_idx=0, amount_in=5980620171)

### Checkpoint 286096770 (1 sandwich)

#### Sandwich 1

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18`
  - **bot_buy** `73SpKqvZgCCY4m8RvNV1w7Q5maxyMyVWcFZAjFB3janX` (tx_idx=3, ev_idx=0, amount_in=10758869533)
  - **victim** `6xsVH46PJVfPwQKkBfPcKc4QCvQB63xZrei3EnmvhJZf` (sender=`0xdcdef8a4a0dd417d7c4218e258519b37e7b1468b95fade5bcc815382760c4660`, tx_idx=4, ev_idx=1, amount_in=32986215438)
  - **bot_sell** `HuPtMTgnvUDVPnf7TBYs1uPsGYQjuLmdnhTSvbXPDbbS` (tx_idx=29, ev_idx=0, amount_in=4414016327315)

### Checkpoint 286106288 (1 sandwich)

#### Sandwich 1

- **pool** `0x9661cca01a5b9b3536883568fa967a2943e237de11a97976795f5adb293892e9`
  - **bot_buy** `2vjCGJx9MtyT9G8B8W9hZZwCbyajReYUHnzQpk6edXnE` (tx_idx=2, ev_idx=0, amount_in=12304792042)
  - **victim** `21XQDmXmBLtXrvybT8eE5xC3dzM9recckcc462vMTsvc` (sender=`0xd191a43038d62c3a7b63330d29ac75256cc2e969b59d30401d7ad014454b4d30`, tx_idx=3, ev_idx=2, amount_in=20626000000)
  - **bot_sell** `D6szkA51YoqRkaJeGB3Ckgpu8jkg6mgUjQxZczhKFJHq` (tx_idx=12, ev_idx=0, amount_in=727507955692)

### Checkpoint 286113631 (1 sandwich)

#### Sandwich 1

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18`
  - **bot_buy** `3ps5sPa2wvZ5gEPLpXqRDsKF4qdw6PDM51aKwUm2xnr4` (tx_idx=4, ev_idx=0, amount_in=11315889403)
  - **victim** `JDwZLDw3qsWpfUQ84Ys6JKkz18vSawEHRBB4d8YoUQap` (sender=`0xdcdef8a4a0dd417d7c4218e258519b37e7b1468b95fade5bcc815382760c4660`, tx_idx=14, ev_idx=2, amount_in=30566599444)
  - **bot_sell** `ACtTMLw21bCWYzpxYc1rgQPhuod2swgiVJ2Yv6CdMoEz` (tx_idx=19, ev_idx=0, amount_in=4193023914598)

### Checkpoint 286116854 (1 sandwich)

#### Sandwich 1

- **pool** `0x155b01dc5dbf6eb319ee5df50d201ae49fdc7a0a074acfe4fe1201acbf181a56`
  - **bot_buy** `5psjRfASZcEebeRWWcLt6CXALqZvjzgZp2rPshCz6GEv` (tx_idx=1, ev_idx=0, amount_in=36386702790)
  - **victim** `AdEMhSBefUyaGdgZxx7vT8Jpyrvrm6gYDRQKWVcAFnvB` (sender=`0x309ae81e218308f6b8c2ebce700a4bd335470353a0b4bcb9e1da4c4ad29d4df6`, tx_idx=2, ev_idx=24, amount_in=305016988168)
  - **bot_sell** `AEzrHrrZeouBJyzmV6K8CqcRb5z5ch9KM44Kj7XcBqAq` (tx_idx=10, ev_idx=0, amount_in=196462695174770)

### Checkpoint 286117788 (1 sandwich)

#### Sandwich 1

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18`
  - **bot_buy** `6atQGDDWz9tp5qDnoq6bNRvYoyDWEHZgSaT3r8pvLALt` (tx_idx=19, ev_idx=0, amount_in=11770586789)
  - **victim** `4QprJHf6nKPpBAYk122YdAAHzKdDMwdA8xzFcUqEqdEP` (sender=`0xdcdef8a4a0dd417d7c4218e258519b37e7b1468b95fade5bcc815382760c4660`, tx_idx=20, ev_idx=1, amount_in=31023631545)
  - **bot_sell** `8hPsaZumVM1vbX6B6HfkTv5iwTRk1xhPsxj5R39fVubD` (tx_idx=21, ev_idx=0, amount_in=4024932895592)

### Checkpoint 286118988 (1 sandwich)

#### Sandwich 1

- **pool** `0x4ba47580ade3fa6d64f699d746aeb2dcad986589fcd2cbc1b11923e0ce94c4af`
  - **bot_buy** `86R4XAM1nNbWK7DB1THAqjiKaW61TEWKSnxGgFrBGyoc` (tx_idx=3, ev_idx=0, amount_in=4878791808)
  - **victim** `AAc5APsLFMwShj1PnNdXVmHzcNhqG7jEc1NepDbhCZJr` (sender=`0x309ae81e218308f6b8c2ebce700a4bd335470353a0b4bcb9e1da4c4ad29d4df6`, tx_idx=4, ev_idx=15, amount_in=92413908316)
  - **bot_sell** `24xWzZbCgKr3fGKFLdGCxzwXBPrpceKmH8o1wG9KyEHB` (tx_idx=9, ev_idx=0, amount_in=16484952914994)

### Checkpoint 286120044 (1 sandwich)

#### Sandwich 1

- **pool** `0xf6477d460ef18e4b8ad3ca2895b1ba25b1ce935d0f0bce34db7d893ddfb6575c`
  - **bot_buy** `8XQ67LqzkVi5hiA6BNBpwyWCNDGEQcEBMGVSnMag3XVo` (tx_idx=4, ev_idx=0, amount_in=93389615677)
  - **victim** `BZi8emWjiy6jS4NuGkJvGxsdbTdojpJNihmRELAPccHt` (sender=`0x20ff63a086b3296ede0f0e059d7e81e3467bec08785b93e80f216f9d483205ff`, tx_idx=5, ev_idx=0, amount_in=1142398172)
  - **bot_sell** `E6fFeefAkKNiSCGKhtcHJWwhSucRpSStq9rdVsyYuKVL` (tx_idx=11, ev_idx=0, amount_in=22031430271733967)

### Checkpoint 286122902 (1 sandwich)

#### Sandwich 1

- **pool** `0xb2d10aba1311b6b50c419c2310a19133200468b1cc543ab117f3b9550a65227a`
  - **bot_buy** `HkV9yP4xZ1NA3azmYsfzQtnAgQZXrBgJjg8vWant1w2d` (tx_idx=5, ev_idx=0, amount_in=94470955083)
  - **victim** `65AqNfvChhjDhZbyMe7b7UwovwDQwT578aNsz4vHAk9j` (sender=`0x897a720a2f25e9796985cc9481ec197b7c80559da3505aefc9bc965743af682e`, tx_idx=6, ev_idx=0, amount_in=7505909230)
  - **bot_sell** `5PUtq1hF5H9WnWdsympZ6oTvtgmrjAv9jU8LPPRozKj3` (tx_idx=11, ev_idx=0, amount_in=21767033741207206)

### Checkpoint 286123174 (1 sandwich)

#### Sandwich 1

- **pool** `0xb2d10aba1311b6b50c419c2310a19133200468b1cc543ab117f3b9550a65227a`
  - **bot_buy** `4x5cWGQ4UFvdJuAQmAacuybPa24YKKar4WjVgwAjGRG2` (tx_idx=3, ev_idx=0, amount_in=94762562974)
  - **victim** `8EPs68q19WrgjPHvnCYWm4cmDiH5XA3FD6K6Ti8m5g4b` (sender=`0xb6ac3d1507a0084df0ce06465b1dbe11519e8fcda73156b49d3c33ef4a4fe256`, tx_idx=11, ev_idx=0, amount_in=817097475)
  - **bot_sell** `EPmNppSwmSspJexa29xwpP7bk1e32wYTPJdgZ1YZ4Kdy` (tx_idx=13, ev_idx=0, amount_in=21661773441562061)

### Checkpoint 286123194 (1 sandwich)

#### Sandwich 1

- **pool** `0xb2d10aba1311b6b50c419c2310a19133200468b1cc543ab117f3b9550a65227a`
  - **bot_buy** `91t7fkhpZqqEu8GdjWZKaZWNavAL3ApCEkmbs2ciDM4E` (tx_idx=8, ev_idx=0, amount_in=94859096728)
  - **victim** `G3ZryadBuUS6qCxvm2os2gUHMMx4MNtvf8VQvtvF4emW` (sender=`0xac273a8cbdde5cf254753e835b5be5a76c139369f04cbd2eae16abe3a515199c`, tx_idx=9, ev_idx=0, amount_in=6896089146)
  - **bot_sell** `Es4dfcfrzCYWgFQqq7K8Be2oF16VzTdf1Tg9VHH6SzAu` (tx_idx=10, ev_idx=0, amount_in=21674320384209883)

### Checkpoint 286124532 (1 sandwich)

#### Sandwich 1

- **pool** `0x4edb54baafdf02a2219b6c327c9acacd79280b9c98cdc1b2b23230de906fa421`
  - **bot_buy** `4PavLyFcnuDnRR7rkcfuzsrw6Ccn1Fv8jp8d3DQVhddd` (tx_idx=5, ev_idx=0, amount_in=23536154447)
  - **victim** `GupJ8cr4YYn3b1bkLnxXMrv8veBtsecEL7SZsu9b9AUP` (sender=`0xd7c6cc85a7794c1db4ee3186804b15810c949202ea95c7b8e681ac118cd3ed90`, tx_idx=7, ev_idx=0, amount_in=20138144115)
  - **bot_sell** `9gAUicBMWHAJUHyjDQQ9Tf8Mr6RG2gyDRkfjnfEYRuK5` (tx_idx=12, ev_idx=0, amount_in=7776110061970)

### Checkpoint 286129103 (1 sandwich)

#### Sandwich 1

- **pool** `0xbba38df125bfe2267af5ebb05d741b2a2364f5893d9ec2f8c856dba0f0365e32`
  - **bot_buy** `4PmhriuankbMdfCMvmkUfZ7vockiKXz33U36cvXmwsHZ` (tx_idx=1, ev_idx=0, amount_in=1042058727)
  - **victim** `FedZd9rqbtw9Ko5YwgbM6GGTpNifvS8ZurxGrSNweHfd` (sender=`0xf3e2d7f83a3e4fe2e006face98aa2068b3d7dd2a3427cf33fb569757362b8018`, tx_idx=2, ev_idx=0, amount_in=2000000000)
  - **bot_sell** `EPy221sWXRmKpj4n8Ms4Jw3aQHEjEtKsqA9HHiegvrQY` (tx_idx=8, ev_idx=0, amount_in=114397214847)

### Checkpoint 286131615 (1 sandwich)

#### Sandwich 1

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18`
  - **bot_buy** `29izRafc1Z8TiBfB5Fy5pmEvGL3piBNTLzrgX9XTbvSn` (tx_idx=5, ev_idx=0, amount_in=12318646193)
  - **victim** `5uGFdr3wWLxmzY7XH3hBNab6b7i3ptGeVxdERssb4r7` (sender=`0xdcdef8a4a0dd417d7c4218e258519b37e7b1468b95fade5bcc815382760c4660`, tx_idx=6, ev_idx=2, amount_in=32391501585)
  - **bot_sell** `HHLc3aqMeBTWGdA6hxbiLSmnscbCFPkc4AvAbSvnu1zD` (tx_idx=8, ev_idx=0, amount_in=3845804592689)

### Checkpoint 286173183 (1 sandwich)

#### Sandwich 1

- **pool** `0x51e883ba7c0b566a26cbc8a94cd33eb0abd418a77cc1e60ad22fd9b1f29cd2ab`
  - **bot_buy** `7SMHQVqMZihc3JNSDtAENNLQGuwZ4XBVAUXrqJnutxKs` (tx_idx=14, ev_idx=0, amount_in=140628009824)
  - **victim** `4KAJhnEkwYm2sEQF4f9dhFQHX6Ji2cZSgQ5GPfxTUUiG` (sender=`0xa95082f22154f31eb43a30025c43de00bf529ec60444c0d0128b5ca9f4441545`, tx_idx=15, ev_idx=6, amount_in=889902311129)
  - **bot_sell** `43HjsH48qCKJhNkfr252tofK9fueBgXxBUP5hsZLqjWd` (tx_idx=19, ev_idx=0, amount_in=106147594)

### Checkpoint 286174439 (1 sandwich)

#### Sandwich 1

- **pool** `0x4edb54baafdf02a2219b6c327c9acacd79280b9c98cdc1b2b23230de906fa421`
  - **bot_buy** `42ovVGn4ff2SnkFRrqU5XF38tWqZttKgg34oM8NHX6ur` (tx_idx=4, ev_idx=0, amount_in=23532204769)
  - **victim** `96jmy2d9v6XTnYTP82R4yWZ8XrM8Y5fXhMk7svbGrVyL` (sender=`0x283adcb1b65b85c3cf856b7ef2b3fbfd328377cd125c3177c729990b0cdbd701`, tx_idx=7, ev_idx=0, amount_in=20131385416)
  - **bot_sell** `qQNPpRbXDhrt1o39NHT3i8K7QRtwaPPJiAWdgd8WUSN` (tx_idx=12, ev_idx=0, amount_in=7777409904261)

### Checkpoint 286184457 (1 sandwich)

#### Sandwich 1

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18`
  - **bot_buy** `6mdkBTFExJEiV8wXZMYpRLJGP8LSRUaej35UVN4C6uU5` (tx_idx=19, ev_idx=0, amount_in=11687432349)
  - **victim** `F4XwLCb5X5VLaanzLukRJNaeq1We7ZAjEj5wUb2zxN5k` (sender=`0xdcdef8a4a0dd417d7c4218e258519b37e7b1468b95fade5bcc815382760c4660`, tx_idx=21, ev_idx=1, amount_in=31819068346)
  - **bot_sell** `Gh8LEAf2XqxKCJE2KZmKeGBRPGa4G9y8gjcbEJVJNuD5` (tx_idx=22, ev_idx=0, amount_in=4054424318351)

### Checkpoint 286204566 (1 sandwich)

#### Sandwich 1

- **pool** `0xca41fce0d0bd9e5249d518d525ef84ca97d0feabf953a2aab82da25f1f4aa3da`
  - **bot_buy** `4wUwnimeBLCiMCRgNAoMaMLXYvw9EvaVRhm5Joz5LhZC` (tx_idx=4, ev_idx=0, amount_in=92884153258)
  - **victim** `FCZcYNeMJQ2w1T7XN67wSit6jUa37HaDaH2wJJeFzgEf` (sender=`0x5501c018c6b6ba9e162442e3c46adb9093db28300b04a48923fadaf89728852e`, tx_idx=6, ev_idx=0, amount_in=1105003586)
  - **bot_sell** `VH7iQr6Hu73r41NcDRaF8tUwCtSZ3wNjvkT61VJRv22` (tx_idx=8, ev_idx=0, amount_in=22119707839343301)

### Checkpoint 286213465 (1 sandwich)

#### Sandwich 1

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18`
  - **bot_buy** `BWcBY4X9xBX2ZbD58XMSCTcAzX1yqZawpc53uFWRfuBV` (tx_idx=5, ev_idx=0, amount_in=2318143160)
  - **victim** `J9AM8KUK1dtniDWJrHCQYjCf38h2ewFWcDiDFNyhKCn4` (sender=`0xdcdef8a4a0dd417d7c4218e258519b37e7b1468b95fade5bcc815382760c4660`, tx_idx=6, ev_idx=1, amount_in=32154788894)
  - **bot_sell** `8ctAGA8TNaBCbNWw36QSEg4QqxqCeYgyBwxmrDZGJAoC` (tx_idx=10, ev_idx=0, amount_in=814433668210)

### Checkpoint 286215467 (1 sandwich)

#### Sandwich 1

- **pool** `0x2d3230025b4615087656952bf5ddb49d7a9b6712ac9aa14977a877f02a16f165`
  - **bot_buy** `5RN4YWjW6QeyHsKbr2T6QeQPKb9pNjSTuRR4tQrVNQNf` (tx_idx=4, ev_idx=0, amount_in=4317535893)
  - **victim** `699MGjWEoeX9MNDKYfnnA6vzsuPwwosUjTiVtarNScwc` (sender=`0x53fd3c0f46b61ec1d5ab143c4c20e3baa40e92ff335520be89a64fb0d587afbe`, tx_idx=5, ev_idx=12, amount_in=6764414507)
  - **bot_sell** `GiaPkaHmo89rW5LK2tNJT5Wb93u1WVgeqNwFGCvKL7eZ` (tx_idx=10, ev_idx=0, amount_in=100034196416)

### Checkpoint 286217258 (1 sandwich)

#### Sandwich 1

- **pool** `0x1de5cc16141c21923bfca33db9bb6c604de5760e4498e75ecdfcf80d62fb5818`
  - **bot_buy** `CujPn1zaMZ1pdbwu7pB9YA61iiZkaPbRLepGqDCdWeqv` (tx_idx=13, ev_idx=0, amount_in=15353557442)
  - **victim** `8bSB6wnrCYqQDZ4NwLZMQEWH8ugsosSaQeDDGC9myHJB` (sender=`0x192c403e87bcf16e7535763f959356043f51d20a8428d0f97890e3160e482c6c`, tx_idx=14, ev_idx=17, amount_in=187518295439)
  - **bot_sell** `64z342tzVzfZK9TBJSUvdWYWJdKxvUaXcQx2DExmK7nb` (tx_idx=15, ev_idx=0, amount_in=92928191304306)

### Checkpoint 286311842 (1 sandwich)

#### Sandwich 1

- **pool** `0x2d3230025b4615087656952bf5ddb49d7a9b6712ac9aa14977a877f02a16f165`
  - **bot_buy** `CQ7SoqP42bUrczYbvrrJXE8dVzVL5BRXU1hKxMAzF5Rq` (tx_idx=1, ev_idx=0, amount_in=4295672261)
  - **victim** `4PNkRMb6s8pMKjTnScTMdWdMSRCNWzAgGNGoHbd4hiPk` (sender=`0x3de285afece613e50ffcdac574994a87502b9ead9da6972aaf9dfcf59e65c8d9`, tx_idx=2, ev_idx=0, amount_in=6680000000)
  - **bot_sell** `DaBkCsEJkRea9iYvM1pueYmWP7MZQSq4X3Ceh4zjoapS` (tx_idx=9, ev_idx=0, amount_in=100537624095)

### Checkpoint 286324352 (1 sandwich)

#### Sandwich 1

- **pool** `0x9661cca01a5b9b3536883568fa967a2943e237de11a97976795f5adb293892e9`
  - **bot_buy** `32Sh3gsom43QFg3uvVd38GCx5yTC9nqX4eiK7yPwYnDx` (tx_idx=1, ev_idx=0, amount_in=12244803219)
  - **victim** `9PCd88PoYda9HxwsANKCsyqqL7QcNewUVcGHcMjqtrAQ` (sender=`0x4c03c0ff14da14dbf5d99542b0b7263b77bcf069a01b38aae5221a85959e0a29`, tx_idx=5, ev_idx=8, amount_in=28500000000)
  - **bot_sell** `DMGPPTq8N9KZR4VC9SnNWTvP25ESSP5nwWWnuzkPM5FY` (tx_idx=19, ev_idx=0, amount_in=731303543071)

### Checkpoint 286335037 (1 sandwich)

#### Sandwich 1

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126`
  - **bot_buy** `G5wq3bsni45Uemq8ZdEFBuV5g67Yv4khH6qHEgKpvKe9` (tx_idx=4, ev_idx=0, amount_in=152931694104)
  - **victim** `2wAottTPXABfBdZpYny1e4nXi3BkC3L4RHepjq4gCrYw` (sender=`0xc75f7537aca579ea1dab152de9d0b71b6b71afdca5fd99a9d2446b913c674daa`, tx_idx=5, ev_idx=6, amount_in=282522431391)
  - **bot_sell** `6XASWnKWbeDng7PxNLNh7VnrFhV9srTta655Xbp1Ug8P` (tx_idx=11, ev_idx=0, amount_in=6682491272)

### Checkpoint 286344779 (1 sandwich)

#### Sandwich 1

- **pool** `0x7852612f5bf73613021f17353985fc186b3b224139c6a2576239132ba5a33b66`
  - **bot_buy** `3nQUpPBdPoZUGhTGLjn9dypzgdTFTYPCn12S7CXCvJQ5` (tx_idx=1, ev_idx=0, amount_in=37921346993)
  - **victim** `GEQn3FsCnkG3rBdbEPrVS6XHbg9PnG4rTK18mBQE6jDT` (sender=`0x3621ae8e7deb58baf8de993a39f30a8c63c0f58bafd1c3bccf1e70144695948c`, tx_idx=3, ev_idx=0, amount_in=16000000000)
  - **bot_sell** `4QZk45sSXZKRS2rV9FUQmx3EonkZxsveY5HZtNeBrNUn` (tx_idx=8, ev_idx=0, amount_in=136563932923)

### Checkpoint 286352029 (1 sandwich)

#### Sandwich 1

- **pool** `0x7852612f5bf73613021f17353985fc186b3b224139c6a2576239132ba5a33b66`
  - **bot_buy** `4tNeamQA5dVwDKHwR4oVnHaAAJCQNZyvuGKFHHjrKcbh` (tx_idx=5, ev_idx=0, amount_in=37962629313)
  - **victim** `2ao5KUMWXPobUXnH7KtFuLP1h3q3PpNi9vZRhtdPYwxQ` (sender=`0x3621ae8e7deb58baf8de993a39f30a8c63c0f58bafd1c3bccf1e70144695948c`, tx_idx=6, ev_idx=0, amount_in=18000000000)
  - **bot_sell** `7XsyPkkjsghoLbitzjWkLv65t7XCxKghgKLhrGY5L4EK` (tx_idx=12, ev_idx=0, amount_in=136432774948)

## Per-checkpoint stats

| Checkpoint | Bot buys | Partial | Full sandwiches |
|------------|----------:|--------:|----------------:|
| 285676352 | 1 | 1 | 0 |
| 285678074 | 1 | 0 | 0 |
| 285709468 | 1 | 0 | 1 |
| 285711508 | 1 | 0 | 1 |
| 285712023 | 1 | 1 | 0 |
| 285712427 | 1 | 0 | 1 |
| 285736227 | 1 | 0 | 0 |
| 285738270 | 1 | 0 | 0 |
| 285740249 | 1 | 0 | 0 |
| 285740637 | 1 | 0 | 0 |
| 285750290 | 1 | 0 | 1 |
| 285752967 | 1 | 0 | 0 |
| 285768408 | 1 | 0 | 1 |
| 285770942 | 1 | 0 | 1 |
| 285772436 | 1 | 0 | 0 |
| 285774668 | 1 | 0 | 1 |
| 285778003 | 1 | 0 | 1 |
| 285779267 | 1 | 0 | 0 |
| 285780033 | 1 | 0 | 0 |
| 285781303 | 1 | 0 | 0 |
| 285781569 | 1 | 1 | 0 |
| 285786538 | 1 | 1 | 0 |
| 285791533 | 1 | 0 | 0 |
| 285794179 | 1 | 1 | 0 |
| 285794980 | 1 | 0 | 1 |
| 285797664 | 1 | 1 | 0 |
| 285797671 | 1 | 1 | 0 |
| 285798420 | 1 | 0 | 1 |
| 285798934 | 1 | 0 | 1 |
| 285799353 | 1 | 0 | 1 |
| 285799416 | 1 | 0 | 1 |
| 285799495 | 1 | 1 | 0 |
| 285799980 | 1 | 0 | 1 |
| 285800066 | 1 | 1 | 0 |
| 285800202 | 1 | 0 | 0 |
| 285800286 | 1 | 0 | 1 |
| 285800569 | 1 | 0 | 1 |
| 285801080 | 1 | 0 | 1 |
| 285801346 | 1 | 0 | 1 |
| 285801353 | 1 | 0 | 1 |
| 285801398 | 1 | 0 | 0 |
| 285801501 | 1 | 0 | 1 |
| 285801627 | 1 | 0 | 1 |
| 285801775 | 1 | 0 | 0 |
| 285802400 | 1 | 1 | 0 |
| 285802695 | 1 | 1 | 0 |
| 285802753 | 1 | 0 | 0 |
| 285804024 | 1 | 0 | 1 |
| 285804946 | 1 | 0 | 1 |
| 285805047 | 1 | 0 | 1 |
| 285807472 | 1 | 0 | 0 |
| 285807503 | 1 | 0 | 0 |
| 285807765 | 1 | 0 | 0 |
| 285808231 | 1 | 0 | 0 |
| 285808268 | 1 | 0 | 1 |
| 285809713 | 1 | 0 | 0 |
| 285811695 | 1 | 0 | 0 |
| 285812439 | 1 | 0 | 1 |
| 285812444 | 1 | 0 | 1 |
| 285817140 | 1 | 0 | 0 |
| 285818228 | 1 | 0 | 0 |
| 285818324 | 1 | 0 | 0 |
| 285818745 | 1 | 0 | 0 |
| 285819928 | 1 | 0 | 1 |
| 285820122 | 1 | 0 | 0 |
| 285820195 | 1 | 0 | 1 |
| 285820744 | 1 | 0 | 1 |
| 285821263 | 1 | 0 | 0 |
| 285821869 | 1 | 0 | 0 |
| 285822012 | 1 | 0 | 0 |
| 285825766 | 1 | 0 | 0 |
| 285827276 | 1 | 0 | 0 |
| 285833171 | 1 | 0 | 1 |
| 285837299 | 1 | 0 | 0 |
| 285839582 | 1 | 0 | 0 |
| 285839601 | 1 | 0 | 0 |
| 285839636 | 1 | 0 | 0 |
| 285839673 | 1 | 0 | 0 |
| 285842649 | 1 | 0 | 0 |
| 285843632 | 1 | 0 | 0 |
| 285848475 | 1 | 1 | 0 |
| 285848735 | 1 | 0 | 0 |
| 285851843 | 1 | 0 | 0 |
| 285853160 | 1 | 0 | 0 |
| 285856171 | 1 | 0 | 0 |
| 285857124 | 1 | 0 | 1 |
| 285868465 | 1 | 1 | 0 |
| 285868773 | 1 | 1 | 0 |
| 285869836 | 1 | 0 | 0 |
| 285871342 | 1 | 0 | 0 |
| 285873702 | 1 | 1 | 0 |
| 285878942 | 1 | 0 | 1 |
| 285883866 | 1 | 1 | 0 |
| 285889276 | 1 | 1 | 0 |
| 285898275 | 1 | 0 | 0 |
| 285898638 | 1 | 1 | 0 |
| 285902737 | 1 | 0 | 1 |
| 285904402 | 1 | 0 | 1 |
| 285907809 | 1 | 1 | 0 |
| 285914914 | 1 | 0 | 1 |
| 285947534 | 1 | 0 | 1 |
| 285952545 | 1 | 0 | 1 |
| 285960105 | 1 | 0 | 0 |
| 285962025 | 1 | 0 | 0 |
| 285970797 | 1 | 0 | 1 |
| 285975848 | 1 | 0 | 1 |
| 285977823 | 1 | 1 | 0 |
| 285989141 | 1 | 0 | 0 |
| 285996696 | 1 | 0 | 0 |
| 286002203 | 1 | 0 | 0 |
| 286004778 | 1 | 0 | 1 |
| 286030544 | 1 | 1 | 0 |
| 286034918 | 1 | 0 | 1 |
| 286039163 | 1 | 0 | 0 |
| 286047127 | 1 | 0 | 0 |
| 286053717 | 1 | 1 | 0 |
| 286053814 | 1 | 0 | 0 |
| 286056859 | 1 | 0 | 0 |
| 286057611 | 1 | 0 | 0 |
| 286063994 | 1 | 0 | 0 |
| 286064920 | 1 | 1 | 0 |
| 286065546 | 1 | 0 | 1 |
| 286065767 | 1 | 0 | 0 |
| 286070902 | 1 | 0 | 0 |
| 286071517 | 1 | 0 | 0 |
| 286072142 | 1 | 0 | 1 |
| 286072249 | 1 | 0 | 1 |
| 286072290 | 1 | 1 | 0 |
| 286073206 | 1 | 0 | 0 |
| 286074845 | 1 | 1 | 0 |
| 286076320 | 1 | 0 | 1 |
| 286078070 | 1 | 1 | 0 |
| 286079146 | 1 | 0 | 0 |
| 286079510 | 1 | 0 | 1 |
| 286084942 | 1 | 0 | 0 |
| 286086669 | 1 | 0 | 1 |
| 286087732 | 1 | 0 | 1 |
| 286088056 | 1 | 1 | 0 |
| 286093460 | 1 | 0 | 1 |
| 286095727 | 1 | 1 | 0 |
| 286096613 | 1 | 0 | 0 |
| 286096770 | 1 | 0 | 1 |
| 286096824 | 1 | 0 | 0 |
| 286103559 | 1 | 1 | 0 |
| 286106288 | 1 | 0 | 1 |
| 286107116 | 1 | 0 | 0 |
| 286107241 | 1 | 0 | 0 |
| 286109963 | 1 | 0 | 0 |
| 286110070 | 1 | 0 | 0 |
| 286110322 | 1 | 0 | 0 |
| 286110378 | 1 | 0 | 0 |
| 286110454 | 1 | 0 | 0 |
| 286113631 | 1 | 0 | 1 |
| 286114609 | 1 | 0 | 0 |
| 286114642 | 1 | 0 | 0 |
| 286114678 | 1 | 1 | 0 |
| 286115413 | 1 | 0 | 0 |
| 286116181 | 1 | 0 | 0 |
| 286116223 | 1 | 1 | 0 |
| 286116753 | 1 | 0 | 0 |
| 286116854 | 1 | 0 | 1 |
| 286117788 | 1 | 0 | 1 |
| 286118988 | 1 | 0 | 1 |
| 286120016 | 1 | 0 | 0 |
| 286120044 | 1 | 0 | 1 |
| 286120158 | 1 | 0 | 0 |
| 286120712 | 1 | 1 | 0 |
| 286122182 | 1 | 1 | 0 |
| 286122582 | 1 | 0 | 0 |
| 286122762 | 1 | 1 | 0 |
| 286122902 | 1 | 0 | 1 |
| 286123111 | 1 | 1 | 0 |
| 286123148 | 1 | 1 | 0 |
| 286123174 | 1 | 0 | 1 |
| 286123194 | 1 | 0 | 1 |
| 286123414 | 1 | 0 | 0 |
| 286123690 | 1 | 0 | 0 |
| 286124532 | 1 | 0 | 1 |
| 286128762 | 1 | 0 | 0 |
| 286129103 | 1 | 0 | 1 |
| 286129981 | 1 | 0 | 0 |
| 286130190 | 1 | 0 | 0 |
| 286130311 | 1 | 1 | 0 |
| 286130474 | 1 | 0 | 0 |
| 286131525 | 1 | 1 | 0 |
| 286131615 | 1 | 0 | 1 |
| 286137924 | 1 | 0 | 0 |
| 286138546 | 1 | 0 | 0 |
| 286138630 | 1 | 0 | 0 |
| 286138661 | 1 | 0 | 0 |
| 286138692 | 1 | 0 | 0 |
| 286138755 | 1 | 1 | 0 |
| 286139018 | 1 | 0 | 0 |
| 286139389 | 1 | 0 | 0 |
| 286140541 | 1 | 0 | 0 |
| 286140742 | 1 | 0 | 0 |
| 286141194 | 1 | 0 | 0 |
| 286142437 | 1 | 0 | 0 |
| 286142468 | 1 | 0 | 0 |
| 286154610 | 1 | 0 | 0 |
| 286156045 | 1 | 1 | 0 |
| 286156172 | 1 | 0 | 0 |
| 286156791 | 1 | 1 | 0 |
| 286159492 | 1 | 1 | 0 |
| 286160643 | 1 | 0 | 0 |
| 286160680 | 1 | 0 | 0 |
| 286160765 | 1 | 1 | 0 |
| 286161801 | 1 | 1 | 0 |
| 286163531 | 1 | 1 | 0 |
| 286168856 | 1 | 0 | 0 |
| 286172226 | 1 | 0 | 0 |
| 286173063 | 1 | 0 | 0 |
| 286173183 | 1 | 0 | 1 |
| 286173188 | 1 | 0 | 0 |
| 286174439 | 1 | 0 | 1 |
| 286176192 | 1 | 0 | 0 |
| 286178367 | 1 | 1 | 0 |
| 286179363 | 1 | 1 | 0 |
| 286184457 | 1 | 0 | 1 |
| 286184513 | 1 | 0 | 0 |
| 286184562 | 1 | 0 | 0 |
| 286184762 | 1 | 0 | 0 |
| 286191542 | 1 | 1 | 0 |
| 286193725 | 1 | 0 | 0 |
| 286194278 | 1 | 0 | 0 |
| 286194311 | 1 | 0 | 0 |
| 286194338 | 1 | 0 | 0 |
| 286194393 | 1 | 0 | 0 |
| 286196752 | 1 | 0 | 0 |
| 286204566 | 1 | 0 | 1 |
| 286206688 | 1 | 1 | 0 |
| 286210014 | 1 | 0 | 0 |
| 286210090 | 1 | 0 | 0 |
| 286210183 | 1 | 0 | 0 |
| 286210973 | 1 | 0 | 0 |
| 286211530 | 1 | 0 | 0 |
| 286212873 | 1 | 0 | 0 |
| 286212907 | 1 | 0 | 0 |
| 286212941 | 1 | 0 | 0 |
| 286212976 | 1 | 0 | 0 |
| 286213390 | 1 | 0 | 0 |
| 286213465 | 1 | 0 | 1 |
| 286214477 | 1 | 0 | 0 |
| 286215467 | 1 | 0 | 1 |
| 286216205 | 1 | 0 | 0 |
| 286216997 | 1 | 1 | 0 |
| 286217258 | 1 | 0 | 1 |
| 286224369 | 1 | 0 | 0 |
| 286224524 | 1 | 0 | 0 |
| 286228138 | 1 | 0 | 0 |
| 286232250 | 1 | 0 | 0 |
| 286245563 | 1 | 1 | 0 |
| 286247052 | 1 | 0 | 0 |
| 286253193 | 1 | 1 | 0 |
| 286289352 | 1 | 0 | 0 |
| 286289588 | 1 | 1 | 0 |
| 286297061 | 1 | 1 | 0 |
| 286302499 | 1 | 1 | 0 |
| 286302896 | 1 | 1 | 0 |
| 286311842 | 1 | 0 | 1 |
| 286314335 | 1 | 1 | 0 |
| 286324352 | 1 | 0 | 1 |
| 286325409 | 1 | 1 | 0 |
| 286327294 | 1 | 0 | 0 |
| 286335037 | 1 | 0 | 1 |
| 286344779 | 1 | 0 | 1 |
| 286352029 | 1 | 0 | 1 |
| 286362353 | 1 | 0 | 0 |

