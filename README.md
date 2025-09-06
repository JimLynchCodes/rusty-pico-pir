# rusty-pico-pir

A few pir motion sensor examples for the pi pico in Rust


### Many Projects

_Note: Each subfolder here is an independent project._

The numbers in the project folder names help sort them in a recommended progression of more complex code and projects.

<br/>

### Embedded Rust CLI Setup

Make sure to add the pi pico ARM Cortex-M build target to your rust toolchain:
```
rustup target add thumbv6m-none-eabi
```

Optional but recommended: install probe-rs
for flashing/debugging.
```
cargo install probe-rs-tools --locked
```

<br/>

### Using The Example Projects

1) Cd into the project directory

2) Make a build

3) Wire up your hardware

4) Flash binary onto your device

<br/>

## Creating New Example Projects

To create new projects, follow these steps:

#### 1) Create a new Rust binary proejct with cargo
```
cargo new --bin X_blah_pico_pir
cd X_blah_pico_pir
```

#### 2) Setup the Cargo.toml file
Add dependencies as needed for your project.

#### 3) Add a .cago/config.toml file
Add this file to tell cargo to use the Cortex-M target

#### 4) Add Code
Add the code to main.rs (and other files, if needed).

#### 5) Add Wiring Diagram
Explain how to everything should be wired up.

