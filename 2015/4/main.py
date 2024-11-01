import hashlib

input = "yzbqklnj"
secret_key = 0
hash = hashlib.md5(f"{input}{secret_key}".encode())

print(f"{hash.hexdigest()} - {secret_key}")

while True:
    secret_key += 1
    str = f"{input}{secret_key}"
    hash = hashlib.md5(f"{input}{secret_key}".encode())
    print(f"{hash.hexdigest()} - {secret_key}")

print(hash.hexdigest())
