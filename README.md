# bin-proxy

simple binary redirection with args and envs injection.

## what is this used for

I got into a situation at work where a program A launched another program B.
I had the need to enable some debug feature flags on B when called by A,  
without having to rebuild A or launch B with a manually copyied setup.

## how does this work

the program loads `RUNTIME_PAYLOAD` from the env as json during compile time.
you can build this json payload string manually or with `build/build.py`.
the final binary will carry over args and envs + payload on top when executed.
