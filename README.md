#### Runlike, (re)written in Rust

    "See this docker container? I wish I could run another one just like it,
    but I'll be damned if I'm going to type all those command-line switches manually!"

This is what `runlike` does. You give it a docker container, it outputs the command line necessary to run another one just like it, including the arguments that went while creating it. It's a real time saver for those that normally deploy their docker containers via some CM tool like Ansible/Chef and then find themselves needing to manually re-run some container.

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
      --cpuset-cpus string             CPUs in which to allow execution
                                       (0-3, 0,1)
      --cpuset-mems string             MEMs in which to allow execution
  -d, --detach                         Run container in background and
                                       print container ID
  -h, --hostname string                Container host name
      --mac-address string             Container MAC address (e.g.,
                                       92:d0:c6:0a:29:33)
  -m, --memory bytes                   Memory limit
      --name string                    Assign a name to the container
      --network string                 Connect a container to a network
                                       (default "default")
      --pid string                     PID namespace to use
      --privileged                     Give extended privileges to this
                                       container
      --runtime string                 Runtime to use for this container
  -u, --user string                    Username or UID (format:
                                       <name|uid>[:<group|gid>])
```

## Not Yet Supported Run Options (PRs are most welcome!)
Pretty much everything else. But work is in progress.

## Roadmap:
- [ ] Support more flags
- [ ] Installation instructions
- [ ] Setup CI and release binaries
- [ ] Run without installation

## Footnote
The work here is based on [Assaf Lavie](https://assaf.io)'s [runlike](https://github.com/lavie/runlike). I was lazy so the README is mostly a ripoff too. It's still a work in progress, and most features aren't yet supported. But we'll get there, sooner than later.
