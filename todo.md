# Todo
- [ ] add check constraint
- [ ] add indexes
- [ ] show unique constraints that have non-default name
- [ ] show primary key constraint that has a non-default name
- [ ] show foreign key constraint that has a non-default name

# Backlog
- [ ] handle deferrable / non-defferrable constraints
- [ ] user-defined types
- [ ] collate
- [ ] Sensible quoting
- [ ] views
- [ ] handle arrays of arrays
- [ ] handle user defined types
- [ ] handle arrays of user defined types

# Done
- [x] handle array types
- [x] show primary key
- [x] show unique constraints
- [x] show varchar lengths
- [x] show foreign key constraints

# Icebox 
- [ ] show "references" columns
- [ ] use saner type name: "double precision -> float8"
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
