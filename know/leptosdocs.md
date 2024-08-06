смотри - почитаю здесь, что есть вообще на этой странице и если тебе нужно будет расширить какую либо информацию на этой страница, ну, например, посчитаешь нужным триггер сделать, то горишь мне, и я открываю этот триггер, где уже более подробно о нем рассказано, начиная с сигнатуры pub fn create_trigger() -> Trigger и далее более расширенная информация о необходимом элементе из leptos

Crate leptos_reactiveCopy item path
source · [−]
The reactive system for the Leptos Web framework.

Fine-Grained Reactivity
Leptos is built on a fine-grained reactive system, which means that individual reactive values (“signals,” sometimes known as observables) trigger the code that reacts to them (“effects,” sometimes known as observers) to re-run. These two halves of the reactive system are inter-dependent. Without effects, signals can change within the reactive system but never be observed in a way that interacts with the outside world. Without signals, effects run once but never again, as there’s no observable value to subscribe to.

Here are the most commonly-used functions and types you’ll need to build a reactive system:

Signals
Signals: create_signal, which returns a (ReadSignal, WriteSignal tuple, or create_rw_signal, which returns a signal RwSignal without this read-write segregation.
Derived Signals: any function that relies on another signal.
Memos: create_memo, which returns a Memo.
Resources: create_resource, which converts an async Future into a synchronous Resource signal.
Triggers: create_trigger, creates a purely reactive Trigger primitive without any associated state.
Effects
Use create_effect when you need to synchronize the reactive system with something outside it (for example: logging to the console, writing to a file or local storage)
The Leptos DOM renderer wraps any Fn in your template with create_effect, so components you write do not need explicit effects to synchronize with the DOM.
Example
use leptos_reactive::*;

// creates a new reactive runtime
// this is omitted from most of the examples in the docs
// you usually won't need to call it yourself
let runtime = create_runtime();
// a signal: returns a (getter, setter) pair
let (count, set_count) = create_signal(0);

// calling the getter gets the value
// can be `count()` on nightly
assert_eq!(count.get(), 0);
// calling the setter sets the value
// can be `set_count(1)` on nightly
set_count.set(1);
// or we can mutate it in place with update()
set_count.update(|n| *n += 1);

// a derived signal: a plain closure that relies on the signal
// the closure will run whenever we *access* double_count()
let double_count = move || count.get() * 2;
assert_eq!(double_count(), 4);

// a memo: subscribes to the signal
// the closure will run only when count changes
let memoized_triple_count = create_memo(move |_| count.get() * 3);
// can be `memoized_triple_count()` on nightly
assert_eq!(memoized_triple_count.get(), 6);

// this effect will run whenever `count` changes
create_effect(move |_| {
    println!("Count = {}", count.get());
});

// disposes of the reactive runtime
runtime.dispose();
Re-exports
pub use diagnostics::SpecialNonReactiveZone;
pub use hydration::SharedContext;
pub use runtime::untrack_with_diagnostics;
pub use suspense::GlobalSuspenseContext;
pub use suspense::SuspenseContext;
pub use oco_ref as oco;
pub use callback::*;
Modules
callback	Callbacks define a standard way to store functions and closures. They are useful for component properties, because they can be used to define optional callback functions, which generic props don’t support.
prelude	This prelude imports all signal types as well as all signal traits needed to use those types.
signal_prelude	This prelude imports all signal types as well as all signal traits needed to use those types.
suspense	Types that handle asynchronous data loading via <Suspense/>.
Macros
update	Provides a simpler way to use SignalUpdate::update.
update_value	Provides a simpler way to use StoredValue::update_value.
with	Provides a simpler way to use SignalWith::with.
with_value	Provides a simpler way to use StoredValue::with_value.
Structs
Disposer	Handle to dispose of a reactive node.
Effect	A handle to an effect, can be used to explicitly dispose of the effect.
FragmentData	Represents its pending <Suspense/> fragment.
MaybeProp	A wrapping type for an optional component prop, which can either be a signal or a non-reactive value, and which may or may not have a value. In other words, this is an Option<MaybeSignal<Option<T>>> that automatically flattens its getters.
Memo	An efficient derived reactive value based on other reactive values.
Owner	A reactive owner.
ReadSignal	The getter for a reactive signal.
Resource	A signal that reflects the current state of an asynchronous task, allowing you to integrate async Futures into the synchronous reactive system.
ResourceId	Unique ID assigned to a Resource.
RuntimeId	Unique ID assigned to a Runtime.
RwSignal	A signal that combines the getter and setter into one value, rather than separating them into a ReadSignal and a WriteSignal. You may prefer this its style, or it may be easier to pass around in a context or as a function argument.
ScopedFuture	Allows running a future that has access to a given scope.
Selector	A conditional signal that only notifies subscribers when a change in the source signal’s value changes whether the given function is true.
Signal	A wrapper for any kind of readable reactive signal: a ReadSignal, Memo, RwSignal, or derived signal closure.
SignalSetter	A wrapper for any kind of settable reactive signal: a WriteSignal, RwSignal, or closure that receives a value and sets a signal depending on it.
StoredValue	A non-reactive wrapper for any value, which can be created with store_value.
TextProp	Describes a value that is either a static or a reactive string, i.e., a String, a &str, or a reactive Fn() -> String.
Trigger	Reactive Trigger, notifies reactive code to rerun.
WriteSignal	The setter for a reactive signal.
Enums
FromUtf8Error	Error returned from Oco::try_from for unsuccessful conversion from Oco<'_, [u8]> to Oco<'_, str>.
MaybeSignal	A wrapper for a value that is either T or Signal<T>.
Oco	“Owned Clones Once”: a smart pointer that can be either a reference, an owned value, or a reference-counted pointer. This is useful for storing immutable values, such as strings, in a way that is cheap to clone and pass around.
SerializationError	Describes errors that can occur while serializing and deserializing data, typically during the process of streaming Resources from the server to the client.
Traits
IntoSignal	Helper trait for converting Fn() -> T closures into Signal<T>.
IntoSignalSetter	Helper trait for converting Fn(T) into SignalSetter<T>.
Serializable	Describes an object that can be serialized to or from a supported format Currently those are JSON and Cbor
SignalDispose	This trait allows disposing a signal before its owner has been disposed.
SignalGet	This trait allows getting an owned value of the signals inner type.
SignalGetUntracked	Trait implemented for all signal types which you can get a value from, such as ReadSignal, Memo, etc., which allows getting the inner value without subscribing to the current scope.
SignalSet	This trait allows setting the value of a signal.
SignalSetUntracked	Trait implemented for all signal types which you can set the inner value, such as WriteSignal and RwSignal, which allows setting the inner value without causing effects which depend on the signal from being run.
SignalStream	This trait allows converting a signal into a async Stream.
SignalUpdate	This trait allows updating the inner value of a signal.
SignalUpdateUntracked	This trait allows updating the signals value without causing dependant effects to run.
SignalWith	This trait allows obtaining an immutable reference to the signal’s inner type.
SignalWithUntracked	This trait allows getting a reference to the signals inner value without creating a dependency on the signal.
Functions
as_child_of_current_owner	Wraps the given function so that, whenever it is called, it creates a child node owned by whichever reactive node was the owner when it was created, runs the function, and returns a disposer that can be used to dispose of the child later.
batch	Batches any reactive updates, preventing effects from running until the whole function has run. This allows you to prevent rerunning effects if multiple signal updates might cause the same effect to run.
create_blocking_resource	Creates a “blocking” Resource. When server-side rendering is used, this resource will cause any <Suspense/> you read it under to block the initial chunk of HTML from being sent to the client. This means that if you set things like HTTP headers or <head> metadata in that <Suspense/>, that header material will be included in the server’s original response.
create_effect	Effects run a certain chunk of code whenever the signals they depend on change. create_effect queues the given function to run once, tracks its dependence on any signal values read within it, and reruns the function whenever the value of a dependency changes.
create_isomorphic_effect	Creates an effect; unlike effects created by create_effect, isomorphic effects will run on the server as well as the client.
create_local_resource	Creates a local Resource, which is a signal that reflects the current state of an asynchronous task, allowing you to integrate async Futures into the synchronous reactive system.
create_local_resource_with_initial_value	Creates a local Resource with the given initial value, which will only generate and run a Future using the fetcher when the source changes.
create_memo	Creates an efficient derived reactive value based on other reactive values.
create_owning_memo	Like create_memo, create_owning_memo creates an efficient derived reactive value based on other reactive values, but with two differences:
create_read_slice	Takes a memoized, read-only slice of a signal. This is equivalent to the read-only half of create_slice.
create_render_effect	Creates an effect exactly like create_effect, but runs immediately rather than being queued until the end of the current microtask. This is mostly used inside the renderer but is available for use cases in which scheduling the effect for the next tick is not optimal.
create_resource	Creates a Resource, which is a signal that reflects the current state of an asynchronous task, allowing you to integrate async Futures into the synchronous reactive system.
create_resource_with_initial_value	Creates a Resource with the given initial value, which will only generate and run a Future using the fetcher when the source changes.
create_runtime	Creates a new reactive runtime and sets it as the current runtime.
create_rw_signal	Creates a reactive signal with the getter and setter unified in one value. You may prefer this style, or it may be easier to pass around in a context or as a function argument.
create_selector	Creates a conditional signal that only notifies subscribers when a change in the source signal’s value changes whether it is equal to the key value (as determined by PartialEq.)
create_selector_with_fn	Creates a conditional signal that only notifies subscribers when a change in the source signal’s value changes whether the given function is true.
create_signal	Creates a signal, the basic reactive primitive.
create_signal_from_stream	Creates a signal that always contains the most recent value emitted by a Stream. If the stream has not yet emitted a value since the signal was created, the signal’s value will be None.
create_slice	Derives a reactive slice of an RwSignal.
create_trigger	Creates a Trigger, a kind of reactive primitive.
create_write_slice	Creates a setter to access one slice of a signal. This is equivalent to the write-only half of create_slice.
current_runtime	The current reactive runtime.
expect_context	Extracts a context value of type T from the reactive system by traversing it upwards, beginning from the current reactive owner and iterating through its parents, if any. The context value should have been provided elsewhere using provide_context.
on_cleanup	Creates a cleanup function, which will be run when the current reactive owner is disposed.
provide_context	Provides a context value of type T to the current reactive node and all of its descendants. This can be consumed using use_context.
queue_microtask	The microtask is a short function which will run after the current task has completed its work and when there is no other code waiting to be run before control of the execution context is returned to the browser’s event loop.
run_as_child	Runs the given function as a child of the current Owner, once.
set_current_runtime	Sets the current reactive runtime.
spawn_local	Spawns and runs a thread-local Future in a platform-independent way.
spawn_local_with_current_owner	Runs a future that has access to the provided Owner’s scope context.
spawn_local_with_owner	Runs a future that has access to the provided Owner’s scope context.
store_value	Creates a non-reactive wrapper for any value by storing it within the reactive system.
try_batch	Attempts to batch any reactive updates, preventing effects from running until the whole function has run. This allows you to prevent rerunning effects if multiple signal updates might cause the same effect to run.
try_spawn_local_with_current_owner	Runs a future that has access to the provided Owner’s scope context.
try_spawn_local_with_owner	Runs a future that has access to the provided Owner’s scope context.
try_with_owner	Runs the given code with the given reactive owner.
untrack	Suspends reactive tracking while running the given function.
use_context	Extracts a context value of type T from the reactive system by traversing it upwards, beginning from the current reactive owner and iterating through its parents, if any. The context value should have been provided elsewhere using provide_context.
watch	A version of create_effect that listens to any dependency that is accessed inside deps and returns a stop handler.
with_current_owner	Wraps the given function so that, whenever it is called, it is run in the reactive scope of whatever the reactive owner was when it was created.
with_owner	Runs the given code with the given reactive owner.