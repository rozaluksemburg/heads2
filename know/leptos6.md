Deployment
There are as many ways to deploy a web application as there are developers, let alone applications. But there are a couple useful tips to keep in mind when deploying an app.

General Advice
Remember: Always deploy Rust apps built in --release mode, not debug mode. This has a huge effect on both performance and binary size.
Test locally in release mode as well. The framework applies certain optimizations in release mode that it does not apply in debug mode, so it’s possible for bugs to surface at this point. (If your app behaves differently or you do encounter a bug, it’s likely a framework-level bug and you should open a GitHub issue with a reproduction.)
See the chapter on "Optimizing WASM Binary Size" for additional tips and tricks to further improve the time-to-interactive metric for your WASM app on first load.
We asked users to submit their deployment setups to help with this chapter.


Deploying a Client-Side-Rendered App
If you’ve been building an app that only uses client-side rendering, working with Trunk as a dev server and build tool, the process is quite easy.

trunk build --release
trunk build will create a number of build artifacts in a dist/ directory. Publishing dist somewhere online should be all you need to deploy your app. This should work very similarly to deploying any JavaScript application.

We've created several example repositories which show how to set up and deploy a Leptos CSR app to various hosting services.

Note: Leptos does not endorse the use of any particular hosting service - feel free to use any service that supports static site deploys.

Examples:

Github Pages
Vercel
Spin (serverless WebAssembly)
Github Pages
Deploying a Leptos CSR app to Github pages is a simple affair. First, go to your Github repo's settings and click on "Pages" in the left side menu. In the "Build and deployment" section of the page, change the "source" to "Github Actions". Then copy the following into a file such as .github/workflows/gh-pages-deploy.yml

Example

name: Release to Github Pages

on:
push:
branches: [main]
workflow_dispatch:

permissions:
contents: write # for committing to gh-pages branch.
pages: write
id-token: write

# Allow only one concurrent deployment, skipping runs queued between the run in-progress and latest queued.
# However, do NOT cancel in-progress runs as we want to allow these production deployments to complete.
concurrency:
group: "pages"
cancel-in-progress: false

jobs:
Github-Pages-Release:

    timeout-minutes: 10

    environment:
    name: github-pages
    url: ${{ steps.deployment.outputs.page_url }}

    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4 # repo checkout

    # Install Rust Nightly Toolchain, with Clippy & Rustfmt
    - name: Install nightly Rust
        uses: dtolnay/rust-toolchain@nightly
        with:
        components: clippy, rustfmt

    - name: Add WASM target
        run: rustup target add wasm32-unknown-unknown

    - name: lint
        run: cargo clippy & cargo fmt


    # If using tailwind...
    # - name: Download and install tailwindcss binary
    #   run: npm install -D tailwindcss && npx tailwindcss -i <INPUT/PATH.css> -o <OUTPUT/PATH.css>  # run tailwind


    - name: Download and install Trunk binary
        run: wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.18.2/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-

    - name: Build with Trunk
        # "${GITHUB_REPOSITORY#*/}" evaluates into the name of the repository
        # using --public-url something will allow trunk to modify all the href paths like from favicon.ico to repo_name/favicon.ico .
        # this is necessary for github pages where the site is deployed to username.github.io/repo_name and all files must be requested
        # relatively as favicon.ico. if we skip public-url option, the href paths will instead request username.github.io/favicon.ico which
        # will obviously return error 404 not found.
        run: ./trunk build --release --public-url "${GITHUB_REPOSITORY#*/}"


    # Deploy to gh-pages branch
    # - name: Deploy 🚀
    #   uses: JamesIves/github-pages-deploy-action@v4
    #   with:
    #     folder: dist


    # Deploy with Github Static Pages

    - name: Setup Pages
        uses: actions/configure-pages@v4
        with:
        enablement: true
        # token:

    - name: Upload artifact
        uses: actions/upload-pages-artifact@v2
        with:
        # Upload dist dir
        path: './dist'

    - name: Deploy to GitHub Pages 🚀
        id: deployment
        uses: actions/deploy-pages@v3
For more on deploying to Github Pages see the example repo here

Vercel
Step 1: Set Up Vercel
In the Vercel Web UI...

Create a new project
Ensure
The "Build Command" is left empty with Override on
The "Output Directory" is changed to dist (which is the default output directory for Trunk builds) and the Override is on

Step 2: Add Vercel Credentials for GitHub Actions
Note: Both the preview and deploy actions will need your Vercel credentials setup in GitHub secrets

Retrieve your Vercel Access Token by going to "Account Settings" > "Tokens" and creating a new token - save the token to use in sub-step 5, below.

Install the Vercel CLI using the npm i -g vercel command, then run vercel login to login to your acccount.

Inside your folder, run vercel link to create a new Vercel project; in the CLI, you will be asked to 'Link to an existing project?' - answer yes, then enter the name you created in step 1. A new .vercel folder will be created for you.

Inside the generated .vercel folder, open the the project.json file and save the "projectId" and "orgId" for the next step.

Inside GitHub, go the repo's "Settings" > "Secrets and Variables" > "Actions" and add the following as Repository secrets:

save your Vercel Access Token (from sub-step 1) as the VERCEL_TOKEN secret
from the .vercel/project.json add "projectID" as VERCEL_PROJECT_ID
from the .vercel/project.json add "orgId" as VERCEL_ORG_ID
For full instructions see "How can I use Github Actions with Vercel"

Step 3: Add Github Action Scripts
Finally, you're ready to simply copy and paste the two files - one for deployment, one for PR previews - from below or from the example repo's .github/workflows/ folder into your own github workflows folder - then, on your next commit or PR deploys will occur automatically.

Production deployment script: vercel_deploy.yml

Example

name: Release to Vercel

on:
push:
branches:
- main
env:
CARGO_TERM_COLOR: always
VERCEL_ORG_ID: ${{ secrets.VERCEL_ORG_ID }}
VERCEL_PROJECT_ID: ${{ secrets.VERCEL_PROJECT_ID }}

jobs:
Vercel-Production-Deployment:
runs-on: ubuntu-latest
environment: production
steps:
- name: git-checkout
uses: actions/checkout@v3

    - uses: dtolnay/rust-toolchain@nightly
        with:
        components: clippy, rustfmt
    - uses: Swatinem/rust-cache@v2
    - name: Setup Rust
        run: |
        rustup target add wasm32-unknown-unknown
        cargo clippy
        cargo fmt --check

    - name: Download and install Trunk binary
        run: wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.18.2/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-


    - name: Build with Trunk
        run: ./trunk build --release

    - name: Install Vercel CLI
        run: npm install --global vercel@latest

    - name: Pull Vercel Environment Information
        run: vercel pull --yes --environment=production --token=${{ secrets.VERCEL_TOKEN }}

    - name: Deploy to Vercel & Display URL
        id: deployment
        working-directory: ./dist
        run: |
        vercel deploy --prod --token=${{ secrets.VERCEL_TOKEN }} >> $GITHUB_STEP_SUMMARY
        echo $GITHUB_STEP_SUMMARY
Preview deployments script: vercel_preview.yml

Example

# For more info re: vercel action see:
# https://github.com/amondnet/vercel-action

name: Leptos CSR Vercel Preview

on:
pull_request:
branches: [ "main" ]

workflow_dispatch:

env:
CARGO_TERM_COLOR: always
VERCEL_ORG_ID: ${{ secrets.VERCEL_ORG_ID }}
VERCEL_PROJECT_ID: ${{ secrets.VERCEL_PROJECT_ID }}

jobs:
fmt:
name: Rustfmt
runs-on: ubuntu-latest
steps:
- uses: actions/checkout@v4
- uses: dtolnay/rust-toolchain@nightly
with:
components: rustfmt
- name: Enforce formatting
run: cargo fmt --check

clippy:
name: Clippy
runs-on: ubuntu-latest
steps:
- uses: actions/checkout@v4
- uses: dtolnay/rust-toolchain@nightly
with:
components: clippy
- uses: Swatinem/rust-cache@v2
- name: Linting
run: cargo clippy -- -D warnings

test:
name: Test
runs-on: ubuntu-latest
needs: [fmt, clippy]
steps:
- uses: actions/checkout@v4
- uses: dtolnay/rust-toolchain@nightly
- uses: Swatinem/rust-cache@v2
- name: Run tests
run: cargo test

build-and-preview-deploy:
runs-on: ubuntu-latest
name: Build and Preview

    needs: [test, clippy, fmt]

    permissions:
    pull-requests: write

    environment:
    name: preview
    url: ${{ steps.preview.outputs.preview-url }}

    steps:
    - name: git-checkout
        uses: actions/checkout@v4

    - uses: dtolnay/rust-toolchain@nightly
    - uses: Swatinem/rust-cache@v2
    - name: Build
        run: rustup target add wasm32-unknown-unknown

    - name: Download and install Trunk binary
        run: wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.18.2/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-


    - name: Build with Trunk
        run: ./trunk build --release

    - name: Preview Deploy
        id: preview
        uses: amondnet/vercel-action@v25.1.1
        with:
        vercel-token: ${{ secrets.VERCEL_TOKEN }}
        github-token: ${{ secrets.GITHUB_TOKEN }}
        vercel-org-id: ${{ secrets.VERCEL_ORG_ID }}
        vercel-project-id: ${{ secrets.VERCEL_PROJECT_ID }}
        github-comment: true
        working-directory: ./dist

    - name: Display Deployed URL
        run: |
        echo "Deployed app URL: ${{ steps.preview.outputs.preview-url }}" >> $GITHUB_STEP_SUMMARY
See the example repo here for more.

Spin - Serverless WebAssembly
Another option is using a serverless platform such as Spin. Although Spin is open source and you can run it on your own infrastructure (eg. inside Kubernetes), the easiest way to get started with Spin in production is to use the Fermyon Cloud.

Start by installing the Spin CLI using the instructions, here, and creating a Github repo for your Leptos CSR project, if you haven't done so already.

Open "Fermyon Cloud" > "User Settings". If you’re not logged in, choose the Login With GitHub button.

In the “Personal Access Tokens”, choose “Add a Token”. Enter the name “gh_actions” and click “Create Token”.

Fermyon Cloud displays the token; click the copy button to copy it to your clipboard.

Go into your Github repo and open "Settings" > "Secrets and Variables" > "Actions" and add the Fermyon cloud token to "Repository secrets" using the variable name "FERMYON_CLOUD_TOKEN"

Copy and paste the following Github Actions scripts (below) into your .github/workflows/<SCRIPT_NAME>.yml files

With the 'preview' and 'deploy' scripts active, Github Actions will now generate previews on pull requests & deploy automatically on updates to your 'main' branch.

Production deployment script: spin_deploy.yml

Example

# For setup instructions needed for Fermyon Cloud, see:
# https://developer.fermyon.com/cloud/github-actions

# For reference, see:
# https://developer.fermyon.com/cloud/changelog/gh-actions-spin-deploy

# For the Fermyon gh actions themselves, see:
# https://github.com/fermyon/actions

name: Release to Spin Cloud

on:
push:
branches: [main]
workflow_dispatch:

permissions:
contents: read
id-token: write

# Allow only one concurrent deployment, skipping runs queued between the run in-progress and latest queued.
# However, do NOT cancel in-progress runs as we want to allow these production deployments to complete.
concurrency:
group: "spin"
cancel-in-progress: false

jobs:
Spin-Release:

    timeout-minutes: 10

    environment:
    name: production
    url: ${{ steps.deployment.outputs.app-url }}

    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4 # repo checkout

    # Install Rust Nightly Toolchain, with Clippy & Rustfmt
    - name: Install nightly Rust
        uses: dtolnay/rust-toolchain@nightly
        with:
        components: clippy, rustfmt

    - name: Add WASM & WASI targets
        run: rustup target add wasm32-unknown-unknown && rustup target add wasm32-wasi

    - name: lint
        run: cargo clippy & cargo fmt


    # If using tailwind...
    # - name: Download and install tailwindcss binary
    #   run: npm install -D tailwindcss && npx tailwindcss -i <INPUT/PATH.css> -o <OUTPUT/PATH.css>  # run tailwind


    - name: Download and install Trunk binary
        run: wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.18.2/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-


    - name: Build with Trunk
        run: ./trunk build --release


    # Install Spin CLI & Deploy

    - name: Setup Spin
        uses: fermyon/actions/spin/setup@v1
        # with:
        # plugins:


    - name: Build and deploy
        id: deployment
        uses: fermyon/actions/spin/deploy@v1
        with:
        fermyon_token: ${{ secrets.FERMYON_CLOUD_TOKEN }}
        # key_values: |-
            # abc=xyz
            # foo=bar
        # variables: |-
            # password=${{ secrets.SECURE_PASSWORD }}
            # apikey=${{ secrets.API_KEY }}

    # Create an explicit message to display the URL of the deployed app, as well as in the job graph
    - name: Deployed URL
        run: |
        echo "Deployed app URL: ${{ steps.deployment.outputs.app-url }}" >> $GITHUB_STEP_SUMMARY
Preview deployment script: spin_preview.yml

Example

# For setup instructions needed for Fermyon Cloud, see:
# https://developer.fermyon.com/cloud/github-actions


# For the Fermyon gh actions themselves, see:
# https://github.com/fermyon/actions

# Specifically:
# https://github.com/fermyon/actions?tab=readme-ov-file#deploy-preview-of-spin-app-to-fermyon-cloud---fermyonactionsspinpreviewv1

name: Preview on Spin Cloud

on:
pull_request:
branches: ["main", "v*"]
types: ['opened', 'synchronize', 'reopened', 'closed']
workflow_dispatch:

permissions:
contents: read
pull-requests: write

# Allow only one concurrent deployment, skipping runs queued between the run in-progress and latest queued.
# However, do NOT cancel in-progress runs as we want to allow these production deployments to complete.
concurrency:
group: "spin"
cancel-in-progress: false

jobs:
Spin-Preview:

    timeout-minutes: 10

    environment:
    name: preview
    url: ${{ steps.preview.outputs.app-url }}

    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4 # repo checkout

    # Install Rust Nightly Toolchain, with Clippy & Rustfmt
    - name: Install nightly Rust
        uses: dtolnay/rust-toolchain@nightly
        with:
        components: clippy, rustfmt

    - name: Add WASM & WASI targets
        run: rustup target add wasm32-unknown-unknown && rustup target add wasm32-wasi

    - name: lint
        run: cargo clippy & cargo fmt


    # If using tailwind...
    # - name: Download and install tailwindcss binary
    #   run: npm install -D tailwindcss && npx tailwindcss -i <INPUT/PATH.css> -o <OUTPUT/PATH.css>  # run tailwind


    - name: Download and install Trunk binary
        run: wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.18.2/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-


    - name: Build with Trunk
        run: ./trunk build --release


    # Install Spin CLI & Deploy

    - name: Setup Spin
        uses: fermyon/actions/spin/setup@v1
        # with:
        # plugins:


    - name: Build and preview
        id: preview
        uses: fermyon/actions/spin/preview@v1
        with:
        fermyon_token: ${{ secrets.FERMYON_CLOUD_TOKEN }}
        github_token: ${{ secrets.GITHUB_TOKEN }}
        undeploy: ${{ github.event.pull_request && github.event.action == 'closed' }}
        # key_values: |-
            # abc=xyz
            # foo=bar
        # variables: |-
            # password=${{ secrets.SECURE_PASSWORD }}
            # apikey=${{ secrets.API_KEY }}


    - name: Display Deployed URL
        run: |
        echo "Deployed app URL: ${{ steps.preview.outputs.app-url }}" >> $GITHUB_STEP_SUMMARY


Deploying a Full-Stack SSR App
It's possible to deploy Leptos fullstack, SSR apps to any number of server or container hosting services. The most simple way to get a Leptos SSR app into production might be to use a VPS service and either run Leptos natively in a VM (see here for more details). Alternatively, you could containerize your Leptos app and run it in Podman or Docker on any colocated or cloud server.

There are a multitude of different deployment setups and hosting services, and in general, Leptos itself is agnostic to the deployment setup you use. With this diversity of deployment targets in mind, on this page we will go over:

creating a Containerfile (or Dockerfile) for use with Leptos SSR apps
Using a Dockerfile to deploy to a cloud service - for example, Fly.io
Deploying Leptos to serverless runtimes - for example, AWS Lambda and JS-hosted WASM runtimes like Deno & Cloudflare
Platforms that have not yet gained Leptos SSR support
Note: Leptos does not endorse the use of any particular method of deployment or hosting service.

Creating a Containerfile
The most popular way for people to deploy full-stack apps built with cargo-leptos is to use a cloud hosting service that supports deployment via a Podman or Docker build. Here’s a sample Containerfile / Dockerfile, which is based on the one we use to deploy the Leptos website.

Debian
# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bullseye as builder

# If you’re using stable, use this instead
# FROM rust:1.74-bullseye as builder

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Build the app
RUN cargo leptos build --release -vv

FROM debian:bookworm-slim as runtime
WORKDIR /app
RUN apt-get update -y \
&& apt-get install -y --no-install-recommends openssl ca-certificates \
&& apt-get autoremove -y \
&& apt-get clean -y \
&& rm -rf /var/lib/apt/lists/*

# -- NB: update binary name from "leptos_start" to match your app name in Cargo.toml --
# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/leptos_start /app/

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site

# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

# -- NB: update binary name from "leptos_start" to match your app name in Cargo.toml --
# Run the server
CMD ["/app/leptos_start"]
Alpine
# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine as builder

RUN apk update && \
apk add --no-cache bash curl npm libc-dev binaryen

RUN npm install -g sass

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-alpine as runner

WORKDIR /app

COPY --from=builder /work/target/release/leptos_start /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

EXPOSE $PORT
ENV LEPTOS_SITE_ROOT=./site

CMD ["/app/leptos_start"]
Read more: gnu and musl build files for Leptos apps.

Cloud Deployments
Deploy to Fly.io
One option for deploying your Leptos SSR app is to use a service like Fly.io, which takes a Dockerfile definition of your Leptos app and runs it in a quick-starting micro-VM; Fly also offers a variety of storage options and managed DBs to use with your projects. The following example will show how to deploy a simple Leptos starter app, just to get you up and going; see here for more about working with storage options on Fly.io if and when required.

First, create a Dockerfile in the root of your application and fill it in with the suggested contents (above); make sure to update the binary names in the Dockerfile example to the name of your own application, and make other adjustments as necessary.

Also, ensure you have the flyctl CLI tool installed, and have an account set up at Fly.io. To install flyctl on MacOS, Linux, or Windows WSL, run:

curl -L https://fly.io/install.sh | sh
If you have issues, or for installing to other platforms see the full instructions here

Then login to Fly.io

fly auth login
and manually launch your app using the command

fly launch
The flyctl CLI tool will walk you through the process of deploying your app to Fly.io.

Note

By default, Fly.io will auto-stop machines that don't have traffic coming to them after a certain period of time. Although Fly.io's lightweight VM's start up quickly, if you want to minimize the latency of your Leptos app and ensure it's always swift to respond, go into the generated fly.toml file and change the min_machines_running to 1 from the default of 0.

See this page in the Fly.io docs for more details.

If you would prefer to use Github Actions to manage your deployments, you will need to create a new access token via the Fly.io web UI.

Go to "Account" > "Access Tokens" and create a token named something like "github_actions", then add the token to your Github repo's secrets by going into your project's Github repo, then clicking "Settings" > "Secrets and Variables" > "Actions" and creating a "New repository secret" with the name "FLY_API_TOKEN".

To generate a fly.toml config file for deployment to Fly.io, you must first run the following from within the project source directory

fly launch --no-deploy
to create a new Fly app and register it with the service. Git commit your new fly.toml file.

To set up the Github Actions deployment workflow, copy the following into a .github/workflows/fly_deploy.yml file:

Example

# For more details, see: https://fly.io/docs/app-guides/continuous-deployment-with-github-actions/

name: Deploy to Fly.io
on:
push:
branches:
- main
jobs:
deploy:
name: Deploy app
runs-on: ubuntu-latest
steps:
- uses: actions/checkout@v4
- uses: superfly/flyctl-actions/setup-flyctl@master
- name: Deploy to fly
id: deployment
run: |
flyctl deploy --remote-only | tail -n 1 >> $GITHUB_STEP_SUMMARY
env:
FLY_API_TOKEN: ${{ secrets.FLY_API_TOKEN }}
On the next commit to your Github main branch, your project will automatically deploy to Fly.io.

See the example repo here.

Railway
Another provider for cloud deployments is Railway. Railway integrates with GitHub to automatically deploy your code.

There is an opinionated community template that gets you started quickly:

Deploy on Railway

The template has renovate setup to keep dependencies up to date and supports GitHub Actions to test your code before a deploy happens.

Railway has a free tier that does not require a credit card, and with how little resources Leptos needs that free tier should last a long time.

See the example repo here.

Deploy to Serverless Runtimes
Leptos supports deploying to FaaS (Function as a Service) or 'serverless' runtimes such as AWS Lambda as well as WinterCG-compatible JS runtimes such as Deno and Cloudflare. Just be aware that serverless environments do place some restrictions on the functionality available to your SSR app when compared with VM or container type deployments (see notes, below).

AWS Lambda
With a little help from the Cargo Lambda tool, Leptos SSR apps can be deployed to AWS Lambda. A starter template repo using Axum as the server is available at leptos-rs/start-aws; the instructions there can be adapted for you to use a Leptos+Actix-web server as well. The starter repo includes a Github Actions script for CI/CD, as well as instructions for setting up your Lambda functions and getting the necessary credentials for cloud deployment.

However, please keep in mind that some native server functionality does not work with FaaS services like Lambda because the environment is not necessarily consistent from one request to the next. In particular, the 'start-aws' docs state that "since AWS Lambda is a serverless platform, you'll need to be more careful about how you manage long-lived state. Writing to disk or using a state extractor will not work reliably across requests. Instead, you'll need a database or other microservices that you can query from the Lambda function."

The other factor to bear in mind is the 'cold-start' time for functions as a service - depending on your use case and the FaaS platform you use, this may or may not meet your latency requirements; you may need to keep one function running at all times to optimize the speed of your requests.

Deno & Cloudflare Workers
Currently, Leptos-Axum supports running in Javascript-hosted WebAssembly runtimes such as Deno, Cloudflare Workers, etc. This option requires some changes to the setup of your source code (for example, in Cargo.toml you must define your app using crate-type = ["cdylib"] and the "wasm" feature must be enabled for leptos_axum). The Leptos HackerNews JS-fetch example demonstrates the required modifications and shows how to run an app in the Deno runtime. Additionally, the leptos_axum crate docs are a helpful reference when setting up your own Cargo.toml file for JS-hosted WASM runtimes.

While the initial setup for JS-hosted WASM runtimes is not onerous, the more important restriction to keep in mind is that since your app will be compiled to WebAssembly (wasm32-unknown-unknown) on the server as well as the client, you must ensure that the crates you use in your app are all WASM-compatible; this may or may not be a deal-breaker depending on your app's requirements, as not all crates in the Rust ecosystem have WASM support.

If you're willing to live with the limitations of WASM server-side, the best place to get started right now is by checking out the example of running Leptos with Deno in the official Leptos Github repo.

Platforms Working on Leptos Support
Deploy to Spin Serverless WASI (with Leptos SSR)
WebAssembly on the server has been gaining steam lately, and the developers of the open source serverless WebAssembly framework Spin are working on natively supporting Leptos. While the Leptos-Spin SSR integration is still in its early stages, there is a working example you may wish to try out.

The full set of instructions to get Leptos SSR & Spin working together are available as a post on the Fermyon blog, or if you want to skip the article and just start playing around with a working starter repo, see here.

Deploy to Shuttle.rs
Several Leptos users have asked about the possibility of using the Rust-friendly Shuttle.rs service to deploy Leptos apps. Unfortunately, Leptos is not officially supported by the Shuttle.rs service at the moment.

However, the folks at Shuttle.rs are committed to getting Leptos support in the future; if you would like to keep up-to-date on the status of that work, keep an eye on this Github issue.

Additionally, some effort has been made to get Shuttle working with Leptos, but to date, deploys to the Shuttle cloud are still not working as expected. That work is available here, if you would like to investigate for yourself or contribute fixes: Leptos Axum Starter Template for Shuttle.rs.

Fly Machines are fast to start and stop, and you don’t pay for their CPU and RAM when they’re in the stopped state. For Fly Apps with a service configured, Fly Proxy can automatically start and stop existing Machines based on incoming requests, so that your app can meet demand without keeping extra Machines running. And if your app needs to have one or more Machines always running in your primary region, then you can set a minimum number of machines to keep running.

The auto start and stop feature also plays well with apps whose Machines exit from within when idle. If your app already shuts down when idle, then the proxy can restart it when there’s traffic.

You might also be interested in learning about scaling the number of machines.

Configure automatic start and stop
The auto start and stop settings apply per service, so you set them within the [[services]] or [http_service] sections of fly.toml:

For example:

Wrap textCopy to clipboard
...
[[services]]
internal_port = 8080
protocol = "tcp"
auto_stop_machines = true
auto_start_machines = true
min_machines_running = 0
...
Briefly, the settings are:

auto_start_machines: Whether Fly Proxy should automatically start Machines based on requests and capacity.
auto_stop_machines: Whether Fly Proxy should automatically stop Machines when the app is idle for several minutes.
min_machines_running: The minimum number of Machines to keep running in the primary region when auto_stop_machines = true.
Concurrency limits for services affect how automatic starts and stops work.

Default values
The default settings for apps that don’t specify any auto start and stop settings in fly.toml are auto_start_machines = true and auto_stop_machines = false.

When you create an app using the fly launch command, the default settings in fly.toml are:

Wrap textCopy to clipboard
...
[http_service]
internal_port = 8080
force_https = true
auto_stop_machines = true
auto_start_machines = true
min_machines_running = 0
...
Recommended settings
If your app already exits when idle, then you can set auto_start_machines = true and auto_stop_machines = false to have Fly Proxy automatically restart the Machines stopped by your app.

If your app doesn’t exit when idle, then we recommend setting auto_stop_machines and auto_start_machines to the same value (either both true or both false) so that Fly Proxy doesn’t stop Machines and never restart them. For example, if auto_start_machines = false and auto_stop_machines = true, then Fly Proxy automatically stops your Machines when there’s low traffic but doesn’t start them again. When all or most of your Machines stop, requests to your app could start failing.

To keep one or more Machines running all the time in your primary region, set min_machines_running to 1 or higher. min_machines_running has no effect unless you set auto_stop_machines = true.

min_machines_running does not apply to Machines running in non-primary regions. For example, if min_machines_running = 1 and there’s no traffic to your app, then Fly Proxy will stop Machines until eventually there is only one Machine running in your primary region.

There’s no “maximum machines running” setting, because the maximum number of Machines is just the total number of Machines you’ve created for your app. Learn more in the How it works section.

How it works
The Fly Proxy runs a process to automatically stop and start existing Fly Machines every few minutes.

The automatic start and stop feature only works on existing Machines and never creates or destroys Machines for you. The maximum number of running Machines is the number of Machines you’ve created for your app using fly scale count or fly machine clone. Learn more about scaling the number of Machines.

Fly Proxy stops Machines
When auto_stop_machines = true in your fly.toml, the proxy looks at Machines running in a single region and uses the concurrency soft_limit setting for each Machine to determine if there’s excess capacity. If the proxy decides there’s excess capacity, it stops exactly one Machine. The proxy repeats this process every few minutes, stopping only one Machine per region, if needed, each time.

If you have the kill_signal and kill_timeout options configured in your fly.toml file, then Fly Proxy uses those settings when it stops a Machine.

Fly Proxy determines excess capacity per region as follows:

If there’s more than one Machine in the region:
the proxy determines how many running Machines are over their soft_limit setting and then calculates excess capacity: excess capacity = num of machines - (num machines over soft limit + 1)
if excess capacity is 1 or greater, then the proxy stops one Machine
If there’s only one Machine in the region:
the proxy checks if the Machine has any traffic
if the Machine has no traffic (a load of 0), then the proxy stops the Machine
Fly Proxy starts Machines
When auto_start_machines = true in your fly.toml, the Fly Proxy restarts a Machine in the nearest region when required.

Fly Proxy determines when to start a Machine as follows:

The proxy waits for a request to your app.
If all the running Machines are above their soft_limit setting, then the proxy starts a stopped Machine in the nearest region (if there are any stopped Machines).
The proxy routes the request to the newly started Machine.
When to stop and start Fly Machines automatically, or not
If your app has highly variable request workloads, then you can set auto_stop_machines and auto_start_machines to true to manage your Fly Machines as demand decreases and increases. This could reduce costs, because you’ll never have to run excess Machines to handle peak load; you’ll only run, and get charged for, the number of Machines that you need.

The difference between this feature and what’s typical in autoscaling, is that it doesn’t create new Machines up to a specified maximum. It automatically starts only existing Machines. For example, if you want to have a maximum of 10 Machines available to service requests, then you need to create 10 Machines for your app.

If you need all your app’s Machines to run continuously, then you can set auto_stop_machines and auto_start_machines to false.

If you only need a certain number of your app’s Machines to run continuously, then you can set auto_stop_machines = true and min_machines_running to 1 or higher.

Stop a Machine by terminating its main process
Setting your app to automatically stop when there’s excess capacity using auto_stop_machines = true is a substitute for when your app doesn’t shut itself down automatically after a period of inactivity. If you want a custom shutdown process for your app, then you can code your app to exit from within when idle.


Optimizing WASM Binary Size
One of the primary downsides of deploying a Rust/WebAssembly frontend app is that splitting a WASM file into smaller chunks to be dynamically loaded is significantly more difficult than splitting a JavaScript bundle. There have been experiments like wasm-split in the Emscripten ecosystem but at present there’s no way to split and dynamically load a Rust/wasm-bindgen binary. This means that the whole WASM binary needs to be loaded before your app becomes interactive. Because the WASM format is designed for streaming compilation, WASM files are much faster to compile per kilobyte than JavaScript files. (For a deeper look, you can read this great article from the Mozilla team on streaming WASM compilation.)

Still, it’s important to ship the smallest WASM binary to users that you can, as it will reduce their network usage and make your app interactive as quickly as possible.

So what are some practical steps?

Things to Do
Make sure you’re looking at a release build. (Debug builds are much, much larger.)
Add a release profile for WASM that optimizes for size, not speed.
For a cargo-leptos project, for example, you can add this to your Cargo.toml:

[profile.wasm-release]
inherits = "release"
opt-level = 'z'
lto = true
codegen-units = 1

# ....

[package.metadata.leptos]
# ....
lib-profile-release = "wasm-release"
This will hyper-optimize the WASM for your release build for size, while keeping your server build optimized for speed. (For a pure client-rendered app without server considerations, just use the [profile.wasm-release] block as your [profile.release].)

Always serve compressed WASM in production. WASM tends to compress very well, typically shrinking to less than 50% its uncompressed size, and it’s trivial to enable compression for static files being served from Actix or Axum.

If you’re using nightly Rust, you can rebuild the standard library with this same profile rather than the prebuilt standard library that’s distributed with the wasm32-unknown-unknown target.

To do this, create a file in your project at .cargo/config.toml

[unstable]
build-std = ["std", "panic_abort", "core", "alloc"]
build-std-features = ["panic_immediate_abort"]
Note that if you're using this with SSR too, the same Cargo profile will be applied. You'll need to explicitly specify your target:

[build]
target = "x86_64-unknown-linux-gnu" # or whatever
Also note that in some cases, the cfg feature has_std will not be set, which may cause build errors with some dependencies which check for has_std. You may fix any build errors due to this by adding:

[build]
rustflags = ["--cfg=has_std"]
And you'll need to add panic = "abort" to [profile.release] in Cargo.toml. Note that this applies the same build-std and panic settings to your server binary, which may not be desirable. Some further exploration is probably needed here.

One of the sources of binary size in WASM binaries can be serde serialization/deserialization code. Leptos uses serde by default to serialize and deserialize resources created with create_resource. You might try experimenting with the miniserde and serde-lite features, which allow you to use those crates for serialization and deserialization instead; each only implements a subset of serde’s functionality, but typically optimizes for size over speed.
Things to Avoid
There are certain crates that tend to inflate binary sizes. For example, the regex crate with its default features adds about 500kb to a WASM binary (largely because it has to pull in Unicode table data!). In a size-conscious setting, you might consider avoiding regexes in general, or even dropping down and calling browser APIs to use the built-in regex engine instead. (This is what leptos_router does on the few occasions it needs a regular expression.)

In general, Rust’s commitment to runtime performance is sometimes at odds with a commitment to a small binary. For example, Rust monomorphizes generic functions, meaning it creates a distinct copy of the function for each generic type it’s called with. This is significantly faster than dynamic dispatch, but increases binary size. Leptos tries to balance runtime performance with binary size considerations pretty carefully; but you might find that writing code that uses many generics tends to increase binary size. For example, if you have a generic component with a lot of code in its body and call it with four different types, remember that the compiler could include four copies of that same code. Refactoring to use a concrete inner function or helper can often maintain performance and ergonomics while reducing binary size.

A Final Thought
Remember that in a server-rendered app, JS bundle size/WASM binary size affects only one thing: time to interactivity on the first load. This is very important to a good user experience: nobody wants to click a button three times and have it do nothing because the interactive code is still loading — but it's not the only important measure.

It’s especially worth remembering that streaming in a single WASM binary means all subsequent navigations are nearly instantaneous, depending only on any additional data loading. Precisely because your WASM binary is not bundle split, navigating to a new route does not require loading additional JS/WASM, as it does in nearly every JavaScript framework. Is this copium? Maybe. Or maybe it’s just an honest trade-off between the two approaches!

Always take the opportunity to optimize the low-hanging fruit in your application. And always test your app under real circumstances with real user network speeds and devices before making any heroic efforts.


Guide: Islands
Leptos 0.5 introduces the new experimental-islands feature. This guide will walk through the islands feature and core concepts, while implementing a demo app using the islands architecture.

The Islands Architecture
The dominant JavaScript frontend frameworks (React, Vue, Svelte, Solid, Angular) all originated as frameworks for building client-rendered single-page apps (SPAs). The initial page load is rendered to HTML, then hydrated, and subsequent navigations are handled directly in the client. (Hence “single page”: everything happens from a single page load from the server, even if there is client-side routing later.) Each of these frameworks later added server-side rendering to improve initial load times, SEO, and user experience.

This means that by default, the entire app is interactive. It also means that the entire app has to be shipped to the client as JavaScript in order to be hydrated. Leptos has followed this same pattern.

You can read more in the chapters on server-side rendering.

But it’s also possible to work in the opposite direction. Rather than taking an entirely-interactive app, rendering it to HTML on the server, and then hydrating it in the browser, you can begin with a plain HTML page and add small areas of interactivity. This is the traditional format for any website or app before the 2010s: your browser makes a series of requests to the server and returns the HTML for each new page in response. After the rise of “single-page apps” (SPA), this approach has sometimes become known as a “multi-page app” (MPA) by comparison.

The phrase “islands architecture” has emerged recently to describe the approach of beginning with a “sea” of server-rendered HTML pages, and adding “islands” of interactivity throughout the page.

Additional Reading
The rest of this guide will look at how to use islands with Leptos. For more background on the approach in general, check out some of the articles below:

Jason Miller, “Islands Architecture”, Jason Miller
Ryan Carniato, “Islands & Server Components & Resumability, Oh My!”
“Islands Architectures” on patterns.dev
Astro Islands
Activating Islands Mode
Let’s start with a fresh cargo-leptos app:

cargo leptos new --git leptos-rs/start
I’m using Actix because I like it. Feel free to use Axum; there should be approximately no server-specific differences in this guide.

I’m just going to run

cargo leptos build
in the background while I fire up my editor and keep writing.

The first thing I’ll do is to add the experimental-islands feature in my Cargo.toml. I need to add this to both leptos and leptos_actix:

leptos = { version = "0.5", features = ["nightly", "experimental-islands"] }
leptos_actix = { version = "0.5", optional = true, features = [
"experimental-islands",
] }
Next I’m going to modify the hydrate function exported from src/lib.rs. I’m going to remove the line that calls leptos::mount_to_body(App) and replace it with

leptos::leptos_dom::HydrationCtx::stop_hydrating();
Each “island” we create will actually act as its own entrypoint, so our hydrate() function just says “okay, hydration’s done now.”

Okay, now fire up your cargo leptos watch and go to http://localhost:3000 (or wherever).

Click the button, and...

Nothing happens!

Perfect.

Note

The starter templates include use app::*; in their hydrate() function definitions. Once you've switched over to islands mode, you are no longer using the imported main App function, so you might think you can delete this. (And in fact, Rust lint tools might issue warnings if you don't!)

However, this can cause issues if you are using a workspace setup. We use wasm-bindgen to independently export an entrypoint for each function. In my experience, if you are using a workspace setup and nothing in your frontend crate actually uses the app crate, those bindings will not be generated correctly. See this discussion for more.

Using Islands
Nothing happens because we’ve just totally inverted the mental model of our app. Rather than being interactive by default and hydrating everything, the app is now plain HTML by default, and we need to opt into interactivity.

This has a big effect on WASM binary sizes: if I compile in release mode, this app is a measly 24kb of WASM (uncompressed), compared to 355kb in non-islands mode. (355kb is quite large for a “Hello, world!” It’s really just all the code related to client-side routing, which isn’t being used in the demo.)

When we click the button, nothing happens, because our whole page is static.

So how do we make something happen?

Let’s turn the HomePage component into an island!

Here was the non-interactive version:

#[component]
fn HomePage() -> impl IntoView {
// Creates a reactive value to update the button
let (count, set_count) = create_signal(0);
let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
Here’s the interactive version:

#[island]
fn HomePage() -> impl IntoView {
// Creates a reactive value to update the button
let (count, set_count) = create_signal(0);
let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
Now when I click the button, it works!

The #[island] macro works exactly like the #[component] macro, except that in islands mode, it designates this as an interactive island. If we check the binary size again, this is 166kb uncompressed in release mode; much larger than the 24kb totally static version, but much smaller than the 355kb fully-hydrated version.

If you open up the source for the page now, you’ll see that your HomePage island has been rendered as a special <leptos-island> HTML element which specifies which component should be used to hydrate it:

<leptos-island data-component="HomePage" data-hkc="0-0-0">
  <h1 data-hk="0-0-2">Welcome to Leptos!</h1>
  <button data-hk="0-0-3">
    Click Me:
    <!-- <DynChild> -->11<!-- </DynChild> -->
  </button>
</leptos-island>
The typical Leptos hydration keys and markers are only present inside the island, only the island is hydrated.

Using Islands Effectively
Remember that only code within an #[island] needs to be compiled to WASM and shipped to the browser. This means that islands should be as small and specific as possible. My HomePage, for example, would be better broken apart into a regular component and an island:

#[component]
fn HomePage() -> impl IntoView {
view! {
<h1>"Welcome to Leptos!"</h1>
<Counter/>
}
}

#[island]
fn Counter() -> impl IntoView {
// Creates a reactive value to update the button
let (count, set_count) = create_signal(0);
let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
Now the <h1> doesn’t need to be included in the client bundle, or hydrated. This seems like a silly distinction now; but note that you can now add as much inert HTML content as you want to the HomePage itself, and the WASM binary size will remain exactly the same.

In regular hydration mode, your WASM binary size grows as a function of the size/complexity of your app. In islands mode, your WASM binary grows as a function of the amount of interactivity in your app. You can add as much non-interactive content as you want, outside islands, and it will not increase that binary size.

Unlocking Superpowers
So, this 50% reduction in WASM binary size is nice. But really, what’s the point?

The point comes when you combine two key facts:

Code inside #[component] functions now only runs on the server.
Children and props can be passed from the server to islands, without being included in the WASM binary.
This means you can run server-only code directly in the body of a component, and pass it directly into the children. Certain tasks that take a complex blend of server functions and Suspense in fully-hydrated apps can be done inline in islands.

We’re going to rely on a third fact in the rest of this demo:

Context can be passed between otherwise-independent islands.
So, instead of our counter demo, let’s make something a little more fun: a tabbed interface that reads data from files on the server.

Passing Server Children to Islands
One of the most powerful things about islands is that you can pass server-rendered children into an island, without the island needing to know anything about them. Islands hydrate their own content, but not children that are passed to them.

As Dan Abramov of React put it (in the very similar context of RSCs), islands aren’t really islands: they’re donuts. You can pass server-only content directly into the “donut hole,” as it were, allowing you to create tiny atolls of interactivity, surrounded on both sides by the sea of inert server HTML.

In the demo code included below, I added some styles to show all server content as a light-blue “sea,” and all islands as light-green “land.” Hopefully that will help picture what I’m talking about!

To continue with the demo: I’m going to create a Tabs component. Switching between tabs will require some interactivity, so of course this will be an island. Let’s start simple for now:

#[island]
fn Tabs(labels: Vec<String>) -> impl IntoView {
let buttons = labels
.into_iter()
.map(|label| view! { <button>{label}</button> })
.collect_view();
view! {
<div style="display: flex; width: 100%; justify-content: space-between;">
{buttons}
</div>
}
}
Oops. This gives me an error

error[E0463]: can't find crate for `serde`
--> src/app.rs:43:1
|
43 | #[island]
| ^^^^^^^^^ can't find crate
Easy fix: let’s cargo add serde --features=derive. The #[island] macro wants to pull in serde here because it needs to serialize and deserialize the labels prop.

Now let’s update the HomePage to use Tabs.

#[component]
fn HomePage() -> impl IntoView {
// these are the files we’re going to read
let files = ["a.txt", "b.txt", "c.txt"];
// the tab labels will just be the file names
let labels = files.iter().copied().map(Into::into).collect();
view! {
<h1>"Welcome to Leptos!"</h1>
<p>"Click any of the tabs below to read a recipe."</p>
<Tabs labels/>
}
}
If you take a look in the DOM inspector, you’ll see the island is now something like

<leptos-island
data-component="Tabs"
data-hkc="0-0-0"
data-props='{"labels":["a.txt","b.txt","c.txt"]}'
></leptos-island>
Our labels prop is getting serialized to JSON and stored in an HTML attribute so it can be used to hydrate the island.

Now let’s add some tabs. For the moment, a Tab island will be really simple:

#[island]
fn Tab(index: usize, children: Children) -> impl IntoView {
view! {
<div>{children()}</div>
}
}
Each tab, for now will just be a <div> wrapping its children.

Our Tabs component will also get some children: for now, let’s just show them all.

#[island]
fn Tabs(labels: Vec<String>, children: Children) -> impl IntoView {
let buttons = labels
.into_iter()
.map(|label| view! { <button>{label}</button> })
.collect_view();
view! {
<div style="display: flex; width: 100%; justify-content: space-around;">
{buttons}
</div>
{children()}
}
}
Okay, now let’s go back into the HomePage. We’re going to create the list of tabs to put into our tab box.

#[component]
fn HomePage() -> impl IntoView {
let files = ["a.txt", "b.txt", "c.txt"];
let labels = files.iter().copied().map(Into::into).collect();
let tabs = move || {
files
.into_iter()
.enumerate()
.map(|(index, filename)| {
let content = std::fs::read_to_string(filename).unwrap();
view! {
<Tab index>
<h2>{filename.to_string()}</h2>
<p>{content}</p>
</Tab>
}
})
.collect_view()
};

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <p>"Click any of the tabs below to read a recipe."</p>
        <Tabs labels>
            <div>{tabs()}</div>
        </Tabs>
    }
}
Uh... What?

If you’re used to using Leptos, you know that you just can’t do this. All code in the body of components has to run on the server (to be rendered to HTML) and in the browser (to hydrate), so you can’t just call std::fs; it will panic, because there’s no access to the local filesystem (and certainly not to the server filesystem!) in the browser. This would be a security nightmare!

Except... wait. We’re in islands mode. This HomePage component really does only run on the server. So we can, in fact, just use ordinary server code like this.

Is this a dumb example? Yes! Synchronously reading from three different local files in a .map() is not a good choice in real life. The point here is just to demonstrate that this is, definitely, server-only content.

Go ahead and create three files in the root of the project called a.txt, b.txt, and c.txt, and fill them in with whatever content you’d like.

Refresh the page and you should see the content in the browser. Edit the files and refresh again; it will be updated.

You can pass server-only content from a #[component] into the children of an #[island], without the island needing to know anything about how to access that data or render that content.

This is really important. Passing server children to islands means that you can keep islands small. Ideally, you don’t want to slap and #[island] around a whole chunk of your page. You want to break that chunk out into an interactive piece, which can be an #[island], and a bunch of additional server content that can be passed to that island as children, so that the non-interactive subsections of an interactive part of the page can be kept out of the WASM binary.

Passing Context Between Islands
These aren’t really “tabs” yet: they just show every tab, all the time. So let’s add some simple logic to our Tabs and Tab components.

We’ll modify Tabs to create a simple selected signal. We provide the read half via context, and set the value of the signal whenever someone clicks one of our buttons.

#[island]
fn Tabs(labels: Vec<String>, children: Children) -> impl IntoView {
let (selected, set_selected) = create_signal(0);
provide_context(selected);

    let buttons = labels
        .into_iter()
        .enumerate()
        .map(|(index, label)| view! {
            <button on:click=move |_| set_selected(index)>
                {label}
            </button>
        })
        .collect_view();
// ...
And let’s modify the Tab island to use that context to show or hide itself:

#[island]
fn Tab(children: Children) -> impl IntoView {
let selected = expect_context::<ReadSignal<usize>>();
view! {
<div style:display=move || if selected() == index {
"block"
} else {
"none"
}>
// ...
Now the tabs behave exactly as I’d expect. Tabs passes the signal via context to each Tab, which uses it to determine whether it should be open or not.

That’s why in HomePage, I made let tabs = move || a function, and called it like {tabs()}: creating the tabs lazily this way meant that the Tabs island would already have provided the selected context by the time each Tab went looking for it.

Our complete tabs demo is about 220kb uncompressed: not the smallest demo in the world, but still about a third smaller than the counter button! Just for kicks, I built the same demo without islands mode, using #[server] functions and Suspense. and it was 429kb. So again, this was about a 50% savings in binary size. And this app includes quite minimal server-only content: remember that as we add additional server-only components and pages, this 220 will not grow.

Overview
This demo may seem pretty basic. It is. But there are a number of immediate takeaways:

50% WASM binary size reduction, which means measurable improvements in time to interactivity and initial load times for clients.
Reduced HTML page size. This one is less obvious, but it’s true and important: HTML generated from #[component]s doesn’t need all the hydration IDs and other boilerplate added.
Reduced data serialization costs. Creating a resource and reading it on the client means you need to serialize the data, so it can be used for hydration. If you’ve also read that data to create HTML in a Suspense, you end up with “double data,” i.e., the same exact data is both rendered to HTML and serialized as JSON, increasing the size of responses, and therefore slowing them down.
Easily use server-only APIs inside a #[component] as if it were a normal, native Rust function running on the server—which, in islands mode, it is!
Reduced #[server]/create_resource/Suspense boilerplate for loading server data.
Future Exploration
The experimental-islands feature included in 0.5 reflects work at the cutting edge of what frontend web frameworks are exploring right now. As it stands, our islands approach is very similar to Astro (before its recent View Transitions support): it allows you to build a traditional server-rendered, multi-page app and pretty seamlessly integrate islands of interactivity.

There are some small improvements that will be easy to add. For example, we can do something very much like Astro's View Transitions approach:

add client-side routing for islands apps by fetching subsequent navigations from the server and replacing the HTML document with the new one
add animated transitions between the old and new document using the View Transitions API
support explicit persistent islands, i.e., islands that you can mark with unique IDs (something like persist:searchbar on the component in the view), which can be copied over from the old to the new document without losing their current state
There are other, larger architectural changes that I’m not sold on yet.

Additional Information
Check out the islands PR, roadmap, and Hackernews demo for additional discussion.

Demo Code
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
view! {
<Router>
<main style="background-color: lightblue; padding: 10px">
<Routes>
<Route path="" view=HomePage/>
</Routes>
</main>
</Router>
}
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
let files = ["a.txt", "b.txt", "c.txt"];
let labels = files.iter().copied().map(Into::into).collect();
let tabs = move || {
files
.into_iter()
.enumerate()
.map(|(index, filename)| {
let content = std::fs::read_to_string(filename).unwrap();
view! {
<Tab index>
<div style="background-color: lightblue; padding: 10px">
<h2>{filename.to_string()}</h2>
<p>{content}</p>
</div>
</Tab>
}
})
.collect_view()
};

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <p>"Click any of the tabs below to read a recipe."</p>
        <Tabs labels>
            <div>{tabs()}</div>
        </Tabs>
    }
}

#[island]
fn Tabs(labels: Vec<String>, children: Children) -> impl IntoView {
let (selected, set_selected) = create_signal(0);
provide_context(selected);

    let buttons = labels
        .into_iter()
        .enumerate()
        .map(|(index, label)| {
            view! {
                <button on:click=move |_| set_selected(index)>
                    {label}
                </button>
            }
        })
        .collect_view();
    view! {
        <div
            style="display: flex; width: 100%; justify-content: space-around;\
            background-color: lightgreen; padding: 10px;"
        >
            {buttons}
        </div>
        {children()}
    }
}

#[island]
fn Tab(index: usize, children: Children) -> impl IntoView {
let selected = expect_context::<ReadSignal<usize>>();
view! {
<div
style:background-color="lightgreen"
style:padding="10px"
style:display=move || if selected() == index {
"block"
} else {
"none"
}
>
{children()}
</div>
}
}


Appendix: How does the Reactive System Work?
You don’t need to know very much about how the reactive system actually works in order to use the library successfully. But it’s always useful to understand what’s going on behind the scenes once you start working with the framework at an advanced level.

The reactive primitives you use are divided into three sets:

Signals (ReadSignal/WriteSignal, RwSignal, Resource, Trigger) Values you can actively change to trigger reactive updates.
Computations (Memos) Values that depend on signals (or other computations) and derive a new reactive value through some pure computation.
Effects Observers that listen to changes in some signals or computations and run a function, causing some side effect.
Derived signals are a kind of non-primitive computation: as plain closures, they simply allow you to refactor some repeated signal-based computation into a reusable function that can be called in multiple places, but they are not represented in the reactive system itself.

All the other primitives actually exist in the reactive system as nodes in a reactive graph.

Most of the work of the reactive system consists of propagating changes from signals to effects, possibly through some intervening memos.

The assumption of the reactive system is that effects (like rendering to the DOM or making a network request) are orders of magnitude more expensive than things like updating a Rust data structure inside your app.

So the primary goal of the reactive system is to run effects as infrequently as possible.

Leptos does this through the construction of a reactive graph.

Leptos’s current reactive system is based heavily on the Reactively library for JavaScript. You can read Milo’s article “Super-Charging Fine-Grained Reactivity” for an excellent account of its algorithm, as well as fine-grained reactivity in general—including some beautiful diagrams!

The Reactive Graph
Signals, memos, and effects all share three characteristics:

Value They have a current value: either the signal’s value, or (for memos and effects) the value returned by the previous run, if any.
Sources Any other reactive primitives they depend on. (For signals, this is an empty set.)
Subscribers Any other reactive primitives that depend on them. (For effects, this is an empty set.)
In reality then, signals, memos, and effects are just conventional names for one generic concept of a “node” in a reactive graph. Signals are always “root nodes,” with no sources/parents. Effects are always “leaf nodes,” with no subscribers. Memos typically have both sources and subscribers.

Simple Dependencies
So imagine the following code:

// A
let (name, set_name) = create_signal("Alice");

// B
let name_upper = create_memo(move |_| name.with(|n| n.to_uppercase()));

// C
create_effect(move |_| {
log!("{}", name_upper());
});

set_name("Bob");
You can easily imagine the reactive graph here: name is the only signal/origin node, the create_effect is the only effect/terminal node, and there’s one intervening memo.

A   (name)
|
B   (name_upper)
|
C   (the effect)
Splitting Branches
Let’s make it a little more complex.

// A
let (name, set_name) = create_signal("Alice");

// B
let name_upper = create_memo(move |_| name.with(|n| n.to_uppercase()));

// C
let name_len = create_memo(move |_| name.len());

// D
create_effect(move |_| {
log!("len = {}", name_len());
});

// E
create_effect(move |_| {
log!("name = {}", name_upper());
});
This is also pretty straightforward: a signal source signal (name/A) divides into two parallel tracks: name_upper/B and name_len/C, each of which has an effect that depends on it.

__A__
|     |
B     C
|     |
E     D
Now let’s update the signal.

set_name("Bob");
We immediately log

len = 3
name = BOB
Let’s do it again.

set_name("Tim");
The log should shows

name = TIM
len = 3 does not log again.

Remember: the goal of the reactive system is to run effects as infrequently as possible. Changing name from "Bob" to "Tim" will cause each of the memos to re-run. But they will only notify their subscribers if their value has actually changed. "BOB" and "TIM" are different, so that effect runs again. But both names have the length 3, so they do not run again.

Reuniting Branches
One more example, of what’s sometimes called the diamond problem.

// A
let (name, set_name) = create_signal("Alice");

// B
let name_upper = create_memo(move |_| name.with(|n| n.to_uppercase()));

// C
let name_len = create_memo(move |_| name.len());

// D
create_effect(move |_| {
log!("{} is {} characters long", name_upper(), name_len());
});
What does the graph look like for this?

__A__
|     |
B     C
|     |
|__D__|
You can see why it's called the “diamond problem.” If I’d connected the nodes with straight lines instead of bad ASCII art, it would form a diamond: two memos, each of which depend on a signal, which feed into the same effect.

A naive, push-based reactive implementation would cause this effect to run twice, which would be bad. (Remember, our goal is to run effects as infrequently as we can.) For example, you could implement a reactive system such that signals and memos immediately propagate their changes all the way down the graph, through each dependency, essentially traversing the graph depth-first. In other words, updating A would notify B, which would notify D; then A would notify C, which would notify D again. This is both inefficient (D runs twice) and glitchy (D actually runs with the incorrect value for the second memo during its first run.)

Solving the Diamond Problem
Any reactive implementation worth its salt is dedicated to solving this issue. There are a number of different approaches (again, see Milo’s article for an excellent overview).

Here’s how ours works, in brief.

A reactive node is always in one of three states:

Clean: it is known not to have changed
Check: it is possible it has changed
Dirty: it has definitely changed
Updating a signal Dirty marks that signal Dirty, and marks all its descendants Check, recursively. Any of its descendants that are effects are added to a queue to be re-run.

    ____A (DIRTY)___
|               |
B (CHECK)    C (CHECK)
|               |
|____D (CHECK)__|
Now those effects are run. (All of the effects will be marked Check at this point.) Before re-running its computation, the effect checks its parents to see if they are dirty. So

So D goes to B and checks if it is Dirty.
But B is also marked Check. So B does the same thing:
B goes to A, and finds that it is Dirty.
This means B needs to re-run, because one of its sources has changed.
B re-runs, generating a new value, and marks itself Clean
Because B is a memo, it then checks its prior value against the new value.
If they are the same, B returns "no change." Otherwise, it returns "yes, I changed."
If B returned “yes, I changed,” D knows that it definitely needs to run and re-runs immediately before checking any other sources.
If B returned “no, I didn’t change,” D continues on to check C (see process above for B.)
If neither B nor C has changed, the effect does not need to re-run.
If either B or C did change, the effect now re-runs.
Because the effect is only marked Check once and only queued once, it only runs once.

If the naive version was a “push-based” reactive system, simply pushing reactive changes all the way down the graph and therefore running the effect twice, this version could be called “push-pull.” It pushes the Check status all the way down the graph, but then “pulls” its way back up. In fact, for large graphs it may end up bouncing back up and down and left and right on the graph as it tries to determine exactly which nodes need to re-run.

Note this important trade-off: Push-based reactivity propagates signal changes more quickly, at the expense of over-re-running memos and effects. Remember: the reactive system is designed to minimize how often you re-run effects, on the (accurate) assumption that side effects are orders of magnitude more expensive than this kind of cache-friendly graph traversal happening entirely inside the library’s Rust code. The measurement of a good reactive system is not how quickly it propagates changes, but how quickly it propagates changes without over-notifying.

Memos vs. Signals
Note that signals always notify their children; i.e., a signal is always marked Dirty when it updates, even if its new value is the same as the old value. Otherwise, we’d have to require PartialEq on signals, and this is actually quite an expensive check on some types. (For example, add an unnecessary equality check to something like some_vec_signal.update(|n| n.pop()) when it’s clear that it has in fact changed.)

Memos, on the other hand, check whether they change before notifying their children. They only run their calculation once, no matter how many times you .get() the result, but they run whenever their signal sources change. This means that if the memo’s computation is very expensive, you may actually want to memoize its inputs as well, so that the memo only re-calculates when it is sure its inputs have changed.

Memos vs. Derived Signals
All of this is cool, and memos are pretty great. But most actual applications have reactive graphs that are quite shallow and quite wide: you might have 100 source signals and 500 effects, but no memos or, in rare case, three or four memos between the signal and the effect. Memos are extremely good at what they do: limiting how often they notify their subscribers that they have changed. But as this description of the reactive system should show, they come with overhead in two forms:

A PartialEq check, which may or may not be expensive.
Added memory cost of storing another node in the reactive system.
Added computational cost of reactive graph traversal.
In cases in which the computation itself is cheaper than this reactive work, you should avoid “over-wrapping” with memos and simply use derived signals. Here’s a great example in which you should never use a memo:

let (a, set_a) = create_signal(1);
// none of these make sense as memos
let b = move || a() + 2;
let c = move || b() % 2 == 0;
let d = move || if c() { "even" } else { "odd" };

set_a(2);
set_a(3);
set_a(5);
Even though memoizing would technically save an extra calculation of d between setting a to 3 and 5, these calculations are themselves cheaper than the reactive algorithm.

At the very most, you might consider memoizing the final node before running some expensive side effect:

let text = create_memo(move |_| {
d()
});
create_effect(move |_| {
engrave_text_into_bar_of_gold(&text());
});


Appendix: The Life Cycle of a Signal
Three questions commonly arise at the intermediate level when using Leptos:

How can I connect to the component lifecycle, running some code when a component mounts or unmounts?
How do I know when signals are disposed, and why do I get an occasional panic when trying to access a disposed signal?
How is it possible that signals are Copy and can be moved into closures and other structures without being explicitly cloned?
The answers to these three questions are closely inter-related, and are each somewhat complicated. This appendix will try to give you the context for understanding the answers, so that you can reason correctly about your application's code and how it runs.

The Component Tree vs. The Decision Tree
Consider the following simple Leptos app:

use leptos::logging::log;
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
let (count, set_count) = create_signal(0);

    view! {
        <button on:click=move |_| set_count.update(|n| *n += 1)>"+1"</button>
        {move || if count() % 2 == 0 {
            view! { <p>"Even numbers are fine."</p> }.into_view()
        } else {
            view! { <InnerComponent count/> }.into_view()
        }}
    }
}

#[component]
pub fn InnerComponent(count: ReadSignal<usize>) -> impl IntoView {
create_effect(move |_| {
log!("count is odd and is {}", count());
});

    view! {
        <OddDuck/>
        <p>{count}</p>
    }
}

#[component]
pub fn OddDuck() -> impl IntoView {
view! {
<p>"You're an odd duck."</p>
}
}
All it does is show a counter button, and then one message if it's even, and a different message if it's odd. If it's odd, it also logs the values in the console.

One way to map out this simple application would be to draw a tree of nested components:

App
|_ InnerComponent
|_ OddDuck
Another way would be to draw the tree of decision points:

root
|_ is count even?
|_ yes
|_ no
If you combine the two together, you'll notice that they don't map onto one another perfectly. The decision tree slices the view we created in InnerComponent into three pieces, and combines part of InnerComponent with the OddDuck component:

DECISION            COMPONENT           DATA    SIDE EFFECTS
root                <App/>              (count) render <button>
|_ is count even?   <InnerComponent/>
|_ yes                                       render even <p>
|_ no                                        start logging the count
<OddDuck/>                  render odd <p>
render odd <p> (in <InnerComponent/>!)
Looking at this table, I notice the following things:

The component tree and the decision tree don't match one another: the "is count even?" decision splits <InnerComponent/> into three parts (one that never changes, one if even, one if odd), and merges one of these with the <OddDuck/> component.
The decision tree and the list of side effects correspond perfectly: each side effect is created at a specific decision point.
The decision tree and the tree of data also line up. It's hard to see with only one signal in the table, but unlike a component, which is a function that can include multiple decisions or none, a signal is always created at a specific line in the tree of decisions.
Here's the thing: The structure of your data and the structure of side effects affect the actual functionality of your application. The structure of your components is just a convenience of authoring. You don't care, and you shouldn't care, which component rendered which <p> tag, or which component created the effect to log the values. All that matters is that they happen at the right times.

In Leptos, components do not exist. That is to say: You can write your application as a tree of components, because that's convenient, and we provide some debugging tools and logging built around components, because that's convenient too. But your components do not exist at runtime: Components are not a unit of change detection or of rendering. They are simply function calls. You can write your whole application in one big component, or split it into a hundred components, and it does not affect the runtime behavior, because components don't really exist.

The decision tree, on the other hand, does exist. And it's really important!

The Decision Tree, Rendering, and Ownership
Every decision point is some kind of reactive statement: a signal or a function that can change over time. When you pass a signal or a function into the renderer, it automatically wraps it in an effect that subscribes to any signals it contains, and updates the view accordingly over time.

This means that when your application is rendered, it creates a tree of nested effects that perfectly mirrors the decision tree. In pseudo-code:

// root
let button = /* render the <button> once */;

// the renderer wraps an effect around the `move || if count() ...`
create_effect(|_| {
if count() % 2 == 0 {
let p = /* render the even <p> */;
} else {
// the user created an effect to log the count
create_effect(|_| {
log!("count is odd and is {}", count());
});

        let p1 = /* render the <p> from OddDuck */;
        let p2 = /* render the second <p> */ 

        // the renderer creates an effect to update the second <p>
        create_effect(|_| {
            // update the content of the <p> with the signal
            p2.set_text_content(count.get());
        });
    }
})
Each reactive value is wrapped in its own effect to update the DOM, or run any other side effects of changes to signals. But you don't need these effects to keep running forever. For example, when count switches from an odd number back to an even number, the second <p> no longer exists, so the effect to keep updating it is no longer useful. Instead of running forever, effects are canceled when the decision that created them changes. In other words, and more precisely: effects are canceled whenever the effect that was running when they were created re-runs. If they were created in a conditional branch, and re-running the effect goes through the same branch, the effect will be created again: if not, it will not.

From the perspective of the reactive system itself, your application's "decision tree" is really a reactive "ownership tree." Simply put, a reactive "owner" is the effect or memo that is currently running. It owns effects created within it, they own their own children, and so on. When an effect is going to re-run, it first "cleans up" its children, then runs again.

So far, this model is shared with the reactive system as it exists in JavaScript frameworks like S.js or Solid, in which the concept of ownership exists to automatically cancel effects.

What Leptos adds is that we add a second, similar meaning to ownership: a reactive owner not only owns its child effects, so that it can cancel them; it also owns its signals (memos, etc.) so that it can dispose of them.

Ownership and the Copy Arena
This is the innovation that allows Leptos to be usable as a Rust UI framework. Traditionally, managing UI state in Rust has been hard, because UI is all about shared mutability. (A simple counter button is enough to see the problem: You need both immutable access to set the text node showing the counter's value, and mutable access in the click handler, and every Rust UI framework is designed around the fact that Rust is designed to prevent exactly that!) Using something like an event handler in Rust traditionally relies on primitives for communicating via shared memory with interior mutability (Rc<RefCell<_>>, Arc<Mutex<_>>) or for shared memory by communicating via channels, either of which often requires explicit .clone()ing to be moved into an event listener. This is kind of fine, but also an enormous inconvenience.

Leptos has always used a form of arena allocation for signals instead. A signal itself is essentially an index into a data structure that's held elsewhere. It's a cheap-to-copy integer type that does not do reference counting on its own, so it can be copied around, moved into event listeners, etc. without explicit cloning.

Instead of Rust lifetimes or reference counting, the life cycles of these signals are determined by the ownership tree.

Just as all effects belong to an owning parent effect, and the children are canceled when the owner reruns, so too all signals belong to an owner, and are disposed of when the parent reruns.

In most cases, this is completely fine. Imagine that in our example above, <OddDuck/> created some other signal that it used to update part of its UI. In most cases, that signal will be used for local state in that component, or maybe passed down as a prop to another component. It's unusual for it to be hoisted up out of the decision tree and used somewhere else in the application. When the count switches back to an even number, it is no longer needed and can be disposed.

However, this means there are two possible issues that can arise.

Signals can be used after they are disposed
The ReadSignal or WriteSignal that you hold is just an integer: say, 3 if it's the 3rd signal in the application. (As always, the reality is a bit more complicated, but not much.) You can copy that number all over the place and use it to say, "Hey, get me signal 3." When the owner cleans up, the value of signal 3 will be invalidated; but the number 3 that you've copied all over the place can't be invalidated. (Not without a whole garbage collector!) That means that if you push signals back "up" the decision tree, and store them somewhere conceptually "higher" in your application than they were created, they can be accessed after being disposed.

If you try to update a signal after it was disposed, nothing bad really happens. The framework will just warn you that you tried to update a signal that no longer exists. But if you try to access one, there's no coherent answer other than panicking: there is no value that could be returned. (There are try_ equivalents to the .get() and .with() methods that will simply return None if a signal has been disposed).

Signals can be leaked if you create them in a higher scope and never dispose of them
The opposite is also true, and comes up particularly when working with collections of signals, like an RwSignal<Vec<RwSignal<_>>>. If you create a signal at a higher level, and pass it down to a component at a lower level, it is not disposed until the higher-up owner is cleaned up.

For example, if you have a todo app that creates a new RwSignal<Todo> for each todo, stores it in an RwSignal<Vec<RwSignal<Todo>>>, and then passes it down to a <Todo/>, that signal is not automatically disposed when you remove the todo from the list, but must be manually disposed, or it will "leak" for as long as its owner is still alive. (See the TodoMVC example for more discussion.)

This is only an issue when you create signals, store them in a collection, and remove them from the collection without manually disposing of them as well.

Connecting the Dots
The answers to the questions we started with should probably make some sense now.

Component Life-Cycle
There is no component life-cycle, because components don't really exist. But there is an ownership lifecycle, and you can use it to accomplish the same things:

before mount: simply running code in the body of a component will run it "before the component mounts"
on mount: create_effect runs a tick after the rest of the component, so it can be useful for effects that need to wait for the view to be mounted to the DOM.
on unmount: You can use on_cleanup to give the reactive system code that should run while the current owner is cleaning up, before running again. Because an owner is around a "decision," this means that on_cleanup will run when your component unmounts: if something can unmount, the renderer must have created an effect that's unmounting it!
Issues with Disposed Signals
Generally speaking, problems can only arise here if you are creating a signal lower down in the ownership tree and storing it somewhere higher up. If you run into issues here, you should instead "hoist" the signal creation up into the parent, and then pass the created signals down—making sure to dispose of them on removal, if needed!

Copy signals
The whole system of Copyable wrapper types (signals, StoredValue, and so on) uses the ownership tree as a close approximation of the life-cycle of different parts of your UI. In effect, it parallels the Rust language's system of lifetimes based on blocks of code with a system of lifetimes based on sections of UI. This can't always be perfectly checked at compile time, but overall we think it's a net positive.