# idot
A simple dotfiles manager.

## Usage

``` 
» tree .
.
├── bar
└── foo
```
Create config `idot.json` (or `idot.toml` / `idot.yaml`)
```json
{
  "links":{
    "~/foo":{
      "target":"foo"
    },
    "~/bar":{
      "target":"bar",
      "relative":true
    }
  },
  "relative":false
}
```

### Check status

``` shell
» idot status
`/home/meetcw/bar` -> `/path/to/dotfiles/bar`
`/home/meetcw/foo` -> `/path/to/dotfiles/foo`
```

### Enable

``` shell
» idot create
Create symbolic link: `~/bar` -> `/path/to/dotfiles/bar`
Create symbolic link: `~/foo` -> `/path/to/dotfiles/foo`
```

### Disable

``` shell
» idot delete
Delete symbolic link: `~/bar`.
Delete symbolic link: `~/foo`.
```