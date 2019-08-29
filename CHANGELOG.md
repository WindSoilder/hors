# [Unreleased]

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
