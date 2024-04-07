pub const HOME_HTML: &str = "<!DOCTYPE html>
<html lang=\"en\">
<head>
   <meta charset=\"UTF-8\">
   <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
   <title>Rust Server</title>
</head>
<body>
   Hello World
</body>
</html>";
pub fn get_home() -> String {HOME_HTML.to_string()} 