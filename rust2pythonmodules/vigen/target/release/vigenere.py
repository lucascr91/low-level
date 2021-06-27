LETTERS = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'
 
def translateMessage(key, message, mode):
    translated = []

    keyIndex = 0
    key = key.upper()

    for symbol in message:
        num = LETTERS.find(symbol.upper())
        if num != -1:
            if mode == 'encrypt':
                num += LETTERS.find(key[keyIndex])
            elif mode == 'decrypt':
                num -= LETTERS.find(key[keyIndex])
            num %= len(LETTERS)

            if symbol.isupper():
                translated.append(LETTERS[num])
            elif symbol.islower():
                translated.append(LETTERS[num].lower())

            keyIndex += 1
            if keyIndex == len(key):
                keyIndex = 0
        else:
            translated.append(symbol)

    return ''.join(translated)

# myKey = 'PIZZA'
# myMessage = "We do not learn, and that what we call learning is only a process of recollection."

# myMode = 'encrypt' # Set to either 'encrypt' or 'decrypt'.

# translated = translateMessage(myKey, myMessage, myMode)
# print(translated)