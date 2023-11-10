def convert(last_mov: int, current_mov: int) -> int:
    return (((last_mov//3)*3+3)%18 + current_mov) % 18


k = 4
for i in range(15):
    print(k, i, convert(k, i))