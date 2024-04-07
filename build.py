import os

def createInline():
   print("creating inline.rs")
   inline_location = "src/inline.rs"

   if os.path.exists(inline_location):
      os.remove(inline_location)

   home_html = open("web/home.html")
   inline_rs = open(inline_location, "x")

   inline_rs.write(f"pub const HOME_HTML: &str = \"{home_html.read().replace("\"", "\\\"")}\";")
   inline_rs.write("\n")
   inline_rs.write("pub fn get_home() -> String {HOME_HTML.to_string()} ")
   inline_rs.close()
   print("inline.rs created!")

def main():
   print("starting...")
   createInline()

if __name__ == "__main__":
   main()
   print("start building")
   os.system("cargo build --release --features inline")