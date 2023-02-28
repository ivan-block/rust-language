SHADOWING.
Shadowing helps eliminate the compile time error that would occur if the let keyword was not used. Shadowing can also help when it is not the same data type.

DATA TYPES.
Rust is a statically typed language, so it must know the data type at compile time and this has to be inputted manually by the programmer. The data type consists of two types; scalar(represents single values) and compound(represents multiple values).
Examples of scalar are integers(signed or unsigned), characters(alphabets, symbols or emojis), booleans(true or false) and floats[decimal integers(f32 or f64*)].
Examples of compound are arrays and tuples.

TUPLES.
Tuples are used to group multiple data types into a compound result. It cannot grow in siz once the values have been fixed (fixed length). It can be printed onto the CLI in two ways;

- Using the position of the values.
- Assigning a placeholder before printing the values.

ARRAYS.
Arrays are use to collect multiple values of data with the same data type. Arrays also have a fixed length. An array can be written in two ways;

- Specifying the data type along with the fixed length of the values to be used.
- Specifying the value along with the length of the values.
  Arrays are mostly considered where values are fixed and do not need to be added to, like the months of the year.

VECTORS.
Vectors are similar to arrays. Difference is vectors do not a have a fixed length, so new values can be added.

FUNCTIONS.
Functions are used to store lines of code which will then be called in the main function. For example, an arithmetic expression can be written in a function called add, and then the add function gets called in the main function to be executed. Functions can have parameters and the data type has to be declared to function properly. In a case where a value or sets of values are to be returned the data type also has to be inputted ( -> i32, &str).

EXPRESSIONS.
Expressions are used to compared conditions and print out a statement if one of the conditions is met.

STRUCTS.
Structs(short for structures) are used to group set of values together, which can be of different data types into a single and meaningful group. Structs can also be used as tuple structs or without any fields.

METHOD SYNTAX.
Methods is similar to functions, but unlike functions, it is defined within the context of a struct(or an enum or the trait of an object), and has parameters, but the parameter always starts with self because it is relating to the struct that was defined before it or is being called on. Point to note is the Rust allows you to use the self tag without the type Self which is abbreviated and allowed in the first parameter spot, and also note that the & is used because it is borrowing fron the instance of the struct.

ENUMS.
Enums(Short for enumerations) helps you define a type by enumerating its possible variants (a possible set of values). One way to define an enum is using the Option, which expresses that a value can either be something or nothing. You can put any type of data in an enum, for example, strings, integers and even structs. An enum can also be in an enum.
