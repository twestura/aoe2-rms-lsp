```rms
terrain_state MODE PARAMETER_1 PARAMETER_2 FLAGS
```

---

Modifies shallows terrain behavior and blending.
UP only, use [effect_amount](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.1niw1edwqhy5) to modify terrain restrictions in DE.


**Arguments**:
- `MODE`: Set to 0 (default 0).
- `PARAMETER_1`: Set to 0 (default 0).
- `PARAMETER_2`: Set to 0 (default 0).
- `FLAGS`: A number in 0&#8288;&ndash;&#8288;7 (default 0). Add values to apply multiple flags.
  - 0: No flags.
  - 1: Enable building on Shallows and allow resources to be placed on Shallows.
  - 2: Thinner blending of Shallows and Beach terrain.
  - 4: Changes Ice blending to use Shallows-style blending.

`MODE`, `PARAMETER_1`, and `PARAMETER_2` have no known effect.
Set all three to 0.

**Example**: Enable all flags for buildable shallows with alternate blending.
```rms
<PLAYER_SETUP>
terrain_state 0 0 0 7
```

**See more**: [terrain_state](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.tdph3oglggjk)
