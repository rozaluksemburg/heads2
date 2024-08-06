// The type argument can be on the right of the equal sign.
let act = Action::<Add, _>::server();
let args = Add { lhs: 5, rhs: 7 };
act.dispatch(args);
assert_eq!(act.value().get(), Some(Ok(12)));

// Or on the left of the equal sign.
let act: Action<Sub, _> = Action::server();
let args = Sub { lhs: 20, rhs: 5 };
act.dispatch(args);
assert_eq!(act.value().get(), Some(Ok(15)));

let not_dispatched = Action::<Add, _>::server();
assert_eq!(not_dispatched.value().get(), None);

#[server]
async fn add(lhs: u8, rhs: u8) -> Result<u8, ServerFnError> {
Ok(lhs + rhs)
}

#[server]
async fn sub(lhs: u8, rhs: u8) -> Result<u8, ServerFnError> {
Ok(lhs - rhs)
}
source
pub fn using_server_fn(self) -> Self
where
I::Error: Clone + 'static,
Associates the URL of the given server function with this action. This enables integration with the ActionForm component in leptos_router.

Trait Implementations
source
impl<I, O> Clone for Action<I, O>
where
I: 'static,
O: 'static,
source
fn clone(&self) -> Self
Returns a copy of the value. Read more
1.0.0 · source
fn clone_from(&mut self, source: &Self)
Performs copy-assignment from source. Read more
source
impl<I, O> Copy for Action<I, O>
where
I: 'static,
O: 'static,
Auto Trait Implementations
impl<I, O> Freeze for Action<I, O>
impl<I, O> !RefUnwindSafe for Action<I, O>
impl<I, O> !Send for Action<I, O>
impl<I, O> !Sync for Action<I, O>
impl<I, O> Unpin for Action<I, O>
where
I: Unpin,
O: Unpin,
impl<I, O> !UnwindSafe for Action<I, O>
Blanket Implementations
source
impl<T> Any for T
where
T: 'static + ?Sized,
source
impl<T> Borrow<T> for T
where
T: ?Sized,
source
impl<T> BorrowMut<T> for T
where
T: ?Sized,
source
impl<T> From<T> for T
source
impl<T> Instrument for T
source
impl<T, U> Into<U> for T
where
U: From<T>,
source
impl<T> ToOwned for T
where
T: Clone,
source
impl<T, U> TryFrom<U> for T
where
U: Into<T>,
source
impl<T, U> TryInto<U> for T
where
U: TryFrom<T>,
source
impl<T> WithSubscriber for T

Struct leptos_server::MultiActionCopy item path
source · [−]
pub struct MultiAction<I, O>(/* private fields */)
where
I: 'static,
O: 'static;
An action that synchronizes multiple imperative async calls to the reactive system, tracking the progress of each one.

Where an Action fires a single call, a MultiAction allows you to keep track of multiple in-flight actions.

If you’re trying to load data by running an async function reactively, you probably want to use a Resource instead. If you’re trying to occasionally run an async function in response to something like a user adding a task to a todo list, you’re in the right place.

async fn send_new_todo_to_api(task: String) -> usize {
// do something...
// return a task id
42
}
let add_todo = create_multi_action(|task: &String| {
// `task` is given as `&String` because its value is available in `input`
send_new_todo_to_api(task.clone())
});

add_todo.dispatch("Buy milk".to_string());
add_todo.dispatch("???".to_string());
add_todo.dispatch("Profit!!!".to_string());
The input to the async function should always be a single value, but it can be of any type. The argument is always passed by reference to the function, because it is stored in Submission::input as well.

// if there's a single argument, just use that
let action1 = create_multi_action(|input: &String| {
let input = input.clone();
async move { todo!() }
});

// if there are no arguments, use the unit type `()`
let action2 = create_multi_action(|input: &()| async { todo!() });

// if there are multiple arguments, use a tuple
let action3 =
create_multi_action(|input: &(usize, String)| async { todo!() });
Implementations
source
impl<I, O> MultiAction<I, O>
where
I: 'static,
O: 'static,
source
pub fn dispatch(&self, input: I)
Calls the async function with a reference to the input type as its argument.

source
pub fn submissions(&self) -> ReadSignal<Vec<Submission<I, O>>>
The set of all submissions to this multi-action.

source
pub fn url(&self) -> Option<String>
The URL associated with the action (typically as part of a server function.) This enables integration with the MultiActionForm component in leptos_router.

source
pub fn version(&self) -> RwSignal<usize>
How many times an action has successfully resolved.

source
pub fn using_server_fn<T: ServerFn>(self) -> Self
Associates the URL of the given server function with this action. This enables integration with the MultiActionForm component in leptos_router.

Trait Implementations
source
impl<I, O> Clone for MultiAction<I, O>
where
I: 'static,
O: 'static,
source
fn clone(&self) -> Self
Returns a copy of the value. Read more
1.0.0 · source
fn clone_from(&mut self, source: &Self)
Performs copy-assignment from source. Read more
source
impl<I, O> Copy for MultiAction<I, O>
where
I: 'static,
O: 'static,
Auto Trait Implementations
impl<I, O> Freeze for MultiAction<I, O>
impl<I, O> !RefUnwindSafe for MultiAction<I, O>
impl<I, O> !Send for MultiAction<I, O>
impl<I, O> !Sync for MultiAction<I, O>
impl<I, O> Unpin for MultiAction<I, O>
where
I: Unpin,
O: Unpin,
impl<I, O> !UnwindSafe for MultiAction<I, O>
Blanket Implementations
source
impl<T> Any for T
where
T: 'static + ?Sized,
source
impl<T> Borrow<T> for T
where
T: ?Sized,
source
impl<T> BorrowMut<T> for T
where
T: ?Sized,
source
impl<T> From<T> for T
source
impl<T> Instrument for T
source
impl<T, U> Into<U> for T
where
U: From<T>,
source
impl<T> ToOwned for T
where
T: Clone,
source
impl<T, U> TryFrom<U> for T
where
U: Into<T>,
source
impl<T, U> TryInto<U> for T
where
U: TryFrom<T>,
source
impl<T> WithSubscriber for T

Struct leptos_server::SubmissionCopy item path
source · [−]
pub struct Submission<I, O>
where
I: 'static,
O: 'static,
{
pub input: RwSignal<Option<I>>,
pub value: RwSignal<Option<O>>,
pub canceled: RwSignal<bool>,
/* private fields */
}
An action that has been submitted by dispatching it to a MultiAction.

Fields
input: RwSignal<Option<I>>
The current argument that was dispatched to the async function. Some while we are waiting for it to resolve, None if it has resolved.

value: RwSignal<Option<O>>
The most recent return value of the async function.

canceled: RwSignal<bool>
Controls this submission has been canceled.

Implementations
source
impl<I, O> Submission<I, O>
where
I: 'static,
O: 'static,
source
pub fn pending(&self) -> ReadSignal<bool>
Whether this submission is currently waiting to resolve.

source
pub fn cancel(&self)
Cancels the submission, preventing it from resolving.

Trait Implementations
source
impl<I, O> Clone for Submission<I, O>
source
fn clone(&self) -> Self
Returns a copy of the value. Read more
1.0.0 · source
fn clone_from(&mut self, source: &Self)
Performs copy-assignment from source. Read more
source
impl<I, O> Copy for Submission<I, O>
Auto Trait Implementations
impl<I, O> Freeze for Submission<I, O>
impl<I, O> RefUnwindSafe for Submission<I, O>
where
I: RefUnwindSafe,
O: RefUnwindSafe,
impl<I, O> Send for Submission<I, O>
where
I: Send,
O: Send,
impl<I, O> Sync for Submission<I, O>
where
I: Sync,
O: Sync,
impl<I, O> Unpin for Submission<I, O>
where
I: Unpin,
O: Unpin,
impl<I, O> UnwindSafe for Submission<I, O>
where
I: UnwindSafe,
O: UnwindSafe,
Blanket Implementations
source
impl<T> Any for T
where
T: 'static + ?Sized,
source
impl<T> Borrow<T> for T
where
T: ?Sized,
source
impl<T> BorrowMut<T> for T
where
T: ?Sized,
source
impl<T> From<T> for T
source
impl<T> Instrument for T
source
impl<T, U> Into<U> for T
where
U: From<T>,
source
impl<T> ToOwned for T
where
T: Clone,
source
impl<T, U> TryFrom<U> for T
where
U: Into<T>,
source
impl<T, U> TryInto<U> for T
where
U: TryFrom<T>,
source
impl<T> WithSubscriber for T

Enum leptos_server::ServerFnErrorCopy item path
source · [−]
pub enum ServerFnError<E = NoCustomError> {
WrappedServerError(E),
Registration(String),
Request(String),
Response(String),
ServerError(String),
Deserialization(String),
Serialization(String),
Args(String),
MissingArg(String),
}
Type for errors that can occur when using server functions.

Unlike ServerFnErrorErr, this does not implement Error. This means that other error types can easily be converted into it using the ? operator.

Variants
WrappedServerError(E)
A user-defined custom error type, which defaults to NoCustomError.

Registration(String)
Error while trying to register the server function (only occurs in case of poisoned RwLock).

Request(String)
Occurs on the client if there is a network error while trying to run function on server.

Response(String)
Occurs on the server if there is an error creating an HTTP response.

ServerError(String)
Occurs when there is an error while actually running the function on the server.

Deserialization(String)
Occurs on the client if there is an error deserializing the server’s response.

Serialization(String)
Occurs on the client if there is an error serializing the server function arguments.

Args(String)
Occurs on the server if there is an error deserializing one of the arguments that’s been sent.

MissingArg(String)
Occurs on the server if there’s a missing argument.

Implementations
source
impl ServerFnError
source
pub fn new(msg: impl ToString) -> ServerFnError
Constructs a new ServerFnError::ServerError from some other type.

Trait Implementations
source
impl<E> Clone for ServerFnError<E>
where
E: Clone,
source
fn clone(&self) -> ServerFnError<E>
Returns a copy of the value. Read more
1.0.0 · source
fn clone_from(&mut self, source: &Self)
Performs copy-assignment from source. Read more
source
impl<E> Debug for ServerFnError<E>
where
E: Debug,
source
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
Formats the value using the given formatter. Read more
source
impl<'de, E> Deserialize<'de> for ServerFnError<E>
where
E: Deserialize<'de>,
source
fn deserialize<__D>(
__deserializer: __D
) -> Result<ServerFnError<E>, <__D as Deserializer<'de>>::Error>
where
__D: Deserializer<'de>,
Deserialize this value from the given Serde deserializer. Read more
source
impl<CustErr> Display for ServerFnError<CustErr>
where
CustErr: Display,
source
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
Formats the value using the given formatter. Read more
source
impl<E> Error for ServerFnError<E>
where
E: Error + 'static,
ServerFnError<E>: Display,
source
fn source(&self) -> Option<&(dyn Error + 'static)>
The lower-level source of this error, if any. Read more
1.0.0 · source
fn description(&self) -> &str
👎Deprecated since 1.42.0: use the Display impl or to_string()
Read more
1.0.0 · source
fn cause(&self) -> Option<&dyn Error>
👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting
source
fn provide<'a>(&'a self, request: &mut Request<'a>)
🔬This is a nightly-only experimental API. (error_generic_member_access)
Provides type based access to context intended for error reports. Read more
source
impl<CustErr> From<CustErr> for ServerFnError<CustErr>
source
fn from(value: CustErr) -> ServerFnError<CustErr>
Converts to this type from the input type.
source
impl<E> From<E> for ServerFnError
where
E: Error,
source
fn from(value: E) -> ServerFnError
Converts to this type from the input type.
source
impl<CustErr> From<ServerFnError<CustErr>> for ServerFnErrorErr<CustErr>
source
fn from(value: ServerFnError<CustErr>) -> ServerFnErrorErr<CustErr>
Converts to this type from the input type.
source
impl<CustErr> From<ServerFnUrlError<CustErr>> for ServerFnError<CustErr>
source
fn from(error: ServerFnUrlError<CustErr>) -> ServerFnError<CustErr>
Converts to this type from the input type.
source
impl<E> PartialEq for ServerFnError<E>
where
E: PartialEq,
source
fn eq(&self, other: &ServerFnError<E>) -> bool
This method tests for self and other values to be equal, and is used by ==.
1.0.0 · source
fn ne(&self, other: &Rhs) -> bool
This method tests for !=. The default implementation is almost always sufficient, and should not be overridden without very good reason.
source
impl<E> Serialize for ServerFnError<E>
where
E: Serialize,
source
fn serialize<__S>(
&self,
__serializer: __S
) -> Result<<__S as Serializer>::Ok, <__S as Serializer>::Error>
where
__S: Serializer,
Serialize this value into the given Serde serializer. Read more
source
impl<CustErr> ServerFnErrorSerde for ServerFnError<CustErr>
where
CustErr: FromStr + Display,
source
fn ser(&self) -> Result<String, Error>
Converts the custom error type to a String.
source
fn de(data: &str) -> ServerFnError<CustErr>
Deserializes the custom error type from a String.
source
impl<E> Eq for ServerFnError<E>
where
E: Eq,
source
impl<E> StructuralPartialEq for ServerFnError<E>
Auto Trait Implementations
impl<E> Freeze for ServerFnError<E>
where
E: Freeze,
impl<E> RefUnwindSafe for ServerFnError<E>
where
E: RefUnwindSafe,
impl<E> Send for ServerFnError<E>
where
E: Send,
impl<E> Sync for ServerFnError<E>
where
E: Sync,
impl<E> Unpin for ServerFnError<E>
where
E: Unpin,
impl<E> UnwindSafe for ServerFnError<E>
where
E: UnwindSafe,
Blanket Implementations
source
impl<T> Any for T
where
T: 'static + ?Sized,
source
impl<T> Borrow<T> for T
where
T: ?Sized,
source
impl<T> BorrowMut<T> for T
where
T: ?Sized,
source
impl<Q, K> Equivalent<K> for Q
where
Q: Eq + ?Sized,
K: Borrow<Q> + ?Sized,
source
impl<Q, K> Equivalent<K> for Q
where
Q: Eq + ?Sized,
K: Borrow<Q> + ?Sized,
source
impl<T> From<!> for T
source
impl<T> From<T> for T
source
impl<CustErr, T, Request> FromReq<Cbor, Request, CustErr> for T
where
Request: Req<CustErr> + Send + 'static,
T: DeserializeOwned,
source
impl<CustErr, T, Request> FromReq<Json, Request, CustErr> for T
where
Request: Req<CustErr> + Send + 'static,
T: DeserializeOwned,
source
impl<CustErr, T, Request> FromReq<Streaming, Request, CustErr> for T
where
Request: Req<CustErr> + Send + 'static,
T: From<ByteStream> + 'static,
source
impl<CustErr, T, Request> FromReq<StreamingText, Request, CustErr> for T
where
Request: Req<CustErr> + Send + 'static,
T: From<TextStream> + 'static,
source
impl<CustErr, T, Response> FromRes<Cbor, Response, CustErr> for T
where
Response: ClientRes<CustErr> + Send,
T: DeserializeOwned + Send,
source
impl<CustErr, T, Response> FromRes<Json, Response, CustErr> for T
where
Response: ClientRes<CustErr> + Send,
T: DeserializeOwned + Send,
source
impl<T> Instrument for T
source
impl<T, U> Into<U> for T
where
U: From<T>,
source
impl<CustErr, T, Request> IntoReq<Cbor, Request, CustErr> for T
where
Request: ClientReq<CustErr>,
T: Serialize + Send,
source
impl<CustErr, T, Request> IntoReq<Json, Request, CustErr> for T
where
Request: ClientReq<CustErr>,
T: Serialize + Send,
source
impl<CustErr, T, Response> IntoRes<Cbor, Response, CustErr> for T
where
Response: Res<CustErr>,
T: Serialize + Send,
source
impl<CustErr, T, Response> IntoRes<Json, Response, CustErr> for T
where
Response: Res<CustErr>,
T: Serialize + Send,
source
impl<T> Serializable for T
where
T: DeserializeOwned + Serialize,
source
impl<T> ToOwned for T
where
T: Clone,
source
impl<T> ToString for T
where
T: Display + ?Sized,
source
impl<T, U> TryFrom<U> for T
where
U: Into<T>,
source
impl<T, U> TryInto<U> for T
where
U: TryFrom<T>,
source
impl<T> WithSubscriber for T
source
impl<T> DeserializeOwned for T
where
T: for<'de> Deserialize<'de>,

Enum leptos_server::ServerFnErrorErrCopy item path
source · [−]
pub enum ServerFnErrorErr<E = NoCustomError> {
WrappedServerError(E),
Registration(String),
Request(String),
ServerError(String),
Deserialization(String),
Serialization(String),
Args(String),
MissingArg(String),
Response(String),
}
Type for errors that can occur when using server functions.

Unlike ServerFnError, this implements std::error::Error. This means it can be used in situations in which the Error trait is required, but it’s not possible to create a blanket implementation that converts other errors into this type.

ServerFnError and ServerFnErrorErr mutually implement From, so it is easy to convert between the two types.

Variants
WrappedServerError(E)
A user-defined custom error type, which defaults to NoCustomError.

Registration(String)
Error while trying to register the server function (only occurs in case of poisoned RwLock).

Request(String)
Occurs on the client if there is a network error while trying to run function on server.

ServerError(String)
Occurs when there is an error while actually running the function on the server.

Deserialization(String)
Occurs on the client if there is an error deserializing the server’s response.

Serialization(String)
Occurs on the client if there is an error serializing the server function arguments.

Args(String)
Occurs on the server if there is an error deserializing one of the arguments that’s been sent.

MissingArg(String)
Occurs on the server if there’s a missing argument.

Response(String)
Occurs on the server if there is an error creating an HTTP response.

Trait Implementations
source
impl<E> Clone for ServerFnErrorErr<E>
where
E: Clone,
source
fn clone(&self) -> ServerFnErrorErr<E>
Returns a copy of the value. Read more
1.0.0 · source
fn clone_from(&mut self, source: &Self)
Performs copy-assignment from source. Read more
source
impl<E> Debug for ServerFnErrorErr<E>
where
E: Debug,
source
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
Formats the value using the given formatter. Read more
source
impl<E> Display for ServerFnErrorErr<E>
where
E: Display,
source
fn fmt(&self, __formatter: &mut Formatter<'_>) -> Result<(), Error>
Formats the value using the given formatter. Read more
source
impl<E> Error for ServerFnErrorErr<E>
where
ServerFnErrorErr<E>: Debug + Display,
1.30.0 · source
fn source(&self) -> Option<&(dyn Error + 'static)>
The lower-level source of this error, if any. Read more
1.0.0 · source
fn description(&self) -> &str
👎Deprecated since 1.42.0: use the Display impl or to_string()
Read more
1.0.0 · source
fn cause(&self) -> Option<&dyn Error>
👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting
source
fn provide<'a>(&'a self, request: &mut Request<'a>)
🔬This is a nightly-only experimental API. (error_generic_member_access)
Provides type based access to context intended for error reports. Read more
source
impl<CustErr> From<ServerFnError<CustErr>> for ServerFnErrorErr<CustErr>
source
fn from(value: ServerFnError<CustErr>) -> ServerFnErrorErr<CustErr>
Converts to this type from the input type.
source
impl<CustErr> From<ServerFnUrlError<CustErr>> for ServerFnErrorErr<CustErr>
source
fn from(error: ServerFnUrlError<CustErr>) -> ServerFnErrorErr<CustErr>
Converts to this type from the input type.
source
impl<E> PartialEq for ServerFnErrorErr<E>
where
E: PartialEq,
source
fn eq(&self, other: &ServerFnErrorErr<E>) -> bool
This method tests for self and other values to be equal, and is used by ==.
1.0.0 · source
fn ne(&self, other: &Rhs) -> bool
This method tests for !=. The default implementation is almost always sufficient, and should not be overridden without very good reason.
source
impl<E> Eq for ServerFnErrorErr<E>
where
E: Eq,
source
impl<E> StructuralPartialEq for ServerFnErrorErr<E>
Auto Trait Implementations
impl<E> Freeze for ServerFnErrorErr<E>
where
E: Freeze,
impl<E> RefUnwindSafe for ServerFnErrorErr<E>
where
E: RefUnwindSafe,
impl<E> Send for ServerFnErrorErr<E>
where
E: Send,
impl<E> Sync for ServerFnErrorErr<E>
where
E: Sync,
impl<E> Unpin for ServerFnErrorErr<E>
where
E: Unpin,
impl<E> UnwindSafe for ServerFnErrorErr<E>
where
E: UnwindSafe,
Blanket Implementations
source
impl<T> Any for T
where
T: 'static + ?Sized,
source
impl<T> Borrow<T> for T
where
T: ?Sized,
source
impl<T> BorrowMut<T> for T
where
T: ?Sized,
source
impl<Q, K> Equivalent<K> for Q
where
Q: Eq + ?Sized,
K: Borrow<Q> + ?Sized,
source
impl<Q, K> Equivalent<K> for Q
where
Q: Eq + ?Sized,
K: Borrow<Q> + ?Sized,
source
impl<T> From<T> for T
source
impl<T> Instrument for T
source
impl<T, U> Into<U> for T
where
U: From<T>,
source
impl<T> ToOwned for T
where
T: Clone,
source
impl<T> ToString for T
where
T: Display + ?Sized,
source
impl<T, U> TryFrom<U> for T
where
U: Into<T>,
source
impl<T, U> TryInto<U> for T
where
U: TryFrom<T>,
source
impl<T> WithSubscriber for T

Function leptos_server::create_actionCopy item path
source · [−]
pub fn create_action<I, O, F, Fu>(action_fn: F) -> Action<I, O>
where
I: 'static,
O: 'static,
F: Fn(&I) -> Fu + 'static,
Fu: Future<Output = O> + 'static,
Creates an Action to synchronize an imperative async call to the synchronous reactive system.

If you’re trying to load data by running an async function reactively, you probably want to use a create_resource instead. If you’re trying to occasionally run an async function in response to something like a user clicking a button, you’re in the right place.

async fn send_new_todo_to_api(task: String) -> usize {
// do something...
// return a task id
42
}
let save_data = create_action(|task: &String| {
// `task` is given as `&String` because its value is available in `input`
send_new_todo_to_api(task.clone())
});

// the argument currently running
let input = save_data.input();
// the most recent returned result
let result_of_call = save_data.value();
// whether the call is pending
let pending = save_data.pending();
// how many times the action has run
// useful for reactively updating something else in response to a `dispatch` and response
let version = save_data.version();

// before we do anything
assert_eq!(input.get(), None); // no argument yet
assert_eq!(pending.get(), false); // isn't pending a response
assert_eq!(result_of_call.get(), None); // there's no "last value"
assert_eq!(version.get(), 0);
// dispatch the action
save_data.dispatch("My todo".to_string());

// when we're making the call
// assert_eq!(input.get(), Some("My todo".to_string()));
// assert_eq!(pending.get(), true); // is pending
// assert_eq!(result_of_call.get(), None); // has not yet gotten a response

// after call has resolved
assert_eq!(input.get(), None); // input clears out after resolved
assert_eq!(pending.get(), false); // no longer pending
assert_eq!(result_of_call.get(), Some(42));
assert_eq!(version.get(), 1);
The input to the async function should always be a single value, but it can be of any type. The argument is always passed by reference to the function, because it is stored in Action::input as well.

// if there's a single argument, just use that
let action1 = create_action(|input: &String| {
let input = input.clone();
async move { todo!() }
});

// if there are no arguments, use the unit type `()`
let action2 = create_action(|input: &()| async { todo!() });

// if there are multiple arguments, use a tuple
let action3 = create_action(|input: &(usize, String)| async { todo!() });

Function leptos_server::create_multi_actionCopy item path
source · [−]
pub fn create_multi_action<I, O, F, Fu>(action_fn: F) -> MultiAction<I, O>
where
I: 'static,
O: 'static,
F: Fn(&I) -> Fu + 'static,
Fu: Future<Output = O> + 'static,
Creates an MultiAction to synchronize an imperative async call to the synchronous reactive system.

If you’re trying to load data by running an async function reactively, you probably want to use a create_resource instead. If you’re trying to occasionally run an async function in response to something like a user clicking a button, you’re in the right place.

async fn send_new_todo_to_api(task: String) -> usize {
// do something...
// return a task id
42
}
let add_todo = create_multi_action(|task: &String| {
// `task` is given as `&String` because its value is available in `input`
send_new_todo_to_api(task.clone())
});

add_todo.dispatch("Buy milk".to_string());
add_todo.dispatch("???".to_string());
add_todo.dispatch("Profit!!!".to_string());

assert_eq!(add_todo.submissions().get().len(), 3);
The input to the async function should always be a single value, but it can be of any type. The argument is always passed by reference to the function, because it is stored in Submission::input as well.

// if there's a single argument, just use that
let action1 = create_multi_action(|input: &String| {
let input = input.clone();
async move { todo!() }
});

// if there are no arguments, use the unit type `()`
let action2 = create_multi_action(|input: &()| async { todo!() });

// if there are multiple arguments, use a tuple
let action3 =
create_multi_action(|input: &(usize, String)| async { todo!() });

Function leptos_server::create_server_actionCopy item path
source · [−]
pub fn create_server_action<S>(
) -> Action<S, Result<S::Output, ServerFnError<S::Error>>>
where
S: Clone + ServerFn,
S::Error: Clone + 'static,
Creates an Action that can be used to call a server function.


#[server(MyServerFn)]
async fn my_server_fn() -> Result<(), ServerFnError> {
todo!()
}

let my_server_action = create_server_action::<MyServerFn>();

Function leptos_server::create_server_multi_actionCopy item path
source · [−]
pub fn create_server_multi_action<S>(
) -> MultiAction<S, Result<S::Output, ServerFnError<S::Error>>>
where
S: Clone + ServerFn,
Creates an MultiAction that can be used to call a server function.

ⓘ

#[server(MyServerFn)]
async fn my_server_fn() -> Result<(), ServerFnError> {
todo!()
}

let my_server_multi_action = create_server_multi_action::<MyServerFn>();


Working with the Server
The previous section described the process of server-side rendering, using the server to generate an HTML version of the page that will become interactive in the browser. So far, everything has been “isomorphic”; in other words, your app has had the “same (iso) shape (morphe)” on the client and the server.

But a server can do a lot more than just render HTML! In fact, a server can do a whole bunch of things your browser can’t, like reading from and writing to a SQL database.

If you’re used to building JavaScript frontend apps, you’re probably used to calling out to some kind of REST API to do this sort of server work. If you’re used to building sites with PHP or Python or Ruby (or Java or C# or...), this server-side work is your bread and butter, and it’s the client-side interactivity that tends to be an afterthought.

With Leptos, you can do both: not only in the same language, not only sharing the same types, but even in the same files!

This section will talk about how to build the uniquely-server-side parts of your application.


Server Functions
If you’re creating anything beyond a toy app, you’ll need to run code on the server all the time: reading from or writing to a database that only runs on the server, running expensive computations using libraries you don’t want to ship down to the client, accessing APIs that need to be called from the server rather than the client for CORS reasons or because you need a secret API key that’s stored on the server and definitely shouldn’t be shipped down to a user’s browser.

Traditionally, this is done by separating your server and client code, and by setting up something like a REST API or GraphQL API to allow your client to fetch and mutate data on the server. This is fine, but it requires you to write and maintain your code in multiple separate places (client-side code for fetching, server-side functions to run), as well as creating a third thing to manage, which is the API contract between the two.

Leptos is one of a number of modern frameworks that introduce the concept of server functions. Server functions have two key characteristics:

Server functions are co-located with your component code, so that you can organize your work by feature, not by technology. For example, you might have a “dark mode” feature that should persist a user’s dark/light mode preference across sessions, and be applied during server rendering so there’s no flicker. This requires a component that needs to be interactive on the client, and some work to be done on the server (setting a cookie, maybe even storing a user in a database.) Traditionally, this feature might end up being split between two different locations in your code, one in your “frontend” and one in your “backend.” With server functions, you’ll probably just write them both in one dark_mode.rs and forget about it.
Server functions are isomorphic, i.e., they can be called either from the server or the browser. This is done by generating code differently for the two platforms. On the server, a server function simply runs. In the browser, the server function’s body is replaced with a stub that actually makes a fetch request to the server, serializing the arguments into the request and deserializing the return value from the response. But on either end, the function can simply be called: you can create an add_todo function that writes to your database, and simply call it from a click handler on a button in the browser!
Using Server Functions
Actually, I kind of like that example. What would it look like? It’s pretty simple, actually.

// todo.rs

#[server(AddTodo, "/api")]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
let mut conn = db().await?;

    match sqlx::query("INSERT INTO todos (title, completed) VALUES ($1, false)")
        .bind(title)
        .execute(&mut conn)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[component]
pub fn BusyButton() -> impl IntoView {
view! {
<button on:click=move |_| {
spawn_local(async {
add_todo("So much to do!".to_string()).await;
});
}>
"Add Todo"
</button>
}
}
You’ll notice a couple things here right away:

Server functions can use server-only dependencies, like sqlx, and can access server-only resources, like our database.
Server functions are async. Even if they only did synchronous work on the server, the function signature would still need to be async, because calling them from the browser must be asynchronous.
Server functions return Result<T, ServerFnError>. Again, even if they only do infallible work on the server, this is true, because ServerFnError’s variants include the various things that can be wrong during the process of making a network request.
Server functions can be called from the client. Take a look at our click handler. This is code that will only ever run on the client. But it can call the function add_todo (using spawn_local to run the Future) as if it were an ordinary async function:
move |_| {
spawn_local(async {
add_todo("So much to do!".to_string()).await;
});
}
Server functions are top-level functions defined with fn. Unlike event listeners, derived signals, and most everything else in Leptos, they are not closures! As fn calls, they have no access to the reactive state of your app or anything else that is not passed in as an argument. And again, this makes perfect sense: When you make a request to the server, the server doesn’t have access to client state unless you send it explicitly. (Otherwise we’d have to serialize the whole reactive system and send it across the wire with every request, which—while it served classic ASP for a while—is a really bad idea.)
Server function arguments and return values both need to be serializable with serde. Again, hopefully this makes sense: while function arguments in general don’t need to be serialized, calling a server function from the browser means serializing the arguments and sending them over HTTP.
There are a few things to note about the way you define a server function, too.

Server functions are created by using the #[server] macro to annotate a top-level function, which can be defined anywhere.
We provide the macro a type name. The type name is used internally as a container to hold, serialize, and deserialize the arguments.
We provide the macro a path. This is a prefix for the path at which we’ll mount a server function handler on our server. (See examples for Actix and Axum.)
You’ll need to have serde as a dependency with the derive featured enabled for the macro to work properly. You can easily add it to Cargo.toml with cargo add serde --features=derive.
Server Function URL Prefixes
You can optionally define a specific URL prefix to be used in the definition of the server function. This is done by providing an optional 2nd argument to the #[server] macro. By default the URL prefix will be /api, if not specified. Here are some examples:

#[server(AddTodo)]         // will use the default URL prefix of `/api`
#[server(AddTodo, "/foo")] // will use the URL prefix of `/foo`
Server Function Encodings
By default, the server function call is a POST request that serializes the arguments as URL-encoded form data in the body of the request. (This means that server functions can be called from HTML forms, which we’ll see in a future chapter.) But there are a few other methods supported. Optionally, we can provide another argument to the #[server] macro to specify an alternate encoding:

#[server(AddTodo, "/api", "Url")]
#[server(AddTodo, "/api", "GetJson")]
#[server(AddTodo, "/api", "Cbor")]
#[server(AddTodo, "/api", "GetCbor")]
The four options use different combinations of HTTP verbs and encoding methods:

Name	Method	Request	Response
Url (default)	POST	URL encoded	JSON
GetJson	GET	URL encoded	JSON
Cbor	POST	CBOR	CBOR
GetCbor	GET	URL encoded	CBOR
In other words, you have two choices:

GET or POST? This has implications for things like browser or CDN caching; while POST requests should not be cached, GET requests can be.
Plain text (arguments sent with URL/form encoding, results sent as JSON) or a binary format (CBOR, encoded as a base64 string)?
But remember: Leptos will handle all the details of this encoding and decoding for you. When you use a server function, it looks just like calling any other asynchronous function!

Why not PUT or DELETE? Why URL/form encoding, and not JSON?

These are reasonable questions. Much of the web is built on REST API patterns that encourage the use of semantic HTTP methods like DELETE to delete an item from a database, and many devs are accustomed to sending data to APIs in the JSON format.

The reason we use POST or GET with URL-encoded data by default is the <form> support. For better or for worse, HTML forms don’t support PUT or DELETE, and they don’t support sending JSON. This means that if you use anything but a GET or POST request with URL-encoded data, it can only work once WASM has loaded. As we’ll see in a later chapter, this isn’t always a great idea.

The CBOR encoding is supported for historical reasons; an earlier version of server functions used a URL encoding that didn’t support nested objects like structs or vectors as server function arguments, which CBOR did. But note that the CBOR forms encounter the same issue as PUT, DELETE, or JSON: they do not degrade gracefully if the WASM version of your app is not available.

Server Functions Endpoint Paths
By default, a unique path will be generated. You can optionally define a specific endpoint path to be used in the URL. This is done by providing an optional 4th argument to the #[server] macro. Leptos will generate the complete path by concatenating the URL prefix (2nd argument) and the endpoint path (4th argument). For example,

#[server(MyServerFnType, "/api", "Url", "hello")]
will generate a server function endpoint at /api/hello that accepts a POST request.

Can I use the same server function endpoint path with multiple encodings?

No. Different server functions must have unique paths. The #[server] macro automatically generates unique paths, but you need to be careful if you choose to specify the complete path manually, as the server looks up server functions by their path.

An Important Note on Security
Server functions are a cool technology, but it’s very important to remember. Server functions are not magic; they’re syntax sugar for defining a public API. The body of a server function is never made public; it’s just part of your server binary. But the server function is a publicly accessible API endpoint, and its return value is just a JSON or similar blob. Do not return information from a server function unless it is public, or you've implemented proper security procedures. These procedures might include authenticating incoming requests, ensuring proper encryption, rate limiting access, and more.

Integrating Server Functions with Leptos
So far, everything I’ve said is actually framework agnostic. (And in fact, the Leptos server function crate has been integrated into Dioxus as well!) Server functions are simply a way of defining a function-like RPC call that leans on Web standards like HTTP requests and URL encoding.

But in a way, they also provide the last missing primitive in our story so far. Because a server function is just a plain Rust async function, it integrates perfectly with the async Leptos primitives we discussed earlier. So you can easily integrate your server functions with the rest of your applications:

Create resources that call the server function to load data from the server
Read these resources under <Suspense/> or <Transition/> to enable streaming SSR and fallback states while data loads.
Create actions that call the server function to mutate data on the server
The final section of this book will make this a little more concrete by introducing patterns that use progressively-enhanced HTML forms to run these server actions.

But in the next few chapters, we’ll actually take a look at some of the details of what you might want to do with your server functions, including the best ways to integrate with the powerful extractors provided by the Actix and Axum server frameworks.


Extractors
The server functions we looked at in the last chapter showed how to run code on the server, and integrate it with the user interface you’re rendering in the browser. But they didn’t show you much about how to actually use your server to its full potential.

Server Frameworks
We call Leptos a “full-stack” framework, but “full-stack” is always a misnomer (after all, it never means everything from the browser to your power company.) For us, “full stack” means that your Leptos app can run in the browser, and can run on the server, and can integrate the two, drawing together the unique features available in each; as we’ve seen in the book so far, a button click on the browser can drive a database read on the server, both written in the same Rust module. But Leptos itself doesn’t provide the server (or the database, or the operating system, or the firmware, or the electrical cables...)

Instead, Leptos provides integrations for the two most popular Rust web server frameworks, Actix Web (leptos_actix) and Axum (leptos_axum). We’ve built integrations with each server’s router so that you can simply plug your Leptos app into an existing server with .leptos_routes(), and easily handle server function calls.

If you haven’t seen our Actix and Axum templates, now’s a good time to check them out.

Using Extractors
Both Actix and Axum handlers are built on the same powerful idea of extractors. Extractors “extract” typed data from an HTTP request, allowing you to access server-specific data easily.

Leptos provides extract helper functions to let you use these extractors directly in your server functions, with a convenient syntax very similar to handlers for each framework.

Actix Extractors
The extract function in leptos_actix takes a handler function as its argument. The handler follows similar rules to an Actix handler: it is an async function that receives arguments that will be extracted from the request and returns some value. The handler function receives that extracted data as its arguments, and can do further async work on them inside the body of the async move block. It returns whatever value you return back out into the server function.

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MyQuery {
foo: String,
}

#[server]
pub async fn actix_extract() -> Result<String, ServerFnError> {
use actix_web::dev::ConnectionInfo;
use actix_web::web::{Data, Query};
use leptos_actix::extract;

    let (Query(search), connection): (Query<MyQuery>, ConnectionInfo) = extract().await?;
    Ok(format!("search = {search:?}\nconnection = {connection:?}",))
}
Axum Extractors
The syntax for the leptos_axum::extract function is very similar.

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MyQuery {
foo: String,
}

#[server]
pub async fn axum_extract() -> Result<String, ServerFnError> {
use axum::{extract::Query, http::Method};
use leptos_axum::extract;

    let (method, query): (Method, Query<MyQuery>) = extract().await?;

    Ok(format!("{method:?} and {query:?}"))
}
These are relatively simple examples accessing basic data from the server. But you can use extractors to access things like headers, cookies, database connection pools, and more, using the exact same extract() pattern.

The Axum extract function only supports extractors for which the state is (). If you need an extractor that uses State, you should use extract_with_state. This requires you to provide the state. You can do this by extending the existing LeptosOptions state using the Axum FromRef pattern, which providing the state as context during render and server functions with custom handlers.

use axum::extract::FromRef;

/// Derive FromRef to allow multiple items in state, using Axum’s
/// SubStates pattern.
#[derive(FromRef, Debug, Clone)]
pub struct AppState{
pub leptos_options: LeptosOptions,
pub pool: SqlitePool
}
Click here for an example of providing context in custom handlers.

Axum State
Axum's typical pattern for dependency injection is to provide a State, which can then be extracted in your route handler. Leptos provides its own method of dependency injection via context. Context can often be used instead of State to provide shared server data (for example, a database connection pool).

let connection_pool = /* some shared state here */;

let app = Router::new()
.leptos_routes_with_context(
&app_state,
routes,
move || provide_context(connection_pool.clone()),
App,
)
// etc.
This context can then be accessed with a simple use_context::<T>() inside your server functions.

If you need to use State in a server function—for example, if you have an existing Axum extractor that requires State, that is also possible using Axum's FromRef pattern and extract_with_state. Essentially you'll need to provide the state both via context and via Axum router state:

#[derive(FromRef, Debug, Clone)]
pub struct MyData {
pub value: usize,
pub leptos_options: LeptosOptions,
}

let app_state = MyData {
value: 42,
leptos_options,
};

// build our application with a route
let app = Router::new()
.leptos_routes_with_context(
&app_state,
routes,
{
let app_state = app_state.clone();
move || provide_context(app_state.clone());
},
App,
)
.fallback(file_and_error_handler)
.with_state(app_state);

// ...
#[server]
pub async fn uses_state() -> Result<(), ServerFnError> {
let state = expect_context::<AppState>();
let SomeStateExtractor(data) = extract_with_state(&state).await?;
// todo
}
A Note about Data-Loading Patterns
Because Actix and (especially) Axum are built on the idea of a single round-trip HTTP request and response, you typically run extractors near the “top” of your application (i.e., before you start rendering) and use the extracted data to determine how that should be rendered. Before you render a <button>, you load all the data your app could need. And any given route handler needs to know all the data that will need to be extracted by that route.

But Leptos integrates both the client and the server, and it’s important to be able to refresh small pieces of your UI with new data from the server without forcing a full reload of all the data. So Leptos likes to push data loading “down” in your application, as far towards the leaves of your user interface as possible. When you click a <button>, it can refresh just the data it needs. This is exactly what server functions are for: they give you granular access to data to be loaded and reloaded.

The extract() functions let you combine both models by using extractors in your server functions. You get access to the full power of route extractors, while decentralizing knowledge of what needs to be extracted down to your individual components. This makes it easier to refactor and reorganize routes: you don’t need to specify all the data a route needs up front.


Responses and Redirects
Extractors provide an easy way to access request data inside server functions. Leptos also provides a way to modify the HTTP response, using the ResponseOptions type (see docs for Actix or Axum) types and the redirect helper function (see docs for Actix or Axum).

ResponseOptions
ResponseOptions is provided via context during the initial server rendering response and during any subsequent server function call. It allows you to easily set the status code for the HTTP response, or to add headers to the HTTP response, e.g., to set cookies.

#[server(TeaAndCookies)]
pub async fn tea_and_cookies() -> Result<(), ServerFnError> {
use actix_web::{cookie::Cookie, http::header, http::header::HeaderValue};
use leptos_actix::ResponseOptions;

    // pull ResponseOptions from context
    let response = expect_context::<ResponseOptions>();

    // set the HTTP status code
    response.set_status(StatusCode::IM_A_TEAPOT);

    // set a cookie in the HTTP response
    let mut cookie = Cookie::build("biscuits", "yes").finish();
    if let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) {
        response.insert_header(header::SET_COOKIE, cookie);
    }
}
redirect
One common modification to an HTTP response is to redirect to another page. The Actix and Axum integrations provide a redirect function to make this easy to do. redirect simply sets an HTTP status code of 302 Found and sets the Location header.

Here’s a simplified example from our session_auth_axum example.

#[server(Login, "/api")]
pub async fn login(
username: String,
password: String,
remember: Option<String>,
) -> Result<(), ServerFnError> {
// pull the DB pool and auth provider from context
let pool = pool()?;
let auth = auth()?;

    // check whether the user exists
    let user: User = User::get_from_username(username, &pool)
        .await
        .ok_or_else(|| {
            ServerFnError::ServerError("User does not exist.".into())
        })?;

    // check whether the user has provided the correct password
    match verify(password, &user.password)? {
        // if the password is correct...
        true => {
            // log the user in
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());

            // and redirect to the home page
            leptos_axum::redirect("/");
            Ok(())
        }
        // if not, return an error
        false => Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        )),
    }
}
This server function can then be used from your application. This redirect works well with the progressively-enhanced <ActionForm/> component: without JS/WASM, the server response will redirect because of the status code and header. With JS/WASM, the <ActionForm/> will detect the redirect in the server function response, and use client-side navigation to redirect to the new page.


Progressive Enhancement (and Graceful Degradation)
I’ve been driving around Boston for about fifteen years. If you don’t know Boston, let me tell you: Massachusetts has some of the most aggressive drivers(and pedestrians!) in the world. I’ve learned to practice what’s sometimes called “defensive driving”: assuming that someone’s about to swerve in front of you at an intersection when you have the right of way, preparing for a pedestrian to cross into the street at any moment, and driving accordingly.

“Progressive enhancement” is the “defensive driving” of web design. Or really, that’s “graceful degradation,” although they’re two sides of the same coin, or the same process, from two different directions.

Progressive enhancement, in this context, means beginning with a simple HTML site or application that works for any user who arrives at your page, and gradually enhancing it with layers of additional features: CSS for styling, JavaScript for interactivity, WebAssembly for Rust-powered interactivity; using particular Web APIs for a richer experience if they’re available and as needed.

Graceful degradation means handling failure gracefully when parts of that stack of enhancement aren’t available. Here are some sources of failure your users might encounter in your app:

Their browser doesn’t support WebAssembly because it needs to be updated.
Their browser can’t support WebAssembly because browser updates are limited to newer OS versions, which can’t be installed on the device. (Looking at you, Apple.)
They have WASM turned off for security or privacy reasons.
They have JavaScript turned off for security or privacy reasons.
JavaScript isn’t supported on their device (for example, some accessibility devices only support HTML browsing)
The JavaScript (or WASM) never arrived at their device because they walked outside and lost WiFi.
They stepped onto a subway car after loading the initial page and subsequent navigations can’t load data.
... and so on.
How much of your app still works if one of these holds true? Two of them? Three?

If the answer is something like “95%... okay, then 90%... okay, then 75%,” that’s graceful degradation. If the answer is “my app shows a blank screen unless everything works correctly,” that’s... rapid unscheduled disassembly.

Graceful degradation is especially important for WASM apps, because WASM is the newest and least-likely-to-be-supported of the four languages that run in the browser (HTML, CSS, JS, WASM).

Luckily, we’ve got some tools to help.

Defensive Design
There are a few practices that can help your apps degrade more gracefully:

Server-side rendering. Without SSR, your app simply doesn’t work without both JS and WASM loading. In some cases this may be appropriate (think internal apps gated behind a login) but in others it’s simply broken.
Native HTML elements. Use HTML elements that do the things that you want, without additional code: <a> for navigation (including to hashes within the page), <details> for an accordion, <form> to persist information in the URL, etc.
URL-driven state. The more of your global state is stored in the URL (as a route param or part of the query string), the more of the page can be generated during server rendering and updated by an <a> or a <form>, which means that not only navigations but state changes can work without JS/WASM.
SsrMode::PartiallyBlocked or SsrMode::InOrder. Out-of-order streaming requires a small amount of inline JS, but can fail if 1) the connection is broken halfway through the response or 2) the client’s device doesn’t support JS. Async streaming will give a complete HTML page, but only after all resources load. In-order streaming begins showing pieces of the page sooner, in top-down order. “Partially-blocked” SSR builds on out-of-order streaming by replacing <Suspense/> fragments that read from blocking resources on the server. This adds marginally to the initial response time (because of the O(n) string replacement work), in exchange for a more complete initial HTML response. This can be a good choice for situations in which there’s a clear distinction between “more important” and “less important” content, e.g., blog post vs. comments, or product info vs. reviews. If you choose to block on all the content, you’ve essentially recreated async rendering.
Leaning on <form>s. There’s been a bit of a <form> renaissance recently, and it’s no surprise. The ability of a <form> to manage complicated POST or GET requests in an easily-enhanced way makes it a powerful tool for graceful degradation. The example in the <Form/> chapter, for example, would work fine with no JS/WASM: because it uses a <form method="GET"> to persist state in the URL, it works with pure HTML by making normal HTTP requests and then progressively enhances to use client-side navigations instead.
There’s one final feature of the framework that we haven’t seen yet, and which builds on this characteristic of forms to build powerful applications: the <ActionForm/>.


<ActionForm/>
<ActionForm/> is a specialized <Form/> that takes a server action, and automatically dispatches it on form submission. This allows you to call a server function directly from a <form>, even without JS/WASM.

The process is simple:

Define a server function using the #[server] macro (see Server Functions.)
Create an action using create_server_action, specifying the type of the server function you’ve defined.
Create an <ActionForm/>, providing the server action in the action prop.
Pass the named arguments to the server function as form fields with the same names.
Note: <ActionForm/> only works with the default URL-encoded POST encoding for server functions, to ensure graceful degradation/correct behavior as an HTML form.

#[server(AddTodo, "/api")]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
todo!()
}

#[component]
fn AddTodo() -> impl IntoView {
let add_todo = create_server_action::<AddTodo>();
// holds the latest *returned* value from the server
let value = add_todo.value();
// check if the server has returned an error
let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    view! {
        <ActionForm action=add_todo>
            <label>
                "Add a Todo"
                // `title` matches the `title` argument to `add_todo`
                <input type="text" name="title"/>
            </label>
            <input type="submit" value="Add"/>
        </ActionForm>
    }
}
It’s really that easy. With JS/WASM, your form will submit without a page reload, storing its most recent submission in the .input() signal of the action, its pending status in .pending(), and so on. (See the Action docs for a refresher, if you need.) Without JS/WASM, your form will submit with a page reload. If you call a redirect function (from leptos_axum or leptos_actix) it will redirect to the correct page. By default, it will redirect back to the page you’re currently on. The power of HTML, HTTP, and isomorphic rendering mean that your <ActionForm/> simply works, even with no JS/WASM.

Client-Side Validation
Because the <ActionForm/> is just a <form>, it fires a submit event. You can use either HTML validation, or your own client-side validation logic in an on:submit. Just call ev.prevent_default() to prevent submission.

The FromFormData trait can be helpful here, for attempting to parse your server function’s data type from the submitted form.

let on_submit = move |ev| {
let data = AddTodo::from_event(&ev);
// silly example of validation: if the todo is "nope!", nope it
if data.is_err() || data.unwrap().title == "nope!" {
// ev.prevent_default() will prevent form submission
ev.prevent_default();
}
}
Complex Inputs
Server function arguments that are structs with nested serializable fields should make use of indexing notation of serde_qs.

use leptos::*;
use leptos_router::*;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct HeftyData {
first_name: String,
last_name: String,
}

#[component]
fn ComplexInput() -> impl IntoView {
let submit = Action::<VeryImportantFn, _>::server();

    view! {
      <ActionForm action=submit>
        <input type="text" name="hefty_arg[first_name]" value="leptos"/>
        <input
          type="text"
          name="hefty_arg[last_name]"
          value="closures-everywhere"
        />
        <input type="submit"/>
      </ActionForm>
    }
}

#[server]
async fn very_important_fn(
hefty_arg: HeftyData,
) -> Result<(), ServerFnError> {
assert_eq!(hefty_arg.first_name.as_str(), "leptos");
assert_eq!(hefty_arg.last_name.as_str(), "closures-everywhere");
Ok(())
}


