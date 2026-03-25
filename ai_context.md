# AI Context - Hotline Miami Clone

## 🎯 PROJECT GOAL
**Score Attack Arcade Game** — one polished level, release on itch.io for feedback.
Focus: **gameplay mechanics > content**

## 📋 CURRENT STATUS
- ✅ Basic gameplay (movement, attack, enemies, collision, AI)
- ✅ Score system with combo multiplier
- ✅ Message-based architecture
- ✅ UI (score, combo, timer)
- ✅ Game Over + Restart system
- ⚠️ In progress: polishing game feel (camera effects, feedback)

## 🔧 TECH STACK
- **Bevy 0.17.3** (current requirement, but open to upgrades if beneficial)
- **bevy_ecs_ldtk 0.13** for level loading
- **LDtk editor** for level design
- **Message/Event system** for decoupled systems
- **AABB collision** (custom implementation)

## 📐 ARCHITECTURE PRINCIPLES

### System Decoupling
**Prefer event-based communication over direct mutations.**

Use Messages/Events to:
- Communicate between systems without tight coupling
- Trigger reactions (score update, sound, particles) from game events
- Avoid per-frame condition checks

❌ Avoid: Checking game state every frame when an event can notify you.
✅ Prefer: Send message when action happens, let systems react.

### Configuration Management
**All game balance values in centralized resource.**
- Use `GameSettings` resource for constants (speeds, ranges, colors)
- Naming: `SCREAMING_SNAKE_CASE` for compile-time constants
- Single source of truth for tuning

### UI Architecture
**Separate camera layer for UI.**
- UI camera renders above game camera
- Mark UI entities for easy querying
- Update UI from resource state, not game state directly

### Modular Organization
**Group by responsibility, not by file size.**
- Core systems: movement, combat, collision
- Resource management: score, game state, settings
- Utility: camera, level loading, UI

## 🚫 SCOPE CONTROL
**Avoid feature creep. Focus on core gameplay loop.**

Don't suggest unless asked:
- Story/plot elements
- Content variety (new enemies, weapons, items)
- Multiple levels (we have duplicates - focus on ONE polished level)
- Meta-systems (achievements, progression, settings menu)

Focus on:
- Game feel (feedback, impact, juice)
- Core mechanic polish (score/combo system)
- One complete, satisfying gameplay loop

## 🎮 CORE GAMEPLAY LOOP
```
Spawn enemies → Player kills → Score + combo × multiplier
↓
Fast kills → Combo grows (×1.0 → ×3.0 max, 3s timer)
↓
Take damage → Combo reset penalty
↓
Player dies → Game Over → Restart (R key)
```

## 💡 NAMING CONVENTIONS
- Systems: `snake_case` (`player_movement`)
- Components/Structs: `PascalCase` (`Player`, `Enemy`)
- Resources: `PascalCase` (`GameState`, `ScoreState`)
- Constants: `SCREAMING_SNAKE_CASE` (`PLAYER_SPEED`)

## 🔍 DEVELOPMENT APPROACH
- Use `info!()` for debugging game events
- Test mechanics incrementally
- Verify APIs exist in current Bevy version before suggesting
- Check newer library versions for improvements, but ensure compatibility before upgrades
- Prefer simple, working solutions over complex, optimal ones

## 📚 REFERENCE
- Bevy docs: https://docs.rs/bevy/0.17.3/bevy/
- Project docs: `bevy_cheatbook.md`, `main_goal.md`
