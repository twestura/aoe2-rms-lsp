```rms
enable_waves SHOW_WAVES
```

---

Controls whether animated beach waves are shown where water borders land.
DE only.

- Enabled by default, only include this command to disable waves.
- Only visible if the player has "Render Beach Waves" enabled in game settings.
- Waves in general are controlled by [water_definition](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.cyw7be9pc0yt).

**Arguments**:
- `SHOW_WAVES`: 1 to enable, 0 to disable (default 1).

**Example**: Disable beach waves.
```rms
<LAND_GENERATION>
enable_waves 0
```

**See more**: [enable_waves](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.fno7myp7722j)
