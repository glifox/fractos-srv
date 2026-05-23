# Fractos server

This is a _very fast implementation_ (`1` not really) for a working server 
to work in pair with [fractos-web](https://github.com/glifox/fractos-web)

config:

- `USER_NAME`: The username for the unique access.
- `USER_PASW`: The password for the unique access.
- `PORT`: The port of the server to listen.
- `DIR`: where the loro documents are going to be save.

## docker

The default configuration for docker is 
- `USER_NAME`: `admin`
- `USER_PASW`: `password`
- `PORT`: `3030`
- `DIR`: `/data`

to test it run:
~~~bash
docker run --rm -t -e RUST_LOG=debug -p 3030:3030 ghcr.io/glifox/fractos-srv
~~~


---
`1` It was not realy a fast implementation, becouse i have developing the tools to get here for quite some time.
