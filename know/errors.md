PS C:\rust\projects\marketplace\new_heads\heads> cargo build
Compiling heads v0.1.0 (C:\rust\projects\marketplace\new_heads\heads)
error[E0432]: unresolved import `leptos_reactive`
--> src\context.rs:5:5
|
5 | use leptos_reactive::{create_scope, create_rw_signal, Scope};
|     ^^^^^^^^^^^^^^^ use of undeclared crate or module `leptos_reactive`

error[E0061]: this function takes 1 argument but 2 arguments were supplied
--> src\context.rs:16:22
|
16   |             history: create_rw_signal(cx, Vec::new()),
|                      ^^^^^^^^^^^^^^^^   ------------
|                                         | |
|                                         | unexpected argument of type `Vec<_>`
|                                         help: remove the extra argument
|
note: function defined here
--> C:\Users\Дмитрий\.cargo\registry\src\index.crates.io-6f17d22bba15001f\leptos_reactive-0.6.13\src\signal.rs:1156:8
|
1156 | pub fn create_rw_signal<T>(value: T) -> RwSignal<T> {
|        ^^^^^^^^^^^^^^^^

error[E0425]: cannot find function `run_scope` in this scope                                                   
--> src\context.rs:67:5
|
67 |     run_scope(runtime, |cx| {
|     ^^^^^^^^^ not found in this scope

Some errors have detailed explanations: E0061, E0425, E0432.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `heads` (bin "heads_bin") due to 3 previous errors
PS C:\rust\projects\marketplace\new_heads\heads>



2 search results for 'E0061':
E0061
Error code E0061 An invalid number of arguments was passed when calling a function. Erroneous code example: fn f(u: i32) {} f(); // error! The number of arguments passed to
Rust error codes index
E0061 E0062 E0063 E0067 E0069 E0070 E0071 E0072 E0073 E0074 E0075 E0076 E0077 E0080 E0081 E0084 E0087 E0088 E0089 E0090 E0091 E0092 E0093 E0094 E0106 E0107 E0109 E0110 E0116 E0117
Error code E0425
An unresolved name was used.

Erroneous code examples:

something_that_doesnt_exist::foo;
// error: unresolved name `something_that_doesnt_exist::foo`

// or:

trait Foo {
fn bar() {
Self; // error: unresolved name `Self`
}
}

// or:

let x = unknown_variable;  // error: unresolved name `unknown_variable`
ⓘ
Please verify that the name wasn't misspelled and ensure that the identifier being referred to is valid for the given situation. Example:

enum something_that_does_exist {
Foo,
}
Or:

mod something_that_does_exist {
pub static foo : i32 = 0i32;
}

something_that_does_exist::foo; // ok!
Or:

let unknown_variable = 12u32;
let x = unknown_variable; // ok!
If the item is not defined in the current module, it must be imported using a use statement, like so:

use foo::bar;
bar();
If the item you are importing is not defined in some super-module of the current module, then it must also be declared as public (e.g., pub fn).


E0425
Error code E0425 An unresolved name was used. Erroneous code examples: something_that_doesnt_exist::foo; // error: unresolved name `something_that_doesnt_exist::foo` // or: trait Foo { fn bar() { Self; // error: unresolved name `Self`


2 search results for 'E0432':
E0432
E0432 An import was unresolved. Erroneous code example: use something::Foo; // error: unresolved import `something::Foo`. In Rust 2015, paths in use statements are relative to the crate root. To import