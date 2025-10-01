# git-footprint
A CLI tool for counting git repository contribution.
![image](/assets/view.jpg)

## Installation 
```
cargo install git-footprint
```


## Usage

`git-footprint --help`
```
A CLI tool for counting git repository contribution

Usage: git-footprint --email <EMAIL> [OPTIONS]

Options:
  -e, --email <EMAIL>  Git user email to filter commits
  -p, --path <PATH>    Directory to scan for Git repositories [default: .]
  -y, --year <YEAR>    Year to analyze (e.g., 2024), defaults to current year
```

### Example
```
git-footprint --email your@email.com --path ~/your/code/dir

git-footprint --email your@email.com --path ~/your/code/dir --year 2025
```


## License
- License: [MIT]() 