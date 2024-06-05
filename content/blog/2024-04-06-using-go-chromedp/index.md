+++
title = "Experimenting with a puppeteer alternative in Go"
date = 2024-04-06
draft = false
+++

# Why?
As developers we sometimes get a bad case of the shiny new object syndrome. I hate to say it but every time I start hacking on something new, the urge to add something new is quite overwhelming. It is really tough to keep an interest in projects for a long time and it starts to become tedious the deeper you go into the weeds. I suppose this is why people list `growth` as one of their top motivations.

I consider that anything new is an opportunity for growth, and doing something over and over in a similar manner quickly becomes a tedious. I've been building various types of scrapers since 2011, and it all started because I wanted to automate a workflow and save myself some time. The time spent on automating this was probably more than if I had done this by hand but it was interesting interacting via `http` from code and crunching the data automatically.

The amount of data on the web is pretty crazy, you have various sources and multiple types of data that can be combined in very interesting ways. Back in those days dropshipping was becoming huge and people were performing arbitrage across Amazon/Ebay/local flea-markets etc.. Tools that were able to perform analytics across these shops were quite trendy, and the market was slightly less crowded, so for me building crawlers seemed like a nice idea to build out a good customer base.

Nowadays due to `RAG` systems, gathering data automatically, breaking it down and feeding it into embedding models and storing it in vector databases for `LLM` information enhancement has come back into the spotlight. In between then and now there have been a few changes in the way data is served up for consumption. Off the top of my head:
- single page apps have gained huge traction, most everyone turning to building their content in a `JS` bundle, loading everything on the fly as the page loads
# How?
There are many alternatives to `puppeteer`, the way they work is by using the `cdp` protocol

# The pleasant surprises
Well, calling them surprises is a bit of a stretch, I have been `golang` over the years and I have to admit it is a pretty nice ecosystem and language. 

# The ugly parts
The way to communicate with the browser is still through the `cdp` protocol and sometimes you need to pass objects only objects that can be serialized.

# Conclusion
- in some ways puppeteer is still better than `chromedp`, working with `iframes` falls short
- `rod` is a nice alternative but its API looks like it was designed for testing, reminds me of `cucumber`
- 


