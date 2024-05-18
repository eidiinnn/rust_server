import os
import shutil

public_location = "./static"
build_location = "target/release/static"

def movePublic():
   print(f"moving the public files to build location")
   shutil.move(public_location, build_location)
   print("public moved!")

def main():
   print("starting...")
   movePublic()

if __name__ == "__main__":
   main()
   print("start building")
   os.system("cargo build --release")