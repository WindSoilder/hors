    [![Crate](https://img.shields.io/crates/v/hors.svg)](https://crates.io/crates/hors)
[![CI](https://github.com/WindSoilder/hors/workflows/CI/badge.svg)](https://github.com/WindSoilder/hors/actions?query=workflow%3ACI)


# [hors](https://crates.io/crates/hors)
Awesome program [howdoi](https://github.com/gleitz/howdoi) which implemented in rust, along with easily usage lib.

For binary usage, please go through the rest of this file.  For lib documentation, please check [here](https://docs.rs/hors/latest/hors/).

# Screenshot
![Screenshots of hors](screenshots/screenshot.png)

# Installation
hors is written in `Rust`.  The recommended way to install `hors` is through `cargo`.
```shell
cargo install hors
```

# Tested platforms
For now, `hors` has been tested with the following platforms:
- linux
- osx
- windows

# Usage
```shell
USAGE:
    hors [FLAGS] [OPTIONS] <query>

ARGS:
    <query>

FLAGS:
    -a, --all              display the full text of answer.
    -d, --disable-proxy    Disable system proxy.
    -h, --help             Prints help information
    -l, --link             display only the answer link.
    -r, --raw              make raw output (not colorized).
    -V, --version          Prints version information

OPTIONS:
    -e, --engine <engine>                    select middle search engine, currently support `bing`, `google` and
                                             `duckduckgo`. [default: duckduckgo]
    -n, --number-answers <number-answers>    number of answers to return. [default: 1]
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

4. How to get more than one answers
```shell
hors "set git remote url" -n 2 -a
```
Here it is:
```
- Answer from https://stackoverflow.com/questions/2432764/how-to-change-the-uri-url-for-a-remote-git-repository
You can

git remote set-url origin new.git.url/here

(see git help remote) or you can just edit .git/config and change the URLs there. You're not in any danger of losing history unless you do something very silly (and if you're worried, just make a copy of your repo, since your repo is your history.)


^_^ ==================================================== ^_^

- Answer from https://stackoverflow.com/questions/42830557/git-remote-add-origin-vs-remote-set-url-origin
below is used to a add a new remote:

git remote add origin git@github.com:User/UserRepo.git

below is used to change the url of an existing remote repository:

git remote set-url origin git@github.com:User/UserRepo.git

below will push your code to the master branch of the remote repository defined with origin and -u let you point your current local branch to the remote master branch:

git push -u origin master

Documentation
```

5. The default search engine is bing, how can I use other search engine?
```shell
hors "set git remote url" -n 2 -a -e "google"
```

# Proxy support
If the network seems blocked, you can try to configure proxy like this:
```shell
export http_proxy=http://127.0.0.1:1087;export https_proxy=http://127.0.0.1:1087;
```

Of cause it should be a valid proxy in your machine.

# Use hors as lib
Hors can be used as a lib, here is an example:

```rust
use std::str::FromStr;
use hors::{self, SearchEngine};

let search_engine: SearchEngine = SearchEngine::from_str("bing").unwrap();
let target_links: Vec<String> = hors::search_links(
    "how to parse json in rust",
    search_engine,
)
.await
.unwrap();
assert_ne!(target_links.len(), 0);
for link in target_links {
    assert!(link.contains("stackoverflow.com"));
}
```

For more information, please check [documentation](https://docs.rs/hors/latest/hors/)

# Special thanks
Very thanks for the awesome project and links :)
- [howdoi](https://github.com/gleitz/howdoi) inspires `hors` (Fow now `hors` is `howdoi` which implements in `rust`).
- [stackoverflow](https://stackoverflow.com/) helps user solve question about coding.

# About the name
`hors` is the abbreviation of `howdoi in rust`.
