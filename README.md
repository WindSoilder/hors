# hors
Awesome program [howdoi](https://github.com/gleitz/howdoi) which implemented in rust

About the name:
`hors` is the abbreviation of `howdoi in rust`.

# Installation
```shell
cargo install hors
```

# Support arguments
```shell
USAGE:
    hors [FLAGS] [OPTIONS] <QUERY>

FLAGS:
    -a, --all        display the full text of the answer.
    -h, --help       Prints help information
    -l, --link       display only the answer link.
    -v, --version    displays the current version of howdoi

OPTIONS:
    -n, --number_answers <number_answers>    number of answers to return [default: 1]

ARGS:
    <QUERY>
```

# Usage example
1.  Want to know how to export pandas dataframe to csv?
```shell
hors "pandas dataframe to csv"
```

Here it is:

```
- Answer from https://stackoverflow.com/questions/16923281/pandas-writing-dataframe-to-csv-file
df.to_csv(file_name, sep='\t')
```

2. If we just want to know where is the answer existed?
```shell
hors "pandas dataframe to csv" -l
```

Here it is:
```
Title - pandas writing dataframe to csv file
https://stackoverflow.com/questions/16923281/pandas-writing-dataframe-to-csv-file
```

3. If we want more about the answer detailed?
```shell
hors "how to parse json in rust" -a
```

Here it is:
```shell
- Answer from https://stackoverflow.com/questions/30292752/how-do-i-parse-a-json-file

Solved by the many helpful members of the Rust community:

extern crate rustc_serialize;
use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("text.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let json = Json::from_str(&data).unwrap();
    println!("{}", json.find_path(&["Address", "Street"]).unwrap());
}
```

4. What if we want to make output code colorized?
```shell
hors "how to parse json in python" -c
```
Here it is:
```python
import json
j = json.loads('{"one" : "1", "two" : "2", "three" : "3"}')
print j['two']
```

5. How to get more than one answers
```shell
hors "what's the differenct between pickle and msgpack in python" -n 2 -a
```
Here it is:
```
- Answer from https://stackoverflow.com/questions/12331633/how-to-gzip-all-files-in-all-sub-directories-into-one-compressed-file-in-bash

tar -zcvf compressFileName.tar.gz folderToCompress


everything in folderToCompress will go to compressFileName

Edit: After review and comments I realized that people may get confused with compressFileName without an extension. If you want you can use .tar.gz extension(as suggested) with the compressFileName

^_^ ==================================================== ^_^

- Answer from https://stackoverflow.com/questions/15279607/can-zlib-create-tar-file-on-windows

gzip and zlib are used for compression. The tar archiving functionality is not provided by them. For creating a .tgz file, you typically use tar with czf flags, which then uses gzip for compression. If you need tar functionality, libarchive looks good for the job.
```

# Special thanks
Very thanks for these awesome project :)
- [howdoi](https://github.com/gleitz/howdoi) which inspired `hors` (And it's the `howdoi` implementation which uses `rust` language).
- [reqwest](https://github.com/seanmonstar/reqwest) provides powerful http request api.
- [syntect](https://github.com/trishume/syntect) provides syntax highlighting feature.
- [rand](https://github.com/rust-random/rand) provides random choice generation.
- [clap](https://github.com/clap-rs/clap) provides command line argument parser.
