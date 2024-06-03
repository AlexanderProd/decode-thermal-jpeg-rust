from libjpeg import decode

if __name__ == "__main__":
    with open("./exiftool_output.bin", 'rb') as file:
        file_content = file.read()

    decoded = decode(file_content)
    print(decoded)
    print("Shape: ", decoded.shape)