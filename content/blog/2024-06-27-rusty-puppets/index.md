+++
title = "Scraping with rust and headless chrome"
date = 2024-06-27
draft = true
[taxonomies]
tags = ["rust", "web scraping", "headless chrome"]
+++

## Why?

Rust is pretty amazing but there are a few things that you might be weary about. There are [few war stories](https://discord.com/blog/why-discord-is-switching-from-go-to-rust) of companies building their entire stack on `rust` or and then living happily ever after. Software is an ever evolving organism so in the [darwinian sense the more adaptable the better](https://www.darwinproject.ac.uk/people/about-darwin/six-things-darwin-never-said/evolution-misquotation). Enough of that though, not here to advocate any particular language or framework, what I want is to share my experience with writing an equivalent scraper in `rust` to [my previous post](../using-go-chromedp/) where I used `golang` and `chromedp`.

The experience using `go` with `chromedp` to automate chrome was pretty good, it is not as powerful as what is available in `puppeteer` so I figured I would have a look at what might be available in the `rust` landscape.


## What?

{{ img(id="rusty-puppets.png", alt="Puppeteer puppeteering rusty humans", caption="Rusty puppeteer") }}

In `rust` there are several libraries that deal with browser automation, a few I have had a look at are:
- [fantocini](https://github.com/jonhoo/fantoccini) - A high-level API for programmatically interacting with web pages through WebDriver, but I want chrome devtools protocol instead.
- [rust-headless-chrome](https://github.com/rust-headless-chrome/rust-headless-chrome) - chrome devtools protocol client library in rust, not as active as the crate I wound up using.
- [chromiumoxide](https://github.com/mattsse/chromiumoxide) - this is the one that seem to be the most active in terms of development so it looks like a good choice at time of writing.

## TL;DR

As I was reading one of my older posts that focuses on quasi live coding I realized it was boring as hell, and if your attention span is that of a goldfish, like mine is, it would probably make sense to just drop in a link to the [repo](https://github.com/adaschevici/rustic-toy-chest/tree/main/rust-crawl-pupp) so that you can download the code and try it out yourself. The repo is a collection of rust prototypes that I have been building for fun and learning, haven't had yet a compelling reason to use rust in production unfortunately :cry:.

## How?

To my surprise the code was closer in structure to the [`puppeteer`](https://pptr.dev/) version than it was to the [`chromedp`](https://github.com/chromedp/chromedp). The `chromedp` version uses nested context declarations to manage the browser and page runtimes, the `rust` version uses a more linear approach. You construct a browser instance and then you can interact with it as a user would. This points at the fact that the `chromiumoxide` api is higher level. 

The way you can set things up to keep your use cases separate is by adding [`clap`](https://docs.rs/clap/latest/clap/) to your project and use command line flags to select the use case you want to run.

You will see that I have covered most cases but not everything is transferable from `puppeteer` or `chromedp` to the `chromiumoxide` version. I will not go through the setup of `rustup`, rust toolchain or `cargo` as this is a basic and well documented process, all you have to do is search for `getting started with rust` and you will find a bunch of resources.

## Show me the code

#### 1. Laying down the foundation

  - set up my project root
    ```bash
    cargo new rust-crawl-pupp
    cd rust-crawl-pupp
    cargo install cargo-edit # this is useful for adding and upgrading dependencies
    ```
  - add dependencies via `cargo add` 
    ```toml
    [dependencies]
    chromiumoxide = { version = "0.5.7", features = ["tokio", "tokio-runtime"] } # this is the main dependency
    chromiumoxide_cdp = "0.5.2" # this is the devtools protocol
    clap = { version = "4.5.7", features = ["derive", "cargo"] } # this is for command line parsing
    futures = "0.3.30" # this is for async programming
    tokio = { version = "1.38.0", features = ["full"] } # this is the async runtime
    tracing = "0.1.40" # this is for logging
    tracing-subscriber = { version = "0.3.18", features = ["registry", "env-filter"] } # this is for logging
    ```

  - add `clap` command line parsing to the project so that each different use case can be called via a subcommand
    define your imports

    ```rust
    use clap::{Parser, Subcommand};
    ```

    define your command structs for parsing the command line arguments, this will allow for each use case to be called with its own subcommand like so `cargo run -- first-project`, `cargo run -- second-project`, and so on.

    ```rust
    #[derive(Parser)]
    #[command(
        name = "OxideCrawler",
        version = "0.1",
        author = "artur",
        about = "An example application using clap"
    )]
    struct Cli {
        #[command(subcommand)]
        command: Commands,
    }
    #[derive(Subcommand, Debug)]
    enum Commands {
        FirstProject {},
        SecondProject {},
        ...
    }
    ```

    the way you can hook this into the main function is via a `match` statement that will call the appropriate function based on the subcommand that was passed in.

    ```rust
    let args = Cli::parse();
    ...
    match &args.command {
        Commands::FirstProject {} => {
            let user_agent = spoof_user_agent(&mut browser).await?;
            info!(user_agent, "User agent detected");
        }
        ...
    }

    ```
#### 2. Starting browser and the browser cleanup
  - use the `launch` method and its options to start the browser, if the viewport and window size are different, the browser will start in windowed mode, with the page size being smaller.
    ```rust
    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .with_head() // this will start the browser in headless mode
            .no_sandbox() // this will disable the sandbox
            .viewport(None) // this will set the viewport size
            .window_size(1400, 1600) // this will set the window size
            .build()?,
    )
    .await?;

    let handle = tokio::task::spawn(async move {
        loop {
            match handler.next().await {
                Some(h) => match h {
                    Ok(_) => continue,
                    Err(_) => break,
                },
                None => break,
            }
        }
    });
    ```
  - the browser cleanup needs to be done correctly and there are two symptoms that you will see if you missed anything:
    - the browser will not close - hangs at the end
    - you might get a warning like the following:
      ```bash
      2024-06-26T08:40:01.418414Z  WARN chromiumoxide::browser: Browser was not closed manually, it will be killed automatically in the background
      ```
    to correctly clean up your browser instance you will have to call these on the code paths that close the browser
    ```rust
    browser.close().await?;
    browser.wait().await?;
    handle.await?;
    ```

#### 3. Use cases
  In the [repo](https://github.com/adaschevici/rustic-toy-chest/tree/main/rust-crawl-pupp) each use case lives in its own module most of the time. There are some cases where you might have two living in the same module when they are very closely related, like in Use Case `c.`.

  a. Spoof your user agent:
    The only way I have found to set your user agent was from the [`Page`](https://docs.rs/chromiumoxide/latest/chromiumoxide/page/struct.Page.html#) module via the `set_user_agent` method

  ```rust
  let page = browser.new_page("about:blank").await?;
  page.set_user_agent(
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) \
       Chrome/58.0.3029.110 Safari/537.36",
  )
  .await?;
  page.goto("https://www.whatismybrowser.com/detect/what-is-my-user-agent")
      .await?;

  ```
  b. Grab the full content of the page is pretty straightforward
  ```rust
      let page = browser
        .new_page("https://scrapingclub.com/exercise/list_infinite_scroll/")
        .await?;
    let content = page.content().await?;
  ```

## Conclusions
