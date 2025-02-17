# Roloc

## Development

Enter nix shell in order to download all needed tools

```bash
nix develop
```

## Commits
You can make a commit by using commitizen to follow conventional commits

```bash
cz commmit
```

## Running
To run application you must execute 
```bash
cargo run -i {path_to_img} -s {optional_svg_output} -k {number of colors}
```

Once you will provide the `-s` parameter there will be no output to STDOUT.

### Todo
- [ ] Sort the colors to compare palletes
- [ ] Add performance testing for algorithm
- [ ] Establish the best default iteration count 
- [ ] Input validation and errors
- [ ] Develop it as library
