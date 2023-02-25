stock = []
with open('stockfish.txt') as f:
    for line in f:
        stock.append(line.split()) 
f.close()


linkus = []
with open('linkus.txt') as f:
    for line in f:
        linkus.append(line.split()) 
f.close()


for i, e in enumerate(linkus):
    if e not in stock:
        print(f"{linkus[i][0]} {linkus[i][1]}")

print("\n\n\n")

for i, e in enumerate(stock):
    if e not in linkus:
        print(f"{stock[i][0]} {stock[i][1]}")
