# fish_url

fish_url is a Rust-based command-line tool that checks various caching and archiving services for a given URL. It's useful for finding cached or archived versions of web pages that might be otherwise inaccessible.

## Features

- Checks multiple caching services:
  - Google Cache
  - Freedium
  - Internet Archive (Wayback Machine)
  - Ghostarchive
- Automatically adds "www" to URLs if missing
- Provides detailed output for each checked service
- Easy to use command-line interface

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust programming language (1.56.0 or later)
- Cargo package manager (usually comes with Rust)

You can install Rust and Cargo from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

## Installation

To install fish_url, follow these steps:

1. Clone the repository:

   ```
   git clone https://github.com/yourusername/fish_url.git
   cd fish_url
   ```

2. Build the project:

   ```
   cargo build --release
   ```

3. The executable will be created in `target/release/fish_url`.

4. (Optional) Add the executable to your PATH:

   - For Linux or macOS:

     ```
     echo 'export PATH="$PATH:$HOME/path/to/fish_url/target/release"' >> ~/.bashrc
     source ~/.bashrc
     ```

     Replace `$HOME/path/to/fish_url` with the actual path to your project directory.

   - For Windows (PowerShell):
     ```
     $env:Path += ";C:\path\to\fish_url\target\release"
     ```
     To make this change permanent, you'll need to add the path to your system's Environment Variables.

## Usage

To use fish_url, follow these steps:

1. Open a terminal or command prompt.
2. If you added fish_url to your PATH, you can run it from anywhere:

   ```
   fish_url <URL>
   ```

   Replace `<URL>` with the web page you want to check.

   If you didn't add it to your PATH, navigate to the project directory and run:

   ```
   ./target/release/fish_url <URL>
   ```

For example:

```
fish_url https://example.com
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

If you have any questions or feedback, please open an issue on the GitHub repository.
