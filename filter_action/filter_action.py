input = ["Zebra", "nice camel", "Horse"]

is_uppercase = True
make_uppercase = True

filter1 = lambda s: s[0] == "H"
filter2 = lambda s: s[0].isupper()

for f in [filter1, filter2]:
    input = [x for x in input if f(x)]

print(input)