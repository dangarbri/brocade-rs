# Brocade Rust
This module provides an interface for looking up products from https://www.brocade.io/

## Usage
Add this module to your dependencies
```
[dependencies]
brocade = "0.1.0"
```

Then in code, use
```
brocade::instance().get("1234567") // Get product info from barcode
```

# Design
I personally feel that reqwests is a little bloated, but it's the most popular request library and I used it.
This lib is written so that in the future a different reqwest library may be used by specifying a feature flag.