# 🦀 Logfy

Logfy is a lightweight and minimal logging crate for Rust, designed to be simple, fast, and easy to integrate into small to medium-sized projects.

It focuses on clarity over complexity and avoids unnecessary abstractions.

## 📌 Scope

Logfy is designed to be simple and unopinionated. It does not provide log filtering, configuration files, or async logging.

## 📦 Installation

Since this crate is not published on [crates.io](https://crates.io/), you can run the following command to add it into your project:

```bash
cargo add logfy --git https://github.com/joaovhmota/logfy.git --branch main
```

## 🖥️ Usage 

> [!WARNING]
> DEBUG logs will not be printed when running in RELEASE mode.

There are currently five macros available for logging:

```rust
use logfy::{critical, debug, error, information, warning};

fn main() {    
    debug!("This is a DEBUG message");
    information!("This is an INFORMATION message");
    warning!("This is a WARNING message");
    error!("This is an ERROR message");
    critical!("And this is a CRITICAL message");
}

// Expected output:
// [03/03/2026 21:59:25] DEBUG This is a DEBUG message
// [03/03/2026 21:59:25] INFORMATION This is an INFORMATION message
// [03/03/2026 21:59:25] WARNING This is a WARNING message
// [03/03/2026 21:59:25] ERROR This is an ERROR message
// [03/03/2026 21:59:25] CRITICAL And this is a CRITICAL message
```

You can also format the message like you would in `println!` as long as the value implements the `Display` trait:

```rust
use logfy::{critical, debug, error, information, warning};

fn main() {  
    let meaning_of_life = 42;
      
    debug!("The meaning of life is {meaning_of_life}");
    information!("The meaning of life is {meaning_of_life}");
    warning!("The meaning of life is {meaning_of_life}");
    error!("The meaning of life is {meaning_of_life}");
    critical!("The meaning of life is {meaning_of_life}");
}

// Expected output:
// [03/03/2026 22:02:31] DEBUG The meaning of life is 42
// [03/03/2026 22:02:31] INFORMATION The meaning of life is 42
// [03/03/2026 22:02:31] WARNING The meaning of life is 42
// [03/03/2026 22:02:31] ERROR The meaning of life is 42
// [03/03/2026 22:02:31] CRITICAL The meaning of life is 42
```

> [!NOTE]
> Logfy automatically colorizes log levels when the terminal supports ANSI colors.
