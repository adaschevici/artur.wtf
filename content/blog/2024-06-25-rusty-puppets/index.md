+++
title = "Scraping with rust and headless chrome"
date = 2024-06-25
draft = true
[taxonomies]
tags = ["rust", "web scraping", "headless chrome"]
+++

## Why?

Rust is pretty amazing but there are a few things that you might be weary about. There are [few war stories](https://discord.com/blog/why-discord-is-switching-from-go-to-rust) of companies building their entire stack on `rust` or and then living happily ever after. Software is an ever evolving organism so in the [darwinian sense the more adaptable the better](https://www.darwinproject.ac.uk/people/about-darwin/six-things-darwin-never-said/evolution-misquotation). Enough of that though, not here to advocate any particular language or framework, what I want is to share my experience with writing an equivalent scraper in `rust` to [my previous post](../using-go-chromedp/) where I used `golang` and `chromedp`.

The experience using `go` with `chromedp` to automate chrome was pretty good, it is not as powerful as what is available in `puppeteer` so I figured I would have a look at what might be available in the `rust` landscape.


## What?

In `rust` there are several libraries that deal with browser automation, a few I have had a look at are:
- [fantocini](https://github.com/jonhoo/fantoccini) - A high-level API for programmatically interacting with web pages through WebDriver, but I want chrome devtools protocol instead.
- [rust-headless-chrome](https://github.com/rust-headless-chrome/rust-headless-chrome) - chrome devtools protocol client library in rust, not as active as the crate I wound up using.
- [chromiumoxide](https://github.com/mattsse/chromiumoxide) - this is the one that seem to be the most active in terms of development so it looks like a good choice at time of writing.
