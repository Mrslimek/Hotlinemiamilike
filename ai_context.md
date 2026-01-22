# AI Context for Hotline Miami Clone in Bevy 0.17.3

## Project Overview
This is a minimalistic Hotline Miami clone built with Rust and Bevy, serving as a learning project for practicing Rust basics through game development.

## Critical Development Rules

### Bevy Version Requirement
- **STRICTLY USE BEVY 0.17.3 ONLY**
- All code suggestions, examples, and advice must be compatible with Bevy 0.17.3
- Do NOT suggest features, APIs, or patterns that were introduced in later versions or previous versions
- Do NOT use features that were removed from 0.17.3
- Always verify that any recommended API exists in Bevy 0.17.3

### Code Guidelines
- Keep code simple and educational - focus on practicing Rust basics
- Use clear, descriptive variable and function names
- Add comments explaining Rust concepts being practiced
- Prefer explicit types over type inference where it aids learning
- Demonstrate proper ownership, borrowing, and lifetime concepts

### Project Structure
- Modular codebase split into Rust modules following ECS principles
- Focus on systems, components, and resources
- Use ECS pattern properly (Entities, Components, Systems)
- Use `Single<T>` for queries that should have exactly one result (like player)
- Avoid fetching Entity IDs in queries when you don't need them
- Refer to `bevy_cheatbook.md` for Query patterns and best practices

## Current Project Status

## Known Issues / Future Improvements

### Potential Future Enhancements
- Add instructions on how to restart

- Add visual attack effects
- Add score system
- Add different enemy types
- Add walls/obstacles
- Add sound effects
- Add level progression

## Important Reminders for AI Assistant
- ALWAYS check if suggested APIs exist in Bevy 0.17.3
- If unsure about an API's availability, stick to documented 0.17.3 features
- Explain any non-obvious Rust concepts
- Keep learning progress in mind - don't overwhelm with advanced features
- Focus on code that demonstrates the basics the user is learning
