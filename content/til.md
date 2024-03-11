+++
title = "Today I Learned"
date = 2024-03-05
draft = false
+++
# TIL 😅 11th March 2024

When you load svg icons from a svg sprite, you can use the `use` tag to reference the icon by its id.
However if the url you use to access the page is not the one that is used internally, you will get a Cross-Origin error.
Came across this with my zola blog, and it seemed to work intermittently, but I couldn't figure out why.


# TIL 😅 10th March 2024

If you want to pass in a list to the [transformers](https://github.com/huggingface/transformers) library you are going to have a bad time.
Try converting the list to a numpy array first. This is important because depending on how complex preprocessing is, the intermediate types need to be validated.

```python
import torch
import numpy as np

a = [1, 2, 3]
try:
    b = torch.tensor(a) # This will fail
except ValueError as e:
    c = torch.tensor(np.array(a)) # This will work
```

