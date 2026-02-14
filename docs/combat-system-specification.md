# Combat System Specification (v0.1)

**Tone Target:** Heroic + Dark Comedy  
**Combat Style:** Turn-based, grid-based  
**Design Goals:**  

* Swingy but recoverable
* Chaos creates positional problems, not instant death
* Crits are dramatic but not coin-flip lethal
* Retreat is viable and tactical
* Weapons and armor meaningfully change playstyle
* Unified margin-based resolution engine

---

# 1. Core Resolution Engine

## Dice Model

```
Roll = 2d10 + ATK + Momentum
Margin = Roll - Target DEF
```

* 2d10 creates a bell curve (reduces absurd streaks)
* Momentum range: -2 to +3 (capped)
* Margin determines outcome tier

---

# 2. Primary Stats

## Offensive & Defensive

| Stat | Typical Early Range | Description                   |
| ---- | ------------------- | ----------------------------- |
| ATK  | 3–8                 | Attack skill modifier         |
| DEF  | 3–8                 | Avoidance and defensive skill |
| POW  | 4–10                | Base damage strength          |
| ARM  | 0–7                 | Flat damage reduction         |
| GRD  | 0–5                 | Shield block threshold        |
| HP   | 30–50               | Health pool                   |

---

# 3. Margin Bands

| Margin   | Outcome       | Mechanical Effect                       |
| -------- | ------------- | --------------------------------------- |
| ≤ -6     | Wild Blunder  | Lose 2 momentum, enemy gains 1          |
| -5 to -1 | Overextended  | No damage, -2 DEF next turn             |
| 0–4      | Scramble      | 25% damage                              |
| 5–11     | Solid Hit     | 100% damage, +1 momentum                |
| 12–17    | Heroic Strike | 150% damage, +2 momentum, apply Stagger |
| 18+      | Spectacular   | 200% damage, +2 momentum, push 1 tile   |

Notes:

* Only bottom two bands deal 0 damage
* Most attacks do *something*
* Blunders create positional risk, not HP loss

---

# 4. Damage Formula

```
BaseDamage = POW + WeaponBonus
RawDamage = BaseDamage × SeverityMultiplier
FinalDamage = max(1, RawDamage - ARM)
```

### Severity Multipliers

| Outcome     | Multiplier |
| ----------- | ---------- |
| Scramble    | 0.25       |
| Solid       | 1.0        |
| Heroic      | 1.5        |
| Spectacular | 2.0        |

Design intent:

* Solid hit ≈ 20–30% HP
* Heroic ≈ 30–40%
* Spectacular ≈ 40–50%
* 5–8 turns typical duel

---

# 5. Momentum System

**Range:** -2 to +3

### Gain:

* Solid Hit: +1
* Heroic/Spectacular: +2
* Enemy Wild Blunder: +1

### Lose:

* Overextended: -1
* Wild Blunder: -2
* Taking Heroic hit: -1

Momentum:

* Adds directly to roll
* Creates flow and swing
* Enables comebacks
* Capped to prevent runaway spirals

---

# 6. Guard System (Shields)

Guard triggers if:

```
Margin < GRD
```

If triggered:

* Damage reduced by 50%
* No Stagger
* Defender loses 1 momentum

Purpose:

* Shields smooth volatility
* Cost tempo
* Do not negate all damage

---

# 7. Status Effects

Short-duration, non-locking.

| Effect       | Description                 |
| ------------ | --------------------------- |
| Stagger      | -1 tile movement next turn  |
| Off-Balance  | -2 DEF next turn            |
| Bleeding     | 2 damage per turn (3 turns) |
| Guard Broken | GRD ignored next hit        |

No stun-lock mechanics.

---

# 8. Weapon Differentiation

Weapons modify:

* Accuracy (ATK modifier)
* Power (WeaponBonus)
* Margin behavior
* Momentum interactions
* Guard interaction
* Risk profile

---

## Example Archetypes

### Rapier (Agile)

* +2 Accuracy
* +1 Power
* Scramble deals 50% instead of 25%
* Solid+ grants +1 extra momentum

Identity: Momentum-focused, precision fighter.

---

### Greataxe (Heavy)

* -1 Accuracy
* +5 Power
* Heroic+: Apply Bleeding
* Wild Blunder: Become Off-Balance

Identity: Explosive but risky.

---

### Spear (Reach)

* +2 Power
* Reach 2 tiles
* +3 margin on first attack vs approaching enemy
* -2 penalty when attacking adjacent

Identity: Positional control.

---

### Dagger (Opportunist)

* +1 Accuracy
* If target Staggered/Off-Balance: +4 margin
* Scramble applies minor Bleed

Identity: Combo finisher.

---

# 9. Armor Differentiation

Armor modifies:

* ARM (flat reduction)
* DEF modifier
* Margin interaction
* Mobility
* Blunder interaction

---

## Armor Archetypes

### Cloth

* ARM 1–2
* DEF +2
* No mobility penalty
* Overextended: no DEF penalty
* Spectacular: +2 damage taken

Identity: Volatile, evasive.

---

### Leather

* ARM 3–4
* Scramble reduced to 0 damage
* Heroic does not apply Stagger

Identity: Smooth survivability.

---

### Chain

* ARM 5
* DEF -1
* Heroic multiplier reduced to 1.25
* Wild Blunder penalties reduced
* -1 movement

Identity: Spike-resistant.

---

### Plate

* ARM 7
* DEF -2
* GRD +2
* Solid/Scramble take -2 additional damage
* -2 movement

Identity: Small-hit immunity, low mobility.

---

# 10. Retreat Mechanics

Disengage action:

If enemy Momentum ≤ 0:

* Free disengage

If enemy Momentum ≥ 1:

* Enemy makes Reaction Roll:

```
2d10 + ATK - Defender DEF
```

If margin ≥ 5:

* Free hit
  Otherwise:
* Escape successful

Armor mobility penalty applies to escape roll.

Design intent:

* Retreat possible
* Risk increases when losing
* Encourages momentum management

---

# 11. Combat Flow Philosophy

This system ensures:

* Most turns matter
* Failure causes board-state problems, not deletion
* Big hits feel earned
* Armor changes survivability profile
* Weapons shape rhythm
* Momentum creates narrative arc
* Death comes from spirals, not one roll

---

# 12. Target Combat Metrics

| Metric                   | Target     |
| ------------------------ | ---------- |
| Average duel length      | 5–8 turns  |
| Solid hit damage         | ~25% HP    |
| Heroic hit damage        | ~35% HP    |
| Spectacular damage       | ~45–50% HP |
| Blunders causing HP loss | Rare       |

---

# 13. Core Identity Summary

This combat system is:

* Unified (single margin engine)
* Swingy but fair
* Positionally meaningful
* Momentum-driven
* Weapon expressive
* Armor expressive
* Recoverable
* Story-generating

# 14. Tuning Levers

If fights are too lethal:
* Increase HP
* Reduce Heroic multiplier (1.5 → 1.4)
* Narrow Heroic band (e.g. 14–17 instead of 12–17)
* Increase ARM values

If fights are too slow:
* Increase POW baseline
* Reduce ARM
* Expand Heroic band downward
* Increase Spectacular multiplier to 2.2

If chaos feels too strong:
* Switch to 2d8 (less variance)
* Raise Solid upper bound
* Reduce Spectacular band

If chaos feels too weak:
* Switch to d20
* Expand Spectacular band
* Increase momentum cap to +4
