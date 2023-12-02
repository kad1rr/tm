# tm - Rust CLI Tool

A simple Rust Command Line Interface (CLI) tool named "tm" for executing commands and getting information about elapsed time.

## Table of Contents

- [Description](#description)
- [Features](#features)
- [Getting Started](#getting-started)
- [Usage](#usage)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

## Description

The "tm" Rust CLI tool allows you to execute commands through the command line. It handles both interactive and non-interactive modes, providing flexibility in usage.

## Features

- Interactive and non-interactive modes
- Cross-platform support (Windows and Unix-like systems)
- Command execution with elapsed time tracking
- Simple and easy to use

## Getting Started

### Prerequisites

- Rust installed on your system

### Installation

Clone the repository:

```bash
git clone https://github.com/kad1rr/tm.git
cd tm
```

Build the project:

```bash
cargo build --release
```

then, add bin path to env PATH
```bash
set PATH=c:\users\admin\my\bin\path\;%PATH%
```

like: 
```bash
set PATH=c:\users\admin\projects\tm\target\release\;%PATH%
```

> Or just copy the created bin file to a path in the PATH env.

## Usage
Run the CLI tool with or without command-line arguments:

```bash
# interactive mode
$: tm
>
echo Hello World
Successfully completed!
Hello World

Command Elapsed Time: 99.4µs
TM Elapsed Time: 128.6µs
```

```bash
# non-interactive mode
$: tm echo Hello World
Successfully completed!
Hello World

Command Elapsed Time: 97.7µs
TM Elapsed Time: 124.9µs
```

## Contributing

Contributions are welcome! If you have any suggestions, bug reports, or improvements, please open an issue or submit a pull request.

    - Fork the repository
    - Create your feature branch (git checkout -b feature/your-feature)
    - Commit your changes (git commit -am 'Add some feature')
    - Push to the branch (git push origin feature/your-feature)
    - Open a pull request

## License
This project is licensed under the [MIT License](https://github.com/kad1rr/tm/blob/master/LICENSE)