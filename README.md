# Excel_To_Csv_Fast

## What does it do?

Turns Excel related XML files into CSV's, making it easier to use the tools you already know and love,
such as:

- grep
- [ripgrep](https://github.com/BurntSushi/ripgrep)

this also allows users to operate on these files with tools that may only funciton on standard text files.

### EG

For instance certain python libraries, Rust, Go and mant other languages have csv parsing libraries, however sometimes they lack in the domain of Excel files - now you can convert them and use your languages native parsing (or it's most popular package) to get the data out and work with it.


## WIP Things

[ ] - Currently there is no direcy CLI, this is aiming to be implemented.

- This obviously comes with the caveat that the files are transformed in-place. Without a way to specify an output directory at the moment.

[ ] - Parsing other documents is a future possibility.
