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
- [ ] handle the following data types:
      smallint, 2 bytes, small-range integer, -32768 to +32767
      integer, 4 bytes, typical choice for integer, -2147483648 to +2147483647
      bigint, 8 bytes, large-range integer, -9223372036854775808 to +9223372036854775807
      decimal, variable, user-specified precision, exact, up to 131072 digits before the decimal point; up to 16383 digits after the decimal point
      numeric, variable, user-specified precision, exact, up to 131072 digits before the decimal point; up to 16383 digits after the decimal point
      real, 4 bytes, variable-precision, inexact, 6 decimal digits precision
      double precision, 8 bytes, variable-precision, inexact, 15 decimal digits precision
      smallserial, 2 bytes, small autoincrementing integer, 1 to 32767
      serial, 4 bytes, autoincrementing integer, 1 to 2147483647
      bigserial, 8 bytes, large autoincrementing integer, 1 to 9223372036854775807

# Done
- [x] handle array types
- [x] show primary key
- [x] show unique constraints
- [x] show varchar lengths
