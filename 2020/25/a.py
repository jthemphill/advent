import sys
import typing as ty

card_pub = int(next(sys.stdin))
door_pub = int(next(sys.stdin))

def get_loop_size(pub: int):
    val = 1
    loop_size = 0
    while val != pub:
        val *= 7
        val %= 20201227
        loop_size += 1
    return loop_size

card_loop = get_loop_size(card_pub)

val = 1
for _ in range(card_loop):
    val *= door_pub
    val %= 20201227

print(f"transform({door_pub}, {card_loop}) = {val}")