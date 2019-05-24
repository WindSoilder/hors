[![Build Status](https://travis-ci.org/WindSoilder/hors.svg?branch=master)](https://travis-ci.org/WindSoilder/hors)

# hors
Awesome program [howdoi](https://github.com/gleitz/howdoi) which implemented in rust

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
    hors [FLAGS] [OPTIONS] <QUERY>

FLAGS:
    -a, --all        display the full text of the answer.
    -c, --color      enable colorized output
    -h, --help       Prints help information
    -l, --link       display only the answer link.
    -v, --version    displays the current version of howdoi

OPTIONS:
    -e, --engine <engine>                    select middle search engine, currently support `bing` and `google`.
                                             [default: bing]
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

6. The default search engine is bing, how can I use other search engine?
```shell
hors "set git remote url" -n 2 -a -e "google"
```

# Special thanks
Very thanks for the awesome project and links :)
- [howdoi](https://github.com/gleitz/howdoi) which inspired `hors` (Fow now `hors` is `howdoi` which implements in `rust`).
- [stackoverflow](https://stackoverflow.com/) helps user solve question about coding.

# About the name
`hors` is the abbreviation of `howdoi in rust`.
