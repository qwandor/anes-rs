# master

* Fix `MoveCursorTo` sequence
  * column, row swapped

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
