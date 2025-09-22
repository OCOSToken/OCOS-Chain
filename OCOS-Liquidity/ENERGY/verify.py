#!/usr/bin/env python3
"""
verify.py
Verilən BTC ünvanlarının on-chain balansını Blockstream API vasitəsilə götürür.
İstifadə: python3 verify.py
Asılılıqlar: requests
"""

import requests
import time

ADDRESSES = [
    "bc1q8t9n02d0u8dg3lrp6vu4d2mdc5gkvntzjky6hv2w0tz532dkf23qanrha8",
    "bc1q550ewd2kzfn6rd0yctsthqe9hp3fggcxrl4nad522fq4luasngqqknrxd7",
    "1PPcoaWYzSupVkibtD3TwViyA1ewCnJQDe",
    "bc1qhglqlqmr7gz8shxrlnw3a4weukjknrtxyn39k7tyzr64dsqsfchs043e34",
    "bc1q00du3cfdkxzds7ra92x8znwackfjwyqm8h67f5mnu3fdjkn6vkkquewrqa",
    "bc1qrzxwa7jlyf7he0dzcgyp4wz9qyzc7j7jlh3e02q6w33p9srh3cuqrr6vyh",
    "bc1qyth7d84cmw7xgw0qf3027mqqz8q8j5rptejwt5cx5h5rxhng54gqn7a04k",
    "bc1q7rnpap3j7nvahwxh2xjqtmuw0e0f8k05ltytys7lhsjsk6ye5p4sxl9s2v"
]

API_BASE = "https://blockstream.info/api"

def get_chain_stats(address):
    url = f"{API_BASE}/address/{address}"
    r = requests.get(url, timeout=20)
    r.raise_for_status()
    return r.json().get("chain_stats", {})

def sats_to_btc(sats):
    return sats / 1e8

def main():
    total_sats = 0
    print("ENERGY token backing proof — on-chain verify\n")
    for addr in ADDRESSES:
        try:
            stats = get_chain_stats(addr)
            funded = stats.get("funded_txo_sum", 0)
            spent = stats.get("spent_txo_sum", 0)
            balance_sats = funded - spent
            total_sats += balance_sats
            print(f"Address: {addr}")
            print(f"  Funded (sats): {funded:,}")
            print(f"  Spent  (sats): {spent:,}")
            print(f"  Balance(sats): {balance_sats:,}  | BTC: {sats_to_btc(balance_sats):.8f}")
            print("-" * 60)
            time.sleep(0.5)  # yumşaq rate-limit
        except Exception as e:
            print(f"Failed to fetch {addr}: {e}")
            print("-" * 60)

    print(f"TOTAL BALANCE (sats): {total_sats:,}")
    print(f"TOTAL BALANCE (BTC) : {sats_to_btc(total_sats):.8f}")

if __name__ == "__main__":
    main()
