+++
title = "Today I Learned"
date = 2024-03-05
draft = false
+++

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

