alphabet = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u",
            "v", "w", "x", "y", "z",]


def encrypt(message, shift):
    encrypted_message = ""
    for letter in message.lower():
        index = alphabet.index(letter)
        encrypted_message += alphabet[index + shift]
    print(encrypted_message)


def decrypt(message, shift):
    encrypted_message = ""
    for letter in message.lower():
        index = alphabet.index(letter)
        encrypted_message += alphabet[index - shift]
    print(encrypted_message)


first_message=str(input("Type E to encrpt a message or D to decrypt a message"))

if first_message.lower() == 'e':
    user_message = str(input("Enter Message to be encrypted"))
    user_shift = int(input("Enter Shift number"))
    encrypt(user_message, user_shift)

elif first_message.lower() == 'd':
    try:
        user_message = str(input("Enter Message to be decrypted"))
        user_shift = int(input("Enter Shift number"))
        decrypt(user_message, user_shift)
    except ValueError:
        print("input not valid ")
else:
    print("input not valid ")