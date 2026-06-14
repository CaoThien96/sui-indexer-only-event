# Bot Sandwich Checkpoint Report

- Generated: 2026-06-13 02:18:27 UTC
- Bot: `0xf3981a28e88f86255713dada5d7b1ebb23b0b9e499e80fa1406bdd74c3364735`
- SwapEvent: `0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::SwapEvent`
- Ordering: `(transaction_sequence_in_checkpoint, event_sequence_in_transaction)`

## Summary

| Metric | Count |
|--------|------:|
| Checkpoints with bot swaps | 125 |
| Bot buys (atob=false) | 68 |
| Full sandwiches (buy→victim→sell) | 12 |
| Partial (buy→victim, no sell in cp) | 19 |
| Sandwich rate (full / bot buys) | 17.6% |
| Bot buy → victim sell (same/later cp) | 65 (95.6%) |
| — same checkpoint | 1 |
| — later checkpoint | 64 |
| Bot buys, no victim sell after | 3 |

## Bot buy → victim sell

After each **bot buy** (`atob=false`), first **victim sell** (`atob=true`, sender≠bot, same pool) in the same checkpoint or any later checkpoint.

### Pair 1

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+2 checkpoints)
  - **bot_buy** cp=286154610 `6jWXjRo4Lcvc7fw64YKfuP1TfuwBr2fiDD9okf3UexjK` (tx_idx=2, ev_idx=0, amount_in=11801616241)
  - **victim_sell** cp=286154612 `CQNqfhEuAr55CtzWBn3sTxvH6WWzP8dikeMKe6JDDnZm` (sender=`0x00006f748f809057fd1ca9ff8d02d89947f9079c26029ea2348657d8467b0000`, tx_idx=11, ev_idx=13, amount_in=2169000000000)

### Pair 2

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286156045 `2chm6DPeqeSZqbTaNfWcRKkjXeGpxgpPLcaPPXGcakeH` (tx_idx=6, ev_idx=0, amount_in=11872684216)
  - **victim_sell** cp=286156048 `8CaCP1iqabgoww5ndbrrFyVudU17QewC5PupmqZXBzfc` (sender=`0x89a1c807393670de16b055f0316232a5627b94bf74dfaa7ac34d3124109acf19`, tx_idx=22, ev_idx=3, amount_in=1390075189307)

### Pair 3

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+585 checkpoints)
  - **bot_buy** cp=286156172 `3za4G5d3FC6LGu94aYRPs4YwY67Ngmc62ixaV2S7yhKL` (tx_idx=18, ev_idx=0, amount_in=11872930115)
  - **victim_sell** cp=286156757 `78ES8T6GHwPiFJrLTQw24xmTP2Xzn1c1otbcRe5SWbn2` (sender=`0x725004a49296de37f77aa3d4a70bb14269d41aebaed4362ab6b8621e3b55d085`, tx_idx=2, ev_idx=0, amount_in=2849645339999)

### Pair 4

- **pool** `0xde265ef8645c680c71b33805de77ce5261a20c58397d83b3915bdbb3a7209d7e` (+2064 checkpoints)
  - **bot_buy** cp=286156791 `5o1dhyEZ5by8r8uhec5mLEdBCcq7p3XybdbpWz6tMkzh` (tx_idx=15, ev_idx=0, amount_in=28050165779)
  - **victim_sell** cp=286158855 `DTebiTyn3Nk28GLCEEzuix4xp4nLC6wxGkmc8V8BNcYq` (sender=`0xc7e16c8399a3218468cdda2ebedb3a038c3bd04590b8110c37756a3592b0c4c2`, tx_idx=18, ev_idx=0, amount_in=7914773716619)

### Pair 5

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+3 checkpoints)
  - **bot_buy** cp=286159492 `Ar2c9nsU29qV5BidDwVdGj6QombdQp9tSn168fbB29tT` (tx_idx=18, ev_idx=0, amount_in=25142648498)
  - **victim_sell** cp=286159495 `FnVJW35vrotc84Unq7y74qjVp88RrmhTEfZYDUGLjt9z` (sender=`0x5ca5872f9743c6624e8da0bece1b5da905bb4959ab8999082e239b2c833942d7`, tx_idx=17, ev_idx=1, amount_in=58755103251)

### Pair 6

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+40 checkpoints)
  - **bot_buy** cp=286160643 `5AkBfFFsqidP3K11hb5H6evjQofWQgicPy7i2CBRCpHo` (tx_idx=4, ev_idx=0, amount_in=12137375941)
  - **victim_sell** cp=286160683 `2uVqpqfNcbzLwPK7AjerwLSVMn47MGVC555mTVx5451J` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=27, ev_idx=7, amount_in=5025564589431)

### Pair 7

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286160680 `FGE9QNxRkMPt4fqhTWK2iTmtLsHy9WrixhffW8LwP3jR` (tx_idx=19, ev_idx=0, amount_in=2428086810)
  - **victim_sell** cp=286160683 `2uVqpqfNcbzLwPK7AjerwLSVMn47MGVC555mTVx5451J` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=27, ev_idx=7, amount_in=5025564589431)

### Pair 8

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286160765 `6nrnmcU8ZghyZHFHkP8H9VtXuajt2Uh9HCqiScbf4yoF` (tx_idx=17, ev_idx=0, amount_in=2433477983)
  - **victim_sell** cp=286160768 `56KfVh9c8aZ6AqwFyjUybr9XUAXHuWG2BxY7dVVooFMW` (sender=`0x89a1c807393670de16b055f0316232a5627b94bf74dfaa7ac34d3124109acf19`, tx_idx=17, ev_idx=3, amount_in=1436751146257)

### Pair 9

- **pool** `0xe79efa7b95f6920dfc46ab38d0fae7419113d19e40e84b41abf8ddf3fd287ae1` (+7235 checkpoints)
  - **bot_buy** cp=286161801 `EMEgTdxoM8fBw2eP3TaRsJH9vtTAkcKd7LV8KcdZ6qP6` (tx_idx=10, ev_idx=0, amount_in=12353309571)
  - **victim_sell** cp=286169036 `Akq1Jrm6obkv5UzF91TzSAnVXSaePnpdTLFba5wiYsWV` (sender=`0x7a6f34429afaf09469793b369fc9fad46bc0fc33036827ae955e424f37477e0a`, tx_idx=2, ev_idx=2, amount_in=1009412633484)

### Pair 10

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286163531 `W2dvYa1rwouBot67v2DoMPpAJAxFcyNfPQ3y98KHqwh` (tx_idx=8, ev_idx=0, amount_in=11986494683)
  - **victim_sell** cp=286163534 `HbdacRfXHnCnDstfZtSaCFZ36RWaj5eZbvK3vrsVfZrX` (sender=`0x457d5ecda9f125053adf94fe357549e11e6fa688679cf94ababb5793ed3916ce`, tx_idx=11, ev_idx=14, amount_in=9695100000000)

### Pair 11

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286168856 `9mUWj3oFRakE6xTa8G6m7PgEYkei1k2qW5UkWf4njVR8` (tx_idx=8, ev_idx=0, amount_in=11789003739)
  - **victim_sell** cp=286168859 `F9Dh6S7LD1oQMfqsq2CmnZK2ortDmiE11r3gdoya5kjA` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=20, ev_idx=7, amount_in=4504596141364)

### Pair 12

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286172226 `EmSnqpQpGEABr8TfdYGytvbuetbpf25y6SSjaeG37vNF` (tx_idx=23, ev_idx=0, amount_in=11873271568)
  - **victim_sell** cp=286172229 `77HJYYwAUPsbCU7Q15QwMokVrbTaVDi6ZNGRa7hN6Eaf` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=13, ev_idx=7, amount_in=3711974742414)

### Pair 13

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286173063 `Gs75RuHiBAJdZefsg6JkDhJtB9CGi7AMGow82Tufx3n8` (tx_idx=19, ev_idx=0, amount_in=25194154419)
  - **victim_sell** cp=286173066 `PS3oRw7oGcVm7JmBaseaJUd645sBFYbSzV82s5Bj3AF` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=28, ev_idx=7, amount_in=45670749288742)

### Pair 14

- **pool** `0x51e883ba7c0b566a26cbc8a94cd33eb0abd418a77cc1e60ad22fd9b1f29cd2ab` (+1 checkpoints)
  - **bot_buy** cp=286173183 `7SMHQVqMZihc3JNSDtAENNLQGuwZ4XBVAUXrqJnutxKs` (tx_idx=14, ev_idx=0, amount_in=140628009824)
  - **victim_sell** cp=286173184 `4iQ11otJRS3oCGzA3YuBWJeCYGbM19gu4ZDVE4LCieZo` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=1, ev_idx=14, amount_in=325545260)

### Pair 15

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286173188 `927amqTkYZNZkTHBgsrgoy5yTCtwtLs6MPPqVLMRogWn` (tx_idx=17, ev_idx=0, amount_in=25452464002)
  - **victim_sell** cp=286173191 `HpDwiqHRzujLgVthccscHvZAMNQiSvdsbegdQUbf11A2` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=6, ev_idx=7, amount_in=22298123907347)

### Pair 16

- **pool** `0x4edb54baafdf02a2219b6c327c9acacd79280b9c98cdc1b2b23230de906fa421` (+71 checkpoints)
  - **bot_buy** cp=286174439 `42ovVGn4ff2SnkFRrqU5XF38tWqZttKgg34oM8NHX6ur` (tx_idx=4, ev_idx=0, amount_in=23532204769)
  - **victim_sell** cp=286174510 `73ehX3akPWCfx2bkLF35mDyA1REnyYWptiXRsZaWx1J8` (sender=`0x283adcb1b65b85c3cf856b7ef2b3fbfd328377cd125c3177c729990b0cdbd701`, tx_idx=13, ev_idx=0, amount_in=1481673795079)

### Pair 17

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+7 checkpoints)
  - **bot_buy** cp=286176192 `651qbkkhX3ed7J6itEf1T4aACoJWpDV4qguMuLW2BDAr` (tx_idx=7, ev_idx=0, amount_in=143831379150)
  - **victim_sell** cp=286176199 `G2BUQYNwXBHoZmihKL3k2sCzh7Fc8bXtY9MPusb2HQJS` (sender=`0x00006f748f809057fd1ca9ff8d02d89947f9079c26029ea2348657d8467b0000`, tx_idx=13, ev_idx=13, amount_in=284000000)

### Pair 18

- **pool** `0x4edb54baafdf02a2219b6c327c9acacd79280b9c98cdc1b2b23230de906fa421` (+63 checkpoints)
  - **bot_buy** cp=286178367 `BC1eK8uK1eBFci6LaM2DQwrfRa3d8q2auc8ZyqWF8kZs` (tx_idx=14, ev_idx=0, amount_in=23463376263)
  - **victim_sell** cp=286178430 `4eYMswrDvXNRndKJMGdexza5Q6NbLmkN4TPxfhWmY9mG` (sender=`0x283adcb1b65b85c3cf856b7ef2b3fbfd328377cd125c3177c729990b0cdbd701`, tx_idx=6, ev_idx=0, amount_in=1610303681867)

### Pair 19

- **pool** `0x4edb54baafdf02a2219b6c327c9acacd79280b9c98cdc1b2b23230de906fa421` (+12 checkpoints)
  - **bot_buy** cp=286179363 `gQqH9M6rvHD55xfBDFYWnLZDanL2g6qbtnuNJ8aM4QJ` (tx_idx=12, ev_idx=0, amount_in=23430584261)
  - **victim_sell** cp=286179375 `Ds9tkc94bph4YLmHbGjMyGKjBbLoKbD6VQBRs7aiVdAV` (sender=`0x6893b30f86bf628824525d482d13ffd12c32af6163b20fa39486a81d3f7b8ea6`, tx_idx=13, ev_idx=0, amount_in=64442117547)

### Pair 20

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286184457 `6mdkBTFExJEiV8wXZMYpRLJGP8LSRUaej35UVN4C6uU5` (tx_idx=19, ev_idx=0, amount_in=11687432349)
  - **victim_sell** cp=286184460 `BiPm9nLoaGXhCkPeCoZsw2ndxZnivW8ZgNAtGTPv5u46` (sender=`0x8af2133a24d1097119305ec4262319ebd54e0e6473976a13e94bfe8f3341716f`, tx_idx=7, ev_idx=3, amount_in=1100497374710)

### Pair 21

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286184513 `2feKNcmeKLkdCj7SHq7icXp7zGyerE39xrX8hZ2Yhpep` (tx_idx=19, ev_idx=0, amount_in=11734615105)
  - **victim_sell** cp=286184516 `FWdewUyj2uZeHKjt5S4YypMphiXzT7gxMAjYtqAXLtwL` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=13, ev_idx=7, amount_in=9942093429199)

### Pair 22

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286184562 `2NNuchXJXmnztX7SrU8nN5VHv72XURETCah3nHt5Sr7P` (tx_idx=14, ev_idx=0, amount_in=11821711635)
  - **victim_sell** cp=286184565 `D7HCmCfY6zfDwe5WCnKqHnxD7vkfCc7icC4wWQdwmHVL` (sender=`0x5347b918a9cc46358da35e787758707a459929f0c0ff921810f0f64c2790e117`, tx_idx=10, ev_idx=7, amount_in=1951506043590)

### Pair 23

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286184762 `4gfLaK6cqkBLfqUb7PqGYJ4ZZD3eGxNdiwkYYEdCTQP5` (tx_idx=1, ev_idx=0, amount_in=11916955211)
  - **victim_sell** cp=286184765 `4GBPrd2aE8CF6yXt9iVirodh7nwRFsuFqExPFRz9UXCx` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=6, ev_idx=7, amount_in=5547458269893)

### Pair 24

- **pool** `0xf9107158e4945d6bbc321c7471e0b7c9854c2d3a1b04aaff6acaa50b8ea203d2` (+3036 checkpoints)
  - **bot_buy** cp=286191542 `Fq9RtYLj86skn2W8Z3RPxFw88NRXugN3YJw3LDBLtoob` (tx_idx=24, ev_idx=0, amount_in=1236510479)
  - **victim_sell** cp=286194578 `42k4BdB2zcRtenuf8rjyFR7FKRCzTswngACx4Zednj89` (sender=`0xd01d8a0692fecae09fbbadd4a61e72eb6f8d14ba305764657df809a063d0e182`, tx_idx=6, ev_idx=15, amount_in=7491614555)

### Pair 25

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286193725 `AXCoEhHf27a8gyxq9TjdVVxF6X9V4yZr5ZK21ekGPV6v` (tx_idx=17, ev_idx=0, amount_in=2284480741)
  - **victim_sell** cp=286193728 `8wJB7tMdVbcosUXt5gtMmYw9QktP6B7xQJGvZPPNPXKV` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=12, ev_idx=7, amount_in=5002717872916)

### Pair 26

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+2 checkpoints)
  - **bot_buy** cp=286194278 `7vdrQBhdtD2pMFihibE27HojFqK8xmy44gkWkSazow2m` (tx_idx=2, ev_idx=0, amount_in=11635112205)
  - **victim_sell** cp=286194280 `GNYu7EcGzBzRK88BUdXjRcF3qLHBzJkpjxh2s68zY2mD` (sender=`0xc15abe9518026b1c8b47357d0901690f5ee9e9fbb92b7fa69581e943d8d065cd`, tx_idx=15, ev_idx=1, amount_in=1059582106672)

### Pair 27

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286194311 `G6gFXqvyQFytQE8DRdAmQ7RLZMs4M3UWoK72ACfMzb7c` (tx_idx=5, ev_idx=0, amount_in=11687632142)
  - **victim_sell** cp=286194314 `Dv713QvU35syiibJ7hfFpZzNPesLpri1dn9vgmka4h5R` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=3, ev_idx=1, amount_in=9229250409555)

### Pair 28

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286194338 `3v6VD8t448ESmRxNGAc5oXXoB1BwHEPxZimZaGmtsjMS` (tx_idx=16, ev_idx=0, amount_in=11718467724)
  - **victim_sell** cp=286194341 `HxxcCEJ8gfBEWbtKBnF2pyupzEHuVppduaGCx46jc6X6` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=3, ev_idx=7, amount_in=5443642014188)

### Pair 29

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286194393 `EDDzTbk5hXDhNRYGJHYx1qAqQJKTWDomECArb5KeBzL5` (tx_idx=1, ev_idx=0, amount_in=11756334352)
  - **victim_sell** cp=286194396 `BW9sneU6BTzRPP3sNNWo4h9u8ovLivd5fyHo9JcGZwu2` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=2, ev_idx=7, amount_in=4677156119697)

### Pair 30

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286196752 `Aj2hoChWrWRWuv29kCdXCM1ibb1sDoiBmRKzsqsAUaww` (tx_idx=8, ev_idx=0, amount_in=2352711403)
  - **victim_sell** cp=286196755 `5HkLrKEwtmK4LWJnZwShYWUH5kL8fPng7fZJ9MaBx5c7` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=11, ev_idx=7, amount_in=3629950612205)

### Pair 31

- **pool** `0xf45b01f23e9951e37733b76c8cc7d22dcd23141aa246a86e17595a7aca610e1d` (+1 checkpoints)
  - **bot_buy** cp=286206688 `DSwgaDFUWe29YstwfHSGWsNXLq52rczoCmLFP7CuXtbh` (tx_idx=3, ev_idx=0, amount_in=48629247223)
  - **victim_sell** cp=286206689 `HtRgtcnb2fa6EewEpX97LraQwfXcKY8v6HQfFJW2vUVo` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=3, ev_idx=8, amount_in=23233514)

### Pair 32

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286210014 `BfWamCdbuk8muB4eudDYCLxeAR4R8PtEvHTbsrrH9kqo` (tx_idx=1, ev_idx=0, amount_in=10735892678)
  - **victim_sell** cp=286210017 `8qucG5Tib4x8YjmGLLXUA4EM94MCVMmrYhLLZFKHtq7Q` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=8, ev_idx=7, amount_in=35846190173078)

### Pair 33

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286210090 `CxQujqbhw4hvHPYFfTrVDovDqqCY77Jp4pRN22Pw86RQ` (tx_idx=12, ev_idx=0, amount_in=10779707627)
  - **victim_sell** cp=286210093 `9Dsv6UcAko71RUjamg3Kc3A2tZymrV3BWypTE7AKNhrx` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=9, ev_idx=7, amount_in=44079723459194)

### Pair 34

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286210183 `BC2zPym5kJvYey2LFZXdrgvzwApaSD7uY7RSuezHjNyp` (tx_idx=4, ev_idx=0, amount_in=11186663806)
  - **victim_sell** cp=286210186 `EhUGV1bKThKJw45332EMLYoneEuCkvU8gWKUvSiRzgWC` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=15, ev_idx=1, amount_in=16252445232749)

### Pair 35

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+25 checkpoints)
  - **bot_buy** cp=286210973 `Ex734rgTCWP3rQS1FzVYmekRTJFkFaeD2TiRrcdTHFNu` (tx_idx=11, ev_idx=0, amount_in=146114433880)
  - **victim_sell** cp=286210998 `8xrkmft7at8MREMFxNDJMr9JkVL6joHuW6H16h2QHyC9` (sender=`0x609b7f187082ccc9a1af38d060dc85cadf76be9fa11e1cc52ac4964a43a377bb`, tx_idx=2, ev_idx=4, amount_in=35350639)

### Pair 36

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286211530 `3BRaSjK62c7rCHf37rsRkMvvChmQzVqjWELJJrnaJ848` (tx_idx=7, ev_idx=0, amount_in=11495817034)
  - **victim_sell** cp=286211533 `35ZrPbKc1cSjaP562sFLtfxpHvd6KcWFEUeUkaN3Mfdc` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=7, ev_idx=7, amount_in=3776694755104)

### Pair 37

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286212873 `D37bwGy55WkUFfpaTjEfGKEtx1EDoJ3NvVobEQDKNjwZ` (tx_idx=11, ev_idx=0, amount_in=11628962839)
  - **victim_sell** cp=286212876 `BVphSbNfHzUogSZNdzYdGhkRFh8JrVMEeuswZ2Z8fJSr` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=8, ev_idx=7, amount_in=8422064996195)

### Pair 38

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286212907 `Eq8Z1ZSosftNt6Czu6ddsTQFSwUk3cZAvzcoqqFCgWzr` (tx_idx=12, ev_idx=0, amount_in=11645778901)
  - **victim_sell** cp=286212910 `BnX4emcRXPs5mcc46TLrQvcwvHVzduVvpehKbj4MEd2K` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=14, ev_idx=7, amount_in=7299882632466)

### Pair 39

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286212941 `GYzH5UbfWaDCm6NhFe3AP61Mn9DQUGnUYbK7uBbTj6Fq` (tx_idx=24, ev_idx=0, amount_in=11670738030)
  - **victim_sell** cp=286212944 `8PMWxK31RnNVAYsJ5LG9Ha7nEcCqygyxPCV7nBFVsHEF` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=19, ev_idx=1, amount_in=6147232660373)

### Pair 40

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+1 checkpoints)
  - **bot_buy** cp=286212976 `7AGqAzKTwbrjCJC1eeppARETVc4EuhUwo7fFJbqMrFj6` (tx_idx=4, ev_idx=0, amount_in=11670167209)
  - **victim_sell** cp=286212977 `G2iD6S8F1grd7s3qZZjKJJiTMySNo1TQnAtRBZ4Dy11V` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=2, ev_idx=1, amount_in=6742451316322)

### Pair 41

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (same checkpoint)
  - **bot_buy** cp=286213390 `ErZLeLgMe5Q4aWH5KTz2PC5FDvwhCXY2HZBDiNj9oyEY` (tx_idx=3, ev_idx=0, amount_in=146091137840)
  - **victim_sell** cp=286213390 `CR72adwc8oh8HJoQryuUV31EQFcBrKM5CcmpQeNwa7s2` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=7, ev_idx=7, amount_in=20829528190562)

### Pair 42

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+2 checkpoints)
  - **bot_buy** cp=286213465 `BWcBY4X9xBX2ZbD58XMSCTcAzX1yqZawpc53uFWRfuBV` (tx_idx=5, ev_idx=0, amount_in=2318143160)
  - **victim_sell** cp=286213467 `3kj4i7RkgPg3XKkkYJcBM6mK1FHKp8PjmsjsPw4DcTuL` (sender=`0x00000cfa6d94cff09b37a1a4dcfc92a993b61fffb71ce95ad9949e2f4cfdad26`, tx_idx=12, ev_idx=7, amount_in=3457564812934)

### Pair 43

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+4 checkpoints)
  - **bot_buy** cp=286214477 `ETSnRPjLBxy4fjMPTHWzXs1rscs8aBLWi6pxvzu4mEH3` (tx_idx=19, ev_idx=0, amount_in=2265103810)
  - **victim_sell** cp=286214481 `Bp4qk9XFdXVyneNVREdvnKc5Hu8MFY87fVVXnNMEtTw` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=2, ev_idx=7, amount_in=9740567126842)

### Pair 44

- **pool** `0x2d3230025b4615087656952bf5ddb49d7a9b6712ac9aa14977a877f02a16f165` (+4 checkpoints)
  - **bot_buy** cp=286215467 `5RN4YWjW6QeyHsKbr2T6QeQPKb9pNjSTuRR4tQrVNQNf` (tx_idx=4, ev_idx=0, amount_in=4317535893)
  - **victim_sell** cp=286215471 `94m7jeBw77QKVpL6zTMPNy9zzPxsZVQinsavE5LqNVR1` (sender=`0x609b7f187082ccc9a1af38d060dc85cadf76be9fa11e1cc52ac4964a43a377bb`, tx_idx=8, ev_idx=4, amount_in=6718338902)

### Pair 45

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+10 checkpoints)
  - **bot_buy** cp=286216205 `F7QygTx4w6zYDKm4nRJnGBZfsEuXUpG5UGD9QVUheG6m` (tx_idx=17, ev_idx=0, amount_in=138847535438)
  - **victim_sell** cp=286216215 `3FodxbDESp7rqoiq6Nt1z8icQ4tDviEdkvU8AdpFjabH` (sender=`0x827e8052c08056ae1a7ba7be78cca0abe56f076ead9c33f9bbf1aec9e37c6988`, tx_idx=93, ev_idx=3, amount_in=2241844992)

### Pair 46

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+3 checkpoints)
  - **bot_buy** cp=286216997 `DCqkcCbkQKETaYSjJ5U3xGa6PE1EsE9jBWJXS8njjBtq` (tx_idx=6, ev_idx=0, amount_in=139145694709)
  - **victim_sell** cp=286217000 `LPdftuVG6LvrGcbqSXthWQj6H7oreGoW5BzQo1qnGE5` (sender=`0x20c03434a59947780aa089a8aa1a2a71b5685f9e65f8a90902132f4008c93f0b`, tx_idx=6, ev_idx=12, amount_in=343000000)

### Pair 47

- **pool** `0x1de5cc16141c21923bfca33db9bb6c604de5760e4498e75ecdfcf80d62fb5818` (+7 checkpoints)
  - **bot_buy** cp=286217258 `CujPn1zaMZ1pdbwu7pB9YA61iiZkaPbRLepGqDCdWeqv` (tx_idx=13, ev_idx=0, amount_in=15353557442)
  - **victim_sell** cp=286217265 `6YDuaghka2ha4Lxedqvc5F6Rcd4NbFT92tujamhd6R9x` (sender=`0x609b7f187082ccc9a1af38d060dc85cadf76be9fa11e1cc52ac4964a43a377bb`, tx_idx=1, ev_idx=1, amount_in=926538237145)

### Pair 48

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+7 checkpoints)
  - **bot_buy** cp=286224369 `B1jJEthEjd7zjDon1V6z3bW5ZA5qobBaFsYEFgFXoJpe` (tx_idx=5, ev_idx=0, amount_in=141157972718)
  - **victim_sell** cp=286224376 `5JWu7eVC2z2v5y16erT4uwTm7ULthzyfdbxevBzuQ1wL` (sender=`0x00006f748f809057fd1ca9ff8d02d89947f9079c26029ea2348657d8467b0000`, tx_idx=45, ev_idx=17, amount_in=408000000)

### Pair 49

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+1 checkpoints)
  - **bot_buy** cp=286224524 `BmoNitG6pYsAuNNrYzjkVVxgsqjnjnPHht4vKhWrmsmT` (tx_idx=10, ev_idx=0, amount_in=141540613248)
  - **victim_sell** cp=286224525 `5AmhDWwBu1fFog1eew3yxZ2G4kYzkG7cdF4nhnyLUnsN` (sender=`0xfcd96a53f698f63541f1cfdfc84b5ad637ec6a03f7bb4877b7c9cf7dbc171905`, tx_idx=8, ev_idx=12, amount_in=25461683)

### Pair 50

- **pool** `0x5cf7e2ec9311d9057e43477a29bd457c51beeb1ddcd151c385a295dbb3c0fb18` (+3 checkpoints)
  - **bot_buy** cp=286228138 `5rwMDrhpMgMBGqPmuMiyy9JEiuqDyjrQCKsjjhy3zCJv` (tx_idx=10, ev_idx=0, amount_in=23052586916)
  - **victim_sell** cp=286228141 `HQWDgwVnUPBAzpuUogCyaMh4CAx3E9Dn7KX2KkkEccwK` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=11, ev_idx=1, amount_in=10949884569204)

### Pair 51

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+3 checkpoints)
  - **bot_buy** cp=286232250 `9DaoidxmPdPXVSUxpoaiG9KBLy1YYpcBRUy5ztSLA8Lk` (tx_idx=9, ev_idx=0, amount_in=147795376610)
  - **victim_sell** cp=286232253 `DLCSywr1pmV1mCbAfkQQ2q26RSbo4R6HbxyTUN1qFFkT` (sender=`0xbfd9fa076ac3dbc1dfeb28fa2ecaa6b500a25c098f2efc8f7ce84b8c6fe3dda2`, tx_idx=10, ev_idx=1, amount_in=12607600361287)

### Pair 52

- **pool** `0xaa2347159a55adaf1d76745e13c2bc91449570d998f6ba8ecbf5129a5d4a0bbf` (+31627 checkpoints)
  - **bot_buy** cp=286245563 `2cWymhoP2fekLsvmm3z6ZUiZ6Jpah5vwocQ1RSRwrUzR` (tx_idx=19, ev_idx=0, amount_in=857301236)
  - **victim_sell** cp=286277190 `GCc96ebxuQUUanzDusVnuTMqG7LAHjdGit7tgkkhGUm4` (sender=`0xd0f78fa93b1e39ab523eb41591c9e631672a0ad9dfd27813f8747de5b31461da`, tx_idx=8, ev_idx=0, amount_in=22093508688305531)

### Pair 53

- **pool** `0xaa2347159a55adaf1d76745e13c2bc91449570d998f6ba8ecbf5129a5d4a0bbf` (+30138 checkpoints)
  - **bot_buy** cp=286247052 `HtQtNpR4Q2SWq8zkp2CxeG9faoNUahkLz16BJFi5ze8e` (tx_idx=8, ev_idx=0, amount_in=474199933)
  - **victim_sell** cp=286277190 `GCc96ebxuQUUanzDusVnuTMqG7LAHjdGit7tgkkhGUm4` (sender=`0xd0f78fa93b1e39ab523eb41591c9e631672a0ad9dfd27813f8747de5b31461da`, tx_idx=8, ev_idx=0, amount_in=22093508688305531)

### Pair 54

- **pool** `0xaf414b3d3bc14b8c92d79947e84dac88db214f60e5e732165f9f25a13843996a` (+4480 checkpoints)
  - **bot_buy** cp=286253193 `4PiKUMLCAb4sXD9QwjcWi46Bz2Zz72HUMp6CnqFBgBew` (tx_idx=12, ev_idx=0, amount_in=5893209448)
  - **victim_sell** cp=286257673 `8ujLqSQvuDzzBJh4BF11q8xkdmYDzAR7nqhrsRazVYBR` (sender=`0x601a43f172b17ae92aaf08572e9c5087b92421919498268612e8a6a1b498a3e3`, tx_idx=2, ev_idx=2, amount_in=272037660)

### Pair 55

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+3 checkpoints)
  - **bot_buy** cp=286289352 `BgygMU3NoxhXB5u36HYoPY5QP2gnW6zhbH9U21LVQfym` (tx_idx=23, ev_idx=0, amount_in=150254922211)
  - **victim_sell** cp=286289355 `9Csj3ARf7h7TJxRQJQVc5f1YHPWFzF6wvy5N8AfaXx3T` (sender=`0x788a9ada3f7ee01cb93352878d84e68dce92a3ebcdd418f7dde34ccba262db6b`, tx_idx=19, ev_idx=4, amount_in=1455224434)

### Pair 56

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+4 checkpoints)
  - **bot_buy** cp=286289588 `5Hw3pbJKHxEgvTAUkiJEvEwwsCHrK2KeDYzuwHxeHw1w` (tx_idx=10, ev_idx=0, amount_in=150254922211)
  - **victim_sell** cp=286289592 `Ay5SNTdgx8WtYt15JExtpma97qSf6ugNfkLYDVDVssF5` (sender=`0x6cae00a08b04f6a4ca7157628ccf60f40078616deab20d2b626bd1de7c8a16c9`, tx_idx=30, ev_idx=7, amount_in=1973754296)

### Pair 57

- **pool** `0x008c0a882b65d966862a47a2b1de308c42be621080cb623543638b6920fd505d` (+49720 checkpoints)
  - **bot_buy** cp=286297061 `EiHfSqAZg7pf4XurAu6JPjpu87ncwLwCN2WjzaWsfCka` (tx_idx=12, ev_idx=0, amount_in=19879523693)
  - **victim_sell** cp=286346781 `25TyBfqN9pttcKnsXaVg5iaA6hWTCxZJY7VweoDYHYqW` (sender=`0xc389831335a36515f56c4de366c202730eee6062f0cacb30146431f1e0712894`, tx_idx=7, ev_idx=0, amount_in=1798699298732098)

### Pair 58

- **pool** `0x2f47d887c4ca1640c48946676dc3ccb40025cdb0aa52f21d6b043568a7c39ffe` (+3 checkpoints)
  - **bot_buy** cp=286302499 `EGYS8yprmjagCCxpiZLbfndwwksG437VVkBSkBQoskVF` (tx_idx=7, ev_idx=0, amount_in=9752294652)
  - **victim_sell** cp=286302502 `CFEd1wx8ZRzNdo6AchoaYUMHKxCaq1AAz8mj8SD9NnEp` (sender=`0x5347b918a9cc46358da35e787758707a459929f0c0ff921810f0f64c2790e117`, tx_idx=4, ev_idx=2, amount_in=26160740284882)

### Pair 59

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+3 checkpoints)
  - **bot_buy** cp=286302896 `G1GfuWrmCMhSB6Aqq4xfgP1JVQABuXsUvjKRNgX9G23F` (tx_idx=21, ev_idx=0, amount_in=36362451575)
  - **victim_sell** cp=286302899 `5JPLaawHYM6EgiE94ND7j6G3QiLrM5Son1JMYSGQaPrz` (sender=`0x2896eec01861e1682730c37457daf4a222217dec9d9ab545f0ccccb643b88e0c`, tx_idx=6, ev_idx=2, amount_in=70453343739)

### Pair 60

- **pool** `0x2d3230025b4615087656952bf5ddb49d7a9b6712ac9aa14977a877f02a16f165` (+3 checkpoints)
  - **bot_buy** cp=286311842 `CQ7SoqP42bUrczYbvrrJXE8dVzVL5BRXU1hKxMAzF5Rq` (tx_idx=1, ev_idx=0, amount_in=4295672261)
  - **victim_sell** cp=286311845 `CNgjstdn4e5jq7x6vCSpNMNg25tKcE4oM49eV1JdA2hN` (sender=`0x89a1c807393670de16b055f0316232a5627b94bf74dfaa7ac34d3124109acf19`, tx_idx=40, ev_idx=2, amount_in=19449318912)

### Pair 61

- **pool** `0xf4238fa592c9ed7f148fd091cb2c4147cb15ad81b797115ce42971923ebf6e4c` (+155 checkpoints)
  - **bot_buy** cp=286314335 `8BkA8X2FJibxJnCHd2vgBUHySfxQWwAsxCMLDBU6zfw9` (tx_idx=19, ev_idx=0, amount_in=14378022914)
  - **victim_sell** cp=286314490 `8KtVX5W33StX3hhEEzTLsy3ifi7wFiGja6iFx7mx8fKu` (sender=`0x9cad98bde3e40d10fec68a6d6de179f53b2fcdce339519db9599fb8fe2b7f6c2`, tx_idx=19, ev_idx=8, amount_in=196724143093)

### Pair 62

- **pool** `0x9661cca01a5b9b3536883568fa967a2943e237de11a97976795f5adb293892e9` (+520 checkpoints)
  - **bot_buy** cp=286324352 `32Sh3gsom43QFg3uvVd38GCx5yTC9nqX4eiK7yPwYnDx` (tx_idx=1, ev_idx=0, amount_in=12244803219)
  - **victim_sell** cp=286324872 `82UTYPieZmqGLHtAzaaeKZNbFwCy5TgtUfx3jxEUuyqU` (sender=`0xd6ed19aac25ee4986feb1bd0c1ee988b7226d39634092b6b898f2f87a015d216`, tx_idx=3, ev_idx=1, amount_in=101622377010)

### Pair 63

- **pool** `0x0254747f5ca059a1972cd7f6016485d51392a3fde608107b93bbaebea550f703` (+3 checkpoints)
  - **bot_buy** cp=286325409 `8pJLW2eowiLQiQV6odBFFKirmoSzfLjJV8AHXhoYgZo9` (tx_idx=1, ev_idx=0, amount_in=152890288712)
  - **victim_sell** cp=286325412 `EsZFcxwjwGrK6XYYZnJ3eKY7tDvy1HbS85CCAcGhCNhr` (sender=`0xa8a6670d32e66762b8ee6d66f57aa847f718551099752a87cfa4ee7058e9b392`, tx_idx=1, ev_idx=1, amount_in=668072992364)

### Pair 64

- **pool** `0x51e883ba7c0b566a26cbc8a94cd33eb0abd418a77cc1e60ad22fd9b1f29cd2ab` (+4 checkpoints)
  - **bot_buy** cp=286327294 `9GCqLXusc25JDcUnruh71wKcx4Safqvc5nTs1dh5KxKE` (tx_idx=9, ev_idx=0, amount_in=152941668740)
  - **victim_sell** cp=286327298 `CNoPJahM3h5VRrFy87zC9ad25WHs2Jo5bc937j1jyk4y` (sender=`0xd265672730b0540ffd3569530682a0f02ef984b703457790554eb0e19329663a`, tx_idx=3, ev_idx=16, amount_in=461840821)

### Pair 65

- **pool** `0xd978d331772a5b90d5a4781e1232d18afd12019d0c35db79e3674beeda8f9126` (+4 checkpoints)
  - **bot_buy** cp=286335037 `G5wq3bsni45Uemq8ZdEFBuV5g67Yv4khH6qHEgKpvKe9` (tx_idx=4, ev_idx=0, amount_in=152931694104)
  - **victim_sell** cp=286335041 `3dFfPchKmaxbniQNg1aPs3DiPY5NK3vUeuSq2WzpqXfk` (sender=`0xa8a6670d32e66762b8ee6d66f57aa847f718551099752a87cfa4ee7058e9b392`, tx_idx=2, ev_idx=1, amount_in=268487998)

## Full sandwiches

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

