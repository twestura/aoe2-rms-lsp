```rms
guard_state OBJECT_TYPE RESOURCE_TYPE RESOURCE_DELTA FLAGS
```

---

Set up additional lose conditions and/or resource trickles based on controlling a specified object.

**Arguments**:
- `OBJECT_TYPE`: An [object](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.nvxriamulybh) or [class](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.lvcoxxnz995p) constant.
  - For Villagers, use `VILLAGER_CLASS` instead of `VILLAGER`.
- `RESOURCE_TYPE`: A [resource amount constant](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit#heading=h.cym0hd55425r).
- `RESOURCE_DELTA`: A number (default 0).
- `FLAGS`: A number in 0&#8288;&ndash;&#8288;7 (default 0). Add values to apply multiple flags.
  - 0: No flags.
  - 1: Lose if the specified `OBJECT_TYPE` is not controlled.
  - 2: Activate a resource trickle of `RESOURCE_TYPE` at `RESOURCE_DELTA / 100` per second while `OBJECT_TYPE` is controlled.
  - 4: Invert the `OBJECT_TYPE` requirement for the other flags.

Only one `guard_state` command may be active per script.

**Example**: Activate a guard state on the King to make a map Regicide even in other game modes (note Spies cannot be swapped for Treason in DE without setting the game mode explicitly).
```rms
<PLAYER_SETUP>
guard_state KING AMOUNT_GOLD 0 1
```

**Example 2**: Slowly drain a player's Food while they do not control the monument.
```rms
<PLAYER_SETUP>
guard_state MONUMENT AMOUNT_FOOD -5 6
```

**Example 3**: Enable a small Gold trickle and configure players to be defeated if all Villagers are lost.
```rms
<PLAYER_SETUP>
guard_state VILLAGER_CLASS AMOUNT_GOLD 10 3
```

**See more**: [guard_state](https://docs.google.com/document/d/1jnhZXoeL9mkRUJxcGlKnO98fIwFKStP_OBozpr0CHXo/edit?tab=t.0#heading=h.7jvxo0vqwqu8)
