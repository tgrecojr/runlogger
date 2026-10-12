# Navigation Specs

Design: `docs/intent/navigation/navigation-design.md` · Prefix: `NAV`

Status: `[x]` = implemented; `[ ]` = specified but not yet implemented; `[D]` = deferred. Tests: `src/input/tests.rs` (NAV-017).

## Lifecycle

- [x] **NAV-001**: When the app starts, the system shall open the run database before switching the terminal to raw mode and the alternate screen, and shall then show the Quick Entry screen.
- [x] **NAV-002**: When the app exits normally, the system shall leave raw mode and the alternate screen and show the cursor.
- [x] **NAV-003**: If the app panics, then the system shall leave raw mode and the alternate screen before the panic message is printed.
- [x] **NAV-004**: While the app is running, the system shall redraw the current screen after every key press and at least every 100 ms.

## Quitting

- [x] **NAV-005**: When the user presses Ctrl+Q or Ctrl+C on any screen, the system shall quit.
- [x] **NAV-006**: When the user presses `q` on any screen other than Quick Entry, the system shall quit.

## Esc Prefix

- [x] **NAV-007**: When the user presses Esc on any screen, the system shall treat the next key as a navigation command without changing the current screen.
- [x] **NAV-008**: When the user presses `1`, `2` or `3` as the key after Esc, the system shall switch to the Quick Entry, Run List or Analytics screen respectively, loading the Run List or Analytics data on the switch.
- [x] **NAV-009**: When the user presses Esc as the key after Esc while the Quick Entry screen is shown, the system shall clear the Quick Entry form.
- [x] **NAV-010**: When the user presses a key other than `1`, `2`, `3`, Esc, `h` or `?` as the key after Esc, the system shall ignore that key and stay on the current screen.
- [ ] **NAV-018**: When the user presses `h` or `?` as the key after Esc, on any screen, the system shall show the Help screen.

## Direct Keys

- [x] **NAV-011**: While a screen other than Quick Entry is shown, when the user presses `1`, `2` or `3`, the system shall switch to the Quick Entry, Run List or Analytics screen respectively, loading the Run List or Analytics data on the switch; when the user presses `h` or `?`, the system shall show the Help screen.
- [x] **NAV-012**: While the Quick Entry screen is shown, the system shall send every key other than Ctrl+Q, Ctrl+C and Esc to the Quick Entry form.

## Shared Chrome

- [x] **NAV-013**: The system shall show a header titled "Run Logger" listing `[1] Quick Entry`, `[2] Run List`, `[3] Analytics` and `[h] Help`, with the current screen's tab in bold yellow and the others in gray.
- [x] **NAV-014**: The system shall show a footer with a line of key hints specific to the current screen.
- [x] **NAV-015**: The Help screen shall describe navigation keys, Quick Entry keys, Run List keys, the 1-mile daily goal, usage tips, and the run database location.
- [ ] **NAV-016**: Every key hint shown in the footer and on the Help screen shall describe what that key actually does on the screen it is shown for.

## Failures

- [x] **NAV-017**: If a storage operation triggered by a key press fails, then the system shall keep running, complete any screen switch the key requested, and leave the error display to the screen that owns the operation, ending the app only on a quit command or a terminal input/output failure.
