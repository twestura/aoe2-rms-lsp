```rms
effect_amount EFFECT_TYPE TYPE ATTRIBUTE_TYPE AMOUNT
```

---

Modifies various aspects of game data.

**Arguments**:
- `EFFECT_TYPE`: An [effect constant](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.90ed1yz7qe0h).
- `TYPE`: An [object](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.nvxriamulybh), [resource](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.cym0hd55425r), [technology](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.ro59uo4jl1z3), [class](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.lvcoxxnz995p), [Advanced Genie Editor](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.6u6ogmgec4g), [effect](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.90ed1yz7qe0h), or [miscellaneous](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.2n1q8vynhc9o) constant.
- `ATTRIBUTE_TYPE`: An [attribute constant](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.oumcl095iabt).
- `AMOUNT`: A number.

**Quirks**:
- When modifying objects, all hidden variations may need to be targeted individually.
- If an object ends up with more than 32,767&nbsp;hitpoints as a result, it is instantly destroyed. Be sure to consider the effects of in-game upgrades and civilization bonuses.
- If an object is disabled with this command, in-game technologies and ages may re-enable it. Civilization tech trees may also override the changes.
- Effects remain applied when generating maps in the scenario editor. The editor must be restarted (or a new scenario created) to reset the effects.

**Example**: Add 10,000 starting food.
```rms
<PLAYER_SETUP>
effect_amount MOD_RESOURCE AMOUNT_STARTING_FOOD ATTR_ADD 10000
```

**Example 2**: Houses support 10/15/20/25 population per age.
```rms
<PLAYER_SETUP>
effect_amount SET_ATTRIBUTE HOUSE ATTR_STORAGE_VALUE 10
effect_amount SET_ATTRIBUTE HOUSE_F ATTR_STORAGE_VALUE 15
effect_amount SET_ATTRIBUTE HOUSE_C ATTR_STORAGE_VALUE 20
effect_amount SET_ATTRIBUTE HOUSE_I ATTR_STORAGE_VALUE 25
```

**Example 3**: Rename the crocodile into an alligator.
```rms
effect_amount GAIA_SET_ATTRIBUTE DLC_CROCODILE ATTR_NAME_ID STRING_ALLIGATOR
```

**See more**: [effect_amount](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.1niw1edwqhy5)
