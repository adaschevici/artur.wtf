+++
title = "Today I Learned"
date = 2024-03-05
draft = false
paginate_by = 3
[extra]
    subheading = "A collection of things I learn every day"
+++

# TIL: :sweat_smile: 3rd June 2024

You can create arbitrarily sized files from the cli using one of the following commands
```sh
# create a 1GB file
dd if=/dev/zero of=1GBfile bs=1M count=1024

# create a 1GB file
head -c 1G /dev/urandom > 1GBfile.txt

# create a 1GB file
truncate -s 1G 1GBfile
```
# TIL: :thinking: 2nd June 2024

There is a new OSS library called `ScrapeGraphAI` that aims to combine scraping with graph logic to transform natural language into scrapers, wodnering how this will work out in the long run.

# TIL: 1st June 2024

There is a submodule in the `go` standard library that allows you to do logging so it is not necessary to use `logrus` for logging.

# TIL :mind_blown: 30th March 2024

You can run `chromedriver` from the command line for trying to access it via cdp protocol programatically.
```sh
# on OSX by default this does not work, developer not verified
chromedriver --port=9222

# OSX prior to running that
cd /path/to/chromedriver
xattr -d com.apple.quarantine chromedriver
```

# TIL :sweat_smile: 29th May 2024

Rust zola static site builder contains in the page object a reading time field that can be used to display the reading time of the page. You can use it in the template like this

```jinja
{{ page.reading_time }}
```

# TIL :sweat_smile: 29th May 2024

In order to use shortcodes in Zola templates you need to import them using `include` directive in the template file.

```jinja
{% include "shortcodes/shortcode.html" %}
``` 

# TIL 😅 28th May 2024

You can figure out number of commits today in git using a simple one liner
```bash
git log --since=midnight --oneline | wc -l
```

# TIL 😅 27th May 2024

The [`go-spew`](https://pkg.go.dev/github.com/davecgh/go-spew/spew) package has been active last more than 7 years ago. You can use the [`litter`](https://pkg.go.dev/github.com/sanity-io/litter) package instead. But no point in using it, just use `json.MarshalIndent` to print the struct in a readable format, or alternatively use `fmt.Printf("%#v", struct)` to print the struct in a readable format.

```go
    fmt.Printf("%#v", targets)
    // or
    jsonStr, _ := json.MarshalIndent(targets, "", "  ")
    fmt.Println(string(jsonStr))
```

# TIL 😅 25th May 2024

_Anchor estimates_ are what you do when you try to shoehorn personal experience into estimating a future task.
It is very error prone.

# TIL 😅 11th March 2024

When you load svg icons from a svg sprite, you can use the `use` tag to reference the icon by its id.
However if the url you use to access the page is not the one that is used internally, you will get a Cross-Origin error.
Came across this with my zola blog, and it seemed to work intermittently, but I couldn't figure out why.


# TIL 😅 10th March 2024

Creating tensors is easy, just use numpy and `torch.tensor` to do it
```python
import torch
import numpy as np

a = [1, 2, 3]
c = torch.tensor(np.array(a)) #  c is a tensor
```

# TIL 🤯 9th March 2024

If you intend to create a vector based search engine, searching a database of assets will give you identical results if you are looking for similarity. If you want to have similarity search but at the same time look for diverse results you can use `max_marginal_relevance_search`, this is available in `langchain`, and it is bound to the vector store. 

