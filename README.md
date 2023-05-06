# Interactive Bill Manager

A mini Rust Project.

## Summary:

A mini command line bills/expenses manager that runs
interactively.

## User stories:

- Stage 1:
  - I want to add bills, including the name and amount owed.
  - I want to view existing bills.
- Stage 2:
  - I want to remove bills.
- Stage 3:
  - I want to edit existing bills.
  - I want to go back if I change my mind.

## Implementation notes:

- Used loop keyword to create an interactive menu.
- Each menu choice should be it's own function, so I can work on the
  the functionality for that menu in isolation.
- Used a vector to store the bills at stage 1 as it's the easiest way, but a
  hashmap was introduced instead at stages 2 and 3 for more efficient data lookups.

## Running the project

Run the following command in the root directory of the project.

```shell
  cargo run -q
```
