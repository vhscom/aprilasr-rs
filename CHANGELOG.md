## Version 0.2.0 (2024-02-27)

### Features

- **Multiple Sessions Support:** Introduces a new feature allowing the creation of multiple sessions for the same `Model`. The `Session` struct has been enhanced, updating the reference handling from an `Arc` to a simple lifetime reference (`&'a Model`). This modification enables users to create and manage multiple sessions concurrently, providing improved flexibility and concurrent processing capabilities.

## Version 0.1.2 (2024-02-15)

### Changes

- Internal improvements and bug fixes.

## Version 0.1.1 (2024-02-14)

### Changes

- Internal adjustments and enhancements.

## Version 0.1.0 (2024-02-13)

### Initial Release

- The initial release of the project. Basic functionality and structure introduced.
