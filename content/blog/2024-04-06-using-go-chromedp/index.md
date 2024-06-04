+++
title = "Experimenting with a puppeteer alternative in Go"
date = 2024-04-06
draft = true
+++

# Why?
As developers we sometimes get a bad case of the shiny new object syndrome. I hate to say it but every time I start hacking on something the urge to add something new is quite overwhelming.

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


