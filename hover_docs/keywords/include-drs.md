```rms
include_drs FILEPATH
```

---

Inserts the code of the map script at the give filepath.

**Arguments**:
- `FILEPATH`: Path to the file to include.
  - Valid file extensions are: `rms`, `rms2`, `inc`, and `def`.
  - If the path contains spaces, it can be surrounded by double quotation marks: `"this is the/path to/example file name.rms"`
  - The filepath is relative to the gamedata folder.

Included files are not transferred in a multiplayer lobby.
Map scripts may only include the default scripts in the game directory.

**Example**: Include the DE&nbsp;seasons file for defining terrains and themed constants.

```rms
#include_drs F_seasons.inc
```

**Example 2**: Include the classic land and water resources file from AoC.

```rms
#include_drs land_and_water_resources.inc
```

**Example 3**: Make a custom version of Blind Random.

```rms
start_random
  percent_chance 20 #include_drs Arabia.rms
  percent_chance 20 #include_drs Baltic.rms
  percent_chance 20 #include_drs Gold_Rush.rms
  percent_chance 20 #include_drs Islands.rms
  percent_chance 20 #include_drs Team_Islands.rms
end_random
```

**See more**: [#include_drs](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.qn6gojo7i9nv)
