+++
title = "Keep your estimates boring"
date = 2024-05-28
[taxonomies]
  tags = ["estimating software projects"]
+++

## Why?
Most everyone I know has got shiny object syndrome. We all want to work on the latest and greatest. I myself am part of that crowd very much. Whenever I start on a project I will never pin the versions for the libraries. That adds anywhere between 0% and 50% on top of the project timeline. The most notable example is Javascript with its myiriad of libraries. You would think everyone is familiar with this meme by now :sweat_smile:.

{{ img(id="js-fw-meme.png", alt="Zero days without new framework") }}

In the first article in the series we talked about getting _anchors_ right and trying to stay away from the personal bias when choosing a good anchor. In _How Big Things Get Done_ the authors also bring in a concept that was new to me: **the reference class**.
The phrase was originally coined in the 1970s by the psychologist [Daniel Kahneman and his colleague Amos Tversky](https://www.newyorker.com/books/page-turner/the-two-friends-who-changed-how-we-think-about-how-we-think) and is regulary used in the context of _reference class forecasting_.
Daniel and Amos refer to two types of views when estimating a project:
- _inside view_ which is the view while working on the project with your personal biases
- _outside view_ which is the view from the outside, looking at the project as a whole and comparing it to similar projects

Enough with the theory, in practice, what we actually want to achieve is to have our estimates work and be as accurate as possible.

## How?

Now that we have the lingo down, let's get into the nitty-gritty. We want to figure out how to calculate the estimates, you got that right **calculate**. The calculations are based on being able to cut down your project from being a special and unique snowflake to a project that is similar to others. It's a combination of statistical and historical analysis of other projects as similar as possible to yours.

---
_Going on a tangent here, wouldn't it be cool if we could have a database of software project estimates with numerical data, situational requirements and conditions, and perhaps how long the project took in the end?_ :thinking:
___

You want to reduce your project to something as generic as possible then look for data about other projects like it. As a sofware developer you may be tempted to think that it is special and unique, but finding the commmonalities will help you get a better estimate.
We could make use of both _inside view_ and _outside view_:
- _outside view_: see how long similar projects took(take the median) - I am referring to the reduced version where you cut out any product differentiators
- _inside view_: see how long the differentiators will take (_this you can break down further as well into common tasks and unique tasks_)

## Into the future with AI


## Conclusion
- **Stay boring**: don't get caught up in the thinking your project is special and unique
- **Use the reference class**: look at similar projects and see how long they took
- **Use both views**: _inside view_ and _outside view_ to get a better estimate
- **Use data**: if you have it, use it to your advantage
