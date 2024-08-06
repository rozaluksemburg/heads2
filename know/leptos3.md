Defining Routes
Getting Started
It’s easy to get started with the router.

First things first, make sure you’ve added the leptos_router package to your dependencies. Like leptos, the router relies on activating a csr, hydrate, or ssr feature. For example, if you’re adding the router to a client-side rendered app, you’ll want to run

cargo add leptos_router --features=csr
It’s important that the router is a separate package from leptos itself. This means that everything in the router can be defined in user-land code. If you want to create your own router, or use no router, you’re completely free to do that!

And import the relevant types from the router, either with something like

use leptos_router::{Route, RouteProps, Router, RouterProps, Routes, RoutesProps};
or simply

use leptos_router::*;
Providing the <Router/>
Routing behavior is provided by the <Router/> component. This should usually be somewhere near the root of your application, the rest of the app.

You shouldn’t try to use multiple <Router/>s in your app. Remember that the router drives global state: if you have multiple routers, which one decides what to do when the URL changes?

Let’s start with a simple <App/> component using the router:

use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
view! {
<Router>
<nav>
/* ... */
</nav>
<main>
/* ... */
</main>
</Router>
}
}
Defining <Routes/>
The <Routes/> component is where you define all the routes to which a user can navigate in your application. Each possible route is defined by a <Route/> component.

You should place the <Routes/> component at the location within your app where you want routes to be rendered. Everything outside <Routes/> will be present on every page, so you can leave things like a navigation bar or menu outside the <Routes/>.

use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
view! {
<Router>
<nav>
/* ... */
</nav>
<main>
// all our routes will appear inside <main>
<Routes>
/* ... */
</Routes>
</main>
</Router>
}
}
Individual routes are defined by providing children to <Routes/> with the <Route/> component. <Route/> takes a path and a view. When the current location matches path, the view will be created and displayed.

The path can include

a static path (/users),
dynamic, named parameters beginning with a colon (/:id),
and/or a wildcard beginning with an asterisk (/user/*any)
The view is a function that returns a view. Any component with no props works here, as does a closure that returns some view.

<Routes>
  <Route path="/" view=Home/>
  <Route path="/users" view=Users/>
  <Route path="/users/:id" view=UserProfile/>
  <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
</Routes>
view takes a Fn() -> impl IntoView. If a component has no props, it can be passed directly into the view. In this case, view=Home is just a shorthand for || view! { <Home/> }.

Now if you navigate to / or to /users you’ll get the home page or the <Users/>. If you go to /users/3 or /blahblah you’ll get a user profile or your 404 page (<NotFound/>). On every navigation, the router determines which <Route/> should be matched, and therefore what content should be displayed where the <Routes/> component is defined.

Note that you can define your routes in any order. The router scores each route to see how good a match it is, rather than simply trying to match them top to bottom.

Simple enough?

Conditional Routes
leptos_router is based on the assumption that you have one and only one <Routes/> component in your app. It uses this to generate routes on the server side, optimize route matching by caching calculated branches, and render your application.

You should not conditionally render <Routes/> using another component like <Show/> or <Suspense/>.

// ❌ don't do this!
view! {
<Show when=|| is_loaded() fallback=|| view! { <p>"Loading"</p> }>
<Routes>
<Route path="/" view=Home/>
</Routes>
</Show>
}
Instead, you can use nested routing to render your <Routes/> once, and conditionally render the router outlet:

// ✅ do this instead!
view! {
<Routes>
// parent route
<Route path="/" view=move || {
view! {
// only show the outlet if data have loaded
<Show when=|| is_loaded() fallback=|| view! { <p>"Loading"</p> }>
<Outlet/>
</Show>
}
}>
// nested child route
<Route path="/" view=Home/>
</Route>
</Routes>
}
If this looks bizarre, don’t worry! The next section of the book is about this kind of nested routing.


Nested Routing
We just defined the following set of routes:

<Routes>
  <Route path="/" view=Home/>
  <Route path="/users" view=Users/>
  <Route path="/users/:id" view=UserProfile/>
  <Route path="/*any" view=NotFound/>
</Routes>
There’s a certain amount of duplication here: /users and /users/:id. This is fine for a small app, but you can probably already tell it won’t scale well. Wouldn’t it be nice if we could nest these routes?

Well... you can!

<Routes>
  <Route path="/" view=Home/>
  <Route path="/users" view=Users>
    <Route path=":id" view=UserProfile/>
  </Route>
  <Route path="/*any" view=NotFound/>
</Routes>
But wait. We’ve just subtly changed what our application does.

The next section is one of the most important in this entire routing section of the guide. Read it carefully, and feel free to ask questions if there’s anything you don’t understand.

Nested Routes as Layout
Nested routes are a form of layout, not a method of route definition.

Let me put that another way: The goal of defining nested routes is not primarily to avoid repeating yourself when typing out the paths in your route definitions. It is actually to tell the router to display multiple <Route/>s on the page at the same time, side by side.

Let’s look back at our practical example.

<Routes>
  <Route path="/users" view=Users/>
  <Route path="/users/:id" view=UserProfile/>
</Routes>
This means:

If I go to /users, I get the <Users/> component.
If I go to /users/3, I get the <UserProfile/> component (with the parameter id set to 3; more on that later)
Let’s say I use nested routes instead:

<Routes>
  <Route path="/users" view=Users>
    <Route path=":id" view=UserProfile/>
  </Route>
</Routes>
This means:

If I go to /users/3, the path matches two <Route/>s: <Users/> and <UserProfile/>.
If I go to /users, the path is not matched.
I actually need to add a fallback route

<Routes>
  <Route path="/users" view=Users>
    <Route path=":id" view=UserProfile/>
    <Route path="" view=NoUser/>
  </Route>
</Routes>
Now:

If I go to /users/3, the path matches <Users/> and <UserProfile/>.
If I go to /users, the path matches <Users/> and <NoUser/>.
When I use nested routes, in other words, each path can match multiple routes: each URL can render the views provided by multiple <Route/> components, at the same time, on the same page.

This may be counter-intuitive, but it’s very powerful, for reasons you’ll hopefully see in a few minutes.

Why Nested Routing?
Why bother with this?

Most web applications contain levels of navigation that correspond to different parts of the layout. For example, in an email app you might have a URL like /contacts/greg, which shows a list of contacts on the left of the screen, and contact details for Greg on the right of the screen. The contact list and the contact details should always appear on the screen at the same time. If there’s no contact selected, maybe you want to show a little instructional text.

You can easily define this with nested routes

<Routes>
  <Route path="/contacts" view=ContactList>
    <Route path=":id" view=ContactInfo/>
    <Route path="" view=|| view! {
      <p>"Select a contact to view more info."</p>
    }/>
  </Route>
</Routes>
You can go even deeper. Say you want to have tabs for each contact’s address, email/phone, and your conversations with them. You can add another set of nested routes inside :id:

<Routes>
  <Route path="/contacts" view=ContactList>
    <Route path=":id" view=ContactInfo>
      <Route path="" view=EmailAndPhone/>
      <Route path="address" view=Address/>
      <Route path="messages" view=Messages/>
    </Route>
    <Route path="" view=|| view! {
      <p>"Select a contact to view more info."</p>
    }/>
  </Route>
</Routes>
The main page of the Remix website, a React framework from the creators of React Router, has a great visual example if you scroll down, with three levels of nested routing: Sales > Invoices > an invoice.

<Outlet/>
Parent routes do not automatically render their nested routes. After all, they are just components; they don’t know exactly where they should render their children, and “just stick it at the end of the parent component” is not a great answer.

Instead, you tell a parent component where to render any nested components with an <Outlet/> component. The <Outlet/> simply renders one of two things:

if there is no nested route that has been matched, it shows nothing
if there is a nested route that has been matched, it shows its view
That’s all! But it’s important to know and to remember, because it’s a common source of “Why isn’t this working?” frustration. If you don’t provide an <Outlet/>, the nested route won’t be displayed.

#[component]
pub fn ContactList() -> impl IntoView {
let contacts = todo!();

view! {
<div style="display: flex">
// the contact list
<For each=contacts
key=|contact| contact.id
children=|contact| todo!()
/>
// the nested child, if any
// don’t forget this!
<Outlet/>
</div>
}
}
Refactoring Route Definitions
You don’t need to define all your routes in one place if you don’t want to. You can refactor any <Route/> and its children out into a separate component.

For example, you can refactor the example above to use two separate components:

#[component]
fn App() -> impl IntoView {
view! {
<Router>
<Routes>
<Route path="/contacts" view=ContactList>
<ContactInfoRoutes/>
<Route path="" view=|| view! {
<p>"Select a contact to view more info."</p>
}/>
</Route>
</Routes>
</Router>
}
}

#[component(transparent)]
fn ContactInfoRoutes() -> impl IntoView {
view! {
<Route path=":id" view=ContactInfo>
<Route path="" view=EmailAndPhone/>
<Route path="address" view=Address/>
<Route path="messages" view=Messages/>
</Route>
}
}
This second component is a #[component(transparent)], meaning it just returns its data, not a view: in this case, it's a RouteDefinition struct, which is what the <Route/> returns. As long as it is marked #[component(transparent)], this sub-route can be defined wherever you want, and inserted as a component into your tree of route definitions.

Nested Routing and Performance
All of this is nice, conceptually, but again—what’s the big deal?

Performance.

In a fine-grained reactive library like Leptos, it’s always important to do the least amount of rendering work you can. Because we’re working with real DOM nodes and not diffing a virtual DOM, we want to “rerender” components as infrequently as possible. Nested routing makes this extremely easy.

Imagine my contact list example. If I navigate from Greg to Alice to Bob and back to Greg, the contact information needs to change on each navigation. But the <ContactList/> should never be rerendered. Not only does this save on rendering performance, it also maintains state in the UI. For example, if I have a search bar at the top of <ContactList/>, navigating from Greg to Alice to Bob won’t clear the search.

In fact, in this case, we don’t even need to rerender the <Contact/> component when moving between contacts. The router will just reactively update the :id parameter as we navigate, allowing us to make fine-grained updates. As we navigate between contacts, we’ll update single text nodes to change the contact’s name, address, and so on, without doing any additional rerendering.

This sandbox includes a couple features (like nested routing) discussed in this section and the previous one, and a couple we’ll cover in the rest of this chapter. The router is such an integrated system that it makes sense to provide a single example, so don’t be surprised if there’s anything you don’t understand.


use leptos::*;
use leptos_router::*;

#[component]
fn App() -> impl IntoView {
view! {
<Router>
<h1>"Contact App"</h1>
// this <nav> will show on every routes,
// because it's outside the <Routes/>
// note: we can just use normal <a> tags
// and the router will use client-side navigation
<nav>
<a href="/">"Home"</a>
<a href="/contacts">"Contacts"</a>
</nav>
<main>
<Routes>
// / just has an un-nested "Home"
<Route path="/" view=|| view! {
<h3>"Home"</h3>
}/>
// /contacts has nested routes
<Route
path="/contacts"
view=ContactList
>
// if no id specified, fall back
<Route path=":id" view=ContactInfo>
<Route path="" view=|| view! {
<div class="tab">
"(Contact Info)"
</div>
}/>
<Route path="conversations" view=|| view! {
<div class="tab">
"(Conversations)"
</div>
}/>
</Route>
// if no id specified, fall back
<Route path="" view=|| view! {
<div class="select-user">
"Select a user to view contact info."
</div>
}/>
</Route>
</Routes>
</main>
</Router>
}
}

#[component]
fn ContactList() -> impl IntoView {
view! {
<div class="contact-list">
// here's our contact list component itself
<h3>"Contacts"</h3>
<div class="contact-list-contacts">
<A href="alice">"Alice"</A>
<A href="bob">"Bob"</A>
<A href="steve">"Steve"</A>
</div>

            // <Outlet/> will show the nested child route
            // we can position this outlet wherever we want
            // within the layout
            <Outlet/>
        </div>
    }
}

#[component]
fn ContactInfo() -> impl IntoView {
// we can access the :id param reactively with `use_params_map`
let params = use_params_map();
let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    // imagine we're loading data from an API here
    let name = move || match id().as_str() {
        "alice" => "Alice",
        "bob" => "Bob",
        "steve" => "Steve",
        _ => "User not found.",
    };

    view! {
        <h4>{name}</h4>
        <div class="contact-info">
            <div class="tabs">
                <A href="" exact=true>"Contact Info"</A>
                <A href="conversations">"Conversations"</A>
            </div>

            // <Outlet/> here is the tabs that are nested
            // underneath the /contacts/:id route
            <Outlet/>
        </div>
    }
}

fn main() {
leptos::mount_to_body(App)
}


Params and Queries
Static paths are useful for distinguishing between different pages, but almost every application wants to pass data through the URL at some point.

There are two ways you can do this:

named route params like id in /users/:id
named route queries like q in /search?q=Foo
Because of the way URLs are built, you can access the query from any <Route/> view. You can access route params from the <Route/> that defines them or any of its nested children.

Accessing params and queries is pretty simple with a couple of hooks:

use_query or use_query_map
use_params or use_params_map
Each of these comes with a typed option (use_query and use_params) and an untyped option (use_query_map and use_params_map).

The untyped versions hold a simple key-value map. To use the typed versions, derive the Params trait on a struct.

Params is a very lightweight trait to convert a flat key-value map of strings into a struct by applying FromStr to each field. Because of the flat structure of route params and URL queries, it’s significantly less flexible than something like serde; it also adds much less weight to your binary.

use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq)]
struct ContactParams {
id: usize
}

#[derive(Params, PartialEq)]
struct ContactSearch {
q: String
}
Note: The Params derive macro is located at leptos::Params, and the Params trait is at leptos_router::Params. If you avoid using glob imports like use leptos::*;, make sure you’re importing the right one for the derive macro.

If you are not using the nightly feature, you will get the error

no function or associated item named `into_param` found for struct `std::string::String` in the current scope
At the moment, supporting both T: FromStr and Option<T> for typed params requires a nightly feature. You can fix this by simply changing the struct to use q: Option<String> instead of q: String.

Now we can use them in a component. Imagine a URL that has both params and a query, like /contacts/:id?q=Search.

The typed versions return Memo<Result<T, _>>. It’s a Memo so it reacts to changes in the URL. It’s a Result because the params or query need to be parsed from the URL, and may or may not be valid.

let params = use_params::<ContactParams>();
let query = use_query::<ContactSearch>();

// id: || -> usize
let id = move || {
params.with(|params| {
params.as_ref()
.map(|params| params.id)
.unwrap_or_default()
})
};
The untyped versions return Memo<ParamsMap>. Again, it’s memo to react to changes in the URL. ParamsMap behaves a lot like any other map type, with a .get() method that returns Option<&String>.

let params = use_params_map();
let query = use_query_map();

// id: || -> Option<String>
let id = move || {
params.with(|params| params.get("id").cloned())
};
This can get a little messy: deriving a signal that wraps an Option<_> or Result<_> can involve a couple steps. But it’s worth doing this for two reasons:

It’s correct, i.e., it forces you to consider the cases, “What if the user doesn’t pass a value for this query field? What if they pass an invalid value?”
It’s performant. Specifically, when you navigate between different paths that match the same <Route/> with only params or the query changing, you can get fine-grained updates to different parts of your app without rerendering. For example, navigating between different contacts in our contact-list example does a targeted update to the name field (and eventually contact info) without needing to replace or rerender the wrapping <Contact/>. This is what fine-grained reactivity is for.
This is the same example from the previous section. The router is such an integrated system that it makes sense to provide a single example highlighting multiple features, even if we haven’t explained them all yet.

use leptos::*;
use leptos_router::*;

#[component]
fn App() -> impl IntoView {
view! {
<Router>
<h1>"Contact App"</h1>
// this <nav> will show on every routes,
// because it's outside the <Routes/>
// note: we can just use normal <a> tags
// and the router will use client-side navigation
<nav>
<a href="/">"Home"</a>
<a href="/contacts">"Contacts"</a>
</nav>
<main>
<Routes>
// / just has an un-nested "Home"
<Route path="/" view=|| view! {
<h3>"Home"</h3>
}/>
// /contacts has nested routes
<Route
path="/contacts"
view=ContactList
>
// if no id specified, fall back
<Route path=":id" view=ContactInfo>
<Route path="" view=|| view! {
<div class="tab">
"(Contact Info)"
</div>
}/>
<Route path="conversations" view=|| view! {
<div class="tab">
"(Conversations)"
</div>
}/>
</Route>
// if no id specified, fall back
<Route path="" view=|| view! {
<div class="select-user">
"Select a user to view contact info."
</div>
}/>
</Route>
</Routes>
</main>
</Router>
}
}

#[component]
fn ContactList() -> impl IntoView {
view! {
<div class="contact-list">
// here's our contact list component itself
<h3>"Contacts"</h3>
<div class="contact-list-contacts">
<A href="alice">"Alice"</A>
<A href="bob">"Bob"</A>
<A href="steve">"Steve"</A>
</div>

            // <Outlet/> will show the nested child route
            // we can position this outlet wherever we want
            // within the layout
            <Outlet/>
        </div>
    }
}

#[component]
fn ContactInfo() -> impl IntoView {
// we can access the :id param reactively with `use_params_map`
let params = use_params_map();
let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    // imagine we're loading data from an API here
    let name = move || match id().as_str() {
        "alice" => "Alice",
        "bob" => "Bob",
        "steve" => "Steve",
        _ => "User not found.",
    };

    view! {
        <h4>{name}</h4>
        <div class="contact-info">
            <div class="tabs">
                <A href="" exact=true>"Contact Info"</A>
                <A href="conversations">"Conversations"</A>
            </div>

            // <Outlet/> here is the tabs that are nested
            // underneath the /contacts/:id route
            <Outlet/>
        </div>
    }
}

fn main() {
leptos::mount_to_body(App)
}


The <A/> Component
Client-side navigation works perfectly fine with ordinary HTML <a> elements. The router adds a listener that handles every click on a <a> element and tries to handle it on the client side, i.e., without doing another round trip to the server to request HTML. This is what enables the snappy “single-page app” navigations you’re probably familiar with from most modern web apps.

The router will bail out of handling an <a> click under a number of situations

the click event has had prevent_default() called on it
the Meta, Alt, Ctrl, or Shift keys were held during click
the <a> has a target or download attribute, or rel="external"
the link has a different origin from the current location
In other words, the router will only try to do a client-side navigation when it’s pretty sure it can handle it, and it will upgrade every <a> element to get this special behavior.

This also means that if you need to opt out of client-side routing, you can do so easily. For example, if you have a link to another page on the same domain, but which isn’t part of your Leptos app, you can just use <a rel="external"> to tell the router it isn’t something it can handle.

The router also provides an <A> component, which does two additional things:

Correctly resolves relative nested routes. Relative routing with ordinary <a> tags can be tricky. For example, if you have a route like /post/:id, <A href="1"> will generate the correct relative route, but <a href="1"> likely will not (depending on where it appears in your view.) <A/> resolves routes relative to the path of the nested route within which it appears.
Sets the aria-current attribute to page if this link is the active link (i.e., it’s a link to the page you’re on). This is helpful for accessibility and for styling. For example, if you want to set the link a different color if it’s a link to the page you’re currently on, you can match this attribute with a CSS selector.
Navigating Programmatically
Your most-used methods of navigating between pages should be with <a> and <form> elements or with the enhanced <A/> and <Form/> components. Using links and forms to navigate is the best solution for accessibility and graceful degradation.

On occasion, though, you’ll want to navigate programmatically, i.e., call a function that can navigate to a new page. In that case, you should use the use_navigate function.

let navigate = leptos_router::use_navigate();
navigate("/somewhere", Default::default());
You should almost never do something like <button on:click=move |_| navigate(/* ... */)>. Any on:click that navigates should be an <a>, for reasons of accessibility.

The second argument here is a set of NavigateOptions, which includes options to resolve the navigation relative to the current route as the <A/> component does, replace it in the navigation stack, include some navigation state, and maintain the current scroll state on navigation.

Once again, this is the same example. Check out the relative <A/> components, and take a look at the CSS in index.html to see the ARIA-based styling.

Struct leptos_router::NavigateOptionsCopy item path
source · [−]
pub struct NavigateOptions {
pub resolve: bool,
pub replace: bool,
pub scroll: bool,
pub state: State,
}
Options that can be used to configure a navigation. Used with use_navigate.

Fields
resolve: bool
Whether the URL being navigated to should be resolved relative to the current route.

replace: bool
If true the new location will replace the current route in the history stack, meaning the “back” button will skip over the current route. (Defaults to false).

scroll: bool
If true, the router will scroll to the top of the window at the end of navigation. Defaults to true.

state: State
State that should be pushed onto the history stack during navigation.

Trait Implementations
source
impl Clone for NavigateOptions
source
fn clone(&self) -> NavigateOptions
Returns a copy of the value. Read more
1.0.0 · source
fn clone_from(&mut self, source: &Self)
Performs copy-assignment from source. Read more
source
impl Debug for NavigateOptions
source
fn fmt(&self, f: &mut Formatter<'_>) -> Result
Formats the value using the given formatter. Read more
source
impl Default for NavigateOptions
source
fn default() -> Self
Returns the “default value” for a type. Read more
Auto Trait Implementations
impl Freeze for NavigateOptions
impl RefUnwindSafe for NavigateOptions
impl !Send for NavigateOptions
impl !Sync for NavigateOptions
impl Unpin for NavigateOptions
impl UnwindSafe for NavigateOptions
Blanket Implementations
source§
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
source
impl<El> ElementDescriptorBounds for El
where
El: Debug,

other code

use leptos::*;
use leptos_router::*;

#[component]
fn App() -> impl IntoView {
view! {
<Router>
<h1>"Contact App"</h1>
// this <nav> will show on every routes,
// because it's outside the <Routes/>
// note: we can just use normal <a> tags
// and the router will use client-side navigation
<nav>
<a href="/">"Home"</a>
<a href="/contacts">"Contacts"</a>
</nav>
<main>
<Routes>
// / just has an un-nested "Home"
<Route path="/" view=|| view! {
<h3>"Home"</h3>
}/>
// /contacts has nested routes
<Route
path="/contacts"
view=ContactList
>
// if no id specified, fall back
<Route path=":id" view=ContactInfo>
<Route path="" view=|| view! {
<div class="tab">
"(Contact Info)"
</div>
}/>
<Route path="conversations" view=|| view! {
<div class="tab">
"(Conversations)"
</div>
}/>
</Route>
// if no id specified, fall back
<Route path="" view=|| view! {
<div class="select-user">
"Select a user to view contact info."
</div>
}/>
</Route>
</Routes>
</main>
</Router>
}
}

#[component]
fn ContactList() -> impl IntoView {
view! {
<div class="contact-list">
// here's our contact list component itself
<h3>"Contacts"</h3>
<div class="contact-list-contacts">
<A href="alice">"Alice"</A>
<A href="bob">"Bob"</A>
<A href="steve">"Steve"</A>
</div>

            // <Outlet/> will show the nested child route
            // we can position this outlet wherever we want
            // within the layout
            <Outlet/>
        </div>
    }
}

#[component]
fn ContactInfo() -> impl IntoView {
// we can access the :id param reactively with `use_params_map`
let params = use_params_map();
let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    // imagine we're loading data from an API here
    let name = move || match id().as_str() {
        "alice" => "Alice",
        "bob" => "Bob",
        "steve" => "Steve",
        _ => "User not found.",
    };

    view! {
        <h4>{name}</h4>
        <div class="contact-info">
            <div class="tabs">
                <A href="" exact=true>"Contact Info"</A>
                <A href="conversations">"Conversations"</A>
            </div>

            // <Outlet/> here is the tabs that are nested
            // underneath the /contacts/:id route
            <Outlet/>
        </div>
    }
}

fn main() {
leptos::mount_to_body(App)
}


The <Form/> Component
Links and forms sometimes seem completely unrelated. But, in fact, they work in very similar ways.

In plain HTML, there are three ways to navigate to another page:

An <a> element that links to another page: Navigates to the URL in its href attribute with the GET HTTP method.
A <form method="GET">: Navigates to the URL in its action attribute with the GET HTTP method and the form data from its inputs encoded in the URL query string.
A <form method="POST">: Navigates to the URL in its action attribute with the POST HTTP method and the form data from its inputs encoded in the body of the request.
Since we have a client-side router, we can do client-side link navigations without reloading the page, i.e., without a full round-trip to the server and back. It makes sense that we can do client-side form navigations in the same way.

The router provides a <Form> component, which works like the HTML <form> element, but uses client-side navigations instead of full page reloads. <Form/> works with both GET and POST requests. With method="GET", it will navigate to the URL encoded in the form data. With method="POST" it will make a POST request and handle the server’s response.

<Form/> provides the basis for some components like <ActionForm/> and <MultiActionForm/> that we’ll see in later chapters. But it also enables some powerful patterns of its own.

For example, imagine that you want to create a search field that updates search results in real time as the user searches, without a page reload, but that also stores the search in the URL so a user can copy and paste it to share results with someone else.

It turns out that the patterns we’ve learned so far make this easy to implement.

async fn fetch_results() {
// some async function to fetch our search results
}

#[component]
pub fn FormExample() -> impl IntoView {
// reactive access to URL query strings
let query = use_query_map();
// search stored as ?q=
let search = move || query().get("q").cloned().unwrap_or_default();
// a resource driven by the search string
let search_results = create_resource(search, fetch_results);

    view! {
        <Form method="GET" action="">
            <input type="search" name="q" value=search/>
            <input type="submit"/>
        </Form>
        <Transition fallback=move || ()>
            /* render search results */
        </Transition>
    }
}
Whenever you click Submit, the <Form/> will “navigate” to ?q={search}. But because this navigation is done on the client side, there’s no page flicker or reload. The URL query string changes, which triggers search to update. Because search is the source signal for the search_results resource, this triggers search_results to reload its resource. The <Transition/> continues displaying the current search results until the new ones have loaded. When they are complete, it switches to displaying the new result.

This is a great pattern. The data flow is extremely clear: all data flows from the URL to the resource into the UI. The current state of the application is stored in the URL, which means you can refresh the page or text the link to a friend and it will show exactly what you’re expecting. And once we introduce server rendering, this pattern will prove to be really fault-tolerant, too: because it uses a <form> element and URLs under the hood, it actually works really well without even loading your WASM on the client.

We can actually take it a step further and do something kind of clever:

view! {
<Form method="GET" action="">
<input type="search" name="q" value=search
oninput="this.form.requestSubmit()"
/>
</Form>
}
You’ll notice that this version drops the Submit button. Instead, we add an oninput attribute to the input. Note that this is not on:input, which would listen for the input event and run some Rust code. Without the colon, oninput is the plain HTML attribute. So the string is actually a JavaScript string. this.form gives us the form the input is attached to. requestSubmit() fires the submit event on the <form>, which is caught by <Form/> just as if we had clicked a Submit button. Now the form will “navigate” on every keystroke or input to keep the URL (and therefore the search) perfectly in sync with the user’s input as they type.

use leptos::*;
use leptos_router::*;

#[component]
fn App() -> impl IntoView {
view! {
<Router>
<h1><code>"<Form/>"</code></h1>
<main>
<Routes>
<Route path="" view=FormExample/>
</Routes>
</main>
</Router>
}
}

#[component]
pub fn FormExample() -> impl IntoView {
// reactive access to URL query
let query = use_query_map();
let name = move || query().get("name").cloned().unwrap_or_default();
let number = move || query().get("number").cloned().unwrap_or_default();
let select = move || query().get("select").cloned().unwrap_or_default();

    view! {
        // read out the URL query strings
        <table>
            <tr>
                <td><code>"name"</code></td>
                <td>{name}</td>
            </tr>
            <tr>
                <td><code>"number"</code></td>
                <td>{number}</td>
            </tr>
            <tr>
                <td><code>"select"</code></td>
                <td>{select}</td>
            </tr>
        </table>
        // <Form/> will navigate whenever submitted
        <h2>"Manual Submission"</h2>
        <Form method="GET" action="">
            // input names determine query string key
            <input type="text" name="name" value=name/>
            <input type="number" name="number" value=number/>
            <select name="select">
                // `selected` will set which starts as selected
                <option selected=move || select() == "A">
                    "A"
                </option>
                <option selected=move || select() == "B">
                    "B"
                </option>
                <option selected=move || select() == "C">
                    "C"
                </option>
            </select>
            // submitting should cause a client-side
            // navigation, not a full reload
            <input type="submit"/>
        </Form>
        // This <Form/> uses some JavaScript to submit
        // on every input
        <h2>"Automatic Submission"</h2>
        <Form method="GET" action="">
            <input
                type="text"
                name="name"
                value=name
                // this oninput attribute will cause the
                // form to submit on every input to the field
                oninput="this.form.requestSubmit()"
            />
            <input
                type="number"
                name="number"
                value=number
                oninput="this.form.requestSubmit()"
            />
            <select name="select"
                onchange="this.form.requestSubmit()"
            >
                <option selected=move || select() == "A">
                    "A"
                </option>
                <option selected=move || select() == "B">
                    "B"
                </option>
                <option selected=move || select() == "C">
                    "C"
                </option>
            </select>
            // submitting should cause a client-side
            // navigation, not a full reload
            <input type="submit"/>
        </Form>
    }
}

fn main() {
leptos::mount_to_body(App)
}





