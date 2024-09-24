# PDF Bookmarker
CLI utility for quickly applying bookmarks to existing PDF file.

# Toc format
This utility uses Toc format to represent bookmark entries.
here is an example.
```code
1 Chapter 1
+2 1-1 Test
++3 1-1-1 Test
+4 1-2 Test
6 Chapter 2
+12 2-1 Test
```
Each line represents bookmark entry. here is syntax for bookmark entry.
```code
+{0,n} page_number title
```
Each entry starts with zero to n repeating **+** symbol representing current depth. separated by single space, page number and title goes after.

# Command line usage
**Overview**
- **apply:**     Applies toc file to pdf's outline
- **clear:**     Clears bookmarks(outlines) from PDF file
- **extract:**   Extracts bookmarks from existing PDF file
- **validate:**  Validates toc file
- **help:**     Print this message or the help of the given subcommand(s)
 
 For detailed information, run ``pdf-bookmark <command> --help``.


# Build instructions
To build this program, you will need following.

 - [Latest rust installation](https://rustup.rs/)
 - **For windows:** Build tools for visual studio 2019 C++(for building mupdf-rs)
 - **For linux:** gcc(you can get them by installing`` build-essential`` on debian based distro, or ``base-devel`` on arch based distro)

Here is steps on how to build the program.

 1. Initialize all submodules by running ``git submodule init && git submodule update --recursive``.
 2. Run ``cargo build --release`` to build.
 3. Compiled executable binary could be found at ``./target/release``.

