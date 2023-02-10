Enums(enumerations) allows you to define more than one possible sets of values.

An enum can have;

1. A variant with no associated data.
2. A variant with naming fields.
3. A variant with strings.
4. A variant with integers(s).

Options enums (option<T>, where T can be of any type) describes a scenario where a function could be something or nothing.
If a value is not specified for the None option, Rust requires us to annotate the data type, e.g option<i32> or option<char>
