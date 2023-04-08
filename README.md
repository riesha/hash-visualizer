# hash-visualizer

little toy project of mine i wrote to visualize the bit randomness of hash functions

## Usage

```
Usage: hash-visualizer.exe [<input>] [-h] [-f <file>]

hash visualizer

Positional Arguments:
  input             unused if file flag is specified, string to use as input
                    instead of a file, defaults to "no input given" if left
                    blank

Options:
  -h, --hilbert     use hilbert curve algorithm instead of linear mapping
  -f, --file        use a file instead of a string as the input
  --help            display usage information
```

## Examples 

hilbert curve
![](demos/hilbert0.png)

linear mapping
![](demos/linear0.png)
