# Todo
- [ ] show foreign key constraints
- [ ] show "references" columns
- [ ] add check constraint
- [ ] show unique constraints that have non-default name
- [ ] show primary key constraint that has a non-default name
- [ ] show foreign key constraint that has a non-default name

# Backlog
- [ ] handle deferrable / non-defferrable constraints
- [ ] user-defined types
- [ ] collate
- [ ] Sensible quoting
- [ ] views
- [ ] constraints (column constraints and table constraints)
  - In order to enable this, we will first need to identify the right
    constraint. Then join on constraint key usage, and constraint column
    usage.
  - It might make sense to pre-join. Moving everything together by 
    key and column. Then creating little tree structs for related values
    for each key thing. 
  - a column constraint has three references: 
    - a key column usage (the origin column in the table)
    - a table constraint usage (the table this is constraining)
    - a table constraint
  - We'll need to join these things together to send it over for formatting.

- [ ] use saner type name: "double precision -> float8"
- [ ] handle arrays of arrays
- [ ] handle user defined types
- [ ] handle arrays of user defined types

# Done
- [x] handle array types
- [x] show primary key
- [x] show unique constraints
