## What is a Knowledge Accumulation Log (KAL)?

A KAL record describes what activities you performed in a day to broaden your knowledge in a particular category (subject/field/specialization). It sounded cool, that's why I made it up! 😛

---

Example list of committed KAL records for an imaginary person:

```
> kal ls --all 2024

# {year}-{ordinal_day} | {category}: {details}

2024-1 | Philosophy: Today, I committed myself to thinking about existence itself 🧠
...
2024-90 | Computer Graphics: Took a deep dive into render pipelines with Vulkan.
2024-91 | Scuba Diving: Joined to a local diving club this afternoon!
...
2024-193 | Cinematography: Practiced common composition techniques 🎥
...
2024-294 | Deep Learning: Learned about autograd engines and the math behind.
...
```

## Configuration File

See [kal.config.toml](./kal.config.toml) file.

## Commands & Arguments

```
Usage: kal.exe <COMMAND>

Commands:
  init    Applies initial migration(s) for a newly created database file. New database files must be created manually at the provided "db_path" location
  commit  Creates a new KAL record [aliases: cm]
  reset   Deletes all KAL records committed today
  list    Lists KAL records committed only today if no argument's provided [aliases: ls]
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
