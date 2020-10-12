[![Crate](https://img.shields.io/crates/v/hors.svg)](https://crates.io/crates/hors)
[![CI](https://github.com/WindSoilder/hors/workflows/CI/badge.svg)](https://github.com/WindSoilder/hors/actions?query=workflow%3ACI)


# [hors](https://crates.io/crates/hors)
Awesome program [howdoi](https://github.com/gleitz/howdoi) which implemented in rust, along with easily usage lib.

It's faster than the original howdoi program.

For binary usage, please go through the rest of this file.  For lib documentation, please check [here](https://docs.rs/hors/latest/hors/).

## What make it fast
1. Implemented in rust, which causes less runtime overhead.
2. Make use of tokio concurrent feature, so hors will make concurrent search when it need to fetch more than 1 answer.
3. Output will be cache, and when you want to search for the same question, hors will likely make less network traffic to get the answer.

Here is a simple benchmark report, run the following command 3 times in my personal computer:
```shell
time hors mysql create table with column comment -a -n 10 --paging never -e bing
rm ~/Library/Caches/hors/answers
```
Note: run `rm` command is aimed to clear local cache.

And it gives me the following output:

```
Executed in    2.55 secs   fish           external
   usr time  232.71 millis  150.00 micros  232.56 millis
   sys time   16.68 millis  562.00 micros   16.12 millis

Executed in    3.68 secs   fish           external
   usr time  252.02 millis  125.00 micros  251.90 millis
   sys time   19.18 millis  550.00 micros   18.63 millis

Executed in    2.55 secs   fish           external
   usr time  237.19 millis  117.00 micros  237.07 millis
   sys time   17.63 millis  565.00 micros   17.06 millis
```

Run the same command with howdoi:
```shell
time howdoi mysql create table with column comment -a -n 4 -e bing -c
```

And it gives me the following output:

```
Executed in    3.48 secs   fish           external
   usr time  303.67 millis  127.00 micros  303.54 millis
   sys time   52.53 millis  601.00 micros   51.93 millis

Executed in    3.65 secs   fish           external
   usr time  305.37 millis  111.00 micros  305.26 millis
   sys time   53.16 millis  549.00 micros   52.61 millis

Executed in    3.34 secs   fish           external
   usr time  319.07 millis   14.24 millis  304.83 millis
   sys time   55.63 millis    3.37 millis   52.26 millis
```

But please note that this simple benchmark is not precise, it highly depends on network information.

# Screenshot
## Simple usage example
![Screenshots of hors gif](screenshots/hors_demo.gif)

## More examples

![Screenshots of hors png](screenshots/screenshot.png)

# Installation
hors is written in `Rust`.  The recommended way to install `hors` is through `cargo`.

```shell
cargo install hors
```

On Windows/Linux/macOS platform, you can download the pre-build-binary from github [release page](https://github.com/WindSoilder/hors/releases/latest)

## On macOS
Hors can be installed from [homebrew](https://brew.sh/).

```shell
brew tap hors-org/hors && brew install hors
```

## On Windows
Hors can be installed from [scoop](https://scoop.sh/)

```shell
scoop bucket add w-bucket https://github.com/hors-org/w-bucket; scoop install hors
```

# Tested platforms
For now, `hors` has been tested with the following platforms:

- Linux
- OSX
- Windows

# Usage
```shell
USAGE:
    hors [FLAGS] [OPTIONS] [query]...

ARGS:
    <query>...

FLAGS:
    -a, --all              display the full text of answer.
    -d, --disable-proxy    Disable system proxy.
    -h, --help             Prints help information
    -l, --link             display only the answer link.
    -r, --raw              make raw output (not colorized).
    -V, --version          Prints version information

OPTIONS:
    -e, --engine <engine>                    select middle search engine, currently support `bing`, `google`,
                                             `duckduckgo`, `stackoverflow`. [env: HORS_ENGINE=bing]  [default:
                                             duckduckgo]
    -n, --number-answers <number-answers>    number of answers to return. [default: 1]
    -p, --paging <paging>                    specify how to page output, can be `auto`, `never` [default: auto]
```

# Usage example
1.  Want to know how to export pandas dataframe to csv?
```shell
hors pandas dataframe to csv
```

Here it is:

```
- Answer from https://stackoverflow.com/questions/16923281/pandas-writing-dataframe-to-csv-file
df.to_csv(file_name, sep='\t')
```

2. If we just want to know where is the answer existed?
```shell
hors pandas dataframe to csv -l
```

Here it is:
```
Title - pandas writing dataframe to csv file
https://stackoverflow.com/questions/16923281/pandas-writing-dataframe-to-csv-file
```

3. If we want more about the answer detailed?
```shell
hors how to parse json in rust -a
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
hors set git remote url -n 2 -a
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
hors set git remote url -n 2 -a -e "google"
```

# Proxy support
If the network seems blocked, you can try to configure proxy like this:
```shell
export http_proxy=http://127.0.0.1:1087;export https_proxy=http://127.0.0.1:1087;
```

Of course, it should be a valid proxy in your machine.

# Paging feature on windows
Hors is using `less` command to make paging feature work, and it's not installed on Windows by default.  You can use scoop to install `less`

```shell
scoop install less
```

Or use `choco`:

```shell
choco install less
```


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

For more information, please check the [documentation](https://docs.rs/hors/latest/hors/)

# Special thanks
Very thanks for the awesome project and links :)
- [howdoi](https://github.com/gleitz/howdoi) inspires `hors` (Fow now `hors` is `howdoi` which implements in `rust`).
- [stackoverflow](https://stackoverflow.com/) helps users solve questions about coding.

# About the name
`hors` is the abbreviation of `howdoi in rust`.
