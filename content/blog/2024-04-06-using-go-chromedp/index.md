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
- websites have become fussy about having their data used by unknown parties, so they have been closing down access and have become very litigious(#TODO: maybe add some cases of court cases Linkedin vs those guys, Financial Times vs OpenAI)
- bot detection and prevention - this one is funny since it is like a flywheel, it built 2 lucrative markets overnight - bot services and anti bot protection
- TBH, it's difficult to predict where this might be heading, it kind of feels like people have been aiming to move all their datas into data centers but since data is becoming so guarded...will they move back to paper?

{{ img(id="all-your-data-are-belong-to-us.png", alt="All your data are belong to us") }}

Because of `SPAs` and the wide adoption of `JS` in websites it is much more convenient to use some sort of browser automation to crawl pages and extract the information. This makes it less prone to badgering the servers, and having to reverse engineer the page content loading, so you will probably want to use either a [`chrome developer tools protocol`](https://chromedevtools.github.io/devtools-protocol/) or [`webdriver`](https://www.w3.org/TR/webdriver/) flavored communications protocol with the browser. Back in the day IIRC I have also used the [`PyQt`](https://www.riverbankcomputing.com/software/pyqt/intro) bindings for acessing the `Qt` browser component but nowadays its mostly straight-up browsers.

These days my goto is `puppeteer`. It's a weird tool that can be easily be used to scrape data from pages. The reason I say it is weird is mainly due to the deceiving nature of the internals, essentially using two `JS` engines that communicate via the `cdp` protocol that is a a very dense beast and does not play nice with complex objects.

Recently it has become more appealing to me to use strongly typed languages. This is probably because I have started to narrow down my experiments to very small code samples that illustrate one thing at time. I would go as far as to call it experiment driven development. Duck typing is fun as you can print pretty much anything you want. I was thinking to use `rust` but it has a very tough learning curve. Node is pretty nice with `mjs` but it's confusing sometimes when it crosses over between the two event loops, also while it is good for communicating on `cdp` it is not really designed for sync code and `python` is a bit boring for me so I decided to look at `go`. Since it is a google language I expected it to have decent support for cdp, and the learning curve is slightly gentler than `rust`.

# How?
Looking at the alternatives there are two that stand out [`chromedp`](https://github.com/chromedp/chromedp) and [rod](https://go-rod.github.io/#/). Rod looks like it is the prodigal son of [`behave`](https://behave.readthedocs.io/en/latest/) and [`cucumber`](https://cucumber.io/) some well established BDD frameworks. Personally I am not finding the `MustYaddaYadda...` very readable and combining it with other custom APIs would probably make it become inconsistent. It has a few nice things in the way it abstracts `iframes` but I am just unable to go past the higher level API.

In the end I wound up choosing `chromedp`. It works pretty well for most use cases, there are some places where it doesn't quite cut it and I wish it did, but by now I have come to terms there is no one technology to rule them all, wouldn't it be nice if that existed?


# The pleasant surprises
Well, calling them surprises is a bit of a stretch, I have been `golang` over the years and I have to admit it is a pretty nice ecosystem and language. 
`chromedp` automates chrome or any binary that you are able to communicate with via [`cdp`](https://github.com/chromedp/chromedp/blob/ebf842c7bc28db77d0bf4d757f5948d769d0866f/allocate.go#L349). The API is somewhat intuitive, haven't found myself diving into the guts of it very often to figure out how stuff works. The good part is that once you extract the data from the nodes you are interested in you can map it to go structs and make use of the go typing system. 

For example you can grab a list of elements via selector:
```go
	var productNodes []*cdp.Node
	if err := chromedp.Run(ctx,
		// visit the target page
		chromedp.Navigate("https://scrapingclub.com/exercise/list_infinite_scroll/"),
		chromedp.Evaluate(script, nil),
		chromedp.WaitVisible(".post:nth-child(60)"),
		chromedp.Nodes(`.post`, &productNodes, chromedp.ByQueryAll),
	); err != nil {
		log.Fatal("Error while trying to grab product items.", err)
	}
```

then map each element to a struct
```go
	for _, node := range productNodes {
		if err := chromedp.Run(ctx,
			chromedp.Text(`h4`, &name, chromedp.ByQuery, chromedp.FromNode(node)),
			chromedp.Text(`h5`, &price, chromedp.ByQuery, chromedp.FromNode(node)),
		); err != nil {
			log.Fatal("Error while trying to grab product items.", err)
		}
		products = append(products, Product{name: name, price: price})
	}
```

Another nice perk is that go is built with concurrency in mind
# The ugly parts
The way to communicate with the browser is still through the `cdp` protocol and sometimes you need to pass objects only objects that can be serialized.

# Conclusion
- in some ways puppeteer is still better than `chromedp`, working with `iframes` falls short
- `rod` is a nice alternative but its API looks like it was designed for testing, reminds me of `cucumber`
- 


