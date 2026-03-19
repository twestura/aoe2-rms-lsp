```rms
set_gaia_civilization CIVILIZATION
```

---

Sets the civilization used by Gaia.

**Arguments**:
- `CIVILIZATION`: number in 0&#8288;&ndash;&#8288;53, default&nbsp;0 (see [Civilizations](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.8ctucmcvyhyv) for a list of predefined constants).

Affects the architectural style of Gaia buildings and the appearance of units with regional variations.

- Also affects civilization bonuses, upgrades, and unique technologies (relevant especially for [Battle Royale](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.yaqdimuqjsaj)).
- The default gaia civilization uses the western European architectural style.
- If used, gaia effects (see [Effect Constants](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.90ed1yz7qe0h)) will not function when applied to units that can be player controlled.
- If used, gaia buildings with [make_indestructible](URL) will be burning.
- By convention used in player setup, but may be used anywhere in the script.
- If used multiple times, only the final instance applies.
- Does not work in the scenario editor.


**Example**: Create a Lithuanian monument for gaia.
```rms
<PLAYER_SETUP>
set_gaia_civilization CIVILIZATION_LITHUANIANS

<OBJECTS_GENERATION>
create_object MONUMENT { }
```

**See more**: [set_gaia_civilization](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.yy69o1bqyfx5)
