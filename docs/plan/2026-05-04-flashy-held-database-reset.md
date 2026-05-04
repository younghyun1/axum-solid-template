# Flashy Held Database Reset

## Goal
- Replace the plain database reset trigger with a conspicuous destructive-action control.
- Require a continuous five-second press before calling the reset endpoint.
- Remove the Home button from the admin database reset interface.

## Tasks
- [x] Inspect the current admin database panel structure and shared button styles.
- [x] Implement hold-to-confirm reset behavior with progress feedback.
- [x] Add focused red/glowing destructive-action styling.
- [x] Remove the database-panel Home action.
- [x] Run frontend build and tests.
