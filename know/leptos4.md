Interlude: Styling
Anyone creating a website or application soon runs into the question of styling. For a small app, a single CSS file is probably plenty to style your user interface. But as an application grows, many developers find that plain CSS becomes increasingly hard to manage.

Some frontend frameworks (like Angular, Vue, and Svelte) provide built-in ways to scope your CSS to particular components, making it easier to manage styles across a whole application without styles meant to modify one small component having a global effect. Other frameworks (like React or Solid) don’t provide built-in CSS scoping, but rely on libraries in the ecosystem to do it for them. Leptos is in this latter camp: the framework itself has no opinions about CSS at all, but provides a few tools and primitives that allow others to build styling libraries.

Here are a few different approaches to styling your Leptos app, other than plain CSS.

TailwindCSS: Utility-first CSS
TailwindCSS is a popular utility-first CSS library. It allows you to style your application by using inline utility classes, with a custom CLI tool that scans your files for Tailwind class names and bundles the necessary CSS.

This allows you to write components like this:

#[component]
fn Home() -> impl IntoView {
let (count, set_count) = create_signal(0);

    view! {
        <main class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
            <p class="px-10 pb-10 text-left">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
            <button
                class="bg-sky-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=move |_| set_count.update(|count| *count += 1)
            >
                {move || if count() == 0 {
                    "Click me!".to_string()
                } else {
                    count().to_string()
                }}
            </button>
        </main>
    }
}
It can be a little complicated to set up the Tailwind integration at first, but you can check out our two examples of how to use Tailwind with a client-side-rendered trunk application or with a server-rendered cargo-leptos application. cargo-leptos also has some built-in Tailwind support that you can use as an alternative to Tailwind’s CLI.

Stylers: Compile-time CSS Extraction
Stylers is a compile-time scoped CSS library that lets you declare scoped CSS in the body of your component. Stylers will extract this CSS at compile time into CSS files that you can then import into your app, which means that it doesn’t add anything to the WASM binary size of your application.

This allows you to write components like this:

use stylers::style;

#[component]
pub fn App() -> impl IntoView {
let styler_class = style! { "App",
#two{
color: blue;
}
div.one{
color: red;
content: raw_str(r#"\hello"#);
font: "1.3em/1.2" Arial, Helvetica, sans-serif;
}
div {
border: 1px solid black;
margin: 25px 50px 75px 100px;
background-color: lightblue;
}
h2 {
color: purple;
}
@media only screen and (max-width: 1000px) {
h3 {
background-color: lightblue;
color: blue
}
}
};

    view! { class = styler_class,
        <div class="one">
            <h1 id="two">"Hello"</h1>
            <h2>"World"</h2>
            <h2>"and"</h2>
            <h3>"friends!"</h3>
        </div>
    }
}
Stylance: Scoped CSS Written in CSS Files
Stylers lets you write CSS inline in your Rust code, extracts it at compile time, and scopes it. Stylance allows you to write your CSS in CSS files alongside your components, import those files into your components, and scope the CSS classes to your components.

This works well with the live-reloading features of trunk and cargo-leptos because edited CSS files can be updated immediately in the browser.

import_style!(style, "app.module.scss");

#[component]
fn HomePage() -> impl IntoView {
view! {
<div class=style::jumbotron/>
}
}
You can edit the CSS directly without causing a Rust recompile.

.jumbotron {
background: blue;
}
Styled: Runtime CSS Scoping
Styled is a runtime scoped CSS library that integrates well with Leptos. It lets you declare scoped CSS in the body of your component function, and then applies those styles at runtime.

use styled::style;

#[component]
pub fn MyComponent() -> impl IntoView {
let styles = style!(
div {
background-color: red;
color: white;
}
);

    styled::view! { styles,
        <div>"This text should be red with white text."</div>
    }
}
Contributions Welcome
Leptos has no opinions on how you style your website or app, but we’re very happy to provide support to any tools you’re trying to create to make it easier. If you’re working on a CSS or styling approach that you’d like to add to this list, please let us know!

Styled: Easy Styling for Leptos Components
If you're looking for an easy way to apply scoped styles to your Leptos components, Styled is the Leptos macro you need. With Styled, you can apply high-level selectors like button or div to specific components, keeping your markup clean and organized.

Installation
Use cargo add in your project root

cargo add styled stylist
Usage
First create a basic Leptos component. This will serve as the foundation for this little guide.

#[component]
pub fn MyComponent(cx: Scope) -> impl IntoView{
view! {
cx,
<div>"hello"</div>
}
}
Next, import the style macro, powered by an awesome crate called Stylist, to create your styles. Just add this to the top of your file.

use styled::style;
You can then use the style macro to create a Result containing your styles. Let's modify our component:

#[component]
pub fn MyComponent(cx: Scope) -> impl IntoView{

let styles = style!(
div {
background-color: red;
color: white;
}
);

view! {
cx,
<div>"hello"</div>
}
}
Now, let's apply those styles with our styled::view! macro!

#[component]
pub fn MyComponent(cx: Scope) -> impl IntoView {

    let styles = style!(
      div {
        background-color: red;
        color: white;
      }
    );

    styled::view! {
        cx,
        styles,
        <div>"This text should be red with white text."</div>
    }
}
Now we can define another component that also uses the div CSS selector but it's styles will only apply to the elements inside of it's enclosing styled::view! macro.

#[component]
pub fn AnotherComponent(cx: Scope) -> impl IntoView {

    // note were using a plain div selector and it wont clash with MyComponent's div style!
    let styles = style!(
      div {
        background-color: blue;
        color: gray;
      }
    );

    styled::view! {
        cx,
        styles,
        <div>"This text should be blue with gray text."</div>
    }
}
Longer Example
// /src/components/button.rs

use crate::theme::get_theme;
use leptos::*;
use styled::style;

#[derive(PartialEq)]
pub enum Variant {
PRIMARY,
SECONDARY,
ALERT,
DISABLED,
}

impl Variant {
pub fn is(&self, variant: &Variant) -> bool {
self == variant
}
}

struct ButtonColors {
text: String,
background: String,
border: String,
}

fn get_colors(variant: &Variant) -> ButtonColors {
let theme = get_theme().unwrap();
match variant {
Variant::PRIMARY => ButtonColors {
text: theme.white(),
background: theme.black(),
border: theme.transparent(),
},
Variant::SECONDARY => ButtonColors {
text: theme.black(),
background: theme.white(),
border: theme.gray.lightest(),
},
Variant::ALERT => ButtonColors {
text: theme.white(),
background: theme.red(),
border: theme.transparent(),
},
Variant::DISABLED => ButtonColors {
text: theme.white(),
background: theme.red(),
border: theme.transparent(),
},
}
}

#[component]
pub fn Button(cx: Scope, variant: Variant) -> impl IntoView {
let disabled = variant.is(&Variant::DISABLED);

    let styles = styles(&variant);

    styled::view! {
        cx,
        styles,
        <button disabled=disabled>"Button"</button>
    }
}

fn styles<'a>(variant: &Variant) -> styled::Result<styled::Style> {
let colors = get_colors(variant);

    style!(
            button {
                color: ${colors.text};
                background-color: ${colors.background};
                border: 1px solid ${colors.border};
                outline: none;
                height: 48px;
                min-width: 154px;
                font-size: 14px;
                font-weight: 700;
                text-align: center;
                box-shadow: rgba(0, 0, 0, 0.05) 0px 1px 2px 0px;
                position: relative;
                box-sizing: border-box;
                vertical-align: middle;
                text-align: center;
                text-overflow: ellipsis;
                text-transform: uppercase;
                overflow: hidden;
                cursor: pointer;
                transition: box-shadow 0.2s;
                margin: 10px;
            }

            & button:active {
                transform: scale(0.99);
            }


            & button::-moz-focus-inner {
                border: none;
            }

            & button::before {
                content: "";
                position: absolute;
                top: 0;
                bottom: 0;
                left: 0;
                right: 0;
                background-color: rgb(255, 255, 255);
                opacity: 0;
                transition: opacity 0.2s;
            }

            & button::after {
                content: "";
                position: absolute;
                left: 50%;
                top: 50%;
                border-radius: 50%;
                padding: 50%;
                background-color: ${colors.text};
                opacity: 0;
                transform: translate(-50%, -50%) scale(1);
                transition: opacity 1s, transform 0.5s;
            }

            & button:hover,
            & button:focus {
                box-shadow: 0 2px 4px -1px rgba(0, 0, 0, 0.2), 0 4px 5px 0 rgba(0, 0, 0, 0.14), 0 1px 10px 0 rgba(0, 0, 0, 0.12);
            }

            & button:hover::before {
                opacity: 0.08;
            }

            & button:hover:focus::before {
                opacity: 0.3;
            }

            & button:active {
                box-shadow: 0 5px 5px -3px rgba(0, 0, 0, 0.2), 0 8px 10px 1px rgba(0, 0, 0, 0.14), 0 3px 14px 2px rgba(0, 0, 0, 0.12);
            }

            & button:active::after {
                opacity: 0.32;
                transform: translate(-50%, -50%) scale(0);
                transition: transform 0s;
            }

            & button:disabled {
                color: rgba(0, 0, 0, 0.28);
                background-color: rgba(0, 0, 0, 0.12);
                box-shadow: none;
                cursor: initial;
            }

            & button:disabled::before {
                opacity: 0;
            }

            & button:disabled::after {
                opacity: 0;
            }

    )
}
// /src/theme/mod.rs
use csscolorparser::Color;

pub fn get_theme() -> Result<Theme, csscolorparser::ParseColorError> {
let theme = Theme {
teal: Colors {
main: Color::from_html("#6FDDDB")?,
darker: Color::from_html("#2BB4B2")?,
lighter: Color::from_html("#7EE1DF")?,
lightest: Color::from_html("#B2EDEC")?,
},
pink: Colors {
main: Color::from_html("#E93EF5")?,
darker: Color::from_html("#C70BD4")?,
lighter: Color::from_html("#F5A4FA")?,
lightest: Color::from_html("#FCE1FD")?,
},
green: Colors {
main: Color::from_html("#54D072")?,
darker: Color::from_html("#30AF4F")?,
lighter: Color::from_html("#82DD98")?,
lightest: Color::from_html("#B4EAC1")?,
},
purple: Colors {
main: Color::from_html("#8C18FB")?,
darker: Color::from_html("#7204DB")?,
lighter: Color::from_html("#B162FC")?,
lightest: Color::from_html("#D0A1FD")?,
},
yellow: Colors {
main: Color::from_html("#E1E862")?,
darker: Color::from_html("#BAC31D")?,
lighter: Color::from_html("#EFF3AC")?,
lightest: Color::from_html("#FAFBE3")?,
},
gray: Colors {
main: Color::from_html("#4a4a4a")?,
darker: Color::from_html("#3d3d3d")?,
lighter: Color::from_html("#939393")?,
lightest: Color::from_html("#c4c4c4")?,
},
red: Color::from_html("#FF5854")?,
black: Color::from_html("#000000")?,
white: Color::from_html("#FFFFFF")?,
transparent: Color::from_html("transparent")?,
};

    Ok(theme)
}

pub struct Theme {
pub teal: Colors,
pub pink: Colors,
pub green: Colors,
pub purple: Colors,
pub yellow: Colors,
pub gray: Colors,
pub red: Color,
pub black: Color,
pub white: Color,
pub transparent: Color,
}

pub struct Colors {
pub main: Color,
pub darker: Color,
pub lighter: Color,
pub lightest: Color,
}

impl Colors {
pub fn main(&self) -> String {
self.main.to_hex_string()
}
pub fn darker(&self) -> String {
self.darker.to_hex_string()
}
pub fn lighter(&self) -> String {
self.lighter.to_hex_string()
}
pub fn lightest(&self) -> String {
self.lightest.to_hex_string()
}
}

impl Theme {
pub fn red(&self) -> String {
self.red.to_hex_string()
}
pub fn black(&self) -> String {
self.black.to_hex_string()
}
pub fn white(&self) -> String {
self.white.to_hex_string()
}
pub fn transparent(&self) -> String {
self.transparent.to_hex_string()
}
}
// /src/app.rs

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
view! { cx,
<Button variant={button::Variant::PRIMARY}/>
<Button variant={button::Variant::SECONDARY}/>
<Button variant={button::Variant::ALERT}/>
}
}

Metadata
So far, everything we’ve rendered has been inside the <body> of the HTML document. And this makes sense. After all, everything you can see on a web page lives inside the <body>.

However, there are plenty of occasions where you might want to update something inside the <head> of the document using the same reactive primitives and component patterns you use for your UI.

That’s where the leptos_meta package comes in.

Metadata Components
leptos_meta provides special components that let you inject data from inside components anywhere in your application into the <head>:

<Title/> allows you to set the document’s title from any component. It also takes a formatter function that can be used to apply the same format to the title set by other pages. So, for example, if you put <Title formatter=|text| format!("{text} — My Awesome Site")/> in your <App/> component, and then <Title text="Page 1"/> and <Title text="Page 2"/> on your routes, you’ll get Page 1 — My Awesome Site and Page 2 — My Awesome Site.

<Link/> takes the standard attributes of the <link> element.

<Stylesheet/> creates a <link rel="stylesheet"> with the href you give.

<Style/> creates a <style> with the children you pass in (usually a string). You can use this to import some custom CSS from another file at compile time <Style>{include_str!("my_route.css")}</Style>.

<Meta/> lets you set <meta> tags with descriptions and other metadata.

<Script/> and <script>
leptos_meta also provides a <Script/> component, and it’s worth pausing here for a second. All of the other components we’ve considered inject <head>-only elements in the <head>. But a <script> can also be included in the body.

There’s a very simple way to determine whether you should use a capital-S <Script/> component or a lowercase-s <script> element: the <Script/> component will be rendered in the <head>, and the <script> element will be rendered wherever in the <body> of your user interface you put it in, alongside other normal HTML elements. These cause JavaScript to load and run at different times, so use whichever is appropriate to your needs.

<Body/> and <Html/>
There are even a couple elements designed to make semantic HTML and styling easier. <Html/> lets you set the lang and dir on your <html> tag from your application code. <Html/> and <Body/> both have class props that let you set their respective class attributes, which is sometimes needed by CSS frameworks for styling.

<Body/> and <Html/> both also have attributes props which can be used to set any number of additional attributes on them via the attr: syntax:

<Html
lang="he"
dir="rtl"
attr:data-theme="dark"
/>
Metadata and Server Rendering
Now, some of this is useful in any scenario, but some of it is especially important for search-engine optimization (SEO). Making sure you have things like appropriate <title> and <meta> tags is crucial. Modern search engine crawlers do handle client-side rendering, i.e., apps that are shipped as an empty index.html and rendered entirely in JS/WASM. But they prefer to receive pages in which your app has been rendered to actual HTML, with metadata in the <head>.

This is exactly what leptos_meta is for. And in fact, during server rendering, this is exactly what it does: collect all the <head> content you’ve declared by using its components throughout your application, and then inject it into the actual <head>.

But I’m getting ahead of myself. We haven’t actually talked about server-side rendering yet. The next chapter will talk about integrating with JavaScript libraries. Then we’ll wrap up the discussion of the client side, and move onto server side rendering.


Integrating with JavaScript: wasm-bindgen, web_sys and HtmlElement
Leptos provides a variety of tools to allow you to build declarative web applications without leaving the world of the framework. Things like the reactive system, component and view macros, and router allow you to build user interfaces without directly interacting with the Web APIs provided by the browser. And they let you do it all directly in Rust, which is great—assuming you like Rust. (And if you’ve gotten this far in the book, we assume you like Rust.)

Ecosystem crates like the fantastic set of utilities provided by leptos-use can take you even further, by providing Leptos-specific reactive wrappers around many Web APIs.

Nevertheless, in many cases you will need to access JavaScript libraries or Web APIs directly. This chapter can help.

Using JS Libraries with wasm-bindgen
Your Rust code can be compiled to a WebAssembly (WASM) module and loaded to run in the browser. However, WASM does not have direct access to browser APIs. Instead, the Rust/WASM ecosystem depends on generating bindings from your Rust code to the JavaScript browser environment that hosts it.

The wasm-bindgen crate is at the center of that ecosystem. It provides both an interface for marking parts of Rust code with annotations telling it how to call JS, and a CLI tool for generating the necessary JS glue code. You’ve been using this without knowing it all along: both trunk and cargo-leptos rely on wasm-bindgen under the hood.

If there is a JavaScript library that you want to call from Rust, you should refer to the wasm-bindgen docs on importing functions from JS. It is relatively easy to import individual functions, classes, or values from JavaScript to use in your Rust app.

It is not always easy to integrate JS libraries into your app directly. In particular, any library that depends on a particular JS framework like React may be hard to integrated. Libraries that manipulate DOM state in some way (for example, rich text editors) should also be used with care: both Leptos and the JS library will probably assume that they are the ultimate source of truth for the app’s state, so you should be careful to separate their responsibilities.

Accessing Web APIs with web-sys
If you just need to access some browser APIs without pulling in a separate JS library, you can do so using the web_sys crate. This provides bindings for all of the Web APIs provided by the browser, with 1:1 mappings from browser types and functions to Rust structs and methods.

In general, if you’re asking “how do I do X with Leptos?” where do X is accessing some Web API, looking up a vanilla JavaScript solution and translating it to Rust using the web-sys docs is a good approach.

After this section, you might find the wasm-bindgen guide chapter on web-sys useful for additional reading.

Enabling features
web_sys is heavily feature-gated to keep compile times low. If you would like to use one of its many APIs, you may need to enable a feature to use it.

The features required to use an item are always listed in its documentation. For example, to use Element::get_bounding_rect_client, you need to enable the DomRect and Element features.

Leptos already enables a whole bunch of features - if the required feature is already enabled here, you won't have to enable it in your own app. Otherwise, add it to your Cargo.toml and you’re good to go!

[dependencies.web-sys]
version = "0.3"
features = ["DomRect"]
However, as the JavaScript standard evolves and APIs are being written, you may want to use browser features that are technically not fully stable yet, such as WebGPU. web_sys will follow the (potentially frequently changing) standard, which means that no stability guarantees are made.

In order to use this, you need to add RUSTFLAGS=--cfg=web_sys_unstable_apis as an environment variable. This can either be done by adding it to every command, or add it to .cargo/config.toml in your repository.

As part of a command:

RUSTFLAGS=--cfg=web_sys_unstable_apis cargo # ...
In .cargo/config.toml:

[env]
RUSTFLAGS = "--cfg=web_sys_unstable_apis"
Accessing raw HtmlElements from your view
The declarative style of the framework means that you don’t need to directly manipulate DOM nodes to build up your user interface. However, in some cases you want direct access to the underlying DOM element that represents part of your view. The section of the book on “uncontrolled inputs” showed how to do this using the NodeRef type.

You may notice that NodeRef::get returns an Option<leptos::HtmlElement<T>>. This is not the same type as a web_sys::HtmlElement, although they are related. So what is this HtmlElement<T> type, and how do you use it?

Overview
web_sys::HtmlElement is the Rust equivalent of the browser’s HTMLElement interface, which is implemented for all HTML elements. It provides access to a minimal set of functions and APIs that are guaranteed to be available for any HTML element. Each particular HTML element then has its own element class, which implements additional functionality. The goal of leptos::HtmlElement<T> is to bridge the gap between elements in your view and these more specific JavaScript types, so that you can access the particular functionality of those elements.

This is implement by using the Rust Deref trait to allow you to dereference a leptos::HtmlElement<T> to the appropriately-typed JS object for that particular element type T.

Definition
Understanding this relationship involves understanding some related traits.

The following simply defines what types are allowed inside the T of leptos::HtmlElement<T> and how it links to web_sys.

pub struct HtmlElement<El> where El: ElementDescriptor { /* ... */ }

pub trait ElementDescriptor: ElementDescriptorBounds { /* ... */ }

pub trait ElementDescriptorBounds: Debug {}
impl<El> ElementDescriptorBounds for El where El: Debug {}

// this is implemented for every single element in `leptos::{html, svg, math}::*`
impl ElementDescriptor for leptos::html::Div { /* ... */ }

// same with this, derefs to the corresponding `web_sys::Html*Element`
impl Deref for leptos::html::Div {
type Target = web_sys::HtmlDivElement;
// ...
}
The following is from web_sys:

impl Deref for web_sys::HtmlDivElement {
type Target = web_sys::HtmlElement;
// ...
}

impl Deref for web_sys::HtmlElement {
type Target = web_sys::Element;
// ...
}

impl Deref for web_sys::Element {
type Target = web_sys::Node;
// ...
}

impl Deref for web_sys::Node {
type Target = web_sys::EventTarget;
// ...
}
web_sys uses long deref chains to emulate the inheritance used in JavaScript. If you can't find the method you're looking for on one type, take a look further down the deref chain. The leptos::html::* types all deref into web_sys::Html*Element or web_sys::HtmlElement. By calling element.method(), Rust will automatically add more derefs as needed to call the correct method!

However, some methods have the same name, such as leptos::HtmlElement::style and web_sys::HtmlElement::style. In these cases, Rust will pick the one that requires the least amount of derefs, which is leptos::HtmlElement::style if you're getting an element straight from a NodeRef. If you wish to use the web_sys method instead, you can manually deref with (*element).style().

If you want to have even more control over which type you are calling a method from, AsRef<T> is implemented for all types that are part of the deref chain, so you can explicitly state which type you want.

See also: The wasm-bindgen Guide: Inheritance in web-sys.

Clones
The web_sys::HtmlElement (and by extension the leptos::HtmlElement too) actually only store references to the HTML element it affects. Therefore, calling .clone() doesn't actually make a new HTML element, it simply gets another reference to the same one. Calling methods that change the element from any of its clones will affect the original element.

Unfortunately, web_sys::HtmlElement does not implement Copy, so you may need to add a bunch of clones especially when using it in closures. Don't worry though, these clones are cheap!

Casting
You can get less specific types through Deref or AsRef, so use those when possible. However, if you need to cast to a more specific type (e.g. from an EventTarget to a HtmlInputElement), you will need to use the methods provided by wasm_bindgen::JsCast (re-exported through web_sys::wasm_bindgen::JsCast). You'll probably only need the dyn_ref method.

use web_sys::wasm_bindgen::JsCast;

let on_click = |ev: MouseEvent| {
let target: HtmlInputElement = ev.current_target().unwrap().dyn_ref().unwrap();
// or, just use the existing `leptos::event_target_*` functions
}
See the event_target_* functions here, if you're curious.

leptos::HtmlElement
The leptos::HtmlElement adds some extra convenience methods to make it easier to manipulate common attributes. These methods were built for the builder syntax, so it takes and returns self. You can just do _ = element.clone().<method>() to ignore the element it returns - it'll still affect the original element, even though it doesn't look like it (see previous section on Clones)!

Here are some of the common methods you may want to use, for example in event listeners or use: directives.

id: overwrites the id on the element.
classes: adds the classes to the element. You can specify multiple classes with a space-separated string. You can also use class to conditionally add a single class: do not add multiple with this method.
attr: sets a key=value attribute to the element.
prop: sets a property on the element: see the distinction between properties and attributes here.
on: adds an event listener to the element. Specify the event type through one of leptos::ev::* (it's the ones in all lowercase).
child: adds an element as the last child of the element.
Take a look at the rest of the leptos::HtmlElement methods too. If none of them fit your requirements, also take a look at leptos-use. Otherwise, you’ll have to use the web_sys APIs.


Wrapping Up Part 1: Client-Side Rendering
So far, everything we’ve written has been rendered almost entirely in the browser. When we create an app using Trunk, it’s served using a local development server. If you build it for production and deploy it, it’s served by whatever server or CDN you’re using. In either case, what’s served is an HTML page with

the URL of your Leptos app, which has been compiled to WebAssembly (WASM)
the URL of the JavaScript used to initialize this WASM blob
an empty <body> element
When the JS and WASM have loaded, Leptos will render your app into the <body>. This means that nothing appears on the screen until JS/WASM have loaded and run. This has some drawbacks:

It increases load time, as your user’s screen is blank until additional resources have been downloaded.
It’s bad for SEO, as load times are longer and the HTML you serve has no meaningful content.
It’s broken for users for whom JS/WASM don’t load for some reason (e.g., they’re on a train and just went into a tunnel before WASM finished loading; they’re using an older device that doesn’t support WASM; they have JavaScript or WASM turned off for some reason; etc.)
These downsides apply across the web ecosystem, but especially to WASM apps.

However, depending on the requirements of your project, you may be fine with these limitations.

If you just want to deploy your Client-Side Rendered website, skip ahead to the chapter on "Deployment" - there, you'll find directions on how best to deploy your Leptos CSR site.

But what do you do if you want to return more than just an empty <body> tag in your index.html page? Use “Server-Side Rendering”!

Whole books could be (and probably have been) written about this topic, but at its core, it’s really simple: rather than returning an empty <body> tag, with SSR, you'll return an initial HTML page that reflects the actual starting state of your app or site, so that while JS/WASM are loading, and until they load, the user can access the plain HTML version.

Part 2 of this book, on Leptos SSR, will cover this topic in some detail!


Part 2: Server Side Rendering
The second part of the book is all about how to turn your beautiful UIs into full-stack Rust + Leptos powered websites and applications.

As you read in the last chapter, there are some limitations to using client-side rendered Leptos apps - over the next few chapters, you'll see how we can overcome those limitations and get the best performance and SEO out of your Leptos apps.

Info

When working with Leptos on the server side, you're free to choose either the Actix-web or the Axum integrations - the full feature set of Leptos is available with either option.

If, however, you need deploy to a WinterCG-compatible runtime like Deno, Cloudflare, etc., then choose the Axum integration as this deployment option is only available with Axum on the server. Lastly, if you'd like to go full-stack WASM/WASI and deploy to WASM-based serverless runtimes, then Axum is your go-to choice here too.

NB: this is a limitation of the web frameworks themselves, not Leptos.


Introducing cargo-leptos
So far, we’ve just been running code in the browser and using Trunk to coordinate the build process and run a local development process. If we’re going to add server-side rendering, we’ll need to run our application code on the server as well. This means we’ll need to build two separate binaries, one compiled to native code and running the server, the other compiled to WebAssembly (WASM) and running in the user’s browser. Additionally, the server needs to know how to serve this WASM version (and the JavaScript required to initialize it) to the browser.

This is not an insurmountable task but it adds some complication. For convenience and an easier developer experience, we built the cargo-leptos build tool. cargo-leptos basically exists to coordinate the build process for your app, handling recompiling the server and client halves when you make changes, and adding some built-in support for things like Tailwind, SASS, and testing.

Getting started is pretty easy. Just run

cargo install cargo-leptos
And then to create a new project, you can run either

# for an Actix template
cargo leptos new --git leptos-rs/start
or

# for an Axum template
cargo leptos new --git leptos-rs/start-axum
Now cd into the directory you’ve created and run

cargo leptos watch
Note: Remember that Leptos has a nightly feature, which each of these starters use. If you're using the stable Rust compiler, that’s fine; just remove the nightly feature from each of the Leptos dependencies in your new Cargo.toml and you should be all set.

Once your app has compiled you can open up your browser to http://localhost:3000 to see it.

cargo-leptos has lots of additional features and built in tools. You can learn more in its README.

But what exactly is happening when you open our browser to localhost:3000? Well, read on to find out.

deep

Features
Parallel build of server and client in watch mode for fast developer feedback.
CSS hot-reload (no page-reload, only CSS updated).
Build server and client for hydration (client-side rendering mode not supported).
Support for both workspace and single-package setup.
SCSS compilation using dart-sass.
CSS transformation and minification using Lightning CSS.
Builds server and client (wasm) binaries using Cargo.
Generates JS - Wasm bindings with wasm-bindgen
Includes support for JS Snippets for when you want to call some JS code from your WASM.
Optimises the wasm with wasm-opt from Binaryen
watch command for automatic rebuilds with browser live-reload.
test command for running tests of the lib and bin packages that makes up the Leptos project.
build build the server and client.
end-to-end command for building, running the server and calling a bash shell hook. The hook would typically launch Playwright or similar.
new command for creating a new project based on templates, using cargo-generate. Current templates include
https://github.com/leptos-rs/start: An Actix starter
https://github.com/leptos-rs/start-axum: An Axum starter
https://github.com/leptos-rs/start-axum-workspace: An Axum starter keeping client and server code in separate crates in a workspace
'no_downloads' feature to allow user management of optional dependencies
Getting started
Install:

cargo install --locked cargo-leptos

If you, for any reason, need the bleeding-edge super fresh version:

cargo install --git https://github.com/leptos-rs/cargo-leptos --locked cargo-leptos

Help:

cargo leptos --help

For setting up your project, have a look at the examples


Dependencies
The dependencies for sass, wasm-opt and cargo-generate are automatically installed in a cache directory when they are used if they are not already installed and found by which. Different versions of the dependencies might accumulate in this directory, so feel free to delete it.

OS	Example
Linux	/home/alice/.cache/cargo-leptos
macOS	/Users/Alice/Library/Caches/cargo-leptos
Windows	C:\Users\Alice\AppData\Local\cargo-leptos
If you wish to make it mandatory to install your dependencies, or are using Nix or NixOs, you can install it with the no_downloads feature enabled to prevent cargo-leptos from trying to download and install them.

cargo install --features no_downloads --locked cargo-leptos


Single-package setup
The single-package setup is where the code for both the frontend and the server is defined in a single package.

Configuration parameters are defined in the package Cargo.toml section [package.metadata.leptos]. See the Parameters reference for a full list of parameters that can be used. All paths are relative to the package root (i.e. to the Cargo.toml file)


Workspace setup
When using a workspace setup both single-package and multi-package projects are supported. The latter is when the frontend and the server reside in different packages.

All workspace members whose Cargo.toml define the [package.metadata.leptos] section are automatically included as Leptos single-package projects. The multi-package projects are defined on the workspace level in the Cargo.toml's section [[workspace.metadata.leptos]] which takes three mandatory parameters:

[[workspace.metadata.leptos]]
# project name
name = "leptos-project"
bin-package = "server"
lib-package = "front"

# more configuration parameters...
Note the double braces: several projects can be defined and one package can be used in several projects.


Build features
When building with cargo-leptos, the frontend, library package, is compiled into wasm using target wasm-unknown-unknown and the features --no-default-features --features=hydrate The server binary is compiled with the features --no-default-features --features=ssr


Parameters reference
These parameters are used either in the workspace section [[workspace.metadata.leptos]] or the package, for single-package setups, section [package.metadata.leptos].

Note that the Cargo Manifest uses the word target with two different meanings. As a package's configured [[bin]] targets and as the compiled output target triple. Here, the latter is referred to as target-triple.

Compilation parameters
# Sets the name of the binary target used.
#
# Optional, only necessary if the bin-package defines more than one target
bin-target = "my-bin-name"

# Enables additional file hashes on outputted css, js, and wasm files
#
# Optional: Defaults to false. Can also be set with the LEPTOS_HASH_FILES=false env var
hash-files = false

# Sets the name for the file cargo-leptos uses to track the most recent hashes
#
# Optional: Defaults to "hash.txt". Can also be set with the LEPTOS_HASH_FILE_NAME="hash.txt" env var
hash-file-name = false

# The features to use when compiling all targets
#
# Optional. Can be extended with the command line parameter --features
features = []

# The features to use when compiling the bin target
#
# Optional. Can be over-ridden with the command line parameter --bin-features
bin-features = ["ssr"]

# If the --no-default-features flag should be used when compiling the bin target
#
# Optional. Defaults to false.
bin-default-features = false

# The profile to use for the bin target when compiling for release
#
# Optional. Defaults to "release".
bin-profile-release = "my-release-profile"

# The profile to use for the bin target when compiling for debug
#
# Optional. Defaults to "debug".
bin-profile-dev = "my-debug-profile"

# The target triple to use when compiling the bin target
#
# Optional. Env: LEPTOS_BIN_TARGET_TRIPLE
bin-target-triple = "x86_64-unknown-linux-gnu"

# The features to use when compiling the lib target
#
# Optional. Can be over-ridden with the command line parameter --lib-features
lib-features = ["hydrate"]

# If the --no-default-features flag should be used when compiling the lib target
#
# Optional. Defaults to false.
lib-default-features = false

# The profile to use for the lib target when compiling for release
#
# Optional. Defaults to "release".
lib-profile-release = "my-release-profile"

# The profile to use for the lib target when compiling for debug
#
# Optional. Defaults to "debug".
lib-profile-dev = "my-debug-profile"

# Fixes cargo bug that prevents incremental compilation (see #203)
#
# Optional. Defaults to false prior to 0.2.3, unconditionally enabled (with the setting becoming deprecated) since 0.2.3 and #216
separate-front-target-dir = true

# Pass additional parameters to the cargo process compiling to WASM
# 
# Optional. No default
lib-cargo-args = ["--timings"]

# Pass additional parameters to the cargo process to build the server
# 
# Optional. No default
bin-cargo-args = ["--timings"]

# The command to run instead of "cargo" when building the server
#
# Optional. No default. Env: LEPTOS_BIN_CARGO_COMMAND
bin-cargo-command = "cross"
Site parameters
These parameters can be overridden by setting the corresponding environment variable. They can also be set in a .env file as cargo-leptos reads the first it finds in the package or workspace directory and any parent directory.

# Sets the name of the output js, wasm and css files.
#
# Optional, defaults to the lib package name or, in a workspace, the project name. Env: LEPTOS_OUTPUT_NAME.
output-name = "myproj"

# The site root folder is where cargo-leptos generate all output.
# NOTE: It is relative to the workspace root when running in a workspace.
# WARNING: all content of this folder will be erased on a rebuild!
#
# Optional, defaults to "/site" in the Cargo target directory. Env: LEPTOS_SITE_ROOT.
site-root = "target/site"

# The site-root relative folder where all compiled output (JS, WASM and CSS) is written.
#
# Optional, defaults to "pkg". Env: LEPTOS_SITE_PKG_DIR.
site-pkg-dir = "pkg"

# The source style file. If it ends with _.sass_ or _.scss_ then it will be compiled by `dart-sass`
# into CSS and processed by lightning css. When release is set, then it will also be minified.
#
# Optional. Env: LEPTOS_STYLE_FILE.
style-file = "style/main.scss"

# The tailwind input file.
#
# Optional, Activates the tailwind build
tailwind-input-file = "style/tailwind.css"

# The tailwind config file.
#
# Optional, defaults to "tailwind.config.js" which if is not present
# is generated for you
tailwind-config-file = "tailwind.config.js"

# The browserlist https://browsersl.ist query used for optimizing the CSS.
#
# Optional, defaults to "defaults". Env: LEPTOS_BROWSERQUERY.
browserquery = "defaults"

# Assets source dir. All files found here will be copied and synchronized to site-root.
# The assets-dir cannot have a sub directory with the same name/path as site-pkg-dir.
#
# Optional. Env: LEPTOS_ASSETS_DIR.
assets-dir = "assets"

# JS source dir. `wasm-bindgen` has the option to include JS snippets from JS files
# with `#[wasm_bindgen(module = "/js/foo.js")]`. A change in any JS file in this dir
# will trigger a rebuild.
#
# Optional. Defaults to "src"
js-dir = "src"

# Additional files your application could depends on.
# A change to any file in those directories will trigger a rebuild.
#
# Optional.
watch-additional-files = ["additional_files", "custom_config.json"]

# The IP and port where the server serves the content. Use it in your server setup.
#
# Optional, defaults to 127.0.0.1:3000. Env: LEPTOS_SITE_ADDR.
site-addr = "127.0.0.1:3000"

# The port number used by the reload server (only used in watch mode).
#
# Optional, defaults 3001. Env: LEPTOS_RELOAD_PORT
reload-port = 3001

# The command used for running end-to-end tests. See the section about End-to-end testing.
#
# Optional. Env: LEPTOS_END2END_CMD.
end2end-cmd = "npx playwright test"

# The directory from which the end-to-end tests are run.
#
# Optional. Env: LEPTOS_END2END_DIR
end2end-dir = "integration"

Environment variables
The following environment variables are set when compiling the lib (front) or bin (server) and when the server is run.

Echoed from the Leptos config:

LEPTOS_OUTPUT_NAME
LEPTOS_SITE_ROOT
LEPTOS_SITE_PKG_DIR
LEPTOS_SITE_ADDR
LEPTOS_RELOAD_PORT
Directories used when building:

LEPTOS_LIB_DIR: The path (relative to the working directory) to the library package
LEPTOS_BIN_DIR: The path (relative to the working directory) to the binary package
Note when using directories:

cargo-leptos changes the working directory to the project root or if in a workspace, the workspace root before building and running.
the two are set to the same value when running in a single-package config.
Avoid using them at run-time unless you can guarantee that the entire project struct is available at runtime as well.
Internally the versions of the external tools called by cargo-leptos are hardcoded. Use these environment variables to override the versions cargo-leptos should use (e.g. LEPTOS_SASS_VERSION=1.69.5):

LEPTOS_CARGO_GENERATE_VERSION
LEPTOS_TAILWIND_VERSION
LEPTOS_SASS_VERSION
LEPTOS_WASM_OPT_VERSION
End-to-end testing
cargo-leptos provides end-to-end testing support for convenience. It is a simple wrapper around a shell command end2end-cmd that is executed in a specific directory end2end-dir.

The end2end-cmd can be any shell command. For running Playwright it would be npx playwright test.

What it does is equivalent to running this manually:

in a terminal, run cargo leptos watch
in a separate terminal, change to the end2end-dir and run the end2end-cmd.
When testing the setup, please try the above first. If that works but cargo leptos end-to-end doesn't then please create a GitHub ticket.


The Life of a Page Load
Before we get into the weeds it might be helpful to have a higher-level overview. What exactly happens between the moment you type in the URL of a server-rendered Leptos app, and the moment you click a button and a counter increases?

I’m assuming some basic knowledge of how the Internet works here, and won’t get into the weeds about HTTP or whatever. Instead, I’ll try to show how different parts of the Leptos APIs map onto each part of the process.

This description also starts from the premise that your app is being compiled for two separate targets:

A server version, often running on Actix or Axum, compiled with the Leptos ssr feature
A browser version, compiled to WebAssembly (WASM) with the Leptos hydrate feature
The cargo-leptos build tool exists to coordinate the process of compiling your app for these two different targets.

On the Server
Your browser makes a GET request for that URL to your server. At this point, the browser knows almost nothing about the page that’s going to be rendered. (The question “How does the browser know where to ask for the page?” is an interesting one, but out of the scope of this tutorial!)
The server receives that request, and checks whether it has a way to handle a GET request at that path. This is what the .leptos_routes() methods in leptos_axum and leptos_actix are for. When the server starts up, these methods walk over the routing structure you provide in <Routes/>, generating a list of all possible routes your app can handle and telling the server’s router “for each of these routes, if you get a request... hand it off to Leptos.”
The server sees that this route can be handled by Leptos. So it renders your root component (often called something like <App/>), providing it with the URL that’s being requested and some other data like the HTTP headers and request metadata.
Your application runs once on the server, building up an HTML version of the component tree that will be rendered at that route. (There’s more to be said here about resources and <Suspense/> in the next chapter.)
The server returns this HTML page, also injecting information on how to load the version of your app that has been compiled to WASM so that it can run in the browser.
The HTML page that’s returned is essentially your app, “dehydrated” or “freeze-dried”: it is HTML without any of the reactivity or event listeners you’ve added. The browser will “rehydrate” this HTML page by adding the reactive system and attaching event listeners to that server-rendered HTML. Hence the two feature flags that apply to the two halves of this process: ssr on the server for “server-side rendering”, and hydrate in the browser for that process of rehydration.

In the Browser
The browser receives this HTML page from the server. It immediately goes back to the server to begin loading the JS and WASM necessary to run the interactive, client side version of the app.
In the meantime, it renders the HTML version.
When the WASM version has reloaded, it does the same route-matching process that the server did. Because the <Routes/> component is identical on the server and in the client, the browser version will read the URL and render the same page that was already returned by the server.
During this initial “hydration” phase, the WASM version of your app doesn’t re-create the DOM nodes that make up your application. Instead, it walks over the existing HTML tree, “picking up” existing elements and adding the necessary interactivity.
Note that there are some trade-offs here. Before this hydration process is complete, the page will appear interactive but won’t actually respond to interactions. For example, if you have a counter button and click it before WASM has loaded, the count will not increment, because the necessary event listeners and reactivity have not been added yet. We’ll look at some ways to build in “graceful degradation” in future chapters.

Client-Side Navigation
The next step is very important. Imagine that the user now clicks a link to navigate to another page in your application.

The browser will not make another round trip to the server, reloading the full page as it would for navigating between plain HTML pages or an application that uses server rendering (for example with PHP) but without a client-side half.

Instead, the WASM version of your app will load the new page, right there in the browser, without requesting another page from the server. Essentially, your app upgrades itself from a server-loaded “multi-page app” into a browser-rendered “single-page app.” This yields the best of both worlds: a fast initial load time due to the server-rendered HTML, and fast secondary navigations because of the client-side routing.

Some of what will be described in the following chapters—like the interactions between server functions, resources, and <Suspense/>—may seem overly complicated. You might find yourself asking, “If my page is being rendered to HTML on the server, why can’t I just .await this on the server? If I can just call library X in a server function, why can’t I call it in my component?” The reason is pretty simple: to enable the upgrade from server rendering to client rendering, everything in your application must be able to run either on the server or in the browser.

This is not the only way to create a website or web framework, of course. But it’s the most common way, and we happen to think it’s quite a good way, to create the smoothest possible experience for your users.


Async Rendering and SSR “Modes”
Server-rendering a page that uses only synchronous data is pretty simple: You just walk down the component tree, rendering each element to an HTML string. But this is a pretty big caveat: it doesn’t answer the question of what we should do with pages that includes asynchronous data, i.e., the sort of stuff that would be rendered under a <Suspense/> node on the client.

When a page loads async data that it needs to render, what should we do? Should we wait for all the async data to load, and then render everything at once? (Let’s call this “async” rendering) Should we go all the way in the opposite direction, just sending the HTML we have immediately down to the client and letting the client load the resources and fill them in? (Let’s call this “synchronous” rendering) Or is there some middle-ground solution that somehow beats them both? (Hint: There is.)

If you’ve ever listened to streaming music or watched a video online, I’m sure you realize that HTTP supports streaming, allowing a single connection to send chunks of data one after another without waiting for the full content to load. You may not realize that browsers are also really good at rendering partial HTML pages. Taken together, this means that you can actually enhance your users’ experience by streaming HTML: and this is something that Leptos supports out of the box, with no configuration at all. And there’s actually more than one way to stream HTML: you can stream the chunks of HTML that make up your page in order, like frames of a video, or you can stream them... well, out of order.

Let me say a little more about what I mean.

Leptos supports all the major ways of rendering HTML that includes asynchronous data:

Synchronous Rendering
Async Rendering
In-Order streaming
Out-of-Order Streaming (and a partially-blocked variant)
Synchronous Rendering
Synchronous: Serve an HTML shell that includes fallback for any <Suspense/>. Load data on the client using create_local_resource, replacing fallback once resources are loaded.
Pros: App shell appears very quickly: great TTFB (time to first byte).
Cons
Resources load relatively slowly; you need to wait for JS + WASM to load before even making a request.
No ability to include data from async resources in the <title> or other <meta> tags, hurting SEO and things like social media link previews.
If you’re using server-side rendering, the synchronous mode is almost never what you actually want, from a performance perspective. This is because it misses out on an important optimization. If you’re loading async resources during server rendering, you can actually begin loading the data on the server. Rather than waiting for the client to receive the HTML response, then loading its JS + WASM, then realize it needs the resources and begin loading them, server rendering can actually begin loading the resources when the client first makes the response. In this sense, during server rendering an async resource is like a Future that begins loading on the server and resolves on the client. As long as the resources are actually serializable, this will always lead to a faster total load time.

This is why create_resource requires resources data to be serializable by default, and why you need to explicitly use create_local_resource for any async data that is not serializable and should therefore only be loaded in the browser itself. Creating a local resource when you could create a serializable resource is always a deoptimization.

Async Rendering
async: Load all resources on the server. Wait until all data are loaded, and render HTML in one sweep.
Pros: Better handling for meta tags (because you know async data even before you render the <head>). Faster complete load than synchronous because async resources begin loading on server.
Cons: Slower load time/TTFB: you need to wait for all async resources to load before displaying anything on the client. The page is totally blank until everything is loaded.
In-Order Streaming
In-order streaming: Walk through the component tree, rendering HTML until you hit a <Suspense/>. Send down all the HTML you’ve got so far as a chunk in the stream, wait for all the resources accessed under the <Suspense/> to load, then render it to HTML and keep walking until you hit another <Suspense/> or the end of the page.
Pros: Rather than a blank screen, shows at least something before the data are ready.
Cons
Loads the shell more slowly than synchronous rendering (or out-of-order streaming) because it needs to pause at every <Suspense/>.
Unable to show fallback states for <Suspense/>.
Can’t begin hydration until the entire page has loaded, so earlier pieces of the page will not be interactive until the suspended chunks have loaded.
Out-of-Order Streaming
Out-of-order streaming: Like synchronous rendering, serve an HTML shell that includes fallback for any <Suspense/>. But load data on the server, streaming it down to the client as it resolves, and streaming down HTML for <Suspense/> nodes, which is swapped in to replace the fallback.
Pros: Combines the best of synchronous and async.
Fast initial response/TTFB because it immediately sends the whole synchronous shell
Fast total time because resources begin loading on the server.
Able to show the fallback loading state and dynamically replace it, instead of showing blank sections for un-loaded data.
Cons: Requires JavaScript to be enabled for suspended fragments to appear in correct order. (This small chunk of JS streamed down in a <script> tag alongside the <template> tag that contains the rendered <Suspense/> fragment, so it does not need to load any additional JS files.)
Partially-blocked streaming: “Partially-blocked” streaming is useful when you have multiple separate <Suspense/> components on the page. It is triggered by setting ssr=SsrMode::PartiallyBlocked on a route, and depending on blocking resources within the view. If one of the <Suspense/> components reads from one or more “blocking resources” (see below), the fallback will not be sent; rather, the server will wait until that <Suspense/> has resolved and then replace the fallback with the resolved fragment on the server, which means that it is included in the initial HTML response and appears even if JavaScript is disabled or not supported. Other <Suspense/> stream in out of order, similar to the SsrMode::OutOfOrder default.
This is useful when you have multiple <Suspense/> on the page, and one is more important than the other: think of a blog post and comments, or product information and reviews. It is not useful if there’s only one <Suspense/>, or if every <Suspense/> reads from blocking resources. In those cases it is a slower form of async rendering.

Pros: Works if JavaScript is disabled or not supported on the user’s device.
Cons
Slower initial response time than out-of-order.
Marginally overall response due to additional work on the server.
No fallback state shown.
Using SSR Modes
Because it offers the best blend of performance characteristics, Leptos defaults to out-of-order streaming. But it’s really simple to opt into these different modes. You do it by adding an ssr property onto one or more of your <Route/> components, like in the ssr_modes example.

<Routes>
    // We’ll load the home page with out-of-order streaming and <Suspense/>
    <Route path="" view=HomePage/>

    // We'll load the posts with async rendering, so they can set
    // the title and metadata *after* loading the data
    <Route
        path="/post/:id"
        view=Post
        ssr=SsrMode::Async
    />
</Routes>
For a path that includes multiple nested routes, the most restrictive mode will be used: i.e., if even a single nested route asks for async rendering, the whole initial request will be rendered async. async is the most restricted requirement, followed by in-order, and then out-of-order. (This probably makes sense if you think about it for a few minutes.)

Blocking Resources
Any Leptos versions later than 0.2.5 (i.e., git main and 0.3.x or later) introduce a new resource primitive with create_blocking_resource. A blocking resource still loads asynchronously like any other async/.await in Rust; it doesn’t block a server thread or anything. Instead, reading from a blocking resource under a <Suspense/> blocks the HTML stream from returning anything, including its initial synchronous shell, until that <Suspense/> has resolved.

Now from a performance perspective, this is not ideal. None of the synchronous shell for your page will load until that resource is ready. However, rendering nothing means that you can do things like set the <title> or <meta> tags in your <head> in actual HTML. This sounds a lot like async rendering, but there’s one big difference: if you have multiple <Suspense/> sections, you can block on one of them but still render a placeholder and then stream in the other.

For example, think about a blog post. For SEO and for social sharing, I definitely want my blog post’s title and metadata in the initial HTML <head>. But I really don’t care whether comments have loaded yet or not; I’d like to load those as lazily as possible.

With blocking resources, I can do something like this:

#[component]
pub fn BlogPost() -> impl IntoView {
let post_data = create_blocking_resource(/* load blog post */);
let comments_data = create_resource(/* load blog comments */);
view! {
<Suspense fallback=|| ()>
{move || {
post_data.with(|data| {
view! {
<Title text=data.title/>
<Meta name="description" content=data.excerpt/>
<article>
/* render the post content */
</article>
}
})
}}
</Suspense>
<Suspense fallback=|| "Loading comments...">
/* render comments data here */
</Suspense>
}
}
The first <Suspense/>, with the body of the blog post, will block my HTML stream, because it reads from a blocking resource. Meta tags and other head elements awaiting the blocking resource will be rendered before the stream is sent.

Combined with the following route definition, which uses SsrMode::PartiallyBlocked, the blocking resource will be fully rendered on the server side, making it accessible to users who disable WebAssembly or JavaScript.

<Routes>
    // We’ll load the home page with out-of-order streaming and <Suspense/>
    <Route path="" view=HomePage/>

    // We'll load the posts with async rendering, so they can set
    // the title and metadata *after* loading the data
    <Route
        path="/post/:id"
        view=Post
        ssr=SsrMode::PartiallyBlocked
    />
</Routes>
The second <Suspense/>, with the comments, will not block the stream. Blocking resources gave me exactly the power and granularity I needed to optimize my page for SEO and user experience.


Where do you expect where do I run? to log?

In the command line where you’re running the server?
In the browser console when you load the page?
Neither?
Both?
Try it out.

...

...

...

Okay, consider the spoiler alerted.

You’ll notice of course that it logs in both places, assuming everything goes according to plan. In fact on the server it logs twice—first during the initial server startup, when Leptos renders your app once to extract the route tree, then a second time when you make a request. Each time you reload the page, where do I run? should log once on the server and once on the client.

If you think about the description in the last couple sections, hopefully this makes sense. Your application runs once on the server, where it builds up a tree of HTML which is sent to the client. During this initial render, where do I run? logs on the server.

Once the WASM binary has loaded in the browser, your application runs a second time, walking over the same user interface tree and adding interactivity.

Does that sound like a waste? It is, in a sense. But reducing that waste is a genuinely hard problem. It’s what some JS frameworks like Qwik are intended to solve, although it’s probably too early to tell whether it’s a net performance gain as opposed to other approaches.

The Potential for Bugs
Okay, hopefully all of that made sense. But what does it have to do with the title of this chapter, which is “Hydration bugs (and how to avoid them)”?

Remember that the application needs to run on both the server and the client. This generates a few different sets of potential issues you need to know how to avoid.

Mismatches between server and client code
One way to create a bug is by creating a mismatch between the HTML that’s sent down by the server and what’s rendered on the client. It’s actually fairly hard to do this unintentionally, I think (at least judging by the bug reports I get from people.) But imagine I do something like this

#[component]
pub fn App() -> impl IntoView {
let data = if cfg!(target_arch = "wasm32") {
vec![0, 1, 2]
} else {
vec![]
};
data.into_iter()
.map(|value| view! { <span>{value}</span> })
.collect_view()
}
In other words, if this is being compiled to WASM, it has three items; otherwise it’s empty.

When I load the page in the browser, I see nothing. If I open the console I see a bunch of warnings:

element with id 0-3 not found, ignoring it for hydration
element with id 0-4 not found, ignoring it for hydration
element with id 0-5 not found, ignoring it for hydration
component with id _0-6c not found, ignoring it for hydration
component with id _0-6o not found, ignoring it for hydration
The WASM version of your app, running in the browser, expects to find three items; but the HTML has none.

Solution
It’s pretty rare that you do this intentionally, but it could happen from somehow running different logic on the server and in the browser. If you’re seeing warnings like this and you don’t think it’s your fault, it’s much more likely that it’s a bug with <Suspense/> or something. Feel free to go ahead and open an issue or discussion on GitHub for help.

Not all client code can run on the server
Imagine you happily import a dependency like gloo-net that you’ve been used to using to make requests in the browser, and use it in a create_resource in a server-rendered app.

You’ll probably instantly see the dreaded message

panicked at 'cannot call wasm-bindgen imported functions on non-wasm targets'
Uh-oh.

But of course this makes sense. We’ve just said that your app needs to run on the client and the server.

Solution
There are a few ways to avoid this:

Only use libraries that can run on both the server and the client. reqwest, for example, works for making HTTP requests in both settings.
Use different libraries on the server and the client, and gate them using the #[cfg] macro. (Click here for an example.)
Wrap client-only code in create_effect. Because create_effect only runs on the client, this can be an effective way to access browser APIs that are not needed for initial rendering.
For example, say that I want to store something in the browser’s localStorage whenever a signal changes.

#[component]
pub fn App() -> impl IntoView {
use gloo_storage::Storage;
let storage = gloo_storage::LocalStorage::raw();
logging::log!("{storage:?}");
}
This panics because I can’t access LocalStorage during server rendering.

But if I wrap it in an effect...

#[component]
pub fn App() -> impl IntoView {
use gloo_storage::Storage;
create_effect(move |_| {
let storage = gloo_storage::LocalStorage::raw();
logging::log!("{storage:?}");
});
}
It’s fine! This will render appropriately on the server, ignoring the client-only code, and then access the storage and log a message on the browser.

Not all server code can run on the client
WebAssembly running in the browser is a pretty limited environment. You don’t have access to a file-system or to many of the other things the standard library may be used to having. Not every crate can even be compiled to WASM, let alone run in a WASM environment.

In particular, you’ll sometimes see errors about the crate mio or missing things from core. This is generally a sign that you are trying to compile something to WASM that can’t be compiled to WASM. If you’re adding server-only dependencies, you’ll want to mark them optional = true in your Cargo.toml and then enable them in the ssr feature definition. (Check out one of the template Cargo.toml files to see more details.)

You can use create_effect to specify that something should only run on the client, and not in the server. Is there a way to specify that something should run only on the server, and not the client?

In fact, there is. The next chapter will cover the topic of server functions in some detail. (In the meantime, you can check out their docs here.)

Crate leptos_serverCopy item path
source · [−]
Structs
Action	An action synchronizes an imperative async call to the synchronous reactive system.
MultiAction	An action that synchronizes multiple imperative async calls to the reactive system, tracking the progress of each one.
Submission	An action that has been submitted by dispatching it to a MultiAction.
Enums
ServerFnError	Type for errors that can occur when using server functions.
ServerFnErrorErr	Type for errors that can occur when using server functions.
Functions
create_action	Creates an Action to synchronize an imperative async call to the synchronous reactive system.
create_multi_action	Creates an MultiAction to synchronize an imperative async call to the synchronous reactive system.
create_server_action	Creates an Action that can be used to call a server function.
create_server_multi_action	Creates an MultiAction that can be used to call a server function.

Struct leptos_server::ActionCopy item path
source · [−]
pub struct Action<I, O>(/* private fields */)
where
I: 'static,
O: 'static;
An action synchronizes an imperative async call to the synchronous reactive system.

If you’re trying to load data by running an async function reactively, you probably want to use a Resource instead. If you’re trying to occasionally run an async function in response to something like a user clicking a button, you’re in the right place.

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
Implementations
source
impl<I, O> Action<I, O>
where
I: 'static,
O: 'static,
source
pub fn dispatch(&self, input: I)
Calls the async function with a reference to the input type as its argument.

source
pub fn new<F, Fu>(action_fn: F) -> Self
where
F: Fn(&I) -> Fu + 'static,
Fu: Future<Output = O> + 'static,
Create an Action.

Action is a type of Signal which represent imperative calls to an asynchronous function. Where a Resource is driven as a function of a Signal, Actions are Action::dispatched by events or handlers.


let act = Action::new(|n: &u8| {
let n = n.to_owned();
async move { n * 2 }
});
act.dispatch(3);
assert_eq!(act.value().get(), Some(6));

// Remember that async functions already return a future if they are
// not `await`ed. You can save keystrokes by leaving out the `async move`

let act2 = Action::new(|n: &String| yell(n.to_owned()));
act2.dispatch(String::from("i'm in a doctest"));
assert_eq!(act2.value().get(), Some("I'M IN A DOCTEST".to_string()));

async fn yell(n: String) -> String {
n.to_uppercase()
}
source
pub fn pending(&self) -> ReadSignal<bool>
Whether the action has been dispatched and is currently waiting for its future to be resolved.

source
pub fn set_pending(&self, pending: bool)
Updates whether the action is currently pending. If the action has been dispatched multiple times, and some of them are still pending, it will not update the pending signal.

source
pub fn url(&self) -> Option<String>
The URL associated with the action (typically as part of a server function.) This enables integration with the ActionForm component in leptos_router.

source
pub fn version(&self) -> RwSignal<usize>
How many times the action has successfully resolved.

source
pub fn input(&self) -> RwSignal<Option<I>>
The current argument that was dispatched to the async function. Some while we are waiting for it to resolve, None if it has resolved.

source
pub fn value(&self) -> RwSignal<Option<O>>
The most recent return value of the async function.

source
impl<I> Action<I, Result<I::Output, ServerFnError<I::Error>>>
where
I: ServerFn + 'static,
source
pub fn server() -> Action<I, Result<I::Output, ServerFnError<I::Error>>>
where
I: ServerFn + Clone,
I::Error: Clone + 'static,
Create an Action to imperatively call a server function.

The struct representing your server function’s arguments should be provided to the Action. Unless specified as an argument to the server macro, the generated struct is your function’s name converted to CamelCase.