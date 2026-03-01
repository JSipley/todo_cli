#Task Refactor Specification

## Field Updates

1. Add **ID** field: Associate every task with an identification that shall be a byte-size hexadecimal valie (i.e. 3D, 2F). Add this field to the top of the Task struct. The **ID** field shall be immutable after the task's creation.
1. **Due_Date** field shall be formatted as _MM/DD/YYYY_. Any input that doesn't match this format should be marked as invalid. Prompt the user to enter a due date with the valid format and ask them to try again if input is not formatted.
	- Special case #1: If the user inputs a due date that's deliniated with a special character other than '/', accept input as valid and reformat the input using '/' as the delimiter (example: 11-11-2025 == 11/11/2025)
	- Special case #2: If the user enters a number that can be formatted into a valid date, then format the number inputted by the user (example: 01062026 == 01/06/2026)
1. Remove the **Important** field
1. Add a **Priority** field that stores a collection of options for the user to choose from. Prompt user to select the following options to set the priority field: ASAP, `Important`, `Medium`, `Minor`, or `None`. None will be the default option is the user doesn't select a priority.
1. Add a **Notes** field; This field will store text.

## New Commands

### Edit

Provide an interface to edit existing tasks. command usage shall look something like this `edit <ID#>`. Retrive the task with the associated ID and allow the user to edit. If the user inputs an invalid ID, notify the user that the ID entered was invalid and display the list of tasks with their assigned ID next to the task description. If the user calls `edit`, display the same task list with ID.

### Help

The **Help** command presents the user with the list of available commands, along with a brief description of what they do.

Example: 
```
New: Create a new task
Edit: Modify an existing task
```

## Styling

A task shall be displayed with its information with the styling depicted below:

```
────────────────────────────────────────

[TASK] ([Priority])
[Notes]
[Date]

────────────────────────────────────────
```

If possible, print the notes content in a slightly smaller font than normal console font size. **Do not do this** if there is no simple solution.