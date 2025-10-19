# Subterfuge Calculator

A Rust-based probability calculator for the world famous Subterfuge board game.  
Simulates battles between units or heroes (using d6 and d20 rolls), with optional modifiers for special rules like *fort* and *ark*.

---

## Installation (Windows + Git Bash)

1. **Install Rust**  
   ```bash
   Invoke-WebRequest https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
   ./rustup-init.exe
   ```

2. **Use the GNU toolchain** (recommended for Git Bash):  
   ```bash
   rustup default stable-x86_64-pc-windows-gnu
   ```

3. **Add Cargo to your PATH** if needed:  
   ```bash
   echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

4. **Verify setup**:  
   ```bash
   cargo --version
   rustc --version
   ```


---

## Build & Run

From the repo root:

```bash
cargo build
cargo run -- 10 10
```

Or run the compiled binary directly:
```bash
./target/debug/subterfuge_cal.exe 10 10
```

---

## Usage

```
USAGE:
  subterfuge_cal [FLAGS] [OPTIONS] <attacker> <defender>

FLAGS:
  -f, --fort         Apply fort bonus
  -a, --ark          Apply ark rule
  -l, --log          Print detailed combat log
  -m, --multi        Run multi-combat simulation

OPTIONS:
  -s, --simulations <N>    Number of Monte Carlo simulations (default: 100000)

ARGS:
  <attacker>  "N" for units or "A,H" for hero (attack,health)
  <defender>  "N" for units or "A,H" for hero (attack,health)
```

---

## Examples

Unit vs unit (10 vs 10):
```bash
./target/debug/subterfuge_cal.exe 10 10
# → attacker has 56.84% chance of winning
```

Hero vs units (10 attack, 3 health vs 10 units):
```bash
./target/debug/subterfuge_cal.exe 10,3 10
# → attacker has 16.09% chance of winning
```

With modifiers:
```bash
cargo run -- --fort --ark --simulations 200000 --log 10,3 10
```

Show help:
```bash
cargo run -- --help
```

---

## Troubleshooting

**Cargo not found:**  
Ensure Rust is installed and `$HOME/.cargo/bin` is in your PATH.

**Linker errors (`link.exe`):**  
Git Bash conflicts with MSVC linker. Fix by switching toolchain:  
```bash
rustup default stable-x86_64-pc-windows-gnu
```
