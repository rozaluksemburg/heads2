Introduction
This book is intended as an introduction to the Leptos Web framework. It will walk through the fundamental concepts you need to build applications, beginning with a simple application rendered in the browser, and building toward a full-stack application with server-side rendering and hydration.

The guide doesn’t assume you know anything about fine-grained reactivity or the details of modern Web frameworks. It does assume you are familiar with the Rust programming language, HTML, CSS, and the DOM and basic Web APIs.

Leptos is most similar to frameworks like Solid (JavaScript) and Sycamore (Rust). There are some similarities to other frameworks like React (JavaScript), Svelte (JavaScript), Yew (Rust), and Dioxus (Rust), so knowledge of one of those frameworks may also make it easier to understand Leptos.

You can find more detailed docs for each part of the API at Docs.rs.

The source code for the book is available here. PRs for typos or clarification are always welcome.


Getting Started
There are two basic paths to getting started with Leptos:

Client-side rendering (CSR) with Trunk - a great option if you just want to make a snappy website with Leptos, or work with a pre-existing server or API. In CSR mode, Trunk compiles your Leptos app to WebAssembly (WASM) and runs it in the browser like a typical Javascript single-page app (SPA). The advantages of Leptos CSR include faster build times and a quicker iterative development cycle, as well as a simpler mental model and more options for deploying your app. CSR apps do come with some disadvantages: initial load times for your end users are slower compared to a server-side rendering approach, and the usual SEO challenges that come along with using a JS single-page app model apply to Leptos CSR apps as well. Also note that, under the hood, an auto-generated snippet of JS is used to load the Leptos WASM bundle, so JS must be enabled on the client device for your CSR app to display properly. As with all software engineering, there are trade-offs here you'll need to consider.

Full-stack, server-side rendering (SSR) with cargo-leptos - SSR is a great option for building CRUD-style websites and custom web apps if you want Rust powering both your frontend and backend. With the Leptos SSR option, your app is rendered to HTML on the server and sent down to the browser; then, WebAssembly is used to instrument the HTML so your app becomes interactive - this process is called 'hydration'. On the server side, Leptos SSR apps integrate closely with your choice of either Actix-web or Axum server libraries, so you can leverage those communities' crates to help build out your Leptos server. The advantages of taking the SSR route with Leptos include helping you get the best initial load times and optimal SEO scores for your web app. SSR apps can also dramatically simplify working across the server/client boundary via a Leptos feature called "server functions", which lets you transparently call functions on the server from your client code (more on this feature later). Full-stack SSR isn't all rainbows and butterflies, though - disadvantages include a slower developer iteration loop (because you need to recompile both the server and client when making Rust code changes), as well as some added complexity that comes along with hydration.

By the end of the book, you should have a good idea of which trade-offs to make and which route to take - CSR or SSR - depending on your project's requirements.

In Part 1 of this book, we'll start with client-side rendering Leptos sites and building reactive UIs using Trunk to serve our JS and WASM bundle to the browser.

We’ll introduce cargo-leptos in Part 2 of this book, which is all about working with the full power of Leptos in its full-stack, SSR mode.

Note

If you're coming from the Javascript world and terms like client-side rendering (CSR) and server-side rendering (SSR) are unfamiliar to you, the easiest way to understand the difference is by analogy:

Leptos' CSR mode is similar to working with React (or a 'signals'-based framework like SolidJS), and focuses on producing a client-side UI which you can use with any tech stack on the server.

Using Leptos' SSR mode is similar to working with a full-stack framework like Next.js in the React world (or Solid's "SolidStart" framework) - SSR helps you build sites and apps that are rendered on the server then sent down to the client. SSR can help to improve your site's loading performance and accessibility as well as make it easier for one person to work on both client- and server-side without needing to context-switch between different languages for frontend and backend.

The Leptos framework can be used either in CSR mode to just make a UI (like React), or you can use Leptos in full-stack SSR mode (like Next.js) so that you can build both your UI and your server with one language: Rust.

Hello World! Getting Set up for Leptos CSR Development
First up, make sure Rust is installed and up-to-date (see here if you need instructions).

If you don’t have it installed already, you can install the "Trunk" tool for running Leptos CSR sites by running the following on the command-line:

cargo install trunk
And then create a basic Rust project

cargo init leptos-tutorial
cd into your new leptos-tutorial project and add leptos as a dependency

cargo add leptos --features=csr,nightly
Or you can leave off nightly if you're using stable Rust

cargo add leptos --features=csr
Using nightly Rust, and the nightly feature in Leptos enables the function-call syntax for signal getters and setters that is used in most of this book.

To use nightly Rust, you can either opt into nightly for all your Rust projects by running

rustup toolchain install nightly
rustup default nightly
or only for this project

rustup toolchain install nightly
cd <into your project>
rustup override set nightly
See here for more details.

If you’d rather use stable Rust with Leptos, you can do that too. In the guide and examples, you’ll just use the ReadSignal::get() and WriteSignal::set() methods instead of calling signal getters and setters as functions.

Make sure you've added the wasm32-unknown-unknown target so that Rust can compile your code to WebAssembly to run in the browser.

rustup target add wasm32-unknown-unknown
Create a simple index.html in the root of the leptos-tutorial directory

<!DOCTYPE html>
<html>
  <head></head>
  <body></body>
</html>
And add a simple “Hello, world!” to your main.rs

use leptos::*;

fn main() {
mount_to_body(|| view! { <p>"Hello, world!"</p> })
}
Your directory structure should now look something like this


leptos_tutorial
├── src
│   └── main.rs
├── Cargo.toml
├── index.html
Now run trunk serve --open from the root of the leptos-tutorial directory. Trunk should automatically compile your app and open it in your default browser. If you make edits to main.rs, Trunk will recompile your source code and live-reload the page.

Welcome to the world of UI development with Rust and WebAssembly (WASM), powered by Leptos and Trunk!

Note

If you are using Windows, note that trunk serve --open may not work. If you have issues with --open, simply use trunk serve and open a browser tab manually.

Now before we get started building your first real UI's with Leptos, there are a couple of things you might want to know to help make your experience with Leptos just a little bit easier.


Leptos Developer Experience Improvements
There are a couple of things you can do to improve your experience of developing websites and apps with Leptos. You may want to take a few minutes and set up your environment to optimize your development experience, especially if you want to code along with the examples in this book.

1) Set up console_error_panic_hook
   By default, panics that happen while running your WASM code in the browser just throw an error in the browser with an unhelpful message like Unreachable executed and a stack trace that points into your WASM binary.

With console_error_panic_hook, you get an actual Rust stack trace that includes a line in your Rust source code.

It's very easy to set up:

Run cargo add console_error_panic_hook in your project
In your main function, add console_error_panic_hook::set_once();
If this is unclear, click here for an example.

Now you should have much better panic messages in the browser console!

2) Editor Autocompletion inside #[component] and #[server]
   Because of the nature of macros (they can expand from anything to anything, but only if the input is exactly correct at that instant) it can be hard for rust-analyzer to do proper autocompletion and other support.

If you run into issues using these macros in your editor, you can explicitly tell rust-analyzer to ignore certain proc macros. For the #[server] macro especially, which annotates function bodies but doesn't actually transform anything inside the body of your function, this can be really helpful.

Starting in Leptos version 0.5.3, rust-analyzer support was added for the #[component] macro, but if you run into issues, you may want to add #[component] to the macro ignore list as well (see below). Note that this means that rust-analyzer doesn't know about your component props, which may generate its own set of errors or warnings in the IDE.

VSCode settings.json:

"rust-analyzer.procMacro.ignored": {
"leptos_macro": [
// optional:
// "component",
"server"
],
}
VSCode with cargo-leptos settings.json:

"rust-analyzer.procMacro.ignored": {
"leptos_macro": [
// optional:
// "component",
"server"
],
},
// if code that is cfg-gated for the `ssr` feature is shown as inactive,
// you may want to tell rust-analyzer to enable the `ssr` feature by default
//
// you can also use `rust-analyzer.cargo.allFeatures` to enable all features
"rust-analyzer.cargo.features": ["ssr"]
neovim with lspconfig:

require('lspconfig').rust_analyzer.setup {
-- Other Configs ...
settings = {
["rust-analyzer"] = {
-- Other Settings ...
procMacro = {
ignored = {
leptos_macro = {
-- optional: --
-- "component",
"server",
},
},
},
},
}
}
Helix, in .helix/languages.toml:

[[language]]
name = "rust"

[language-server.rust-analyzer]
config = { procMacro = { ignored = { leptos_macro = [
# Optional:
# "component",
"server"
] } } }
Zed, in settings.json:

{
-- Other Settings ...
"lsp": {
"rust-analyzer": {
"procMacro": {
"ignored": [
// optional:
// "component",
"server"
]
}
}
}
}
SublimeText 3, under LSP-rust-analyzer.sublime-settings in Goto Anything... menu:

// Settings in here override those in "LSP-rust-analyzer/LSP-rust-analyzer.sublime-settings"
{
"rust-analyzer.procMacro.ignored": {
"leptos_macro": [
// optional:
// "component",
"server"
],
},
}
3) Set up leptosfmt With Rust Analyzer (optional)
   leptosfmt is a formatter for the Leptos view! macro (inside of which you'll typically write your UI code). Because the view! macro enables an 'RSX' (like JSX) style of writing your UI's, cargo-fmt has a harder time auto-formatting your code that's inside the view! macro. leptosfmt is a crate that solves your formatting issues and keeps your RSX-style UI code looking nice and tidy!

leptosfmt can be installed and used via the command line or from within your code editor:

First, install the tool with cargo install leptosfmt.

If you just want to use the default options from the command line, just run leptosfmt ./**/*.rs from the root of your project to format all the rust files using leptosfmt.

If you wish to set up your editor to work with leptosfmt, or if you wish to customize your leptosfmt experience, please see the instructions available on the leptosfmt github repo's README.md page.

Just note that it's recommended to set up your editor with leptosfmt on a per-workspace basis for best results.


The Leptos Community and leptos-* Crates
The Community
One final note before we get to building with Leptos: if you haven't already, feel free to join the growing community on the Leptos Discord and on Github. Our Discord channel in particular is very active and friendly - we'd love to have you there!

Note

If you find a chapter or an explanation that isn't clear while you're working your way through the Leptos book, just mention it in the "docs-and-education" channel or ask a question in "help" so we can clear things up and update the book for others.

As you get further along in your Leptos journey and find that you have questions about "how to do 'x' with Leptos", then search the Discord "help" channel to see if a similar question has been asked before, or feel free to post your own question - the community is quite helpful and very responsive.

The "Discussions" on Github are also a great place for asking questions and keeping up with Leptos announcements.

And of course, if you run into any bugs while developing with Leptos or would like to make a feature request (or contribute a bug fix / new feature), open up an issue on the Github issue tracker.

Leptos-* Crates
The community has built a growing number of Leptos-related crates that will help you get productive with Leptos projects more quickly - check out the list of crates built on top of Leptos and contributed by the community on the Awesome Leptos repo on Github.

If you want to find the newest, up-and-coming Leptos-related crates, check out the "Tools and Libraries" section of the Leptos Discord. In that section, there are channels for the Leptos view! macro formatter (in the "leptosfmt" channel); there's a channel for the utility library "leptos-use"; another channel for the UI component libary "leptonic"; and a "libraries" channel where new leptos-* crates are discussed before making their way into the growing list of crates and resources available on Awesome Leptos.


Part 1: Building User Interfaces
In the first part of the book, we're going to look at building user interfaces on the client-side using Leptos. Under the hood, Leptos and Trunk are bundling up a snippet of Javascript which will load up the Leptos UI, which has been compiled to WebAssembly to drive the interactivity in your CSR (client-side rendered) website.

Part 1 will introduce you to the basic tools you need to build a reactive user interface powered by Leptos and Rust. By the end of Part 1, you should be able to build a snappy synchronous website that's rendered in the browser and which you can deploy on any static-site hosting service, like Github Pages or Vercel.

Info

To get the most out of this book, we encourage you to code along with the examples provided. In the Getting Started and Leptos DX chapters, we showed you how to set up a basic project with Leptos and Trunk, including WASM error handling in the browser. That basic setup is enough to get you started developing with Leptos.

If you'd prefer to get started using a more full-featured template which demonstrates how to set up a few of the basics you'd see in a real Leptos project, such as routing, (covered later in the book), injecting <Title> and <Meta> tags into the page head, and a few other niceties, then feel free to utilize the leptos-rs start-trunk template repo to get up and running.

The start-trunk template requires that you have Trunk and cargo-generate installed, which you can get by running cargo install trunk and cargo install cargo-generate.

To use the template to set up your project, just run

cargo generate --git https://github.com/leptos-community/start-csr

then run

trunk serve --port 3000 --open

in the newly created app's directory to start developing your app. The Trunk server will reload your app on file changes, making development relatively seamless.


A Basic Component
That “Hello, world!” was a very simple example. Let’s move on to something a little more like an ordinary app.

First, let’s edit the main function so that, instead of rendering the whole app, it just renders an <App/> component. Components are the basic unit of composition and design in most web frameworks, and Leptos is no exception. Conceptually, they are similar to HTML elements: they represent a section of the DOM, with self-contained, defined behavior. Unlike HTML elements, they are in PascalCase, so most Leptos applications will start with something like an <App/> component.

fn main() {
leptos::mount_to_body(|| view! { <App/> })
}
Now let’s define our <App/> component itself. Because it’s relatively simple, I’ll give you the whole thing up front, then walk through it line by line.

#[component]
fn App() -> impl IntoView {
let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                set_count(3);
            }
        >
            "Click me: "
            // on stable, this is move || count.get();
            {move || count()}
        </button>
    }
}
The Component Signature
#[component]
Like all component definitions, this begins with the #[component] macro. #[component] annotates a function so it can be used as a component in your Leptos application. We’ll see some of the other features of this macro in a couple chapters.

fn App() -> impl IntoView
Every component is a function with the following characteristics

It takes zero or more arguments of any type.
It returns impl IntoView, which is an opaque type that includes anything you could return from a Leptos view.
Component function arguments are gathered together into a single props struct which is built by the view macro as needed.

The Component Body
The body of the component function is a set-up function that runs once, not a render function that reruns multiple times. You’ll typically use it to create a few reactive variables, define any side effects that run in response to those values changing, and describe the user interface.

let (count, set_count) = create_signal(0);
create_signal creates a signal, the basic unit of reactive change and state management in Leptos. This returns a (getter, setter) tuple. To access the current value, you’ll use count.get() (or, on nightly Rust, the shorthand count()). To set the current value, you’ll call set_count.set(...) (or set_count(...)).

.get() clones the value and .set() overwrites it. In many cases, it’s more efficient to use .with() or .update(); check out the docs for ReadSignal and WriteSignal if you’d like to learn more about those trade-offs at this point.

The View
Leptos defines user interfaces using a JSX-like format via the view macro.

view! {
<button
// define an event listener with on:
on:click=move |_| {
set_count(3);
}
>
// text nodes are wrapped in quotation marks
"Click me: "
// blocks can include Rust code
{move || count()}
</button>
}
This should mostly be easy to understand: it looks like HTML, with a special on:click to define a click event listener, a text node that’s formatted like a Rust string, and then...

{move || count()}
whatever that is.

People sometimes joke that they use more closures in their first Leptos application than they’ve ever used in their lives. And fair enough. Basically, passing a function into the view tells the framework: “Hey, this is something that might change.”

When we click the button and call set_count, the count signal is updated. This move || count() closure, whose value depends on the value of count, reruns, and the framework makes a targeted update to that one specific text node, touching nothing else in your application. This is what allows for extremely efficient updates to the DOM.

Now, if you have Clippy on—or if you have a particularly sharp eye—you might notice that this closure is redundant, at least if you’re in nightly Rust. If you’re using Leptos with nightly Rust, signals are already functions, so the closure is unnecessary. As a result, you can write a simpler view:

view! {
<button /* ... */>
"Click me: "
// identical to {move || count()}
{count}
</button>
}
Remember—and this is very important—only functions are reactive. This means that {count} and {count()} do very different things in your view. {count} passes in a function, telling the framework to update the view every time count changes. {count()} accesses the value of count once, and passes an i32 into the view, rendering it once, unreactively. You can see the difference in the CodeSandbox below!

Let’s make one final change. set_count(3) is a pretty useless thing for a click handler to do. Let’s replace “set this value to 3” with “increment this value by 1”:

move |_| {
set_count.update(|n| *n += 1);
}
You can see here that while set_count just sets the value, set_count.update() gives us a mutable reference and mutates the value in place. Either one will trigger a reactive update in our UI.

Throughout this tutorial, we’ll use CodeSandbox to show interactive examples. To show the browser in the sandbox, you may need to click Add DevTools > Other Previews > 8080. Hover over any of the variables to show Rust-Analyzer details and docs for what’s going on. Feel free to fork the examples to play with them yourself!


view: Dynamic Classes, Styles and Attributes
So far we’ve seen how to use the view macro to create event listeners and to create dynamic text by passing a function (such as a signal) into the view.

But of course there are other things you might want to update in your user interface. In this section, we’ll look at how to update classes, styles and attributes dynamically, and we’ll introduce the concept of a derived signal.

Let’s start with a simple component that should be familiar: click a button to increment a counter.

#[component]
fn App() -> impl IntoView {
let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
            {move || count()}
        </button>
    }
}
So far, this is just the example from the last chapter.

Dynamic Classes
Now let’s say I’d like to update the list of CSS classes on this element dynamically. For example, let’s say I want to add the class red when the count is odd. I can do this using the class: syntax.

class:red=move || count() % 2 == 1
class: attributes take

the class name, following the colon (red)
a value, which can be a bool or a function that returns a bool
When the value is true, the class is added. When the value is false, the class is removed. And if the value is a function that accesses a signal, the class will reactively update when the signal changes.

Now every time I click the button, the text should toggle between red and black as the number switches between even and odd.

<button
on:click=move |_| {
set_count.update(|n| *n += 1);
}
// the class: syntax reactively updates a single class
// here, we'll set the `red` class when `count` is odd
class:red=move || count() % 2 == 1
>
    "Click me"
</button>
If you’re following along, make sure you go into your index.html and add something like this:

<style>
  .red {
    color: red;
  }
</style>
Some CSS class names can’t be directly parsed by the view macro, especially if they include a mix of dashes and numbers or other characters. In that case, you can use a tuple syntax: class=("name", value) still directly updates a single class.

class=("button-20", move || count() % 2 == 1)
Dynamic Styles
Individual CSS properties can be directly updated with a similar style: syntax.

    let (x, set_x) = create_signal(0);
        view! {
            <button
                on:click={move |_| {
                    set_x.update(|n| *n += 10);
                }}
                // set the `style` attribute
                style="position: absolute"
                // and toggle individual CSS properties with `style:`
                style:left=move || format!("{}px", x() + 100)
                style:background-color=move || format!("rgb({}, {}, 100)", x(), 100)
                style:max-width="400px"
                // Set a CSS variable for stylesheet use
                style=("--columns", x)
            >
                "Click to Move"
            </button>
    }
Dynamic Attributes
The same applies to plain attributes. Passing a plain string or primitive value to an attribute gives it a static value. Passing a function (including a signal) to an attribute causes it to update its value reactively. Let’s add another element to our view:

<progress
max="50"
// signals are functions, so `value=count` and `value=move || count.get()`
// are interchangeable.
value=count
/>
Now every time we set the count, not only will the class of the <button> be toggled, but the value of the <progress> bar will increase, which means that our progress bar will move forward.

Derived Signals
Let’s go one layer deeper, just for fun.

You already know that we create reactive interfaces just by passing functions into the view. This means that we can easily change our progress bar. For example, suppose we want it to move twice as fast:

<progress
max="50"
value=move || count() * 2
/>
But imagine we want to reuse that calculation in more than one place. You can do this using a derived signal: a closure that accesses a signal.

let double_count = move || count() * 2;

/* insert the rest of the view */
<progress
max="50"
// we use it once here
value=double_count
/>
<p>
    "Double Count: "
    // and again here
    {double_count}
</p>
Derived signals let you create reactive computed values that can be used in multiple places in your application with minimal overhead.

Note: Using a derived signal like this means that the calculation runs once per signal change (when count() changes) and once per place we access double_count; in other words, twice. This is a very cheap calculation, so that’s fine. We’ll look at memos in a later chapter, which were designed to solve this problem for expensive calculations.

Advanced Topic: Injecting Raw HTML
The view macro provides support for an additional attribute, inner_html, which can be used to directly set the HTML contents of any element, wiping out any other children you’ve given it. Note that this does not escape the HTML you provide. You should make sure that it only contains trusted input or that any HTML entities are escaped, to prevent cross-site scripting (XSS) attacks.

let html = "<p>This HTML will be injected.</p>";
view! {
  <div inner_html=html/>
}

full view macros docs

Macro leptos::viewCopy item path
source · [−]
view!() { /* proc-macro */ }
The view macro uses RSX (like JSX, but Rust!) It follows most of the same rules as HTML, with the following differences:

Text content should be provided as a Rust string, i.e., double-quoted:
view! { <p>"Here’s some text"</p> };
Self-closing tags need an explicit / as in XML/XHTML
ⓘ
// ❌ not like this
view! { <input type="text" name="name"> }
// ✅ add that slash
view! { <input type="text" name="name" /> }
Components (functions annotated with #[component]) can be inserted as camel-cased tags. (Generics on components are specified as <Component<T>/>, not the turbofish <Component::<T>/>.)
view! { <div><Counter initial_value=3 /></div> }
Dynamic content can be wrapped in curly braces ({ }) to insert text nodes, elements, or set attributes. If you insert a signal here, Leptos will create an effect to update the DOM whenever the value changes. (“Signal” here means Fn() -> T where T is the appropriate type for that node: a String in case of text nodes, a bool for class: attributes, etc.)

Attributes can take a wide variety of primitive types that can be converted to strings. They can also take an Option, in which case Some sets the attribute and None removes the attribute.

ⓘ
let (count, set_count) = create_signal(0);

view! {
// ❌ not like this: `count.get()` returns an `i32`, not a function
  <p>{count.get()}</p>
  // ✅ this is good: Leptos sees the function and knows it's a dynamic value
  <p>{move || count.get()}</p>
  // 🔥 with the `nightly` feature, `count` is a function, so `count` itself can be passed directly into the view
  <p>{count}</p>
}
Event handlers can be added with on: attributes. In most cases, the events are given the correct type based on the event name.
view! {
  <button on:click=|ev| {
    log::debug!("click event: {ev:#?}");
  }>
    "Click me"
  </button>
}
DOM properties can be set with prop: attributes, which take any primitive type or JsValue (or a signal that returns a primitive or JsValue). They can also take an Option, in which case Some sets the property and None deletes the property.
let (name, set_name) = create_signal("Alice".to_string());

view! {
<input
type="text"
name="user_name"
value={move || name.get()} // this only sets the default value!
prop:value={move || name.get()} // here's how you update values. Sorry, I didn’t invent the DOM.
on:click=move |ev| set_name.set(event_target_value(&ev)) // `event_target_value` is a useful little Leptos helper
/>
}
Classes can be toggled with class: attributes, which take a bool (or a signal that returns a bool).
let (count, set_count) = create_signal(2);
view! { <div class:hidden-div={move || count.get() < 3}>"Now you see me, now you don’t."</div> }
Class names can include dashes, and since v0.5.0 can include a dash-separated segment of only numbers.

let (count, set_count) = create_signal(2);
view! { <div class:hidden-div-25={move || count.get() < 3}>"Now you see me, now you don’t."</div> }
Class names cannot include special symbols.

ⓘ
let (count, set_count) = create_signal(2);
// class:hidden-[div]-25 is invalid attribute name
view! { <div class:hidden-[div]-25={move || count.get() < 3}>"Now you see me, now you don’t."</div> }
However, you can pass arbitrary class names using the syntax class=("name", value).

let (count, set_count) = create_signal(2);
// this allows you to use CSS frameworks that include complex class names
view! {
  <div
    class=("is-[this_-_really]-necessary-42", move || count.get() < 3)
  >
    "Now you see me, now you don’t."
  </div>
}
Individual styles can also be set with style: or style=("property-name", value) syntax.
let (x, set_x) = create_signal(0);
let (y, set_y) = create_signal(0);
view! {
  <div
    style="position: absolute"
    style:left=move || format!("{}px", x.get())
    style:top=move || format!("{}px", y.get())
    style=("background-color", move || format!("rgb({}, {}, 100)", x.get(), y.get()))
  >
    "Moves when coordinates change"
  </div>
}
You can use the node_ref or _ref attribute to store a reference to its DOM element in a NodeRef to use later.
use leptos::html::Input;

let (value, set_value) = create_signal(0);
let my_input = create_node_ref::<Input>();
view! { <input type="text" _ref=my_input/> }
// `my_input` now contains an `Element` that we can use anywhere
You can add the same class to every element in the view by passing in a special class = {/* ... */}, argument after ``. This is useful for injecting a class provided by a scoped styling library.
let class = "mycustomclass";
view! { class = class,
  <div> // will have class="mycustomclass"
    <p>"Some text"</p> // will also have class "mycustomclass"
  </div>
}
You can set any HTML element’s innerHTML with the inner_html attribute on an element. Be careful: this HTML will not be escaped, so you should ensure that it only contains trusted input.
let html = "<p>This HTML will be injected.</p>";
view! {
  <div inner_html=html/>
}
Here’s a simple example that shows off several of these features, put together


pub fn SimpleCounter() -> impl IntoView {
// create a reactive signal with the initial value
let (value, set_value) = create_signal(0);

    // create event handlers for our buttons
    // note that `value` and `set_value` are `Copy`, so it's super easy to move them into closures
    let clear = move |_ev| set_value.set(0);
    let decrement = move |_ev| set_value.update(|value| *value -= 1);
    let increment = move |_ev| set_value.update(|value| *value += 1);

    view! {
        <div>
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement>"-1"</button>
            <span>"Value: " {move || value.get().to_string()} "!"</span>
            <button on:click=increment>"+1"</button>
        </div>
    }
}

+ one more code

use leptos::*;

#[component]
fn App() -> impl IntoView {
let (count, set_count) = create_signal(0);

    // a "derived signal" is a function that accesses other signals
    // we can use this to create reactive values that depend on the
    // values of one or more other signals
    let double_count = move || count() * 2;

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
            // the class: syntax reactively updates a single class
            // here, we'll set the `red` class when `count` is odd
            class:red=move || count() % 2 == 1
        >
            "Click me"
        </button>
        // NOTE: self-closing tags like <br> need an explicit /
        <br/>

        // We'll update this progress bar every time `count` changes
        <progress
            // static attributes work as in HTML
            max="50"

            // passing a function to an attribute
            // reactively sets that attribute
            // signals are functions, so `value=count` and `value=move || count.get()`
            // are interchangeable.
            value=count
        >
        </progress>
        <br/>

        // This progress bar will use `double_count`
        // so it should move twice as fast!
        <progress
            max="50"
            // derived signals are functions, so they can also
            // reactively update the DOM
            value=double_count
        >
        </progress>
        <p>"Count: " {count}</p>
        <p>"Double Count: " {double_count}</p>
    }
}

fn main() {
leptos::mount_to_body(App)
}




Components and Props
So far, we’ve been building our whole application in a single component. This is fine for really tiny examples, but in any real application you’ll need to break the user interface out into multiple components, so you can break your interface down into smaller, reusable, composable chunks.

Let’s take our progress bar example. Imagine that you want two progress bars instead of one: one that advances one tick per click, one that advances two ticks per click.

You could do this by just creating two <progress> elements:

let (count, set_count) = create_signal(0);
let double_count = move || count() * 2;

view! {
<progress
max="50"
value=count
/>
<progress
max="50"
value=double_count
/>
}
But of course, this doesn’t scale very well. If you want to add a third progress bar, you need to add this code another time. And if you want to edit anything about it, you need to edit it in triplicate.

Instead, let’s create a <ProgressBar/> component.

#[component]
fn ProgressBar() -> impl IntoView {
view! {
<progress
max="50"
// hmm... where will we get this from?
value=progress
/>
}
}
There’s just one problem: progress is not defined. Where should it come from? When we were defining everything manually, we just used the local variable names. Now we need some way to pass an argument into the component.

Component Props
We do this using component properties, or “props.” If you’ve used another frontend framework, this is probably a familiar idea. Basically, properties are to components as attributes are to HTML elements: they let you pass additional information into the component.

In Leptos, you define props by giving additional arguments to the component function.

#[component]
fn ProgressBar(
progress: ReadSignal<i32>
) -> impl IntoView {
view! {
<progress
max="50"
// now this works
value=progress
/>
}
}
Now we can use our component in the main <App/> component’s view.

#[component]
fn App() -> impl IntoView {
let (count, set_count) = create_signal(0);
view! {
<button on:click=move |_| { set_count.update(|n| *n += 1); }>
"Click me"
</button>
// now we use our component!
<ProgressBar progress=count/>
}
}
Using a component in the view looks a lot like using an HTML element. You’ll notice that you can easily tell the difference between an element and a component because components always have PascalCase names. You pass the progress prop in as if it were an HTML element attribute. Simple.

Reactive and Static Props
You’ll notice that throughout this example, progress takes a reactive ReadSignal<i32>, and not a plain i32. This is very important.

Component props have no special meaning attached to them. A component is simply a function that runs once to set up the user interface. The only way to tell the interface to respond to changes is to pass it a signal type. So if you have a component property that will change over time, like our progress, it should be a signal.

optional Props
Right now the max setting is hard-coded. Let’s take that as a prop too. But let’s add a catch: let’s make this prop optional by annotating the particular argument to the component function with #[prop(optional)].

#[component]
fn ProgressBar(
// mark this prop optional
// you can specify it or not when you use <ProgressBar/>
#[prop(optional)]
max: u16,
progress: ReadSignal<i32>
) -> impl IntoView {
view! {
<progress
max=max
value=progress
/>
}
}
Now, we can use <ProgressBar max=50 progress=count/>, or we can omit max to use the default value (i.e., <ProgressBar progress=count/>). The default value on an optional is its Default::default() value, which for a u16 is going to be 0. In the case of a progress bar, a max value of 0 is not very useful.

So let’s give it a particular default value instead.

default props
You can specify a default value other than Default::default() pretty simply with #[prop(default = ...).

#[component]
fn ProgressBar(
#[prop(default = 100)]
max: u16,
progress: ReadSignal<i32>
) -> impl IntoView {
view! {
<progress
max=max
value=progress
/>
}
}
Generic Props
This is great. But we began with two counters, one driven by count, and one by the derived signal double_count. Let’s recreate that by using double_count as the progress prop on another <ProgressBar/>.

#[component]
fn App() -> impl IntoView {
let (count, set_count) = create_signal(0);
let double_count = move || count() * 2;

    view! {
        <button on:click=move |_| { set_count.update(|n| *n += 1); }>
            "Click me"
        </button>
        <ProgressBar progress=count/>
        // add a second progress bar
        <ProgressBar progress=double_count/>
    }
}
Hm... this won’t compile. It should be pretty easy to understand why: we’ve declared that the progress prop takes ReadSignal<i32>, and double_count is not ReadSignal<i32>. As rust-analyzer will tell you, its type is || -> i32, i.e., it’s a closure that returns an i32.

There are a couple ways to handle this. One would be to say: “Well, I know that a ReadSignal is a function, and I know that a closure is a function; maybe I could just take any function?” If you’re savvy, you may know that both these implement the trait Fn() -> i32. So you could use a generic component:

#[component]
fn ProgressBar<F>(
#[prop(default = 100)]
max: u16,
progress: F
) -> impl IntoView
where
F: Fn() -> i32 + 'static,
{
view! {
<progress
max=max
value=progress
/>
}
}
This is a perfectly reasonable way to write this component: progress now takes any value that implements this Fn() trait.

This generic can also be specified inline:

#[component]
fn ProgressBar<F: Fn() -> i32 + 'static>(
#[prop(default = 100)] max: u16,
progress: F,
) -> impl IntoView {
view! {
<progress
max=max
value=progress
/>
}
}
Note that generic component props can’t be specified with an impl yet (progress: impl Fn() -> i32 + 'static,), in part because they’re actually used to generate a struct ProgressBarProps, and struct fields cannot be impl types. The #[component] macro may be further improved in the future to allow inline impl generic props.

Generics need to be used somewhere in the component props. This is because props are built into a struct, so all generic types must be used somewhere in the struct. This is often easily accomplished using an optional PhantomData prop. You can then specify a generic in the view using the syntax for expressing types: <Component<T>/> (not with the turbofish-style <Component::<T>/>).

#[component]
fn SizeOf<T: Sized>(#[prop(optional)] _ty: PhantomData<T>) -> impl IntoView {
std::mem::size_of::<T>()
}

#[component]
pub fn App() -> impl IntoView {
view! {
<SizeOf<usize>/>
<SizeOf<String>/>
}
}
Note that there are some limitations. For example, our view macro parser can’t handle nested generics like <SizeOf<Vec<T>>/>.

into Props
There’s one more way we could implement this, and it would be to use #[prop(into)]. This attribute automatically calls .into() on the values you pass as props, which allows you to easily pass props with different values.

In this case, it’s helpful to know about the Signal type. Signal is an enumerated type that represents any kind of readable reactive signal. It can be useful when defining APIs for components you’ll want to reuse while passing different sorts of signals. The MaybeSignal type is useful when you want to be able to take either a static or reactive value.

#[component]
fn ProgressBar(
#[prop(default = 100)]
max: u16,
#[prop(into)]
progress: Signal<i32>
) -> impl IntoView
{
view! {
<progress
max=max
value=progress
/>
}
}

#[component]
fn App() -> impl IntoView {
let (count, set_count) = create_signal(0);
let double_count = move || count() * 2;

    view! {
        <button on:click=move |_| { set_count.update(|n| *n += 1); }>
            "Click me"
        </button>
        // .into() converts `ReadSignal` to `Signal`
        <ProgressBar progress=count/>
        // use `Signal::derive()` to wrap a derived signal
        <ProgressBar progress=Signal::derive(double_count)/>
    }
}
Optional Generic Props
Note that you can’t specify optional generic props for a component. Let’s see what would happen if you try:

#[component]
fn ProgressBar<F: Fn() -> i32 + 'static>(
#[prop(optional)] progress: Option<F>,
) -> impl IntoView {
progress.map(|progress| {
view! {
<progress
max=100
value=progress
/>
}
})
}

#[component]
pub fn App() -> impl IntoView {
view! {
<ProgressBar/>
}
}
Rust helpfully gives the error

xx |         <ProgressBar/>
|          ^^^^^^^^^^^ cannot infer type of the type parameter `F` declared on the function `ProgressBar`
|
help: consider specifying the generic argument
|
xx |         <ProgressBar::<F>/>
|                     +++++
You can specify generics on components with a <ProgressBar<F>/> syntax (no turbofish in the view macro). Specifying the correct type here is not possible; closures and functions in general are unnameable types. The compiler can display them with a shorthand, but you can’t specify them.

However, you can get around this by providing a concrete type using Box<dyn _> or &dyn _:

#[component]
fn ProgressBar(
#[prop(optional)] progress: Option<Box<dyn Fn() -> i32>>,
) -> impl IntoView {
progress.map(|progress| {
view! {
<progress
max=100
value=progress
/>
}
})
}

#[component]
pub fn App() -> impl IntoView {
view! {
<ProgressBar/>
}
}
Because the Rust compiler now knows the concrete type of the prop, and therefore its size in memory even in the None case, this compiles fine.

In this particular case, &dyn Fn() -> i32 will cause lifetime issues, but in other cases, it may be a possibility.

Documenting Components
This is one of the least essential but most important sections of this book. It’s not strictly necessary to document your components and their props. It may be very important, depending on the size of your team and your app. But it’s very easy, and bears immediate fruit.

To document a component and its props, you can simply add doc comments on the component function, and each one of the props:

/// Shows progress toward a goal.
#[component]
fn ProgressBar(
/// The maximum value of the progress bar.
#[prop(default = 100)]
max: u16,
/// How much progress should be displayed.
#[prop(into)]
progress: Signal<i32>,
) -> impl IntoView {
/* ... */
}
That’s all you need to do. These behave like ordinary Rust doc comments, except that you can document individual component props, which can’t be done with Rust function arguments.

This will automatically generate documentation for your component, its Props type, and each of the fields used to add props. It can be a little hard to understand how powerful this is until you hover over the component name or props and see the power of the #[component] macro combined with rust-analyzer here.

Advanced Topic: #[component(transparent)]
All Leptos components return -> impl IntoView. Some, though, need to return some data directly without any additional wrapping. These can be marked with #[component(transparent)], in which case they return exactly the value they return, without the rendering system transforming them in any way.

This is mostly used in two situations:

Creating wrappers around <Suspense/> or <Transition/>, which return a transparent suspense structure to integrate with SSR and hydration properly.
Refactoring <Route/> definitions for leptos_router out into separate components, because <Route/> is a transparent component that returns a RouteDefinition struct rather than a view.
In general, you should not need to use transparent components unless you are creating custom wrapping components that fall into one of these two categories.


use leptos::*;

// Composing different components together is how we build
// user interfaces. Here, we'll define a reusable <ProgressBar/>.
// You'll see how doc comments can be used to document components
// and their properties.

/// Shows progress toward a goal.
#[component]
fn ProgressBar(
// Marks this as an optional prop. It will default to the default
// value of its type, i.e., 0.
#[prop(default = 100)]
/// The maximum value of the progress bar.
max: u16,
// Will run `.into()` on the value passed into the prop.
#[prop(into)]
// `Signal<T>` is a wrapper for several reactive types.
// It can be helpful in component APIs like this, where we
// might want to take any kind of reactive value
/// How much progress should be displayed.
progress: Signal<i32>,
) -> impl IntoView {
view! {
<progress
max={max}
value=progress
/>
<br/>
}
}

#[component]
fn App() -> impl IntoView {
let (count, set_count) = create_signal(0);

    let double_count = move || count() * 2;

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me"
        </button>
        <br/>
        // If you have this open in CodeSandbox or an editor with
        // rust-analyzer support, try hovering over `ProgressBar`,
        // `max`, or `progress` to see the docs we defined above
        <ProgressBar max=50 progress=count/>
        // Let's use the default max value on this one
        // the default is 100, so it should move half as fast
        <ProgressBar progress=count/>
        // Signal::derive creates a Signal wrapper from our derived signal
        // using double_count means it should move twice as fast
        <ProgressBar max=50 progress=Signal::derive(double_count)/>
    }
}

fn main() {
leptos::mount_to_body(App)
}


Iteration
Whether you’re listing todos, displaying a table, or showing product images, iterating over a list of items is a common task in web applications. Reconciling the differences between changing sets of items can also be one of the trickiest tasks for a framework to handle well.

Leptos supports two different patterns for iterating over items:

For static views: Vec<_>
For dynamic lists: <For/>
Static Views with Vec<_>
Sometimes you need to show an item repeatedly, but the list you’re drawing from does not often change. In this case, it’s important to know that you can insert any Vec<IV> where IV: IntoView into your view. In other words, if you can render T, you can render Vec<T>.

let values = vec![0, 1, 2];
view! {
// this will just render "012"
<p>{values.clone()}</p>
// or we can wrap them in <li>
<ul>
{values.into_iter()
.map(|n| view! { <li>{n}</li>})
.collect::<Vec<_>>()}
</ul>
}
Leptos also provides a .collect_view() helper function that allows you to collect any iterator of T: IntoView into Vec<View>.

let values = vec![0, 1, 2];
view! {
// this will just render "012"
<p>{values.clone()}</p>
// or we can wrap them in <li>
<ul>
{values.into_iter()
.map(|n| view! { <li>{n}</li>})
.collect_view()}
</ul>
}
The fact that the list is static doesn’t mean the interface needs to be static. You can render dynamic items as part of a static list.

// create a list of 5 signals
let length = 5;
let counters = (1..=length).map(|idx| create_signal(idx));

// each item manages a reactive view
// but the list itself will never change
let counter_buttons = counters
.map(|(count, set_count)| {
view! {
<li>
<button
on:click=move |_| set_count.update(|n| *n += 1)
>
{count}
</button>
</li>
}
})
.collect_view();

view! {
<ul>{counter_buttons}</ul>
}
You can render a Fn() -> Vec<_> reactively as well. But note that every time it changes, this will rerender every item in the list. This is quite inefficient! Fortunately, there’s a better way.

Dynamic Rendering with the <For/> Component
The <For/> component is a keyed dynamic list. It takes three props:

each: a function (such as a signal) that returns the items T to be iterated over
key: a key function that takes &T and returns a stable, unique key or ID
children: renders each T into a view
key is, well, the key. You can add, remove, and move items within the list. As long as each item’s key is stable over time, the framework does not need to rerender any of the items, unless they are new additions, and it can very efficiently add, remove, and move items as they change. This allows for extremely efficient updates to the list as it changes, with minimal additional work.

Creating a good key can be a little tricky. You generally do not want to use an index for this purpose, as it is not stable—if you remove or move items, their indices change.

But it’s a great idea to do something like generating a unique ID for each row as it is generated, and using that as an ID for the key function.

Check out the <DynamicList/> component below for an example.



// Iteration is a very common task in most applications.
// So how do you take a list of data and render it in the DOM?
// This example will show you the two ways:
// 1) for mostly-static lists, using Rust iterators
// 2) for lists that grow, shrink, or move items, using <For/>

#[component]
fn App() -> impl IntoView {
view! {
<h1>"Iteration"</h1>
<h2>"Static List"</h2>
<p>"Use this pattern if the list itself is static."</p>
<StaticList length=5/>
<h2>"Dynamic List"</h2>
<p>"Use this pattern if the rows in your list will change."</p>
<DynamicList initial_length=5/>
}
}

/// A list of counters, without the ability
/// to add or remove any.
#[component]
fn StaticList(
/// How many counters to include in this list.
length: usize,
) -> impl IntoView {
// create counter signals that start at incrementing numbers
let counters = (1..=length).map(|idx| create_signal(idx));

    // when you have a list that doesn't change, you can
    // manipulate it using ordinary Rust iterators
    // and collect it into a Vec<_> to insert it into the DOM
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button
                        on:click=move |_| set_count.update(|n| *n += 1)
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect::<Vec<_>>();

    // Note that if `counter_buttons` were a reactive list
    // and its value changed, this would be very inefficient:
    // it would rerender every row every time the list changed.
    view! {
        <ul>{counter_buttons}</ul>
    }
}

/// A list of counters that allows you to add or
/// remove counters.
#[component]
fn DynamicList(
/// The number of counters to begin with.
initial_length: usize,
) -> impl IntoView {
// This dynamic list will use the <For/> component.
// <For/> is a keyed list. This means that each row
// has a defined key. If the key does not change, the row
// will not be re-rendered. When the list changes, only
// the minimum number of changes will be made to the DOM.

    // `next_counter_id` will let us generate unique IDs
    // we do this by simply incrementing the ID by one
    // each time we create a counter
    let mut next_counter_id = initial_length;

    // we generate an initial list as in <StaticList/>
    // but this time we include the ID along with the signal
    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();

    // now we store that initial list in a signal
    // this way, we'll be able to modify the list over time,
    // adding and removing counters, and it will change reactively
    let (counters, set_counters) = create_signal(initial_counters);

    let add_counter = move |_| {
        // create a signal for the new counter
        let sig = create_signal(next_counter_id + 1);
        // add this counter to the list of counters
        set_counters.update(move |counters| {
            // since `.update()` gives us `&mut T`
            // we can just use normal Vec methods like `push`
            counters.push((next_counter_id, sig))
        });
        // increment the ID so it's always unique
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>
                "Add Counter"
            </button>
            <ul>
                // The <For/> component is central here
                // This allows for efficient, key list rendering
                <For
                    // `each` takes any function that returns an iterator
                    // this should usually be a signal or derived signal
                    // if it's not reactive, just render a Vec<_> instead of <For/>
                    each=counters
                    // the key should be unique and stable for each row
                    // using an index is usually a bad idea, unless your list
                    // can only grow, because moving items around inside the list
                    // means their indices will change and they will all rerender
                    key=|counter| counter.0
                    // `children` receives each item from your `each` iterator
                    // and returns a view
                    children=move |(id, (count, set_count))| {
                        view! {
                            <li>
                                <button
                                    on:click=move |_| set_count.update(|n| *n += 1)
                                >
                                    {count}
                                </button>
                                <button
                                    on:click=move |_| {
                                        set_counters.update(|counters| {
                                            counters.retain(|(counter_id, (signal, _))| {
                                                // NOTE: in this example, we are creating the signals
                                                // in the scope of the parent. This means the memory used to
                                                // store them will not be reclaimed until the parent component
                                                // is unmounted. Here, we're removing the signal early (i.e, before
                                                // the DynamicList is unmounted), so we manually dispose of the signal
                                                // to avoid leaking memory.
                                                //
                                                // This is only necessary in an example with nested signals like this one.
                                                if counter_id == &id {
                                                    signal.dispose();
                                                }
                                                counter_id != &id
                                            })
                                        });
                                    }
                                >
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

fn main() {
leptos::mount_to_body(App)
}


Iterating over More Complex Data with <For/>
This chapter goes into iteration over nested data structures in a bit more depth. It belongs here with the other chapter on iteration, but feel free to skip it and come back if you’d like to stick with simpler subjects for now.

The Problem
I just said that the framework does not rerender any of the items in one of the rows, unless the key has changed. This probably makes sense at first, but it can easily trip you up.

Let’s consider an example in which each of the items in our row is some data structure. Imagine, for example, that the items come from some JSON array of keys and values:

#[derive(Debug, Clone)]
struct DatabaseEntry {
key: String,
value: i32,
}
Let’s define a simple component that will iterate over the rows and display each one:

#[component]
pub fn App() -> impl IntoView {
// start with a set of three rows
let (data, set_data) = create_signal(vec![
DatabaseEntry {
key: "foo".to_string(),
value: 10,
},
DatabaseEntry {
key: "bar".to_string(),
value: 20,
},
DatabaseEntry {
key: "baz".to_string(),
value: 15,
},
]);
view! {
// when we click, update each row,
// doubling its value
<button on:click=move |_| {
set_data.update(|data| {
for row in data {
row.value *= 2;
}
});
// log the new value of the signal
logging::log!("{:?}", data.get());
}>
"Update Values"
</button>
// iterate over the rows and display each value
<For
each=data
key=|state| state.key.clone()
let:child
>
<p>{child.value}</p>
</For>
}
}
Note the let:child syntax here. In the previous chapter we introduced <For/> with a children prop. We can actually create this value directly in the children of the <For/> component, without breaking out of the view macro: the let:child combined with <p>{child.value}</p> above is the equivalent of

children=|child| view! { <p>{child.value}</p> }
When you click the Update Values button... nothing happens. Or rather: the signal is updated, the new value is logged, but the {child.value} for each row doesn’t update.

Let’s see: is that because we forgot to add a closure to make it reactive? Let’s try {move || child.value}.

...Nope. Still nothing.

Here’s the problem: as I said, each row is only rerendered when the key changes. We’ve updated the value for each row, but not the key for any of the rows, so nothing has rerendered. And if you look at the type of child.value, it’s a plain i32, not a reactive ReadSignal<i32> or something. This means that even if we wrap a closure around it, the value in this row will never update.

We have three possible solutions:

change the key so that it always updates when the data structure changes
change the value so that it’s reactive
take a reactive slice of the data structure instead of using each row directly
Option 1: Change the Key
Each row is only rerendered when the key changes. Our rows above didn’t rerender, because the key didn’t change. So: why not just force the key to change?

<For
each=data
key=|state| (state.key.clone(), state.value)
let:child
>
    <p>{child.value}</p>
</For>
Now we include both the key and the value in the key. This means that whenever the value of a row changes, <For/> will treat it as if it’s an entirely new row, and replace the previous one.

Pros
This is very easy. We can make it even easier by deriving PartialEq, Eq, and Hash on DatabaseEntry, in which case we could just key=|state| state.clone().

Cons
This is the least efficient of the three options. Every time the value of a row changes, it throws out the previous <p> element and replaces it with an entirely new one. Rather than making a fine-grained update to the text node, in other words, it really does rerender the entire row on every change, and this is expensive in proportion to how complex the UI of the row is.

You’ll notice we also end up cloning the whole data structure so that <For/> can hold onto a copy of the key. For more complex structures, this can become a bad idea fast!

Option 2: Nested Signals
If we do want that fine-grained reactivity for the value, one option is to wrap the value of each row in a signal.

#[derive(Debug, Clone)]
struct DatabaseEntry {
key: String,
value: RwSignal<i32>,
}
RwSignal<_> is a “read-write signal,” which combines the getter and setter in one object. I’m using it here because it’s a little easier to store in a struct than separate getters and setters.

#[component]
pub fn App() -> impl IntoView {
// start with a set of three rows
let (data, set_data) = create_signal(vec![
DatabaseEntry {
key: "foo".to_string(),
value: create_rw_signal(10),
},
DatabaseEntry {
key: "bar".to_string(),
value: create_rw_signal(20),
},
DatabaseEntry {
key: "baz".to_string(),
value: create_rw_signal(15),
},
]);
view! {
// when we click, update each row,
// doubling its value
<button on:click=move |_| {
data.with(|data| {
for row in data {
row.value.update(|value| *value *= 2);
}
});
// log the new value of the signal
logging::log!("{:?}", data.get());
}>
"Update Values"
</button>
// iterate over the rows and display each value
<For
each=data
key=|state| state.key.clone()
let:child
>
<p>{child.value}</p>
</For>
}
}
This version works! And if you look in the DOM inspector in your browser, you’ll see that unlike in the previous version, in this version only the individual text nodes are updated. Passing the signal directly into {child.value} works, as signals do keep their reactivity if you pass them into the view.

Note that I changed the set_data.update() to a data.with(). .with() is the non-cloning way of accessing a signal’s value. In this case, we are only updating the internal values, not updating the list of values: because signals maintain their own state, we don’t actually need to update the data signal at all, so the immutable .with() is fine here.

In fact, this version doesn’t update data, so the <For/> is essentially a static list as in the last chapter, and this could just be a plain iterator. But the <For/> is useful if we want to add or remove rows in the future.

Pros
This is the most efficient option, and fits directly with the rest of the mental model of the framework: values that change over time are wrapped in signals so the interface can respond to them.

Cons
Nested reactivity can be cumbersome if you’re receiving data from an API or another data source you don’t control, and you don’t want to create a different struct wrapping each field in a signal.

Option 3: Memoized Slices
Leptos provides a primitive called create_memo, which creates a derived computation that only triggers a reactive update when its value has changed.

This allows you to create reactive values for subfields of a larger data structure, without needing to wrap the fields of that structure in signals.

Most of the application can remain the same as the initial (broken) version, but the <For/> will be updated to this:

<For
each=move || data().into_iter().enumerate()
key=|(_, state)| state.key.clone()
children=move |(index, _)| {
let value = create_memo(move |_| {
data.with(|data| data.get(index).map(|d| d.value).unwrap_or(0))
});
view! {
<p>{value}</p>
}
}
/>
You’ll notice a few differences here:

we convert the data signal into an enumerated iterator
we use the children prop explicitly, to make it easier to run some non-view code
we define a value memo and use that in the view. This value field doesn’t actually use the child being passed into each row. Instead, it uses the index and reaches back into the original data to get the value.
Every time data changes, now, each memo will be recalculated. If its value has changed, it will update its text node, without rerendering the whole row.

Pros
We get the same fine-grained reactivity of the signal-wrapped version, without needing to wrap the data in signals.

Cons
It’s a bit more complex to set up this memo-per-row inside the <For/> loop rather than using nested signals. For example, you’ll notice that we have to guard against the possibility that the data[index] would panic by using data.get(index), because this memo may be triggered to re-run once just after the row is removed. (This is because the memo for each row and the whole <For/> both depend on the same data signal, and the order of execution for multiple reactive values that depend on the same signal isn’t guaranteed.)

Note also that while memos memoize their reactive changes, the same calculation does need to re-run to check the value every time, so nested reactive signals will still be more efficient for pinpoint updates here.


Forms and Inputs
Forms and form inputs are an important part of interactive apps. There are two basic patterns for interacting with inputs in Leptos, which you may recognize if you’re familiar with React, SolidJS, or a similar framework: using controlled or uncontrolled inputs.

Controlled Inputs
In a "controlled input," the framework controls the state of the input element. On every input event, it updates a local signal that holds the current state, which in turn updates the value prop of the input.

There are two important things to remember:

The input event fires on (almost) every change to the element, while the change event fires (more or less) when you unfocus the input. You probably want on:input, but we give you the freedom to choose.
The value attribute only sets the initial value of the input, i.e., it only updates the input up to the point that you begin typing. The value property continues updating the input after that. You usually want to set prop:value for this reason. (The same is true for checked and prop:checked on an <input type="checkbox">.)
let (name, set_name) = create_signal("Controlled".to_string());

view! {
<input type="text"
on:input=move |ev| {
// event_target_value is a Leptos helper function
// it functions the same way as event.target.value
// in JavaScript, but smooths out some of the typecasting
// necessary to make this work in Rust
set_name(event_target_value(&ev));
}

        // the `prop:` syntax lets you update a DOM property,
        // rather than an attribute.
        prop:value=name
    />
    <p>"Name is: " {name}</p>
}
Why do you need prop:value?
Web browsers are the most ubiquitous and stable platform for rendering graphical user interfaces in existence. They have also maintained an incredible backwards compatibility over their three decades of existence. Inevitably, this means there are some quirks.

One odd quirk is that there is a distinction between HTML attributes and DOM element properties, i.e., between something called an “attribute” which is parsed from HTML and can be set on a DOM element with .setAttribute(), and something called a “property” which is a field of the JavaScript class representation of that parsed HTML element.

In the case of an <input value=...>, setting the value attribute is defined as setting the initial value for the input, and setting value property sets its current value. It maybe easiest to understand this by opening about:blank and running the following JavaScript in the browser console, line by line:

// create an input and append it to the DOM
const el = document.createElement("input");
document.body.appendChild(el);

el.setAttribute("value", "test"); // updates the input
el.setAttribute("value", "another test"); // updates the input again

// now go and type into the input: delete some characters, etc.

el.setAttribute("value", "one more time?");
// nothing should have changed. setting the "initial value" does nothing now

// however...
el.value = "But this works";
Many other frontend frameworks conflate attributes and properties, or create a special case for inputs that sets the value correctly. Maybe Leptos should do this too; but for now, I prefer giving users the maximum amount of control over whether they’re setting an attribute or a property, and doing my best to educate people about the actual underlying browser behavior rather than obscuring it.

Uncontrolled Inputs
In an "uncontrolled input," the browser controls the state of the input element. Rather than continuously updating a signal to hold its value, we use a NodeRef to access the input when we want to get its value.

In this example, we only notify the framework when the <form> fires a submit event. Note the use of the leptos::html module, which provides a bunch of types for every HTML element.

let (name, set_name) = create_signal("Uncontrolled".to_string());

let input_element: NodeRef<html::Input> = create_node_ref();

view! {
<form on:submit=on_submit> // on_submit defined below
<input type="text"
value=name
node_ref=input_element
/>
<input type="submit" value="Submit"/>
</form>
<p>"Name is: " {name}</p>
}
The view should be pretty self-explanatory by now. Note two things:

Unlike in the controlled input example, we use value (not prop:value). This is because we’re just setting the initial value of the input, and letting the browser control its state. (We could use prop:value instead.)
We use node_ref=... to fill the NodeRef. (Older examples sometimes use _ref. They are the same thing, but node_ref has better rust-analyzer support.)
NodeRef is a kind of reactive smart pointer: we can use it to access the underlying DOM node. Its value will be set when the element is rendered.

let on_submit = move |ev: leptos::ev::SubmitEvent| {
// stop the page from reloading!
ev.prevent_default();

    // here, we'll extract the value from the input
    let value = input_element()
        // event handlers can only fire after the view
        // is mounted to the DOM, so the `NodeRef` will be `Some`
        .expect("<input> should be mounted")
        // `leptos::HtmlElement<html::Input>` implements `Deref`
        // to a `web_sys::HtmlInputElement`.
        // this means we can call`HtmlInputElement::value()`
        // to get the current value of the input
        .value();
    set_name(value);
};
Our on_submit handler will access the input’s value and use it to call set_name. To access the DOM node stored in the NodeRef, we can simply call it as a function (or using .get()). This will return Option<leptos::HtmlElement<html::Input>>, but we know that the element has already been mounted (how else did you fire this event!), so it's safe to unwrap here.

We can then call .value() to get the value out of the input, because NodeRef gives us access to a correctly-typed HTML element.

Take a look at web_sys and HtmlElement to learn more about using a leptos::HtmlElement. Also see the full CodeSandbox example at the end of this page.

Special Cases: <textarea> and <select>
Two form elements tend to cause some confusion, in different ways.

<textarea>
Unlike <input>, the <textarea> element does not support a value attribute. Instead, it receives its value as a plain text node in its HTML children.

In the current version of Leptos (in fact in Leptos 0.1-0.6), creating a dynamic child inserts a comment marker node. This can cause incorrect <textarea> rendering (and issues during hydration) if you try to use it to show dynamic content.

Instead, you can pass a non-reactive initial value as a child, and use prop:value to set its current value. (<textarea> doesn’t support the value attribute, but does support the value property...)

view! {
<textarea
prop:value=move || some_value.get()
on:input=/* etc */
>
/* plain-text initial value, does not change if the signal changes */
{some_value.get_untracked()}
</textarea>
}
<select>
The <select> element also does not have a value attribute, nor a value property. Instead, its value is determined by the selected attribute of its <option> fields. Some frameworks obscure this with a value field on <select>; if you try this in Leptos (or vanilla JavaScript) it won’t work.

To use the selected field:

let (value, set_value) = create_signal("B".to_string());
view! {
<select on:change=move |ev| {
let new_value = event_target_value(&ev);
set_value(new_value);
}>
<option
value="A"
selected=move || value() == "A"
>
"A"
</option>
<option
value="B"
selected=move || value() == "B"
>
"B"
</option>
</select>
}
That's somewhat repetitive, but can easily be refactored:

#[component]
pub fn App() -> impl IntoView {
let (value, set_value) = create_signal("B".to_string());
view! {
<select on:change=move |ev| {
let new_value = event_target_value(&ev);
set_value(new_value);
}>
<SelectOption value is="A"/>
<SelectOption value is="B"/>
<SelectOption value is="C"/>
</select>
}
}

#[component]
pub fn SelectOption(is: &'static str, value: ReadSignal<String>) -> impl IntoView {
view! {
<option
value=is
selected=move || value() == is
>
{is}
</option>
}
}
Tip: the single value attribute in the component is equivalent to value=value. This is only the case for components: in HTML elements, a single value attribute is equivalent to value=true. This is expected to be made consistent in the next major version of Leptos; see this issue for more details.

use leptos::{ev::SubmitEvent, *};

#[component]
fn App() -> impl IntoView {
view! {
<h2>"Controlled Component"</h2>
<ControlledComponent/>
<h2>"Uncontrolled Component"</h2>
<UncontrolledComponent/>
}
}

#[component]
fn ControlledComponent() -> impl IntoView {
// create a signal to hold the value
let (name, set_name) = create_signal("Controlled".to_string());

    view! {
        <input type="text"
            // fire an event whenever the input changes
            on:input=move |ev| {
                // event_target_value is a Leptos helper function
                // it functions the same way as event.target.value
                // in JavaScript, but smooths out some of the typecasting
                // necessary to make this work in Rust
                set_name(event_target_value(&ev));
            }

            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            //
            // IMPORTANT: the `value` *attribute* only sets the
            // initial value, until you have made a change.
            // The `value` *property* sets the current value.
            // This is a quirk of the DOM; I didn't invent it.
            // Other frameworks gloss this over; I think it's
            // more important to give you access to the browser
            // as it really works.
            //
            // tl;dr: use prop:value for form inputs
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

#[component]
fn UncontrolledComponent() -> impl IntoView {
// import the type for <input>
use leptos::html::Input;

    let (name, set_name) = create_signal("Uncontrolled".to_string());

    // we'll use a NodeRef to store a reference to the input element
    // this will be filled when the element is created
    let input_element: NodeRef<Input> = create_node_ref();

    // fires when the form `submit` event happens
    // this will store the value of the <input> in our signal
    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> to exist")
            // `NodeRef` implements `Deref` for the DOM element type
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_name(value);
    };

    view! {
        <form on:submit=on_submit>
            <input type="text"
                // here, we use the `value` *attribute* to set only
                // the initial value, letting the browser maintain
                // the state after that
                value=name

                // store a reference to this input in `input_element`
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}

// This `main` function is the entry point into the app
// It just mounts our component to the <body>
// Because we defined it as `fn App`, we can now use it in a
// template as <App/>
fn main() {
leptos::mount_to_body(App)
}


Control Flow
In most applications, you sometimes need to make a decision: Should I render this part of the view, or not? Should I render <ButtonA/> or <WidgetB/>? This is control flow.

A Few Tips
When thinking about how to do this with Leptos, it’s important to remember a few things:

Rust is an expression-oriented language: control-flow expressions like if x() { y } else { z } and match x() { ... } return their values. This makes them very useful for declarative user interfaces.
For any T that implements IntoView—in other words, for any type that Leptos knows how to render—Option<T> and Result<T, impl Error> also implement IntoView. And just as Fn() -> T renders a reactive T, Fn() -> Option<T> and Fn() -> Result<T, impl Error> are reactive.
Rust has lots of handy helpers like Option::map, Option::and_then, Option::ok_or, Result::map, Result::ok, and bool::then that allow you to convert, in a declarative way, between a few different standard types, all of which can be rendered. Spending time in the Option and Result docs in particular is one of the best ways to level up your Rust game.
And always remember: to be reactive, values must be functions. You’ll see me constantly wrap things in a move || closure, below. This is to ensure that they actually rerun when the signal they depend on changes, keeping the UI reactive.
So What?
To connect the dots a little: this means that you can actually implement most of your control flow with native Rust code, without any control-flow components or special knowledge.

For example, let’s start with a simple signal and derived signal:

let (value, set_value) = create_signal(0);
let is_odd = move || value() & 1 == 1;
If you don’t recognize what’s going on with is_odd, don’t worry about it too much. It’s just a simple way to test whether an integer is odd by doing a bitwise AND with 1.

We can use these signals and ordinary Rust to build most control flow.

if statements
Let’s say I want to render some text if the number is odd, and some other text if it’s even. Well, how about this?

view! {
<p>
{move || if is_odd() {
"Odd"
} else {
"Even"
}}
</p>
}
An if expression returns its value, and a &str implements IntoView, so a Fn() -> &str implements IntoView, so this... just works!

Option<T>
Let’s say we want to render some text if it’s odd, and nothing if it’s even.

let message = move || {
if is_odd() {
Some("Ding ding ding!")
} else {
None
}
};

view! {
<p>{message}</p>
}
This works fine. We can make it a little shorter if we’d like, using bool::then().

let message = move || is_odd().then(|| "Ding ding ding!");
view! {
<p>{message}</p>
}
You could even inline this if you’d like, although personally I sometimes like the better cargo fmt and rust-analyzer support I get by pulling things out of the view.

match statements
We’re still just writing ordinary Rust code, right? So you have all the power of Rust’s pattern matching at your disposal.

let message = move || {
match value() {
0 => "Zero",
1 => "One",
n if is_odd() => "Odd",
_ => "Even"
}
};
view! {
<p>{message}</p>
}
And why not? YOLO, right?

Preventing Over-Rendering
Not so YOLO.

Everything we’ve just done is basically fine. But there’s one thing you should remember and try to be careful with. Each one of the control-flow functions we’ve created so far is basically a derived signal: it will rerun every time the value changes. In the examples above, where the value switches from even to odd on every change, this is fine.

But consider the following example:

let (value, set_value) = create_signal(0);

let message = move || if value() > 5 {
"Big"
} else {
"Small"
};

view! {
<p>{message}</p>
}
This works, for sure. But if you added a log, you might be surprised

let message = move || if value() > 5 {
logging::log!("{}: rendering Big", value());
"Big"
} else {
logging::log!("{}: rendering Small", value());
"Small"
};
As a user clicks a button, you’d see something like this:

1: rendering Small
2: rendering Small
3: rendering Small
4: rendering Small
5: rendering Small
6: rendering Big
7: rendering Big
8: rendering Big
... ad infinitum
Every time value changes, it reruns the if statement. This makes sense, with how reactivity works. But it has a downside. For a simple text node, rerunning the if statement and rerendering isn’t a big deal. But imagine it were like this:

let message = move || if value() > 5 {
<Big/>
} else {
<Small/>
};
This rerenders <Small/> five times, then <Big/> infinitely. If they’re loading resources, creating signals, or even just creating DOM nodes, this is unnecessary work.

<Show/>
The <Show/> component is the answer. You pass it a when condition function, a fallback to be shown if the when function returns false, and children to be rendered if when is true.

let (value, set_value) = create_signal(0);

view! {
<Show
when=move || { value() > 5 }
fallback=|| view! { <Small/> }
>
    <Big/>
  </Show>
}
<Show/> memoizes the when condition, so it only renders its <Small/> once, continuing to show the same component until value is greater than five; then it renders <Big/> once, continuing to show it indefinitely or until value goes below five and then renders <Small/> again.

This is a helpful tool to avoid rerendering when using dynamic if expressions. As always, there's some overhead: for a very simple node (like updating a single text node, or updating a class or attribute), a move || if ... will be more efficient. But if it’s at all expensive to render either branch, reach for <Show/>.

Note: Type Conversions
There‘s one final thing it’s important to say in this section.

The view macro doesn’t return the most-generic wrapping type View. Instead, it returns things with types like Fragment or HtmlElement<Input>. This can be a little annoying if you’re returning different HTML elements from different branches of a conditional:

view! {
<main>
{move || match is_odd() {
true if value() == 1 => {
// returns HtmlElement<Pre>
view! { <pre>"One"</pre> }
},
false if value() == 2 => {
// returns HtmlElement<P>
view! { <p>"Two"</p> }
}
// returns HtmlElement<Textarea>
_ => view! { <textarea>{value()}</textarea> }
}}
</main>
}
This strong typing is actually very powerful, because HtmlElement is, among other things, a smart pointer: each HtmlElement<T> type implements Deref for the appropriate underlying web_sys type. In other words, in the browser your view returns real DOM elements, and you can access native DOM methods on them.

But it can be a little annoying in conditional logic like this, because you can’t return different types from different branches of a condition in Rust. There are two ways to get yourself out of this situation:

If you have multiple HtmlElement types, convert them to HtmlElement<AnyElement> with .into_any()
If you have a variety of view types that are not all HtmlElement, convert them to Views with .into_view().
Here’s the same example, with the conversion added:

view! {
<main>
{move || match is_odd() {
true if value() == 1 => {
// returns HtmlElement<Pre>
view! { <pre>"One"</pre> }.into_any()
},
false if value() == 2 => {
// returns HtmlElement<P>
view! { <p>"Two"</p> }.into_any()
}
// returns HtmlElement<Textarea>
_ => view! { <textarea>{value()}</textarea> }.into_any()
}}
</main>
}

other code


use leptos::*;

#[component]
fn App() -> impl IntoView {
let (value, set_value) = create_signal(0);
let is_odd = move || value() & 1 == 1;
let odd_text = move || if is_odd() { Some("How odd!") } else { None };

    view! {
        <h1>"Control Flow"</h1>

        // Simple UI to update and show a value
        <button on:click=move |_| set_value.update(|n| *n += 1)>
            "+1"
        </button>
        <p>"Value is: " {value}</p>

        <hr/>

        <h2><code>"Option<T>"</code></h2>
        // For any `T` that implements `IntoView`,
        // so does `Option<T>`

        <p>{odd_text}</p>
        // This means you can use `Option` methods on it
        <p>{move || odd_text().map(|text| text.len())}</p>

        <h2>"Conditional Logic"</h2>
        // You can do dynamic conditional if-then-else
        // logic in several ways
        //
        // a. An "if" expression in a function
        //    This will simply re-render every time the value
        //    changes, which makes it good for lightweight UI
        <p>
            {move || if is_odd() {
                "Odd"
            } else {
                "Even"
            }}
        </p>

        // b. Toggling some kind of class
        //    This is smart for an element that's going to
        //    toggled often, because it doesn't destroy
        //    it in between states
        //    (you can find the `hidden` class in `index.html`)
        <p class:hidden=is_odd>"Appears if even."</p>

        // c. The <Show/> component
        //    This only renders the fallback and the child
        //    once, lazily, and toggles between them when
        //    needed. This makes it more efficient in many cases
        //    than a {move || if ...} block
        <Show when=is_odd
            fallback=|| view! { <p>"Even steven"</p> }
        >
            <p>"Oddment"</p>
        </Show>

        // d. Because `bool::then()` converts a `bool` to
        //    `Option`, you can use it to create a show/hide toggled
        {move || is_odd().then(|| view! { <p>"Oddity!"</p> })}

        <h2>"Converting between Types"</h2>
        // e. Note: if branches return different types,
        //    you can convert between them with
        //    `.into_any()` (for different HTML element types)
        //    or `.into_view()` (for all view types)
        {move || match is_odd() {
            true if value() == 1 => {
                // <pre> returns HtmlElement<Pre>
                view! { <pre>"One"</pre> }.into_any()
            },
            false if value() == 2 => {
                // <p> returns HtmlElement<P>
                // so we convert into a more generic type
                view! { <p>"Two"</p> }.into_any()
            }
            _ => view! { <textarea>{value()}</textarea> }.into_any()
        }}
    }
}

fn main() {
leptos::mount_to_body(App)
}


Error Handling
In the last chapter, we saw that you can render Option<T>: in the None case, it will render nothing, and in the Some(T) case, it will render T (that is, if T implements IntoView). You can actually do something very similar with a Result<T, E>. In the Err(_) case, it will render nothing. In the Ok(T) case, it will render the T.

Let’s start with a simple component to capture a number input.

#[component]
fn NumericInput() -> impl IntoView {
let (value, set_value) = create_signal(Ok(0));

    // when input changes, try to parse a number from the input
    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <label>
            "Type a number (or not!)"
            <input type="number" on:input=on_input/>
            <p>
                "You entered "
                <strong>{value}</strong>
            </p>
        </label>
    }
}
Every time you change the input, on_input will attempt to parse its value into a 32-bit integer (i32), and store it in our value signal, which is a Result<i32, _>. If you type the number 42, the UI will display

You entered 42
But if you type the stringfoo, it will display

You entered
This is not great. It saves us using .unwrap_or_default() or something, but it would be much nicer if we could catch the error and do something with it.

You can do that, with the <ErrorBoundary/> component.

<ErrorBoundary/>
An <ErrorBoundary/> is a little like the <Show/> component we saw in the last chapter. If everything’s okay—which is to say, if everything is Ok(_)—it renders its children. But if there’s an Err(_) rendered among those children, it will trigger the <ErrorBoundary/>’s fallback.

Let’s add an <ErrorBoundary/> to this example.

#[component]
fn NumericInput() -> impl IntoView {
let (value, set_value) = create_signal(Ok(0));

    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or something that's not a number!)"
            <input type="number" on:input=on_input/>
            <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        // we can render a list of errors as strings, if we'd like
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                .collect_view()
                            }
                        </ul>
                    </div>
                }
            >
                <p>"You entered " <strong>{value}</strong></p>
            </ErrorBoundary>
        </label>
    }
}
Now, if you type 42, value is Ok(42) and you’ll see

You entered 42

If you type foo, value is Err(_) and the fallback will render. We’ve chosen to render the list of errors as a String, so you’ll see something like

Not a number! Errors:
- cannot parse integer from empty string
  If you fix the error, the error message will disappear and the content you’re wrapping in an <ErrorBoundary/> will appear again.

use leptos::*;

#[component]
fn App() -> impl IntoView {
let (value, set_value) = create_signal(Ok(0));

    // when input changes, try to parse a number from the input
    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or something that's not a number!)"
            <input type="number" on:input=on_input/>
            // If an `Err(_) had been rendered inside the <ErrorBoundary/>,
            // the fallback will be displayed. Otherwise, the children of the
            // <ErrorBoundary/> will be displayed.
            <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        // we can render a list of errors
                        // as strings, if we'd like
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                .collect::<Vec<_>>()
                            }
                        </ul>
                    </div>
                }
            >
                <p>
                    "You entered "
                    // because `value` is `Result<i32, _>`,
                    // it will render the `i32` if it is `Ok`,
                    // and render nothing and trigger the error boundary
                    // if it is `Err`. It's a signal, so this will dynamically
                    // update when `value` changes
                    <strong>{value}</strong>
                </p>
            </ErrorBoundary>
        </label>
    }
}

fn main() {
leptos::mount_to_body(App)
}


























