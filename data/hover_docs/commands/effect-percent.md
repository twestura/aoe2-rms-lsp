```rms
effect_percent EFFECT_TYPE TYPE ATTRIBUTE_TYPE PERCENT
```

---

Identical to [effect_amount](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.1niw1edwqhy5), but the specified value is divided by&nbsp;100, allowing greater precision.

**Arguments**:
- `EFFECT_TYPE`: An [effect constant](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.90ed1yz7qe0h).
- `TYPE`: An [object](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.nvxriamulybh), [resource](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.cym0hd55425r), [technology](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.ro59uo4jl1z3), [class](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.lvcoxxnz995p), [Advanced Genie Editor](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.6u6ogmgec4g), [effect](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.90ed1yz7qe0h), or [miscellaneous](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.2n1q8vynhc9o) constant.
- `ATTRIBUTE_TYPE`: An [attribute constant](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.oumcl095iabt).
- `PERCENT`: A number. The applied value is `PERCENT / 100`.

Since [effect_amount](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.1niw1edwqhy5) now accepts floating point arguments, effect_percent is now deprecated.

**Example**: Add 0.3&nbsp;speed to all villagers (30&nbsp;/&nbsp;100&nbsp;=&nbsp;0.3).
```rms
<PLAYER_SETUP>
effect_percent ADD_ATTRIBUTE VILLAGER_CLASS ATTR_MOVE_SPEED 30
```

**See more**: [effect_percent](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.7siepjsm3bdc)
