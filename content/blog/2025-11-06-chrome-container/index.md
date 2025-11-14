+++
title = "Rusty puppets, Websockets and Voyeurism (part II): Driving Chromium in Docker with a Window"
date = 2025-11-06
draft = true
[taxonomies]
tags = ["rust", "docker", "containers", "chrome", "chromium", "arm64"]
+++

## TL;DR

You swapped Chrome → Chromium for better arm64 support, strapped on VNC + noVNC to watch the chaos, made Alpine optional to chase size gains, and pimped a Makefile so everything feels like a dead man's switch.

## The Backstory

In some of my previous experiments I have used browser automation with Chrome to extract info from pages, usually via a wrapper around CDP. CDP has a pretty neat and quite huge API for driving Chrome and Chromium based browsers. Instead of using an existing wrapper, I built my own in Rust with `tungstenite` to communicate over WebSocket.

{{ img(id="rusty-puppets-docker-chrome", alt="Hero image", caption="Chromium inside docker") }}

## Why?

Ideally what I want is a server style primitive component that is able to speak CDP over a websocket without having to babysit a local browser instance. Normally the way you run most of the chrome automations is by running the process and if you want some transparency into what is happening you flip `--headless` flag to false, but this is not what you want, the automation is outside the docker container, the container only holds the browser with the remote debugging port exposed. Something else that tripped me up was that there were no Chrome repos with arm64 builds that i could run on my M1 mac, so I switched to Chromium that supports arm64 better.

Yes, "headless" is efficient; no, I don't trust it until I can see it wiggle, that is why an extra feature that felt right was having VNC enabled on one of the container variants.

## What?

Components (lego brick style)

- Chromium (not Chrome) – better support for arm64 builds, CDP at 9222.
- Socat - port forwarding for CDP, since binding to 0.0.0.0 did not work well with Chromium.
- Xvfb + lightweight WM (fluxbox) – fake display for the VNC stream.
- VNC server (x11vnc) – the eyeballs.
- noVNC + websockify – view it in the browser at :6080.
- supervisord – herd cats (multiple daemons).
- Alpine (optional) – minimal base; trade-offs: fonts, glibc shims, weird edges.
- Rust client – talks to <http://container:9222/json>

## Architecture

```less
+------------------ Docker Container --------------------+
|                                                        |
|  [Xvfb] --- [WM] --- [VNC Server] --- [noVNC]          |
|                         ^             (HTTP 8080)      |
|                         |                              |
|                    screen:0                            |
|                                                        |
|  [Chromium --headless=new --remote-debugging-port=9222]|
|                                         (WS:9222)      |
|                                ^                       |
|                                |                       |
|                            [Socat proxy]               |
+--------------------------------------------------------+

 Outside:
   Rust (chromiumoxide/others) --> http://host:<exposed-socat-port>
   Human -> http://host:6080 (noVNC)

```

## How?

1. Image strategy (two by two):

Essentially what I wanted was efficiency at the container level and at the browser level. I applied that in practice by using `alpine` for smaller image size and running headless for better runtime efficiency. However running blind is kinda hard to introspect so I added two additional images with VNC support and those came with both `alpine` and `debian` flavors.

- Debian/Ubuntu base: fewer papercuts, bigger image, fastest to "it works."
- Alpine base: smallest, but bring your own fonts, codecs, and glibc cuddles.

> Pro tip: start Debian for DX, ship Alpine once you tame fonts/codecs.

2. Dockerfile (there is a common base between `debian` and `alpine`)

```dockerfile
#....
# Default ports
ENV CHROME_PORT=9222
ENV SOCAT_PORT=9224
ENV VNC_PORT=5900
ENV NOVNC_PORT=6080

#....packages common to both debian and alpine
  vim \ # want to edit stuff?
  chromium \ # duh
  socat \ # port forwarding for the 9222 remote debugging port
  curl \ # healthcheck and manual testing
  net-tools \ # netstat useful for listing open ports
  iproute2 \ # ss useful for listing open ports
  ca-certificates \
  procps \


#.... Install VNC and GUI components
  supervisor \
  xvfb \
  x11vnc \
  websockify \
  fluxbox \
  git \

#....

# Install noVNC from source (most reliable method)
RUN cd /opt && \
  git clone --depth 1 https://github.com/novnc/noVNC.git && \
  cd noVNC && \
  ln -s vnc.html index.html

#....

# Create necessary directories with proper permissions
RUN mkdir -p /home/chrome/data && \
  mkdir -p /var/log/supervisor && \
  chown -R chrome:chrome /home/chrome && \
  chown -R chrome:chrome /var/log/supervisor


# Copy startup script
ARG STEALTH=basic
COPY ${STEALTH}.sh /usr/local/bin/start-chrome.sh
RUN chmod +x /usr/local/bin/start-chrome.sh && \
  chown chrome:chrome /usr/local/bin/start-chrome.sh

#....


EXPOSE ${SOCAT_PORT} ${VNC_PORT} ${NOVNC_PORT}

CMD ["supervisord", "-n", "-c", "/etc/supervisor/conf.d/supervisord.conf"]
```

3. Dockerfile (lightweight Debian to get a more frictionless experience):

```dockerfile
FROM debian:bookworm-slim
ENV DEBIAN_FRONTEND=noninteractive

# Install base dependencies
RUN apt-get update && apt-get install -y \
  #....
  fonts-liberation \
  fonts-noto-color-emoji \
  fonts-roboto \
  fonts-noto \
  libasound2 \
  libatk-bridge2.0-0 \
  libatk1.0-0 \
  libatspi2.0-0 \
  libcups2 \
  libdbus-1-3 \
  libdrm2 \
  libgbm1 \
  libgtk-3-0 \
  libnspr4 \
  libnss3 \
  libwayland-client0 \
  libxcomposite1 \
  libxdamage1 \
  libxfixes3 \
  libxkbcommon0 \
  libxrandr2 \
  xdg-utils \
  && rm -rf /var/lib/apt/lists/*

# Install VNC and GUI components
RUN apt-get update && apt-get install -y --no-install-recommends \
  gnupg \
  #....
  python3 \
  python3-numpy \
  && rm -rf /var/lib/apt/lists/*

#....

# Create a non-root user
RUN useradd -m -s /bin/bash chrome

#....

# Copy supervisord config
COPY supervisord.conf.debian /etc/supervisor/conf.d/supervisord.conf

#....
```

4. The `alpine` version, similar but not quite the same, some commands are different and some of the package names and dependencies differ too:

```dockerfile
FROM alpine:3.19

# Default ports
ENV CHROME_PORT=9222
ENV SOCAT_PORT=9224
ENV VNC_PORT=5900
ENV NOVNC_PORT=6080

# Install base dependencies and Chromium
RUN apk add --no-cache \
  # Core utilities
  bash \ # using some custom scripts to launch the browser
  # ....
  xdpyinfo \ # x11 utilities that are not included by default
  xauth \
  xprop \
  xwininfo \
  # Fonts
  font-liberation \
  font-noto \
  font-noto-emoji \
  font-noto-cjk \
  # Chromium dependencies
  libstdc++ \
  harfbuzz \
  nss \
  freetype \
  ttf-freefont \
  wqy-zenhei \
  # Audio/Video libraries
  alsa-lib \
  at-spi2-core \
  cups-libs \
  dbus-libs \
  libdrm \
  mesa-gbm \
  libxcomposite \
  libxdamage \
  libxfixes \
  libxkbcommon \
  libxrandr \
  wayland-libs-client \
  # X11 libraries
  libx11 \
  libxext \
  libxrender \
  libxtst \
  libxi

# Install VNC and GUI components
RUN apk add --no-cache \
  #....
  python3 \
  py3-numpy \
  py3-pip

#....

# Create a non-root user (Alpine uses adduser instead of useradd)
RUN adduser -D -s /bin/sh chrome

#....

# Copy supervisord config
COPY supervisord.conf.alpine /etc/supervisor/conf.d/supervisord.conf

#....
```
