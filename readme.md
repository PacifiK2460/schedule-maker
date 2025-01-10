# Schedule Maker 📆

A ==simple uni schedule maker== that allows you to input your subjects and generate all possible schedules for the semester.

Every possible schedule is generated into a separate `.ics` file that can be imported into your calendar app or supported calendar viewer. [^1]

---

**Usage**:
: `schedule_maker[.exe] [OPTIONS] --input-file <INPUT_FILE>`

**Options**:

`-i, --input-file <INPUT_FILE>`
: Input file to process

`-o, --output-directory <OUTPUT_DIRECTORY>`
: Output directory to export to (defaults to current directory) [default: .]

`-h, --help`
: Print help

`-V, --version`
: Print version

**Example**:

```bash
schedule_maker[.exe] --input-file subjects.toml --output-directory ./out/semester1
```

---

**Input file format**:

The input file is a TOML file that contains the following fields, where each subject is a table and the days field is an array of tables.

```toml
[[subject]]
name = "Programming"
class_id = "410P-T21B"
professor = "John Doe"
days = [
  { day = "monday", start = 11:00:00, end = 11:55:00 },
  { day = "tuesday", start = 11:00:00, end = 11:55:00 },
  { day = "thursday", start = 11:00:00, end = 11:55:00 },
  { day = "wednesday", start = 11:00:00, end = 11:55:00 },
  { day = "friday", start = 11:00:00, end = 11:55:00 },
]
```

---

**Example**:

```toml
[[subject]]
name = "Programming"
class_id = "410P-T21B"
professor = "John Doe"
days = [
  { day = "monday", start = 11:00:00, end = 11:55:00 },
  { day = "tuesday", start = 11:00:00, end = 11:55:00 },
  { day = "thursday", start = 11:00:00, end = 11:55:00 },
  { day = "wednesday", start = 11:00:00, end = 11:55:00 },
  { day = "friday", start = 11:00:00, end = 11:55:00 },
]

[[subject]]
name = "Programming"
class_id = "410P-T21A"
professor = "John Doe"
days = [
  { day = "monday", start = 10:00:00, end = 10:55:00 },
  { day = "tuesday", start = 10:00:00, end = 10:55:00 },
  { day = "thursday", start = 10:00:00, end = 10:55:00 },
  { day = "wednesday", start = 10:00:00, end = 10:55:00 },
  { day = "friday", start = 10:00:00, end = 10:55:00 },
]

[[subject]]
name = "Programming"
class_id = "410P-T21C"
professor = "John Doe"
days = [
  { day = "monday", start = 12:00:00, end = 12:55:00 },
  { day = "tuesday", start = 12:00:00, end = 12:55:00 },
  { day = "thursday", start = 12:00:00, end = 12:55:00 },
  { day = "wednesday", start = 12:00:00, end = 12:55:00 },
  { day = "friday", start = 12:00:00, end = 12:55:00 },
]

[[subject]]
name = "Data Structures"
class_id = "650P-T21A"
professor = "John Doe"
days = [
  { day = "monday", start = 09:00:00, end = 10:55:00 },
  { day = "tuesday", start = 09:00:00, end = 10:55:00 },
]

[[subject]]
name = "Data Structures"
class_id = "650P-T21B"
professor = "John Doe"
days = [
  { day = "monday", start = 10:00:00, end = 10:55:00 },
  { day = "tuesday", start = 10:00:00, end = 10:55:00 },
]

[[subject]]
name = "Data Structures"
class_id = "650P-T21C"
professor = "John Doe"
days = [
  { day = "monday", start = 11:00:00, end = 11:55:00 },
  { day = "tuesday", start = 11:00:00, end = 11:55:00 },
]

[[subject]]
name = "Data Structures"
class_id = "650P-T21A"
professor = "John Doe"
days = [
  { day = "monday", start = 12:00:00, end = 12:55:00 },
  { day = "tuesday", start = 12:00:00, end = 12:55:00 },
]
```

[^1]: I recommend [this](https://larrybolt.github.io/online-ics-feed-viewer/).
