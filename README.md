# grate
A simple sql migrations file gen tool

## example
```bash
grate --name party --path migrations
```

```bash
✅ created ./migrations/1678912821_party.up.sql
✅ created ./migrations/1678912821_party.down.sql
```

## params
`name` - required. the name of the migration, will be appended to the file name after the timestamp.
`path` - optional. defaults to `./migrations`. will generate a new directory if non exists.
