import os
import shutil

dirpath = os.path.dirname(os.path.realpath(__file__))
static_location = os.path.join(dirpath, "static")
build_location = os.path.join(dirpath, "target/release/static")

def movePublic():
   print("moving the static files to build location")
   if os.path.exists(build_location):
      shutil.rmtree(build_location)
   shutil.copytree(static_location, build_location)
   print("public moved!")

def main():
   print("starting...")
   os.system("cargo build --release")
   movePublic()

if __name__ == "__main__":
   main()
  