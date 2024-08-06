Parent-Child Communication
You can think of your application as a nested tree of components. Each component handles its own local state and manages a section of the user interface, so components tend to be relatively self-contained.

Sometimes, though, you’ll want to communicate between a parent component and its child. For example, imagine you’ve defined a <FancyButton/> component that adds some styling, logging, or something else to a <button/>. You want to use a <FancyButton/> in your <App/> component. But how can you communicate between the two?

It’s easy to communicate state from a parent component to a child component. We covered some of this in the material on components and props. Basically if you want the parent to communicate to the child, you can pass a ReadSignal, a Signal, or even a MaybeSignal as a prop.

But what about the other direction? How can a child send notifications about events or state changes back up to the parent?

There are four basic patterns of parent-child communication in Leptos.


1. Pass a WriteSignal
   One approach is simply to pass a WriteSignal from the parent down to the child, and update it in the child. This lets you manipulate the state of the parent from the child.

#[component]
pub fn App() -> impl IntoView {
let (toggled, set_toggled) = create_signal(false);
view! {
<p>"Toggled? " {toggled}</p>
<ButtonA setter=set_toggled/>
}
}

#[component]
pub fn ButtonA(setter: WriteSignal<bool>) -> impl IntoView {
view! {
<button
on:click=move |_| setter.update(|value| *value = !*value)
>
"Toggle"
</button>
}
}
This pattern is simple, but you should be careful with it: passing around a WriteSignal can make it hard to reason about your code. In this example, it’s pretty clear when you read <App/> that you are handing off the ability to mutate toggled, but it’s not at all clear when or how it will change. In this small, local example it’s easy to understand, but if you find yourself passing around WriteSignals like this throughout your code, you should really consider whether this is making it too easy to write spaghetti code.

2. Use a Callback
   Another approach would be to pass a callback to the child: say, on_click.

#[component]
pub fn App() -> impl IntoView {
let (toggled, set_toggled) = create_signal(false);
view! {
<p>"Toggled? " {toggled}</p>
<ButtonB on_click=move |_| set_toggled.update(|value| *value = !*value)/>
}
}


#[component]
pub fn ButtonB(#[prop(into)] on_click: Callback<MouseEvent>) -> impl IntoView
{
view! {
<button on:click=on_click>
"Toggle"
</button>
}
}
You’ll notice that whereas <ButtonA/> was given a WriteSignal and decided how to mutate it, <ButtonB/> simply fires an event: the mutation happens back in <App/>. This has the advantage of keeping local state local, preventing the problem of spaghetti mutation. But it also means the logic to mutate that signal needs to exist up in <App/>, not down in <ButtonB/>. These are real trade-offs, not a simple right-or-wrong choice.

Note the way we use the Callback<In, Out> type. This is basically a wrapper around a closure Fn(In) -> Out that is also Copy and makes it easy to pass around.

We also used the #[prop(into)] attribute so we can pass a normal closure into on_click. Please see the chapter "into Props" for more details.

2.1 Use Closure instead of Callback
You can use a Rust closure Fn(MouseEvent) directly instead of Callback:

#[component]
pub fn App() -> impl IntoView {
let (toggled, set_toggled) = create_signal(false);
view! {
<p>"Toggled? " {toggled}</p>
<ButtonB on_click=move |_| set_toggled.update(|value| *value = !*value)/>
}
}


#[component]
pub fn ButtonB<F>(on_click: F) -> impl IntoView
where
F: Fn(MouseEvent) + 'static
{
view! {
<button on:click=on_click>
"Toggle"
</button>
}
}
The code is very similar in this case. On more advanced use-cases using a closure might require some cloning compared to using a Callback.

Note the way we declare the generic type F here for the callback. If you’re confused, look back at the generic props section of the chapter on components.

3. Use an Event Listener
   You can actually write Option 2 in a slightly different way. If the callback maps directly onto a native DOM event, you can add an on: listener directly to the place you use the component in your view macro in <App/>.

#[component]
pub fn App() -> impl IntoView {
let (toggled, set_toggled) = create_signal(false);
view! {
<p>"Toggled? " {toggled}</p>
// note the on:click instead of on_click
// this is the same syntax as an HTML element event listener
<ButtonC on:click=move |_| set_toggled.update(|value| *value = !*value)/>
}
}


#[component]
pub fn ButtonC() -> impl IntoView {
view! {
<button>"Toggle"</button>
}
}
This lets you write way less code in <ButtonC/> than you did for <ButtonB/>, and still gives a correctly-typed event to the listener. This works by adding an on: event listener to each element that <ButtonC/> returns: in this case, just the one <button>.

Of course, this only works for actual DOM events that you’re passing directly through to the elements you’re rendering in the component. For more complex logic that doesn’t map directly onto an element (say you create <ValidatedForm/> and want an on_valid_form_submit callback) you should use Option 2.

4. Providing a Context
   This version is actually a variant on Option 1. Say you have a deeply-nested component tree:

#[component]
pub fn App() -> impl IntoView {
let (toggled, set_toggled) = create_signal(false);
view! {
<p>"Toggled? " {toggled}</p>
<Layout/>
}
}

#[component]
pub fn Layout() -> impl IntoView {
view! {
<header>
<h1>"My Page"</h1>
</header>
<main>
<Content/>
</main>
}
}

#[component]
pub fn Content() -> impl IntoView {
view! {
<div class="content">
<ButtonD/>
</div>
}
}

#[component]
pub fn ButtonD<F>() -> impl IntoView {
todo!()
}
Now <ButtonD/> is no longer a direct child of <App/>, so you can’t simply pass your WriteSignal to its props. You could do what’s sometimes called “prop drilling,” adding a prop to each layer between the two:

#[component]
pub fn App() -> impl IntoView {
let (toggled, set_toggled) = create_signal(false);
view! {
<p>"Toggled? " {toggled}</p>
<Layout set_toggled/>
}
}

#[component]
pub fn Layout(set_toggled: WriteSignal<bool>) -> impl IntoView {
view! {
<header>
<h1>"My Page"</h1>
</header>
<main>
<Content set_toggled/>
</main>
}
}

#[component]
pub fn Content(set_toggled: WriteSignal<bool>) -> impl IntoView {
view! {
<div class="content">
<ButtonD set_toggled/>
</div>
}
}

#[component]
pub fn ButtonD<F>(set_toggled: WriteSignal<bool>) -> impl IntoView {
todo!()
}
This is a mess. <Layout/> and <Content/> don’t need set_toggled; they just pass it through to <ButtonD/>. But I need to declare the prop in triplicate. This is not only annoying but hard to maintain: imagine we add a “half-toggled” option and the type of set_toggled needs to change to an enum. We have to change it in three places!

Isn’t there some way to skip levels?

There is!

4.1 The Context API
You can provide data that skips levels by using provide_context and use_context. Contexts are identified by the type of the data you provide (in this example, WriteSignal<bool>), and they exist in a top-down tree that follows the contours of your UI tree. In this example, we can use context to skip the unnecessary prop drilling.

#[component]
pub fn App() -> impl IntoView {
let (toggled, set_toggled) = create_signal(false);

    // share `set_toggled` with all children of this component
    provide_context(set_toggled);

    view! {
        <p>"Toggled? " {toggled}</p>
        <Layout/>
    }
}

// <Layout/> and <Content/> omitted
// To work in this version, drop their references to set_toggled

#[component]
pub fn ButtonD() -> impl IntoView {
// use_context searches up the context tree, hoping to
// find a `WriteSignal<bool>`
// in this case, I .expect() because I know I provided it
let setter = use_context::<WriteSignal<bool>>()
.expect("to have found the setter provided");

    view! {
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "Toggle"
        </button>
    }
}
The same caveats apply to this as to <ButtonA/>: passing a WriteSignal around should be done with caution, as it allows you to mutate state from arbitrary parts of your code. But when done carefully, this can be one of the most effective techniques for global state management in Leptos: simply provide the state at the highest level you’ll need it, and use it wherever you need it lower down.

Note that there are no performance downsides to this approach. Because you are passing a fine-grained reactive signal, nothing happens in the intervening components (<Layout/> and <Content/>) when you update it. You are communicating directly between <ButtonD/> and <App/>. In fact—and this is the power of fine-grained reactivity—you are communicating directly between a button click in <ButtonD/> and a single text node in <App/>. It’s as if the components themselves don’t exist at all. And, well... at runtime, they don’t. It’s just signals and effects, all the way down.

use leptos::{ev::MouseEvent, *};

// This highlights four different ways that child components can communicate
// with their parent:
// 1) <ButtonA/>: passing a WriteSignal as one of the child component props,
//    for the child component to write into and the parent to read
// 2) <ButtonB/>: passing a closure as one of the child component props, for
//    the child component to call
// 3) <ButtonC/>: adding an `on:` event listener to a component
// 4) <ButtonD/>: providing a context that is used in the component (rather than prop drilling)

#[derive(Copy, Clone)]
struct SmallcapsContext(WriteSignal<bool>);

#[component]
pub fn App() -> impl IntoView {
// just some signals to toggle three classes on our <p>
let (red, set_red) = create_signal(false);
let (right, set_right) = create_signal(false);
let (italics, set_italics) = create_signal(false);
let (smallcaps, set_smallcaps) = create_signal(false);

    // the newtype pattern isn't *necessary* here but is a good practice
    // it avoids confusion with other possible future `WriteSignal<bool>` contexts
    // and makes it easier to refer to it in ButtonC
    provide_context(SmallcapsContext(set_smallcaps));

    view! {
        <main>
            <p
                // class: attributes take F: Fn() => bool, and these signals all implement Fn()
                class:red=red
                class:right=right
                class:italics=italics
                class:smallcaps=smallcaps
            >
                "Lorem ipsum sit dolor amet."
            </p>

            // Button A: pass the signal setter
            <ButtonA setter=set_red/>

            // Button B: pass a closure
            <ButtonB on_click=move |_| set_right.update(|value| *value = !*value)/>

            // Button B: use a regular event listener
            // setting an event listener on a component like this applies it
            // to each of the top-level elements the component returns
            <ButtonC on:click=move |_| set_italics.update(|value| *value = !*value)/>

            // Button D gets its setter from context rather than props
            <ButtonD/>
        </main>
    }
}

/// Button A receives a signal setter and updates the signal itself
#[component]
pub fn ButtonA(
/// Signal that will be toggled when the button is clicked.
setter: WriteSignal<bool>,
) -> impl IntoView {
view! {
<button
on:click=move |_| setter.update(|value| *value = !*value)
>
"Toggle Red"
</button>
}
}

/// Button B receives a closure
#[component]
pub fn ButtonB<F>(
/// Callback that will be invoked when the button is clicked.
on_click: F,
) -> impl IntoView
where
F: Fn(MouseEvent) + 'static,
{
view! {
<button
on:click=on_click
>
"Toggle Right"
</button>
}

    // just a note: in an ordinary function ButtonB could take on_click: impl Fn(MouseEvent) + 'static
    // and save you from typing out the generic
    // the component macro actually expands to define a
    //
    // struct ButtonBProps<F> where F: Fn(MouseEvent) + 'static {
    //   on_click: F
    // }
    //
    // this is what allows us to have named props in our component invocation,
    // instead of an ordered list of function arguments
    // if Rust ever had named function arguments we could drop this requirement
}

/// Button C is a dummy: it renders a button but doesn't handle
/// its click. Instead, the parent component adds an event listener.
#[component]
pub fn ButtonC() -> impl IntoView {
view! {
<button>
"Toggle Italics"
</button>
}
}

/// Button D is very similar to Button A, but instead of passing the setter as a prop
/// we get it from the context
#[component]
pub fn ButtonD() -> impl IntoView {
let setter = use_context::<SmallcapsContext>().unwrap().0;

    view! {
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "Toggle Small Caps"
        </button>
    }
}

fn main() {
leptos::mount_to_body(App)
}


Component Children
It’s pretty common to want to pass children into a component, just as you can pass children into an HTML element. For example, imagine I have a <FancyForm/> component that enhances an HTML <form>. I need some way to pass all its inputs.

view! {
<Form>
<fieldset>
<label>
"Some Input"
<input type="text" name="something"/>
</label>
</fieldset>
<button>"Submit"</button>
</Form>
}
How can you do this in Leptos? There are basically two ways to pass components to other components:

render props: properties that are functions that return a view
the children prop: a special component property that includes anything you pass as a child to the component.
In fact, you’ve already seen these both in action in the <Show/> component:

view! {
<Show
// `when` is a normal prop
when=move || value() > 5
// `fallback` is a "render prop": a function that returns a view
fallback=|| view! { <Small/> }
>
    // `<Big/>` (and anything else here)
    // will be given to the `children` prop
    <Big/>
  </Show>
}
Let’s define a component that takes some children and a render prop.

#[component]
pub fn TakesChildren<F, IV>(
/// Takes a function (type F) that returns anything that can be
/// converted into a View (type IV)
render_prop: F,
/// `children` takes the `Children` type
children: Children,
) -> impl IntoView
where
F: Fn() -> IV,
IV: IntoView,
{
view! {
<h2>"Render Prop"</h2>
{render_prop()}

        <h2>"Children"</h2>
        {children()}
    }
}
render_prop and children are both functions, so we can call them to generate the appropriate views. children, in particular, is an alias for Box<dyn FnOnce() -> Fragment>. (Aren't you glad we named it Children instead?)

If you need a Fn or FnMut here because you need to call children more than once, we also provide ChildrenFn and ChildrenMut aliases.

We can use the component like this:

view! {
<TakesChildren render_prop=|| view! { <p>"Hi, there!"</p> }>
// these get passed to `children`
"Some text"
<span>"A span"</span>
</TakesChildren>
}
Manipulating Children
The Fragment type is basically a way of wrapping a Vec<View>. You can insert it anywhere into your view.

But you can also access those inner views directly to manipulate them. For example, here’s a component that takes its children and turns them into an unordered list.

#[component]
pub fn WrapsChildren(children: Children) -> impl IntoView {
// Fragment has `nodes` field that contains a Vec<View>
let children = children()
.nodes
.into_iter()
.map(|child| view! { <li>{child}</li> })
.collect_view();

    view! {
        <ul>{children}</ul>
    }
}
Calling it like this will create a list:


view! {
<WrapsChildren>
"A"
"B"
"C"
</WrapsChildren>
}

other code

use leptos::*;

// Often, you want to pass some kind of child view to another
// component. There are two basic patterns for doing this:
// - "render props": creating a component prop that takes a function
//   that creates a view
// - the `children` prop: a special property that contains content
//   passed as the children of a component in your view, not as a
//   property

#[component]
pub fn App() -> impl IntoView {
let (items, set_items) = create_signal(vec![0, 1, 2]);
let render_prop = move || {
// items.with(...) reacts to the value without cloning
// by applying a function. Here, we pass the `len` method
// on a `Vec<_>` directly
let len = move || items.with(Vec::len);
view! {
<p>"Length: " {len}</p>
}
};

    view! {
        // This component just displays the two kinds of children,
        // embedding them in some other markup
        <TakesChildren
            // for component props, you can shorthand
            // `render_prop=render_prop` => `render_prop`
            // (this doesn't work for HTML element attributes)
            render_prop
        >
            // these look just like the children of an HTML element
            <p>"Here's a child."</p>
            <p>"Here's another child."</p>
        </TakesChildren>
        <hr/>
        // This component actually iterates over and wraps the children
        <WrapsChildren>
            <p>"Here's a child."</p>
            <p>"Here's another child."</p>
        </WrapsChildren>
    }
}

/// Displays a `render_prop` and some children within markup.
#[component]
pub fn TakesChildren<F, IV>(
/// Takes a function (type F) that returns anything that can be
/// converted into a View (type IV)
render_prop: F,
/// `children` takes the `Children` type
/// this is an alias for `Box<dyn FnOnce() -> Fragment>`
/// ... aren't you glad we named it `Children` instead?
children: Children,
) -> impl IntoView
where
F: Fn() -> IV,
IV: IntoView,
{
view! {
<h1><code>"<TakesChildren/>"</code></h1>
<h2>"Render Prop"</h2>
{render_prop()}
<hr/>
<h2>"Children"</h2>
{children()}
}
}

/// Wraps each child in an `<li>` and embeds them in a `<ul>`.
#[component]
pub fn WrapsChildren(children: Children) -> impl IntoView {
// children() returns a `Fragment`, which has a
// `nodes` field that contains a Vec<View>
// this means we can iterate over the children
// to create something new!
let children = children()
.nodes
.into_iter()
.map(|child| view! { <li>{child}</li> })
.collect::<Vec<_>>();

    view! {
        <h1><code>"<WrapsChildren/>"</code></h1>
        // wrap our wrapped children in a UL
        <ul>{children}</ul>
    }
}

fn main() {
leptos::mount_to_body(App)
}


No Macros: The View Builder Syntax
If you’re perfectly happy with the view! macro syntax described so far, you’re welcome to skip this chapter. The builder syntax described in this section is always available, but never required.

For one reason or another, many developers would prefer to avoid macros. Perhaps you don’t like the limited rustfmt support. (Although, you should check out leptosfmt, which is an excellent tool!) Perhaps you worry about the effect of macros on compile time. Perhaps you prefer the aesthetics of pure Rust syntax, or you have trouble context-switching between an HTML-like syntax and your Rust code. Or perhaps you want more flexibility in how you create and manipulate HTML elements than the view macro provides.

If you fall into any of those camps, the builder syntax may be for you.

The view macro expands an HTML-like syntax to a series of Rust functions and method calls. If you’d rather not use the view macro, you can simply use that expanded syntax yourself. And it’s actually pretty nice!

First off, if you want you can even drop the #[component] macro: a component is just a setup function that creates your view, so you can define a component as a simple function call:

pub fn counter(initial_value: i32, step: u32) -> impl IntoView { }
Elements are created by calling a function with the same name as the HTML element:

p()
You can add children to the element with .child(), which takes a single child or a tuple or array of types that implement IntoView.

p().child((em().child("Big, "), strong().child("bold "), "text"))
Attributes are added with .attr(). This can take any of the same types that you could pass as an attribute into the view macro (types that implement IntoAttribute).

p().attr("id", "foo").attr("data-count", move || count().to_string())
Similarly, the class:, prop:, and style: syntaxes map directly onto .class(), .prop(), and .style() methods.

Event listeners can be added with .on(). Typed events found in leptos::ev prevent typos in event names and allow for correct type inference in the callback function.

button()
.on(ev::click, move |_| set_count.update(|count| *count = 0))
.child("Clear")
Many additional methods can be found in the HtmlElement docs, including some methods that are not directly available in the view macro.

All of this adds up to a very Rusty syntax to build full-featured views, if you prefer this style.

/// A simple counter view.
// A component is really just a function call: it runs once to create the DOM and reactive system
pub fn counter(initial_value: i32, step: u32) -> impl IntoView {
let (count, set_count) = create_signal(0);
div().child((
button()
// typed events found in leptos::ev
// 1) prevent typos in event names
// 2) allow for correct type inference in callbacks
.on(ev::click, move |_| set_count.update(|count| *count = 0))
.child("Clear"),
button()
.on(ev::click, move |_| set_count.update(|count| *count -= 1))
.child("-1"),
span().child(("Value: ", move || count.get(), "!")),
button()
.on(ev::click, move |_| set_count.update(|count| *count += 1))
.child("+1"),
))
}
This also has the benefit of being more flexible: because these are all plain Rust functions and methods, it’s easier to use them in things like iterator adapters without any additional “magic”:

// take some set of attribute names and values
let attrs: Vec<(&str, AttributeValue)> = todo!();
// you can use the builder syntax to “spread” these onto the
// element in a way that’s not possible with the view macro
let p = attrs
.into_iter()
.fold(p(), |el, (name, value)| el.attr(name, value));
Performance Note
One caveat: the view macro applies significant optimizations in server-side-rendering (SSR) mode to improve HTML rendering performance significantly (think 2-4x faster, depending on the characteristics of any given app). It does this by analyzing your view at compile time and converting the static parts into simple HTML strings, rather than expanding them into the builder syntax.

This means two things:

The builder syntax and view macro should not be mixed, or should only be mixed very carefully: at least in SSR mode, the output of the view should be treated as a “black box” that can’t have additional builder methods applied to it without causing inconsistencies.
Using the builder syntax will result in less-than-optimal SSR performance. It won’t be slow, by any means (and it’s worth running your own benchmarks in any case), just slower than the view-optimized version.


Reactivity
Leptos is built on top of a fine-grained reactive system, designed to run expensive side effects (like rendering something in a browser, or making a network request) as infrequently as possible in response to change, reactive values.

So far we’ve seen signals in action. These chapters will go into a bit more depth, and look at effects, which are the other half of the story.


Working with Signals
So far we’ve used some simple examples of create_signal, which returns a ReadSignal getter and a WriteSignal setter.

Getting and Setting
There are four basic signal operations:

.get() clones the current value of the signal and tracks any future changes to the value reactively.
.with() takes a function, which receives the current value of the signal by reference (&T), and tracks any future changes.
.set() replaces the current value of the signal and notifies any subscribers that they need to update.
.update() takes a function, which receives a mutable reference to the current value of the signal (&mut T), and notifies any subscribers that they need to update. (.update() doesn’t return the value returned by the closure, but you can use .try_update() if you need to; for example, if you’re removing an item from a Vec<_> and want the removed item.)
Calling a ReadSignal as a function is syntax sugar for .get(). Calling a WriteSignal as a function is syntax sugar for .set(). So

let (count, set_count) = create_signal(0);
set_count(1);
logging::log!(count());
is the same as

let (count, set_count) = create_signal(0);
set_count.set(1);
logging::log!(count.get());
You might notice that .get() and .set() can be implemented in terms of .with() and .update(). In other words, count.get() is identical with count.with(|n| n.clone()), and count.set(1) is implemented by doing count.update(|n| *n = 1).

But of course, .get() and .set() (or the plain function-call forms!) are much nicer syntax.

However, there are some very good use cases for .with() and .update().

For example, consider a signal that holds a Vec<String>.

let (names, set_names) = create_signal(Vec::new());
if names().is_empty() {
set_names(vec!["Alice".to_string()]);
}
In terms of logic, this is simple enough, but it’s hiding some significant inefficiencies. Remember that names().is_empty() is sugar for names.get().is_empty(), which clones the value (it’s names.with(|n| n.clone()).is_empty()). This means we clone the whole Vec<String>, run is_empty(), and then immediately throw away the clone.

Likewise, set_names replaces the value with a whole new Vec<_>. This is fine, but we might as well just mutate the original Vec<_> in place.

let (names, set_names) = create_signal(Vec::new());
if names.with(|names| names.is_empty()) {
set_names.update(|names| names.push("Alice".to_string()));
}
Now our function simply takes names by reference to run is_empty(), avoiding that clone.

And if you have Clippy on, or if you have sharp eyes, you may notice we can make this even neater:

if names.with(Vec::is_empty) {
// ...
}
After all, .with() simply takes a function that takes the value by reference. Since Vec::is_empty takes &self, we can pass it in directly and avoid the unnecessary closure.

There are some helper macros to make using .with() and .update() easier to use, especially when using multiple signals.

let (first, _) = create_signal("Bob".to_string());
let (middle, _) = create_signal("J.".to_string());
let (last, _) = create_signal("Smith".to_string());
If you wanted to concatenate these 3 signals together without unnecessary cloning, you would have to write something like:

let name = move || {
first.with(|first| {
middle.with(|middle| last.with(|last| format!("{first} {middle} {last}")))
})
};
Which is very long and annoying to write.

Instead, you can use the with! macro to get references to all the signals at the same time.

let name = move || with!(|first, middle, last| format!("{first} {middle} {last}"));
This expands to the same thing as above. Take a look at the with! docs for more info, and the corresponding macros update!, with_value! and update_value!.

Making signals depend on each other
Often people ask about situations in which some signal needs to change based on some other signal’s value. There are three good ways to do this, and one that’s less than ideal but okay under controlled circumstances.

Good Options
1) B is a function of A. Create a signal for A and a derived signal or memo for B.

let (count, set_count) = create_signal(1);
let derived_signal_double_count = move || count() * 2;
let memoized_double_count = create_memo(move |_| count() * 2);
For guidance on whether to use a derived signal or a memo, see the docs for create_memo

2) C is a function of A and some other thing B. Create signals for A and B and a derived signal or memo for C.

let (first_name, set_first_name) = create_signal("Bridget".to_string());
let (last_name, set_last_name) = create_signal("Jones".to_string());
let full_name = move || with!(|first_name, last_name| format!("{first_name} {last_name}"));
3) A and B are independent signals, but sometimes updated at the same time. When you make the call to update A, make a separate call to update B.

let (age, set_age) = create_signal(32);
let (favorite_number, set_favorite_number) = create_signal(42);
// use this to handle a click on a `Clear` button
let clear_handler = move |_| {
set_age(0);
set_favorite_number(0);
};
If you really must...
4) Create an effect to write to B whenever A changes. This is officially discouraged, for several reasons: a) It will always be less efficient, as it means every time A updates you do two full trips through the reactive process. (You set A, which causes the effect to run, as well as any other effects that depend on A. Then you set B, which causes any effects that depend on B to run.) b) It increases your chances of accidentally creating things like infinite loops or over-re-running effects. This is the kind of ping-ponging, reactive spaghetti code that was common in the early 2010s and that we try to avoid with things like read-write segregation and discouraging writing to signals from effects.

In most situations, it’s best to rewrite things such that there’s a clear, top-down data flow based on derived signals or memos. But this isn’t the end of the world.

I’m intentionally not providing an example here. Read the create_effect docs to figure out how this would work.

Function leptos::create_effectCopy item path
source · [−]
pub fn create_effect<T>(f: impl Fn(Option<T>) -> T + 'static) -> Effect<T>
where
T: 'static,
Effects run a certain chunk of code whenever the signals they depend on change. create_effect queues the given function to run once, tracks its dependence on any signal values read within it, and reruns the function whenever the value of a dependency changes.

Effects are intended to run side-effects of the system, not to synchronize state within the system. In other words: don’t write to signals within effects, unless you’re coordinating with some other non-reactive side effect. (If you need to define a signal that depends on the value of other signals, use a derived signal or create_memo).

This first run is queued for the next microtask, i.e., it runs after all other synchronous code has completed. In practical terms, this means that if you use create_effect in the body of the component, it will run after the view has been created and (presumably) mounted. (If you need an effect that runs immediately, use create_render_effect.)

The effect function is called with an argument containing whatever value it returned the last time it ran. On the initial run, this is None.

By default, effects do not run on the server. This means you can call browser-specific APIs within the effect function without causing issues. If you need an effect to run on the server, use create_isomorphic_effect.

let (a, set_a) = create_signal(0);
let (b, set_b) = create_signal(0);

// ✅ use effects to interact between reactive state and the outside world
create_effect(move |_| {
// immediately prints "Value: 0" and subscribes to `a`
log::debug!("Value: {}", a.get());
});

set_a.set(1);
// ✅ because it's subscribed to `a`, the effect reruns and prints "Value: 1"

// ❌ don't use effects to synchronize state within the reactive system
create_effect(move |_| {
// this technically works but can cause unnecessary re-renders
// and easily lead to problems like infinite loops
set_b.set(a.get() + 1);
});


Responding to Changes with create_effect
We’ve made it this far without having mentioned half of the reactive system: effects.

Reactivity works in two halves: updating individual reactive values (“signals”) notifies the pieces of code that depend on them (“effects”) that they need to run again. These two halves of the reactive system are inter-dependent. Without effects, signals can change within the reactive system but never be observed in a way that interacts with the outside world. Without signals, effects run once but never again, as there’s no observable value to subscribe to. Effects are quite literally “side effects” of the reactive system: they exist to synchronize the reactive system with the non-reactive world outside it.

Hidden behind the whole reactive DOM renderer that we’ve seen so far is a function called create_effect.

create_effect takes a function as its argument. It immediately runs the function. If you access any reactive signal inside that function, it registers the fact that the effect depends on that signal with the reactive runtime. Whenever one of the signals that the effect depends on changes, the effect runs again.

let (a, set_a) = create_signal(0);
let (b, set_b) = create_signal(0);

create_effect(move |_| {
// immediately prints "Value: 0" and subscribes to `a`
log::debug!("Value: {}", a());
});
The effect function is called with an argument containing whatever value it returned the last time it ran. On the initial run, this is None.

By default, effects do not run on the server. This means you can call browser-specific APIs within the effect function without causing issues. If you need an effect to run on the server, use create_isomorphic_effect.

Auto-tracking and Dynamic Dependencies
If you’re familiar with a framework like React, you might notice one key difference. React and similar frameworks typically require you to pass a “dependency array,” an explicit set of variables that determine when the effect should rerun.

Because Leptos comes from the tradition of synchronous reactive programming, we don’t need this explicit dependency list. Instead, we automatically track dependencies depending on which signals are accessed within the effect.

This has two effects (no pun intended). Dependencies are:

Automatic: You don’t need to maintain a dependency list, or worry about what should or shouldn’t be included. The framework simply tracks which signals might cause the effect to rerun, and handles it for you.
Dynamic: The dependency list is cleared and updated every time the effect runs. If your effect contains a conditional (for example), only signals that are used in the current branch are tracked. This means that effects rerun the absolute minimum number of times.
If this sounds like magic, and if you want a deep dive into how automatic dependency tracking works, check out this video. (Apologies for the low volume!)

Effects as Zero-Cost-ish Abstraction
While they’re not a “zero-cost abstraction” in the most technical sense—they require some additional memory use, exist at runtime, etc.—at a higher level, from the perspective of whatever expensive API calls or other work you’re doing within them, effects are a zero-cost abstraction. They rerun the absolute minimum number of times necessary, given how you’ve described them.

Imagine that I’m creating some kind of chat software, and I want people to be able to display their full name, or just their first name, and to notify the server whenever their name changes:

let (first, set_first) = create_signal(String::new());
let (last, set_last) = create_signal(String::new());
let (use_last, set_use_last) = create_signal(true);

// this will add the name to the log
// any time one of the source signals changes
create_effect(move |_| {
log(
if use_last() {
format!("{} {}", first(), last())
} else {
first()
},
)
});
If use_last is true, effect should rerun whenever first, last, or use_last changes. But if I toggle use_last to false, a change in last will never cause the full name to change. In fact, last will be removed from the dependency list until use_last toggles again. This saves us from sending multiple unnecessary requests to the API if I change last multiple times while use_last is still false.

To create_effect, or not to create_effect?
Effects are intended to synchronize the reactive system with the non-reactive world outside, not to synchronize between different reactive values. In other words: using an effect to read a value from one signal and set it in another is always sub-optimal.

If you need to define a signal that depends on the value of other signals, use a derived signal or create_memo. Writing to a signal inside an effect isn’t the end of the world, and it won’t cause your computer to light on fire, but a derived signal or memo is always better—not only because the dataflow is clear, but because the performance is better.

let (a, set_a) = create_signal(0);

// ⚠️ not great
let (b, set_b) = create_signal(0);
create_effect(move |_| {
set_b(a() * 2);
});

// ✅ woo-hoo!
let b = move || a() * 2;
If you need to synchronize some reactive value with the non-reactive world outside—like a web API, the console, the filesystem, or the DOM—writing to a signal in an effect is a fine way to do that. In many cases, though, you’ll find that you’re really writing to a signal inside an event listener or something else, not inside an effect. In these cases, you should check out leptos-use to see if it already provides a reactive wrapping primitive to do that!

If you’re curious for more information about when you should and shouldn’t use create_effect, check out this video for a more in-depth consideration!

Effects and Rendering
We’ve managed to get this far without mentioning effects because they’re built into the Leptos DOM renderer. We’ve seen that you can create a signal and pass it into the view macro, and it will update the relevant DOM node whenever the signal changes:

let (count, set_count) = create_signal(0);

view! {
<p>{count}</p>
}
This works because the framework essentially creates an effect wrapping this update. You can imagine Leptos translating this view into something like this:

let (count, set_count) = create_signal(0);

// create a DOM element
let document = leptos::document();
let p = document.create_element("p").unwrap();

// create an effect to reactively update the text
create_effect(move |prev_value| {
// first, access the signal’s value and convert it to a string
let text = count().to_string();

    // if this is different from the previous value, update the node
    if prev_value != Some(text) {
        p.set_text_content(&text);
    }

    // return this value so we can memoize the next update
    text
});
Every time count is updated, this effect will rerun. This is what allows reactive, fine-grained updates to the DOM.

Explicit, Cancelable Tracking with watch
In addition to create_effect, Leptos provides a watch function, which can be used for two main purposes:

Separating tracking and responding to changes by explicitly passing in a set of values to track.
Canceling tracking by calling a stop function.
Like create_resource, watch takes a first argument, which is reactively tracked, and a second, which is not. Whenever a reactive value in its deps argument is changed, the callback is run. watch returns a function that can be called to stop tracking the dependencies.

let (num, set_num) = create_signal(0);

let stop = watch(
move || num.get(),
move |num, prev_num, _| {
log::debug!("Number: {}; Prev: {:?}", num, prev_num);
},
false,
);

set_num.set(1); // > "Number: 1; Prev: Some(0)"

stop(); // stop watching

set_num.set(2); // (nothing happens)

other code

use leptos::html::Input;
use leptos::*;

#[derive(Copy, Clone)]
struct LogContext(RwSignal<Vec<String>>);

#[component]
fn App() -> impl IntoView {
// Just making a visible log here
// You can ignore this...
let log = create_rw_signal::<Vec<String>>(vec![]);
let logged = move || log().join("\n");

    // the newtype pattern isn't *necessary* here but is a good practice
    // it avoids confusion with other possible future `RwSignal<Vec<String>>` contexts
    // and makes it easier to refer to it
    provide_context(LogContext(log));

    view! {
        <CreateAnEffect/>
        <pre>{logged}</pre>
    }
}

#[component]
fn CreateAnEffect() -> impl IntoView {
let (first, set_first) = create_signal(String::new());
let (last, set_last) = create_signal(String::new());
let (use_last, set_use_last) = create_signal(true);

    // this will add the name to the log
    // any time one of the source signals changes
    create_effect(move |_| {
        log(if use_last() {
            with!(|first, last| format!("{first} {last}"))
        } else {
            first()
        })
    });

    view! {
        <h1>
            <code>"create_effect"</code>
            " Version"
        </h1>
        <form>
            <label>
                "First Name"
                <input
                    type="text"
                    name="first"
                    prop:value=first
                    on:change=move |ev| set_first(event_target_value(&ev))
                />
            </label>
            <label>
                "Last Name"
                <input
                    type="text"
                    name="last"
                    prop:value=last
                    on:change=move |ev| set_last(event_target_value(&ev))
                />
            </label>
            <label>
                "Show Last Name"
                <input
                    type="checkbox"
                    name="use_last"
                    prop:checked=use_last
                    on:change=move |ev| set_use_last(event_target_checked(&ev))
                />
            </label>
        </form>
    }
}

#[component]
fn ManualVersion() -> impl IntoView {
let first = create_node_ref::<Input>();
let last = create_node_ref::<Input>();
let use_last = create_node_ref::<Input>();

    let mut prev_name = String::new();
    let on_change = move |_| {
        log("      listener");
        let first = first.get().unwrap();
        let last = last.get().unwrap();
        let use_last = use_last.get().unwrap();
        let this_one = if use_last.checked() {
            format!("{} {}", first.value(), last.value())
        } else {
            first.value()
        };

        if this_one != prev_name {
            log(&this_one);
            prev_name = this_one;
        }
    };

    view! {
        <h1>"Manual Version"</h1>
        <form on:change=on_change>
            <label>"First Name" <input type="text" name="first" node_ref=first/></label>
            <label>"Last Name" <input type="text" name="last" node_ref=last/></label>
            <label>
                "Show Last Name" <input type="checkbox" name="use_last" checked node_ref=use_last/>
            </label>
        </form>
    }
}

#[component]
fn EffectVsDerivedSignal() -> impl IntoView {
let (my_value, set_my_value) = create_signal(String::new());
// Don't do this.
/*let (my_optional_value, set_optional_my_value) = create_signal(Option::<String>::None);

    create_effect(move |_| {
        if !my_value.get().is_empty() {
            set_optional_my_value(Some(my_value.get()));
        } else {
            set_optional_my_value(None);
        }
    });*/

    // Do this
    let my_optional_value =
        move || (!my_value.with(String::is_empty)).then(|| Some(my_value.get()));

    view! {
        <input prop:value=my_value on:input=move |ev| set_my_value(event_target_value(&ev))/>

        <p>
            <code>"my_optional_value"</code>
            " is "
            <code>
                <Show when=move || my_optional_value().is_some() fallback=|| view! { "None" }>
                    "Some(\""
                    {my_optional_value().unwrap()}
                    "\")"
                </Show>
            </code>
        </p>
    }
}

#[component]
pub fn Show<F, W, IV>(
/// The components Show wraps
children: Box<dyn Fn() -> Fragment>,
/// A closure that returns a bool that determines whether this thing runs
when: W,
/// A closure that returns what gets rendered if the when statement is false
fallback: F,
) -> impl IntoView
where
W: Fn() -> bool + 'static,
F: Fn() -> IV + 'static,
IV: IntoView,
{
let memoized_when = create_memo(move |_| when());

    move || match memoized_when.get() {
        true => children().into_view(),
        false => fallback().into_view(),
    }
}

fn log(msg: impl std::fmt::Display) {
let log = use_context::<LogContext>().unwrap().0;
log.update(|log| log.push(msg.to_string()));
}

fn main() {
leptos::mount_to_body(App)
}


Interlude: Reactivity and Functions
One of our core contributors said to me recently: “I never used closures this often until I started using Leptos.” And it’s true. Closures are at the heart of any Leptos application. It sometimes looks a little silly:

// a signal holds a value, and can be updated
let (count, set_count) = create_signal(0);

// a derived signal is a function that accesses other signals
let double_count = move || count() * 2;
let count_is_odd = move || count() & 1 == 1;
let text = move || if count_is_odd() {
"odd"
} else {
"even"
};

// an effect automatically tracks the signals it depends on
// and reruns when they change
create_effect(move |_| {
logging::log!("text = {}", text());
});

view! {
<p>{move || text().to_uppercase()}</p>
}
Closures, closures everywhere!

But why?

Functions and UI Frameworks
Functions are at the heart of every UI framework. And this makes perfect sense. Creating a user interface is basically divided into two phases:

initial rendering
updates
In a web framework, the framework does some kind of initial rendering. Then it hands control back over to the browser. When certain events fire (like a mouse click) or asynchronous tasks finish (like an HTTP request finishing), the browser wakes the framework back up to update something. The framework runs some kind of code to update your user interface, and goes back asleep until the browser wakes it up again.

The key phrase here is “runs some kind of code.” The natural way to “run some kind of code” at an arbitrary point in time—in Rust or in any other programming language—is to call a function. And in fact every UI framework is based on rerunning some kind of function over and over:

virtual DOM (VDOM) frameworks like React, Yew, or Dioxus rerun a component or render function over and over, to generate a virtual DOM tree that can be reconciled with the previous result to patch the DOM
compiled frameworks like Angular and Svelte divide your component templates into “create” and “update” functions, rerunning the update function when they detect a change to the component’s state
in fine-grained reactive frameworks like SolidJS, Sycamore, or Leptos, you define the functions that rerun
That’s what all our components are doing.

Take our typical <SimpleCounter/> example in its simplest form:

#[component]
pub fn SimpleCounter() -> impl IntoView {
let (value, set_value) = create_signal(0);

    let increment = move |_| set_value.update(|value| *value += 1);

    view! {
        <button on:click=increment>
            {value}
        </button>
    }
}
The SimpleCounter function itself runs once. The value signal is created once. The framework hands off the increment function to the browser as an event listener. When you click the button, the browser calls increment, which updates value via set_value. And that updates the single text node represented in our view by {value}.

Closures are key to reactivity. They provide the framework with the ability to rerun the smallest possible unit of your application in response to a change.

So remember two things:

Your component function is a setup function, not a render function: it only runs once.
For values in your view template to be reactive, they must be functions: either signals (which implement the Fn traits) or closures.


Testing Your Components
Testing user interfaces can be relatively tricky, but really important. This article will discuss a couple principles and approaches for testing a Leptos app.

1. Test business logic with ordinary Rust tests
   In many cases, it makes sense to pull the logic out of your components and test it separately. For some simple components, there’s no particular logic to test, but for many it’s worth using a testable wrapping type and implementing the logic in ordinary Rust impl blocks.

For example, instead of embedding logic in a component directly like this:

#[component]
pub fn TodoApp() -> impl IntoView {
let (todos, set_todos) = create_signal(vec![Todo { /* ... */ }]);
// ⚠️ this is hard to test because it's embedded in the component
let num_remaining = move || todos.with(|todos| {
todos.iter().filter(|todo| !todo.completed).sum()
});
}
You could pull that logic out into a separate data structure and test it:

pub struct Todos(Vec<Todo>);

impl Todos {
pub fn num_remaining(&self) -> usize {
self.0.iter().filter(|todo| !todo.completed).sum()
}
}

#[cfg(test)]
mod tests {
#[test]
fn test_remaining() {
// ...
}
}

#[component]
pub fn TodoApp() -> impl IntoView {
let (todos, set_todos) = create_signal(Todos(vec![Todo { /* ... */ }]));
// ✅ this has a test associated with it
let num_remaining = move || todos.with(Todos::num_remaining);
}
In general, the less of your logic is wrapped into your components themselves, the more idiomatic your code will feel and the easier it will be to test.

2. Test components with end-to-end (e2e) testing
   Our examples directory has several examples with extensive end-to-end testing, using different testing tools.

The easiest way to see how to use these is to take a look at the test examples themselves:

wasm-bindgen-test with counter
This is a fairly simple manual testing setup that uses the wasm-pack test command.

Sample Test
#[wasm_bindgen_test]
fn clear() {
let document = leptos::document();
let test_wrapper = document.create_element("section").unwrap();
let _ = document.body().unwrap().append_child(&test_wrapper);

    mount_to(
        test_wrapper.clone().unchecked_into(),
        || view! { <SimpleCounter initial_value=10 step=1/> },
    );

    let div = test_wrapper.query_selector("div").unwrap().unwrap();
    let clear = test_wrapper
        .query_selector("button")
        .unwrap()
        .unwrap()
        .unchecked_into::<web_sys::HtmlElement>();

    clear.click();

assert_eq!(
div.outer_html(),
// here we spawn a mini reactive system to render the test case
run_scope(create_runtime(), || {
// it's as if we're creating it with a value of 0, right?
let (value, set_value) = create_signal(0);

        // we can remove the event listeners because they're not rendered to HTML
        view! {
            <div>
                <button>"Clear"</button>
                <button>"-1"</button>
                <span>"Value: " {value} "!"</span>
                <button>"+1"</button>
            </div>
        }
        // the view returned an HtmlElement<Div>, which is a smart pointer for
        // a DOM element. So we can still just call .outer_html()
        .outer_html()
    })
);
}
wasm-bindgen-test with counters_stable
This more developed test suite uses a system of fixtures to refactor the manual DOM manipulation of the counter tests and easily test a wide range of cases.

Sample Test
use super::*;
use crate::counters_page as ui;
use pretty_assertions::assert_eq;

#[wasm_bindgen_test]
fn should_increase_the_total_count() {
// Given
ui::view_counters();
ui::add_counter();

    // When
    ui::increment_counter(1);
    ui::increment_counter(1);
    ui::increment_counter(1);

    // Then
    assert_eq!(ui::total(), 3);
}
Playwright with counters_stable
These tests use the common JavaScript testing tool Playwright to run end-to-end tests on the same example, using a library and testing approach familiar to many who have done frontend development before.

Sample Test
import { test, expect } from "@playwright/test";
import { CountersPage } from "./fixtures/counters_page";

test.describe("Increment Count", () => {
test("should increase the total count", async ({ page }) => {
const ui = new CountersPage(page);
await ui.goto();
await ui.addCounter();

    await ui.incrementCount();
    await ui.incrementCount();
    await ui.incrementCount();

    await expect(ui.total).toHaveText("3");
});
});
Gherkin/Cucumber Tests with todo_app_sqlite
You can integrate any testing tool you’d like into this flow. This example uses Cucumber, a testing framework based on natural language.

@add_todo
Feature: Add Todo

    Background:
        Given I see the app

    @add_todo-see
    Scenario: Should see the todo
        Given I set the todo as Buy Bread
        When I click the Add button
        Then I see the todo named Buy Bread

    # @allow.skipped
    @add_todo-style
    Scenario: Should see the pending todo
        When I add a todo as Buy Oranges
        Then I see the pending todo
The definitions for these actions are defined in Rust code.

use crate::fixtures::{action, world::AppWorld};
use anyhow::{Ok, Result};
use cucumber::{given, when};

#[given("I see the app")]
#[when("I open the app")]
async fn i_open_the_app(world: &mut AppWorld) -> Result<()> {
let client = &world.client;
action::goto_path(client, "").await?;

    Ok(())
}

#[given(regex = "^I add a todo as (.*)$")]
#[when(regex = "^I add a todo as (.*)$")]
async fn i_add_a_todo_titled(world: &mut AppWorld, text: String) -> Result<()> {
let client = &world.client;
action::add_todo(client, text.as_str()).await?;

    Ok(())
}

// etc.
Learning More
Feel free to check out the CI setup in the Leptos repo to learn more about how to use these tools in your own application. All of these testing methods are run regularly against actual Leptos example apps.


Working with async
So far we’ve only been working with synchronous user interfaces: You provide some input, the app immediately processes it and updates the interface. This is great, but is a tiny subset of what web applications do. In particular, most web apps have to deal with some kind of asynchronous data loading, usually loading something from an API.

Asynchronous data is notoriously hard to integrate with the synchronous parts of your code. Leptos provides a cross-platform spawn_local function that makes it easy to run a Future, but there’s much more to it than that.

In this chapter, we’ll see how Leptos helps smooth out that process for you.


Loading Data with Resources
A Resource is a reactive data structure that reflects the current state of an asynchronous task, allowing you to integrate asynchronous Futures into the synchronous reactive system. Rather than waiting for its data to load with .await, you transform the Future into a signal that returns Some(T) if it has resolved, and None if it’s still pending.

You do this by using the create_resource function. This takes two arguments:

a source signal, which will generate a new Future whenever it changes
a fetcher function, which takes the data from that signal and returns a Future
Here’s an example

// our source signal: some synchronous, local state
let (count, set_count) = create_signal(0);

// our resource
let async_data = create_resource(
count,
// every time `count` changes, this will run
|value| async move {
logging::log!("loading data from API");
load_data(value).await
},
);
To create a resource that simply runs once, you can pass a non-reactive, empty source signal:

let once = create_resource(|| (), |_| async move { load_data().await });
To access the value you can use .get() or .with(|data| /* */). These work just like .get() and .with() on a signal—get clones the value and returns it, with applies a closure to it—but for any Resource<_, T>, they always return Option<T>, not T: because it’s always possible that your resource is still loading.

So, you can show the current state of a resource in your view:

let once = create_resource(|| (), |_| async move { load_data().await });
view! {
<h1>"My Data"</h1>
{move || match once.get() {
None => view! { <p>"Loading..."</p> }.into_view(),
Some(data) => view! { <ShowData data/> }.into_view()
}}
}
Resources also provide a refetch() method that allows you to manually reload the data (for example, in response to a button click) and a loading() method that returns a ReadSignal<bool> indicating whether the resource is currently loading or not.

use gloo_timers::future::TimeoutFuture;
use leptos::*;

// Here we define an async function
// This could be anything: a network request, database read, etc.
// Here, we just multiply a number by 10
async fn load_data(value: i32) -> i32 {
// fake a one-second delay
TimeoutFuture::new(1_000).await;
value * 10
}

#[component]
fn App() -> impl IntoView {
// this count is our synchronous, local state
let (count, set_count) = create_signal(0);

    // create_resource takes two arguments after its scope
    let async_data = create_resource(
        // the first is the "source signal"
        count,
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |value| async move { load_data(value).await },
    );
    // whenever the source signal changes, the loader reloads

    // you can also create resources that only load once
    // just return the unit type () from the source signal
    // that doesn't depend on anything: we just load it once
    let stable = create_resource(|| (), |_| async move { load_data(1).await });

    // we can access the resource values with .read()
    // this will reactively return None before the Future has resolved
    // and update to Some(T) when it has resolved
    let async_result = move || {
        async_data
            .read()
            .map(|value| format!("Server returned {value:?}"))
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".into())
    };

    // the resource's loading() method gives us a
    // signal to indicate whether it's currently loading
    let loading = async_data.loading();
    let is_loading = move || if loading() { "Loading..." } else { "Idle." };

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me"
        </button>
        <p>
            <code>"stable"</code>": " {move || stable.read()}
        </p>
        <p>
            <code>"count"</code>": " {count}
        </p>
        <p>
            <code>"async_value"</code>": "
            {async_result}
            <br/>
            {is_loading}
        </p>
    }
}

fn main() {
leptos::mount_to_body(App)
}


<Suspense/>
In the previous chapter, we showed how you can create a simple loading screen to show some fallback while a resource is loading.

let (count, set_count) = create_signal(0);
let once = create_resource(count, |count| async move { load_a(count).await });

view! {
<h1>"My Data"</h1>
{move || match once.get() {
None => view! { <p>"Loading..."</p> }.into_view(),
Some(data) => view! { <ShowData data/> }.into_view()
}}
}
But what if we have two resources, and want to wait for both of them?

let (count, set_count) = create_signal(0);
let (count2, set_count2) = create_signal(0);
let a = create_resource(count, |count| async move { load_a(count).await });
let b = create_resource(count2, |count| async move { load_b(count).await });

view! {
<h1>"My Data"</h1>
{move || match (a.get(), b.get()) {
(Some(a), Some(b)) => view! {
<ShowA a/>
<ShowA b/>
}.into_view(),
_ => view! { <p>"Loading..."</p> }.into_view()
}}
}
That’s not so bad, but it’s kind of annoying. What if we could invert the flow of control?

The <Suspense/> component lets us do exactly that. You give it a fallback prop and children, one or more of which usually involves reading from a resource. Reading from a resource “under” a <Suspense/> (i.e., in one of its children) registers that resource with the <Suspense/>. If it’s still waiting for resources to load, it shows the fallback. When they’ve all loaded, it shows the children.

let (count, set_count) = create_signal(0);
let (count2, set_count2) = create_signal(0);
let a = create_resource(count, |count| async move { load_a(count).await });
let b = create_resource(count2, |count| async move { load_b(count).await });

view! {
<h1>"My Data"</h1>
<Suspense
fallback=move || view! { <p>"Loading..."</p> }
>
<h2>"My Data"</h2>
<h3>"A"</h3>
{move || {
a.get()
.map(|a| view! { <ShowA a/> })
}}
<h3>"B"</h3>
{move || {
b.get()
.map(|b| view! { <ShowB b/> })
}}
</Suspense>
}
Every time one of the resources is reloading, the "Loading..." fallback will show again.

This inversion of the flow of control makes it easier to add or remove individual resources, as you don’t need to handle the matching yourself. It also unlocks some massive performance improvements during server-side rendering, which we’ll talk about during a later chapter.

<Await/>
If you’re simply trying to wait for some Future to resolve before rendering, you may find the <Await/> component helpful in reducing boilerplate. <Await/> essentially combines a resource with the source argument || () with a <Suspense/> with no fallback.

In other words:

It only polls the Future once, and does not respond to any reactive changes.
It does not render anything until the Future resolves.
After the Future resolves, it binds its data to whatever variable name you choose and then renders its children with that variable in scope.
async fn fetch_monkeys(monkey: i32) -> i32 {
// maybe this didn't need to be async
monkey * 2
}
view! {
<Await
// `future` provides the `Future` to be resolved
future=|| fetch_monkeys(3)
// the data is bound to whatever variable name you provide
let:data
>
// you receive the data by reference and can use it in your view here
<p>{*data} " little monkeys, jumping on the bed."</p>
</Await>
}

other code

use gloo_timers::future::TimeoutFuture;
use leptos::*;

async fn important_api_call(name: String) -> String {
TimeoutFuture::new(1_000).await;
name.to_ascii_uppercase()
}

#[component]
fn App() -> impl IntoView {
let (name, set_name) = create_signal("Bill".to_string());

    // this will reload every time `name` changes
    let async_data = create_resource(name, |name| async move { important_api_call(name).await });

    view! {
        <input
            on:input=move |ev| {
                set_name(event_target_value(&ev));
            }
            prop:value=name
        />
        <p><code>"name:"</code> {name}</p>
        <Suspense
            // the fallback will show whenever a resource
            // read "under" the suspense is loading
            fallback=move || view! { <p>"Loading..."</p> }
        >
            // the children will be rendered once initially,
            // and then whenever any resources has been resolved
            <p>
                "Your shouting name is "
                {move || async_data.get()}
            </p>
        </Suspense>
    }
}

fn main() {
leptos::mount_to_body(App)
}


<Transition/>
You’ll notice in the <Suspense/> example that if you keep reloading the data, it keeps flickering back to "Loading...". Sometimes this is fine. For other times, there’s <Transition/>.

<Transition/> behaves exactly the same as <Suspense/>, but instead of falling back every time, it only shows the fallback the first time. On all subsequent loads, it continues showing the old data until the new data are ready. This can be really handy to prevent the flickering effect, and to allow users to continue interacting with your application.

This example shows how you can create a simple tabbed contact list with <Transition/>. When you select a new tab, it continues showing the current contact until the new data loads. This can be a much better user experience than constantly falling back to a loading message.

use leptos::*;

async fn important_api_call(id: usize) -> String {
TimeoutFuture::new(1_000).await;
match id {
0 => "Alice",
1 => "Bob",
2 => "Carol",
_ => "User not found",
}
.to_string()
}

#[component]
fn App() -> impl IntoView {
let (tab, set_tab) = create_signal(0);

    // this will reload every time `tab` changes
    let user_data = create_resource(tab, |tab| async move { important_api_call(tab).await });

    view! {
        <div class="buttons">
            <button
                on:click=move |_| set_tab(0)
                class:selected=move || tab() == 0
            >
                "Tab A"
            </button>
            <button
                on:click=move |_| set_tab(1)
                class:selected=move || tab() == 1
            >
                "Tab B"
            </button>
            <button
                on:click=move |_| set_tab(2)
                class:selected=move || tab() == 2
            >
                "Tab C"
            </button>
        </div>
        <Transition
            // the fallback will show initially
            // on subsequent reloads, the current child will
            // continue showing
            fallback=move || view! { <p>"Loading initial data..."</p> }
        >
            <p>
                {move || user_data.get()}
            </p>
        </Transition>
        {move || if user_data.loading().get() {
            "Hang on..."
        } else {
            ""
        }}
    }
}

fn main() {
leptos::mount_to_body(App)
}


Mutating Data with Actions
We’ve talked about how to load async data with resources. Resources immediately load data and work closely with <Suspense/> and <Transition/> components to show whether data is loading in your app. But what if you just want to call some arbitrary async function and keep track of what it’s doing?

Well, you could always use spawn_local. This allows you to just spawn an async task in a synchronous environment by handing the Future off to the browser (or, on the server, Tokio or whatever other runtime you’re using). But how do you know if it’s still pending? Well, you could just set a signal to show whether it’s loading, and another one to show the result...

All of this is true. Or you could use the final async primitive: create_action.

Actions and resources seem similar, but they represent fundamentally different things. If you’re trying to load data by running an async function, either once or when some other value changes, you probably want to use create_resource. If you’re trying to occasionally run an async function in response to something like a user clicking a button, you probably want to use create_action.

Say we have some async function we want to run.

async fn add_todo_request(new_title: &str) -> Uuid {
/* do some stuff on the server to add a new todo */
}
create_action takes an async function that takes a reference to a single argument, which you could think of as its “input type.”

The input is always a single type. If you want to pass in multiple arguments, you can do it with a struct or tuple.

// if there's a single argument, just use that
let action1 = create_action(|input: &String| {
let input = input.clone();
async move { todo!() }
});

// if there are no arguments, use the unit type `()`
let action2 = create_action(|input: &()| async { todo!() });

// if there are multiple arguments, use a tuple
let action3 = create_action(
|input: &(usize, String)| async { todo!() }
);
Because the action function takes a reference but the Future needs to have a 'static lifetime, you’ll usually need to clone the value to pass it into the Future. This is admittedly awkward but it unlocks some powerful features like optimistic UI. We’ll see a little more about that in future chapters.

So in this case, all we need to do to create an action is

let add_todo_action = create_action(|input: &String| {
let input = input.to_owned();
async move { add_todo_request(&input).await }
});
Rather than calling add_todo_action directly, we’ll call it with .dispatch(), as in

add_todo_action.dispatch("Some value".to_string());
You can do this from an event listener, a timeout, or anywhere; because .dispatch() isn’t an async function, it can be called from a synchronous context.

Actions provide access to a few signals that synchronize between the asynchronous action you’re calling and the synchronous reactive system:

let submitted = add_todo_action.input(); // RwSignal<Option<String>>
let pending = add_todo_action.pending(); // ReadSignal<bool>
let todo_id = add_todo_action.value(); // RwSignal<Option<Uuid>>
This makes it easy to track the current state of your request, show a loading indicator, or do “optimistic UI” based on the assumption that the submission will succeed.

let input_ref = create_node_ref::<Input>();

view! {
<form
on:submit=move |ev| {
ev.prevent_default(); // don't reload the page...
let input = input_ref.get().expect("input to exist");
add_todo_action.dispatch(input.value());
}
>
<label>
"What do you need to do?"
<input type="text"
node_ref=input_ref
/>
</label>
<button type="submit">"Add Todo"</button>
</form>
// use our loading state
<p>{move || pending().then("Loading...")}</p>
}
Now, there’s a chance this all seems a little over-complicated, or maybe too restricted. I wanted to include actions here, alongside resources, as the missing piece of the puzzle. In a real Leptos app, you’ll actually most often use actions alongside server functions, create_server_action, and the <ActionForm/> component to create really powerful progressively-enhanced forms. So if this primitive seems useless to you... Don’t worry! Maybe it will make sense later. (Or check out our todo_app_sqlite example now.)

use gloo_timers::future::TimeoutFuture;
use leptos::{html::Input, *};
use uuid::Uuid;

// Here we define an async function
// This could be anything: a network request, database read, etc.
// Think of it as a mutation: some imperative async action you run,
// whereas a resource would be some async data you load
async fn add_todo(text: &str) -> Uuid {
_ = text;
// fake a one-second delay
TimeoutFuture::new(1_000).await;
// pretend this is a post ID or something
Uuid::new_v4()
}

#[component]
fn App() -> impl IntoView {
// an action takes an async function with single argument
// it can be a simple type, a struct, or ()
let add_todo = create_action(|input: &String| {
// the input is a reference, but we need the Future to own it
// this is important: we need to clone and move into the Future
// so it has a 'static lifetime
let input = input.to_owned();
async move { add_todo(&input).await }
});

    // actions provide a bunch of synchronous, reactive variables
    // that tell us different things about the state of the action
    let submitted = add_todo.input();
    let pending = add_todo.pending();
    let todo_id = add_todo.value();

    let input_ref = create_node_ref::<Input>();

    view! {
        <form
            on:submit=move |ev| {
                ev.prevent_default(); // don't reload the page...
                let input = input_ref.get().expect("input to exist");
                add_todo.dispatch(input.value());
            }
        >
            <label>
                "What do you need to do?"
                <input type="text"
                    node_ref=input_ref
                />
            </label>
            <button type="submit">"Add Todo"</button>
        </form>
        <p>{move || pending().then(|| "Loading...")}</p>
        <p>
            "Submitted: "
            <code>{move || format!("{:#?}", submitted())}</code>
        </p>
        <p>
            "Pending: "
            <code>{move || format!("{:#?}", pending())}</code>
        </p>
        <p>
            "Todo ID: "
            <code>{move || format!("{:#?}", todo_id())}</code>
        </p>
    }
}

fn main() {
leptos::mount_to_body(App)
}


Projecting Children
As you build components you may occasionally find yourself wanting to “project” children through multiple layers of components.

The Problem
Consider the following:

pub fn LoggedIn<F, IV>(fallback: F, children: ChildrenFn) -> impl IntoView
where
F: Fn() -> IV + 'static,
IV: IntoView,
{
view! {
<Suspense
fallback=|| ()
>
<Show
// check whether user is verified
// by reading from the resource
when=move || todo!()
fallback=fallback
>
{children()}
</Show>
</Suspense>
}
}
This is pretty straightforward: when the user is logged in, we want to show children. If the user is not logged in, we want to show fallback. And while we’re waiting to find out, we just render (), i.e., nothing.

In other words, we want to pass the children of <LoggedIn/> through the <Suspense/> component to become the children of the <Show/>. This is what I mean by “projection.”

This won’t compile.

error[E0507]: cannot move out of `fallback`, a captured variable in an `Fn` closure
error[E0507]: cannot move out of `children`, a captured variable in an `Fn` closure
The problem here is that both <Suspense/> and <Show/> need to be able to construct their children multiple times. The first time you construct <Suspense/>’s children, it would take ownership of fallback and children to move them into the invocation of <Show/>, but then they're not available for future <Suspense/> children construction.

The Details
Feel free to skip ahead to the solution.

If you want to really understand the issue here, it may help to look at the expanded view macro. Here’s a cleaned-up version:

Suspense(
::leptos::component_props_builder(&Suspense)
.fallback(|| ())
.children({
// fallback and children are moved into this closure
Box::new(move || {
{
// fallback and children captured here
leptos::Fragment::lazy(|| {
vec![
(Show(
::leptos::component_props_builder(&Show)
.when(|| true)
// but fallback is moved into Show here
.fallback(fallback)
// and children is moved into Show here
.children(children)
.build(),
)
.into_view()),
]
})
}
})
})
.build(),
)
All components own their props; so the <Show/> in this case can’t be called because it only has captured references to fallback and children.

Solution
However, both <Suspense/> and <Show/> take ChildrenFn, i.e., their children should implement the Fn type so they can be called multiple times with only an immutable reference. This means we don’t need to own children or fallback; we just need to be able to pass 'static references to them.

We can solve this problem by using the store_value primitive. This essentially stores a value in the reactive system, handing ownership off to the framework in exchange for a reference that is, like signals, Copy and 'static, which we can access or modify through certain methods.

In this case, it’s really simple:

pub fn LoggedIn<F, IV>(fallback: F, children: ChildrenFn) -> impl IntoView
where
F: Fn() -> IV + 'static,
IV: IntoView,
{
let fallback = store_value(fallback);
let children = store_value(children);
view! {
<Suspense
fallback=|| ()
>
<Show
when=|| todo!()
fallback=move || fallback.with_value(|fallback| fallback())
>
{children.with_value(|children| children())}
</Show>
</Suspense>
}
}
At the top level, we store both fallback and children in the reactive scope owned by LoggedIn. Now we can simply move those references down through the other layers into the <Show/> component and call them there.

A Final Note
Note that this works because <Show/> and <Suspense/> only need an immutable reference to their children (which .with_value can give it), not ownership.

In other cases, you may need to project owned props through a function that takes ChildrenFn and therefore needs to be called more than once. In this case, you may find the clone: helper in theview macro helpful.

Consider this example

#[component]
pub fn App() -> impl IntoView {
let name = "Alice".to_string();
view! {
<Outer>
<Inner>
<Inmost name=name.clone()/>
</Inner>
</Outer>
}
}

#[component]
pub fn Outer(children: ChildrenFn) -> impl IntoView {
children()
}

#[component]
pub fn Inner(children: ChildrenFn) -> impl IntoView {
children()
}

#[component]
pub fn Inmost(name: String) -> impl IntoView {
view! {
<p>{name}</p>
}
}
Even with name=name.clone(), this gives the error

cannot move out of `name`, a captured variable in an `Fn` closure
It’s captured through multiple levels of children that need to run more than once, and there’s no obvious way to clone it into the children.

In this case, the clone: syntax comes in handy. Calling clone:name will clone name before moving it into <Inner/>’s children, which solves our ownership issue.

view! {
<Outer>
<Inner clone:name>
<Inmost name=name.clone()/>
</Inner>
</Outer>
}
These issues can be a little tricky to understand or debug, because of the opacity of the view macro. But in general, they can always be solved.


Global State Management
So far, we've only been working with local state in components, and we’ve seen how to coordinate state between parent and child components. On occasion, there are times where people look for a more general solution for global state management that can work throughout an application.

In general, you do not need this chapter. The typical pattern is to compose your application out of components, each of which manages its own local state, not to store all state in a global structure. However, there are some cases (like theming, saving user settings, or sharing data between components in different parts of your UI) in which you may want to use some kind of global state management.

The three best approaches to global state are

Using the router to drive global state via the URL
Passing signals through context
Creating a global state struct and creating lenses into it with create_slice
Option #1: URL as Global State
In many ways, the URL is actually the best way to store global state. It can be accessed from any component, anywhere in your tree. There are native HTML elements like <form> and <a> that exist solely to update the URL. And it persists across page reloads and between devices; you can share a URL with a friend or send it from your phone to your laptop and any state stored in it will be replicated.

The next few sections of the tutorial will be about the router, and we’ll get much more into these topics.

But for now, we'll just look at options #2 and #3.

Option #2: Passing Signals through Context
In the section on parent-child communication, we saw that you can use provide_context to pass signal from a parent component to a child, and use_context to read it in the child. But provide_context works across any distance. If you want to create a global signal that holds some piece of state, you can provide it and access it via context anywhere in the descendants of the component where you provide it.

A signal provided via context only causes reactive updates where it is read, not in any of the components in between, so it maintains the power of fine-grained reactive updates, even at a distance.

We start by creating a signal in the root of the app and providing it to all its children and descendants using provide_context.

#[component]
fn App() -> impl IntoView {
// here we create a signal in the root that can be consumed
// anywhere in the app.
let (count, set_count) = create_signal(0);
// we'll pass the setter to specific components,
// but provide the count itself to the whole app via context
provide_context(count);

    view! {
        // SetterButton is allowed to modify the count
        <SetterButton set_count/>
        // These consumers can only read from it
        // But we could give them write access by passing `set_count` if we wanted
        <FancyMath/>
        <ListItems/>
    }
}
<SetterButton/> is the kind of counter we’ve written several times now. (See the sandbox below if you don’t understand what I mean.)

<FancyMath/> and <ListItems/> both consume the signal we’re providing via use_context and do something with it.

/// A component that does some "fancy" math with the global count
#[component]
fn FancyMath() -> impl IntoView {
// here we consume the global count signal with `use_context`
let count = use_context::<ReadSignal<u32>>()
// we know we just provided this in the parent component
.expect("there to be a `count` signal provided");
let is_even = move || count() & 1 == 0;

    view! {
        <div class="consumer blue">
            "The number "
            <strong>{count}</strong>
            {move || if is_even() {
                " is"
            } else {
                " is not"
            }}
            " even."
        </div>
    }
}
Note that this same pattern can be applied to more complex state. If you have multiple fields you want to update independently, you can do that by providing some struct of signals:

#[derive(Copy, Clone, Debug)]
struct GlobalState {
count: RwSignal<i32>,
name: RwSignal<String>
}

impl GlobalState {
pub fn new() -> Self {
Self {
count: create_rw_signal(0),
name: create_rw_signal("Bob".to_string())
}
}
}

#[component]
fn App() -> impl IntoView {
provide_context(GlobalState::new());

    // etc.
}
Option #3: Create a Global State Struct and Slices
You may find it cumbersome to wrap each field of a structure in a separate signal like this. In some cases, it can be useful to create a plain struct with non-reactive fields, and then wrap that in a signal.

#[derive(Copy, Clone, Debug, Default)]
struct GlobalState {
count: i32,
name: String
}

#[component]
fn App() -> impl IntoView {
provide_context(create_rw_signal(GlobalState::default()));

    // etc.
}
But there’s a problem: because our whole state is wrapped in one signal, updating the value of one field will cause reactive updates in parts of the UI that only depend on the other.

let state = expect_context::<RwSignal<GlobalState>>();
view! {
<button on:click=move |_| state.update(|state| state.count += 1)>"+1"</button>
<p>{move || state.with(|state| state.name.clone())}</p>
}
In this example, clicking the button will cause the text inside <p> to be updated, cloning state.name again! Because signals are the atomic unit of reactivity, updating any field of the signal triggers updates to everything that depends on the signal.

There’s a better way. You can take fine-grained, reactive slices by using create_memo or create_slice (which uses create_memo but also provides a setter). “Memoizing” a value means creating a new reactive value which will only update when it changes. “Memoizing a slice” means creating a new reactive value which will only update when some field of the state struct updates.

Here, instead of reading from the state signal directly, we create “slices” of that state with fine-grained updates via create_slice. Each slice signal only updates when the particular piece of the larger struct it accesses updates. This means you can create a single root signal, and then take independent, fine-grained slices of it in different components, each of which can update without notifying the others of changes.

/// A component that updates the count in the global state.
#[component]
fn GlobalStateCounter() -> impl IntoView {
let state = expect_context::<RwSignal<GlobalState>>();

    // `create_slice` lets us create a "lens" into the data
    let (count, set_count) = create_slice(

        // we take a slice *from* `state`
        state,
        // our getter returns a "slice" of the data
        |state| state.count,
        // our setter describes how to mutate that slice, given a new value
        |state, n| state.count = n,
    );

    view! {
        <div class="consumer blue">
            <button
                on:click=move |_| {
                    set_count(count() + 1);
                }
            >
                "Increment Global Count"
            </button>
            <br/>
            <span>"Count is: " {count}</span>
        </div>
    }
}
Clicking this button only updates state.count, so if we create another slice somewhere else that only takes state.name, clicking the button won’t cause that other slice to update. This allows you to combine the benefits of a top-down data flow and of fine-grained reactive updates.

Note: There are some significant drawbacks to this approach. Both signals and memos need to own their values, so a memo will need to clone the field’s value on every change. The most natural way to manage state in a framework like Leptos is always to provide signals that are as locally-scoped and fine-grained as they can be, not to hoist everything up into global state. But when you do need some kind of global state, create_slice can be a useful tool.

use leptos::*;

// So far, we've only been working with local state in components
// We've only seen how to communicate between parent and child components
// But there are also more general ways to manage global state
//
// The three best approaches to global state are
// 1. Using the router to drive global state via the URL
// 2. Passing signals through context
// 3. Creating a global state struct and creating lenses into it with `create_slice`
//
// Option #1: URL as Global State
// The next few sections of the tutorial will be about the router.
// So for now, we'll just look at options #2 and #3.

// Option #2: Pass Signals through Context
//
// In virtual DOM libraries like React, using the Context API to manage global
// state is a bad idea: because the entire app exists in a tree, changing
// some value provided high up in the tree can cause the whole app to render.
//
// In fine-grained reactive libraries like Leptos, this is simply not the case.
// You can create a signal in the root of your app and pass it down to other
// components using provide_context(). Changing it will only cause rerendering
// in the specific places it is actually used, not the whole app.
#[component]
fn Option2() -> impl IntoView {
// here we create a signal in the root that can be consumed
// anywhere in the app.
let (count, set_count) = create_signal(0);
// we'll pass the setter to specific components,
// but provide the count itself to the whole app via context
provide_context(count);

    view! {
        <h2>"Option 2: Passing Signals"</h2>
        // SetterButton is allowed to modify the count
        <SetterButton set_count/>
        // These consumers can only read from it
        // But we could give them write access by passing `set_count` if we wanted
        <div style="display: flex">
            <FancyMath/>
            <ListItems/>
        </div>
    }
}

/// A button that increments our global counter.
#[component]
fn SetterButton(set_count: WriteSignal<u32>) -> impl IntoView {
view! {
<div class="provider red">
<button on:click=move |_| set_count.update(|count| *count += 1)>
"Increment Global Count"
</button>
</div>
}
}

/// A component that does some "fancy" math with the global count
#[component]
fn FancyMath() -> impl IntoView {
// here we consume the global count signal with `use_context`
let count = use_context::<ReadSignal<u32>>()
// we know we just provided this in the parent component
.expect("there to be a `count` signal provided");
let is_even = move || count() & 1 == 0;

    view! {
        <div class="consumer blue">
            "The number "
            <strong>{count}</strong>
            {move || if is_even() {
                " is"
            } else {
                " is not"
            }}
            " even."
        </div>
    }
}

/// A component that shows a list of items generated from the global count.
#[component]
fn ListItems() -> impl IntoView {
// again, consume the global count signal with `use_context`
let count = use_context::<ReadSignal<u32>>().expect("there to be a `count` signal provided");

    let squares = move || {
        (0..count())
            .map(|n| view! { <li>{n}<sup>"2"</sup> " is " {n * n}</li> })
            .collect::<Vec<_>>()
    };

    view! {
        <div class="consumer green">
            <ul>{squares}</ul>
        </div>
    }
}

// Option #3: Create a Global State Struct
//
// You can use this approach to build a single global data structure
// that holds the state for your whole app, and then access it by
// taking fine-grained slices using `create_slice` or `create_memo`,
// so that changing one part of the state doesn't cause parts of your
// app that depend on other parts of the state to change.

#[derive(Default, Clone, Debug)]
struct GlobalState {
count: u32,
name: String,
}

#[component]
fn Option3() -> impl IntoView {
// we'll provide a single signal that holds the whole state
// each component will be responsible for creating its own "lens" into it
let state = create_rw_signal(GlobalState::default());
provide_context(state);

    view! {
        <h2>"Option 3: Passing Signals"</h2>
        <div class="red consumer" style="width: 100%">
            <h3>"Current Global State"</h3>
            <pre>
                {move || {
                    format!("{:#?}", state.get())
                }}
            </pre>
        </div>
        <div style="display: flex">
            <GlobalStateCounter/>
            <GlobalStateInput/>
        </div>
    }
}

/// A component that updates the count in the global state.
#[component]
fn GlobalStateCounter() -> impl IntoView {
let state = use_context::<RwSignal<GlobalState>>().expect("state to have been provided");

    // `create_slice` lets us create a "lens" into the data
    let (count, set_count) = create_slice(

        // we take a slice *from* `state`
        state,
        // our getter returns a "slice" of the data
        |state| state.count,
        // our setter describes how to mutate that slice, given a new value
        |state, n| state.count = n,
    );

    view! {
        <div class="consumer blue">
            <button
                on:click=move |_| {
                    set_count(count() + 1);
                }
            >
                "Increment Global Count"
            </button>
            <br/>
            <span>"Count is: " {count}</span>
        </div>
    }
}

/// A component that updates the name in the global state.
#[component]
fn GlobalStateInput() -> impl IntoView {
let state = use_context::<RwSignal<GlobalState>>().expect("state to have been provided");

    // this slice is completely independent of the `count` slice
    // that we created in the other component
    // neither of them will cause the other to rerun
    let (name, set_name) = create_slice(
        // we take a slice *from* `state`
        state,
        // our getter returns a "slice" of the data
        |state| state.name.clone(),
        // our setter describes how to mutate that slice, given a new value
        |state, n| state.name = n,
    );

    view! {
        <div class="consumer green">
            <input
                type="text"
                prop:value=name
                on:input=move |ev| {
                    set_name(event_target_value(&ev));
                }
            />
            <br/>
            <span>"Name is: " {name}</span>
        </div>
    }
}
// This `main` function is the entry point into the app
// It just mounts our component to the <body>
// Because we defined it as `fn App`, we can now use it in a
// template as <App/>
fn main() {
leptos::mount_to_body(|| view! { <Option2/><Option3/> })
}


The Basics
Routing drives most websites. A router is the answer to the question, “Given this URL, what should appear on the page?”

A URL consists of many parts. For example, the URL https://my-cool-blog.com/blog/search?q=Search#results consists of

a scheme: https
a domain: my-cool-blog.com
a path: /blog/search
a query (or search): ?q=Search
a hash: #results
The Leptos Router works with the path and query (/blog/search?q=Search). Given this piece of the URL, what should the app render on the page?

The Philosophy
In most cases, the path should drive what is displayed on the page. From the user’s perspective, for most applications, most major changes in the state of the app should be reflected in the URL. If you copy and paste the URL and open it in another tab, you should find yourself more or less in the same place.

In this sense, the router is really at the heart of the global state management for your application. More than anything else, it drives what is displayed on the page.

The router handles most of this work for you by mapping the current location to particular components.