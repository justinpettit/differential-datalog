# Differential Datalog (DDlog)

DDlog is a *bottom-up*, *incremental*, *in-memory*, *typed* Datalog engine for writing
*application-integrated* deductive database engines.

1. **Bottom-up**: DDlog starts from a set of *ground facts* (i.e., facts provided by the user) and
computes *all* possible derived facts by following Datalog rules, in a bottom-up fashion.  In
contrast, top-down engines are optimized to answer individual user queries without computing all
possible facts ahead of time.  For example, given a Datalog program that computes pairs of connected
vertices in a graph, a bottom-up engine maintains the set of all such pairs.  A top-down engine, on
the other hand, is triggered by a user query to determine whether a pair of vertices is connected
and handles the query by searching for a derivation chain back to ground facts.  The bottom-up
approach is preferable in applications where all derived facts must be computed ahead of time and in
applications where the cost of initial computation is amortized across a large number of queries.

2. **Incremental**: whenever the set of ground facts changes, DDlog only performs the minimum computation
necessary to compute all changes in the derived facts.  This has significant performance benefits for many queries.

3. **In-memory**: DDlog stores and processes data in memory.  In a typical use case, a DDlog program
is used in conjunction with a persistent database, with database records being fed to DDlog as
ground facts and the derived facts computed by DDlog being written back to the database.

    At the moment, DDlog can only operate on databases that completely fit the memory of a single
    machine. (This may change in the future, as DDlog builds on the differential dataflow library that
    supports distributed computation over partitioned data).

4. **Typed**: Although Datalog is a programming language, in its classical textbook form it
is more of a mathematical formalism than a practical tool for programmers.  In particular, pure
Datalog does not have concepts like data types, arithmetics, strings or functions.  To facilitate
writing of safe, clear, and concise code, DDlog extends pure Datalog with:

    1. A powerful type system, including Booleans, unlimited precision integers, bitvectors, strings,
    tuples, and tagged unions.

    2. Standard integer and bitvector arithmetic.

    3. A simple procedural language that allows expressing many computations natively in DDlog without
resorting to external functions.

    4. String operations, including string concatenation and interpolation.

5. **Integrated**: while DDlog programs can be run interactively via a command line interface, its
primary use case is to integrate with other applications that require deductive database
functionality.  A DDlog program is compiled into a Rust library that can be linked against a Rust or
C/C++ program (bindings for other languages can be easily added, but Rust and C are the only ones we
support at the moment).  This enables good performance, but somewhat limits the flexibility, as
changes to the relational schema or rules require re-compilation.

Follow the [tutorial](doc/tutorial/tutorial.md) for a step-by-step introduction to DDlog.

DDlog language can be found [here](doc/language_reference/language_reference.md).

Instructions for writing and testing your own Datalog programs are [here](doc/testing/testing.md).

# Installing from sources

## Prerequisites
We have only tested our software on Ubuntu Linux.

The compilers are written in Haskell.  One needs the Glasgow Haskell
Compiler.  The Haskell compiler is managed by the
[stack](https://github.com/commercialhaskell/stack) tool.

To download stack (if you don't already have it) use:

```
wget -qO- https://get.haskellstack.org/ | sh
```

You will also need to install the Rust toolchain:

```
curl https://sh.rustup.rs -sSf | sh
```

**Note:** The `rustup` script adds path to Rust toolchain binaries (typically, `$HOME/.cargo/bin`)
to `~/.profile`, so that it becomes effective at the next login attempt.  To configure your current
shell run `source $HOME/.cargo/env`.

## Building

To build the software execute the following command anywhere inside the project directory tree:

```
stack build
```

To run the tests execute:

```
stack test
```

**Note:** Running tests for the first time will take a while (~30 minutes on my system).

To install DDlog binaries in Haskell stack's default binary directory:

```
stack install
```

To install to a different location:

```
stack install --local-bin-path <custom_path>
```

# vim syntax highlighting

Create a symlink to `tools/dl.vim` from the `~/.vim/syntax/` directory to enable differential 
datalog syntax highlighting in `.dl` files.

# Using IntelliJ IDEA

You can download and install the community edition of IntelliJ IDEA
from https://www.jetbrains.com/idea/download.

Start IntelliJ.

Install the IntelliJ-Hakell plugin: (File/Settings/Plugins -- search).

Follow the instructions here: https://github.com/rikvdkleij/intellij-haskell to
get started with the plugin.

# Debugging with GHCi

To run the test suite with the GHCi debugger:

```
stack ghci --ghci-options -isrc --ghci-options -itest differential-datalog:differential-datalog-test
```

and type `do main` in the command prompt.

# Building with profiling info enabled

```
stack clean
```

followed by

```
stack build --profile
```

or

```
stack test --profile
```
