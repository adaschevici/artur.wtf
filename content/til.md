+++
title = "Today I Learned"
date = 2024-03-05
draft = false
+++

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

