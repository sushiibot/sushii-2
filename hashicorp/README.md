# HashiCorp Deployment

This is 100000% overkill and completely unnecessary to use with the scale of sushii.

But it shiny and cool so why not ¯\\\_(ツ)_/¯

## Motivation

Mainly wanted to separate services across multiple servers, for... isolation idk
in case something deadlocks and eats up resources for other services.

I don't really want to hardcode IPs and other values.

## Infrastructure

All servers are deployed on [Hetzner Cloud](https://hetzner.cloud/?ref=zcvAUvYIXilC)
(* ref link for €20 in credit). Big fan of Hetzner Cloud as they are both really
cheap compared to other mainstream providers like DigitalOcean / Linode /
Vulture / etc. and still have pretty good performance. The only minor downside
is servers are only in Germany / Finland which means a decent amount of latency.
So far it's only really noticeable when SSHing into servers and typing which is
fine for me.

## 
