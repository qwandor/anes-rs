# master

* Add `ReportCursorPosition` sequence
* Add `EnableMouseEvents`, `DisableMouseEvents` sequences
* Add ANSI escape sequence parser (feature `parser`, disabled by default)
  * Add `Parser` engine
  * Add `KeyCode`, `KeyModifiers`, `Mouse`, `MouseButton`, `Sequence` types

# Version 0.1.5

* Add `execute!` macro to execute ANSI escape sequences
* Add `queue!` macro to queue ANSI escape sequences

# Version 0.1.4

* All sequences and related structures auto derive `Copy`, `Clone`, `Debug`, `Hash`, `Eq`, `PartialEq`
* Add `sgr!` macro for select graphic rendition sequences
* Add display attributes (bold, italic, ...) related sequences
  * `SetAttribute`
  * `ResetAttributes`
* Improve documentation  

# Version 0.1.3

* New color related sequences
  * `SetBackgroundColor`
  * `SetForegroundColor`

# Version 0.1.2

* Fix `MoveCursorTo` sequence
  * column, row swapped
* Add interactive test
  * Run it with `cargo run --bin interactive-test` 

# Version 0.1.1

* Introduce `sequence!` macro to allow to define custom ANSI sequences 

# Version 0.1.0
  
## Breaking Changes

- No values modification
  - `MoveCursorTo(1, 1)` moves the cursor to the top/left cell
    (previously `MoveCursorTo(0, 0)`)
- Rename all existing cursor related sequences
  - `Hide` -> `HideCursor`
  - `Show` -> `ShowCursor`
  - `EnableBlinking` -> `EnableCursorBlinking`
  - `DisableBlinking` -> `DisableCursorBlinking`
  - `MoveTo` -> `MoveCursorTo`
  - `MoveLeft` -> `MoveCursorLeft`
  - `MoveRight` -> `MoveCursorRight`
  - `MoveUp` -> `MoveCursorUp`
  - `MoveDown` -> `MoveCursorDown`

## New Sequences

- New buffer related sequences
  - `SwitchBufferToAlternate`
  - `SwitchBufferToNormal`
  - `ScrollBufferUp`
  - `ScrollBufferDown`
  - `ClearLine`
  - `ClearBuffer`
- New window related sequences
  - `ResizeTextArea`
- New cursor related sequences
  - `MoveCursorToColumn`
  - `MoveCursorToNextLine`
  - `MoveCursorToPreviousLine`
  
# Version 0.0.2

- Initial release
- New `cursor` module sequences 
  - `MoveTo`, `MoveUp`, `MoveDown`, `MoveLeft`, `MoveRight`
  - `Hide`, `Show`, `EnableBlinking`, `DisableBlinking`
