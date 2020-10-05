# [Unreleased]
## Added
- New google front page style is supported.

# [0.7.1] - 2020-09-15
## Added
- Paging feature is supported, when the answer is long, hors will visually separate the answers from the rest of the terminal output.  You can customize this feature through `-p` or `--paging` argument.
- Can download through more convenient tools, like `brew` on `macOS`, `scoop` on `Windows`.

## Fixed
- Fix `-a` option sometimes leads to `Can't get answer from <xxx>` error message.

# [0.6.10] - 2020-08-31
## Fixed
- Fit for new css attribute of so.

## Changed
- Little reduce final binary size.

# [0.6.9] - 2020-08-25
## Added
- Support stackoverflow internal search engine, we can run with `-e stackoverflow` to enable this.

## Fixed
- When search throught search engine failed, will try to provide a more user friendly message.

# [0.6.8] - 2020-08-16
## Added
- Support non-truecolor terminal colorize.

# [0.6.6] - 2020-08-12
## Fixed
- search with `duckduckgo` engine is more stable.
- make output more friendly, `hors` will try to select proper syntax to colorize output.

# [0.6.5] - 2020-08-08
## Added
- introduce `HORS_ENGINE` env variable, to specify which search engine to use.  For now it can be `bing`, `google`, `duckduckgo`.
- Make use of `rustls`, so `openssl` is no longer needed.

## Fixed
- For now `"` between query string is no-longer needed.  So you can run `hors pandas dataframe to csv -l`.

# [0.6.3] - 2020-08-07
## Added
- Add *-r* argument, if you don't need colorize output.

## Removed
- Remove *-c* argument, for now colorize output is default behavior.

# [0.6.2] - 2020-04-19
## Changed
- Don't need to install `bindgen` for now.

# [0.6.1] - 2020-01-30
## Added
- Support socks5 system proxy.

## Changed
- Upgrade reqwest version to 0.10.1.
- Change all hors relative api to async.

# [0.5.0] - 2019-12-09
## Added
- Some functions and structs are public from lib, for more information, please check the doc.

## Changed
- `Error` has been redesigned, for now it will be an `enum` rather than `struct`.
- Rename from `HorsError` to `Error`.
- Documentation improved.
- Rename from `hors::engine::search_link` to `hors::engine::search_link_with_client`, the original `hors::engine::search_link` will be more simply to use.
- Rename from `hors::answer::get_answers` to `hors::answer::get_answers_with_client`, the original `hors::answer::get_answers` will be more simply to use.

# [0.4.1] - 2019-09-10
## Added
- support *duckduckgo* search engine.  To apply it, use can use `-e duckduckgo`.

## Fixed
- Hors will output more friendly error messages, all error messages will go to *stderr*.

## Changed
- For now the default search engine is duckduckgo.

# [0.4.0] - 2019-08-29
## Added
- support *--disable_proxy* argumet.

## Changed
- `hors` will use system proxy by default.  If you don't want to use proxy, please run hors with `--disable_proxy` arguments.

## Fixed
- Hors will try to use `http` to get stackoverflow links, if the `https` schema go failed.  This can improve search success rate.

# [0.3.4] - 2019-08-10
## Changed
- Refactory code structure, and it should be quicker.

# [0.3.3] - 2019-06-03
## Fixed
- Support google search engine again.  You can use `-e google` to use google search engine.

# [0.3.2] - 2019-06-01
## Added
- Code has been re-structured, so integration test can be introduced for hors easily.

## Removed
- Remove google search engine support temporary (Because there are issues here, and it can't be solved quickly)

## Fixed
- hors may runs forever with *-l* argument in the previous version, it's fixed now.

# [0.3.1] - 2019-05-26
## Added
- More friendly documentation.

# [0.3.0] - 2019-05-24
## Added
- Hors will use internal cache to improve performance.

# [0.2.0] - 2019-05-16
## Added
- Support google search engine
- Support *-e* argument

## Changed
- Hors will select answer by voted information.  Which means that if an answer is not accepted, but it's the most voted, hor will show that answer for you.

# [0.1.0] - 2019-05-11
## Added
- The first implementation of hors
- support *-l* argument
- support *-c* argument
- support *-n* argument
- support *-a* argument
