+++
title = "Scraping with rust and headless chrome"
date = 2024-06-25
draft = true
[taxonomies]
tags = ["rust", "web scraping", "headless chrome"]
+++

## Why?

Rust is pretty amazing but there are a few things that you might be weary about. There are [few war stories](https://discord.com/blog/why-discord-is-switching-from-go-to-rust) of companies building their entire stack on `rust` or and then living happily ever after. Software is an ever evolving organism so in the [darwinian sense the more adaptable the better](https://www.darwinproject.ac.uk/people/about-darwin/six-things-darwin-never-said/evolution-misquotation). Enough of that though, not here to advocate any particular language or framework, what I want is to share my experience with writing an equivalent scraper in `rust` to [my previous post](../using-go-chromedp/) where I used `golang` and `chromedp`.
