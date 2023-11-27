# Decklist Search CLI
Searches a text file for card names from another text file.

Example: say we have file `decklist.txt` with the following:
```
10 Forest
12 Mountain
1 Lathiel, the Bounteous Dawn
```

We can search another text file for any occurence of "Forest", "Mountain", or "Lathiel, the Bounteous Dawn".

## Installation
For now, I haven't gotten a working release for Windows, but there's a working Linux executable at `target/x86_64-unknown-linux-gnu/release/decklist-search`. You can simply download this file to use the CLI on Linux.

I suggest, for future Windows users, adding the executable's folder to the path. I suggest, for Linux users, creating a symlink for the file in `/usr/bin` or wherever your executables are kept. I don't know how to use a Mac.

## Usage
If you've added the executable to your path or created a symlink, you should be able to use the file by calling `decklist-search` or whatever you named the link in your system. Otherwise, replace `decklist-search` in the following with the path to the executable.

Basic example: searching for all names from `decklist.txt` in `inventory.csv`
```bash
decklist-search path/to/decklist.txt path/to/inventory.csv 
```

### Options
Case sensitive: boolean

```bash
decklist-search path/to/decklist.txt path/to/inventory.csv -c
```
