# deprops
## Like Gemfiles for your repo dependencies

Define all your repo dependencies on a file external to your version control system.

## Installation
Clone the repo and link `bin/deprepos` into your `$PATH`.

## Example

Create a file called `example.repo.deps` with the following content:

```
git this/will/be/clone/here origin/stable https://example.com/here
```

By default, `deprepos` will read `repo.deps` on the current directory, but it
could be defined when running.

```bash
deprepos example.repo.deps
```

## Future work
- Mercurial (?)
