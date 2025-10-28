# Gauss-Jordan Rust NxN Solver

## NxN System of Linear Equations Calculator made in Rust

This project is a calculator for Systems of Linear equations consisting of N equations and N variables, where N is any natural number chosen by the user. The calculator allows the user to input the desired N number, and then is prompted for each of the coefficients for each variable, and the constant term, on each of the equations that will form the system. Once the matrix is determined, the calculator performs a series of transformations that follow the Gauss-Jordan elimination method. Once the matrix is solved, it will display the results to the user.

The results can vary depending on the system, with the posibilities being:

* Unique solution, so the the calculator shows the value for each of the N variables on the system.

* Parameterized solutions, on which case it will show the resulting equations that express the poisble solutions.

* The system has no solution, on which case the calculator will display a message informing on this result.

## How to install and use this software

To run this project, you must have the Rust toolchain installed. This includes the Rust compiler (rustc) and the package manager (cargo).
The easiest way to install is using rustup, the official installer.

* to install rustup run the following command on your terminal :
```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
```
If you are on Windows, you can download the installer directly from the official website or use the Windows Subsystem for Linux (WSL).

* Configure your current shell to recognize the Rust environment:
```bash
source $HOME/.cargo/env
```

* Verify the installation:
```bash
rustc --version
cargo --version
```

* Clone the repository:
```bash
git clone https://github.com/manuelenrq9/rust-nxn-solver.git
```

* Move to the project location and run the calculator using Cargo:
```bash
cargo run
```
