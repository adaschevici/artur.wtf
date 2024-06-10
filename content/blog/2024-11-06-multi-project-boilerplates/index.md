+++
title = "Using copier to aggregate multiple project boilerplates"
date = 2024-11-06
draft = true
+++

## Why?

Technically when you start a new project the best way to approach it is by using the `CLI` tool of the realm, such as `svelte-kit`, `astro`, `django-cli` etc..., you get the idea. The huge bonus to doing this is that you get the best practices baked in and as new standards are created the `CLI` gets updated.

So far the frontend has been a lot luckier with the tools as far as project generation goes, every major framework having come out with their own project generation tool, some having more than one possibly due to (multiple thought camps: TODO: better way to phrase this) multiple thought camps.

There are some backend frameworks that have project generation tools too but so far it seems to be difficult to agree on the structure. The best you can do is find a way to structure it that looks like the majority and makes sense for you. I have been building spiders and crawlers for data ingestion pipelines using `python` at first and then `node` and `go`. Even more recently I have been looking at hacking out some tweaks in some of my `neovim` plugins(that is lua).

For example neovim plugins have a pretty standard setup, they will have a folder layout something like the following:

```bash
scratcher.nvim/
├── README.md
├── lua
│   └── scratcher
│       └── init.lua
└── plugin
    └── scratcher.lua
```
The standard way of naming things seems to be gravitating towards having some conventions as you can see so setting up a new plugin would be pretty much repetitive and automatable. And it will probably save you some time and willpower, provided you know what your final architecture needs to look like.

## How?

If you are coming from `python` like I am then you may already  be familiar with [`cookie-cutter`](TODO: add link). I have looked at what it does and how it might benefit me in a few cases but every time it was a matter of balancing out the timeline and trying to stay away from over engineering.
