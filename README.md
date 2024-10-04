#### Runlike, (re)written in Rust

    "See this docker container? I wish I could run another one just like it,
    but I'll be damned if I'm going to type all those command-line switches manually!"

This is what `runlike` does. You give it a docker container, it outputs the command line necessary to run another one just like it, including the arguments that went while creating it. It's a real time saver for those that normally deploy their docker containers via some CM tool like Ansible/Chef and then find themselves needing to manually re-run some container.

#### The Rust appeal
The two primary benefits that come with using `runlike-rs` are:
1. It's at least 70% faster than its Python counterpart.
2. It comes as a single, compiled binary. No Python runtime required.

# Usage
    runlike <container-name>
This prints out what you need to run to get a similar container. You can do `$(runlike container-name)` to simply execute its output in one step.

`-p` breaks the command line down to nice, pretty lines. For example:

    $ runlike -p redis

    docker run \
        --name=redis \
        -e "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin" \
        -e "REDIS_VERSION=2.8.9" \
        -e "REDIS_DOWNLOAD_URL=http://download.redis.io/releases/redis-2.8.9.tar.gz" \
        -e "REDIS_DOWNLOAD_SHA1=003ccdc175816e0a751919cf508f1318e54aac1e" \
        -p 0.0.0.0:6379:6379/tcp \
        --detach=true \
        myrepo/redis:7860c450dbee9878d5215595b390b9be8fa94c89 \
        redis-server --slaveof 172.31.17.84 6379

Feeding it the output of `docker inspect` also works:

```
docker inspect <container-name> | runlike --stdin
```

`--no-name` will omit the container name from the output (to avoid collisions).


# Status

This is very much a work in progress. Many `docker run` options aren't yet supported, but the most commonly used ones are. Feel free to send pull requests if you add any or if you happen to fix any (of the many) bugs this package undoubtedly has.

Probably **shouldn't use this in production** yet. If you do, double check that it's actually running what you want it to run.

## Supported Run Options

```
      --add-host list                  Add a custom host-to-IP mapping
                                       (host:ip)
      --cap-add list                   Add Linux capabilities
      --cap-drop list                  Drop Linux capabilities
                                       (0-3, 0,1)
      --cpuset-cpus string             CPUs in which to allow execution
                                       (0-3, 0,1)
      --cpuset-mems string             MEMs in which to allow execution
  -d, --detach                         Run container in background and
                                       print container ID
      --device list                    Add a host device to the container
      --dns list                       Set custom DNS servers
  -e, --env list                       Set environment variables
      --expose list                    Expose a port or a range of ports
  -h, --hostname string                Container host name
      --mac-address string             Container MAC address (e.g.,
                                       92:d0:c6:0a:29:33)
  -m, --memory bytes                   Memory limit
      --memory-reservation bytes       Memory soft limit
      --name string                    Assign a name to the container
      --network string                 Connect a container to a network
                                       (default "default")
      --pid string                     PID namespace to use
      --privileged                     Give extended privileges to this
                                       container
  -p, --publish list                   Publish a container's port(s) to
                                       the host
      --restart string                 Restart policy to apply when a
                                       container exits (default "no")
      --rm                             Automatically remove the container
                                       when it exits
      --runtime string                 Runtime to use for this container
  -t, --tty                            Allocate a pseudo-TTY
  -u, --user string                    Username or UID (format:
                                       <name|uid>[:<group|gid>])
  -v, --volume list                    Bind mount a volume
      --volumes-from list              Mount volumes from the specified
                                       container(s)
  -w, --workdir string                 Working directory inside the container
```

## Installation Instructions
Just download the binary from releases and you're good to go. Or run the following commands:
```bash
# For Linux
wget -O /tmp/runlike https://github.com/ByteBaker/runlike-rs/releases/latest/download/runlike-linux-amd64 && sudo mv /tmp/runlike /usr/local/bin/runlike && sudo chmod +x /usr/local/bin/runlike

# For MacOS
wget -O /tmp/runlike https://github.com/ByteBaker/runlike-rs/releases/latest/download/runlike-macos-amd64 && sudo mv /tmp/runlike /usr/local/bin/runlike && sudo chmod +x /usr/local/bin/runlike

```

## Not Yet Supported Run Options (PRs are most welcome!)
Everything not present in the list. But work is in progress.

## Roadmap:
- [ ] Support more flags
- [x] Installation instructions
- [x] Setup CI and release binaries
- [ ] Run without installation

## Footnote
The work here is based on [Assaf Lavie](https://assaf.io)'s [runlike](https://github.com/lavie/runlike). I was lazy so the README is mostly a ripoff too. It's still a work in progress, and some features aren't yet supported. But we'll get there, sooner than later.
