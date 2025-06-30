# cantree

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/l/cantree)](LICENSE)
[![Build](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Maintenance](https://img.shields.io/badge/maintained-yes-blue)]()

## Introduction
**cantree** is a simple binary search tree implementation in Rust that uniquely identifies each node using the Cantor pairing function. The tree stores `OrderedPair` values.

The project supports classic BST operations like:
- Insertion
- Search
- Removal
- Visual representation (ASCII tree with optional color)

## Authors
This project was developed by **José Carlos da Paz Silva** (<carlos.paz.707@ufrn.edu.br>) as part of the _Estrutura de Dados Básicas II_ course at **UFRN**.

## Directory structure
The source code is organized into logical modules for clarity and modularity:

```
src/
├── tree/          # Tree data structures
│ ├── bst.rs       # BinarySearchTree implementation
│ └── node.rs      # Node definition
├── utils/         # Utility components
│ ├── cantor.rs    # Cantor pairing function for key generation
│ └── pair.rs      # OrderedPair struct definition
├── lib.rs         # Library entry point
└── main.rs        # Executable for testing the tree
```

## Compiling and Running
To compile and run **cantree**, you must have **Rust** and **Cargo** installed.  
If you haven't already, install them via [https://rustup.rs](https://rustup.rs):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Once Rust is set up, follow the steps:

1. Clone this repository:

```bash
git clone https://github.com/carlos-paz12/cantree.git
```

2. Navigate to the project directory:

```bash
cd cantree
```

3. Compile and build the program using `cargo`:

```bash
cargo build
```

4. Run:

```bash
cargo run
```

You should see a sequence of test cases executed in the terminal, showcasing insertions, deletions, and tree state visualizations.

---

&copy; DIMAp | Departamento de Informática e Matemática Aplicada - 2025
