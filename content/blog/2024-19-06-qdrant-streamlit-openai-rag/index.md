+++
title = "Building my own chatgpt agent replica on OpenAI's GPT-4"
date = 2024-06-19
draft = true
+++

## Why?

[OpenAI](https://openai.com/) has been making it easier and easier to build out [GPT agents](https://www.deeplearning.ai/the-batch/how-agents-can-improve-llm-performance/) that make use of your own data to improve the generated responses of the pretrained models.

Agents give a way to inject knowledge about your specific proprietary data into your pipeline, without actually sharing any private information about it. You can also improve the recency of your data too which makes you less dependent on the model's training cycle.

OpenAI has improved the DX, UX and APIs since version 3.5, and has made it easier to create `agents` and embed your data into your custom [`GPTs`](https://openai.com/index/introducing-gpts/). They have lowered the barrier to entry which means that virtually anyone can build their own assistants that would be able to respond to queries about their data. This is perfect for people to experiment on building products. IMO this is a very good approach to enable product discovery for the masses.

Most big AI contenders on the market provide you with a toolbox of high level abstractions and low to no code solutions. The weird thing about my approach to learning things is that not having some understanding of the first principles of the tech I'm using makes me feel a bit helpless, this is why I figured trying to build my own `RAG` system would be a good way to figure out the nuts and bolts.

## What?

I wanted to get a project for running my own pipeline with somewhat interchangeable parts. Models can be swapped around so that you can make the most of the latest models either available on [`Hugginface`](https://huggingface.co/), [`OpenAI`](https://openai.com/) or wherever.

Because things are moving so fast in model research the top contenders are surpassing each other every day pretty much. A custom pipeline  would allow us to quickly iterate and test out new models as they come out.

What I wound up building is a [`Streamlit`](https://streamlit.io/) app that uses [`qdrant`](https://qdrant.com/) to index and search data extracted from a collection of `pdf` document. The app is a simple chat interface where you can ask questions about the data and get responses from a mixture of `GPT-4` and the indexed data.

## How?

1. Setting up the environment
   - use `pyenv` to manage python versions
     ```bash
     # update versions
     pyenv update
     # install any python version
     pyenv install 3.12.3 # as of writing this
     ```
Talk about tech used: Streamlit, qdrant, GPT-4o, etc.

## Conclusions
how it went, what I learned, etc.

