# Task Refactor Specification

## Field Updates

1. Add **ID** field: Associate every task with an identification that shall be a byte-size hexadecimal value (i.e. 3D, 2F). Add this field to the top of the Task struct. The **ID** field shall be immutable after the task's creation.
	- IDs are generated sequentially starting from `00`, incrementing by 1 for each new task (i.e. `00`, `01`, `02`, ..., `FF`).
	- IDs must be unique across all tasks. If all 256 values are exhausted, display an error and prevent new task creation.
	- The **ID** is stored as an `<ID>` element in XML and is read back on load to preserve the original value.
	- The `done` command shall accept a task ID (hex) rather than a 0-based index, consistent with how `edit` works.

1. **Due_Date** field shall be formatted as _MM/DD/YYYY_. Any input that doesn't match this format should be marked as invalid. Prompt the user to enter a due date with the valid format and ask them to try again if input is not formatted correctly. Re-prompt in a loop until valid input is received.
	- Special case #1: If the user inputs a due date delimited by `-`, `.`, or ` ` instead of `/`, accept the input as valid and reformat it using `/` as the delimiter (example: `11-11-2025` == `11/11/2025`).
	- Special case #2: If the user enters an 8-digit number with no delimiter, format it as MM/DD/YYYY (example: `01062026` == `01/06/2026`). Inputs with fewer than 8 digits are not accepted under this case.
	- Semantic validation: after formatting, verify that the month is in range 01–12 and the day is in range 01–31. Inputs that fail this check are invalid and should trigger a re-prompt.

1. Remove the **Important** field.

1. Add a **Priority** field. Prompt the user to select a priority by entering the corresponding number from the following menu:
	```
	1. ASAP
	2. Important
	3. Medium
	4. Minor
	5. None
	```
	`None` is the default if the user presses Enter without selecting. Re-prompt if the input is not a number in 1–5.

1. Add a **Notes** field; This field will store text. The user may leave it blank.

## XML Persistence

The following XML elements map to the updated Task fields. All elements are children of `<Task>`:

| Field       | XML Element     |
|-------------|-----------------|
| ID          | `<ID>`          |
| Description | `<Description>` |
| Due_Date    | `<Due_Date>`    |
| Priority    | `<Priority>`    |
| Notes       | `<Notes>`       |

When reading an existing XML file that is missing new fields, apply these defaults:
- `<ID>`: assign the next available sequential ID at load time.
- `<Priority>`: default to `None`.
- `<Notes>`: default to an empty string.

## New Commands

### Edit

Provide an interface to edit existing tasks. Command usage: `edit <ID>` where `<ID>` is the task's hex ID. Retrieve the task with the associated ID and present each field's current value as the prompt prefix. The user may press Enter to keep the current value for any field. The **ID** field is not editable.

If the user inputs an invalid or unrecognized ID, notify the user and display the list of tasks with their hex ID next to the description. If the user calls `edit` with no argument, display the same task list.

Due date validation rules (see Field Updates) apply when editing the Due_Date field.

### Help

The **Help** command presents the user with the list of available commands, along with a brief description of what they do.

Example:
```
New:  Create a new task
Edit: Modify an existing task
Done: Mark a task as complete
View: Display all tasks
Help: Show this help message
```

If the user enters an unknown command, display the help output instead of only an error message.

## Styling

A task shall be displayed with its information with the styling depicted below:

```
────────────────────────────────────────

[Description] ([Priority])
[Notes]
[Date]

────────────────────────────────────────
```

If possible, print the notes content in a slightly smaller font than normal console font size. **Do not do this** if there is no simple solution.
