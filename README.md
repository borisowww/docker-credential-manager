# docker-env credentials store manager

## A simple docker credentials mangager in Rust.

Based on https://docs.docker.com/engine/reference/commandline/login/

When pulling images from private registries, you need to provide credentials. This is usually done by creating
a `~/.docker/config.json` file. This is not very convenient when you have multiple registries to pull from.

This tiny rust tool allows you to store credentials based on the registry URL.
Whenever docker calls for credentials, this tool will read the input of the docker command and look for the registry URL credentials.

Compile into a deb package via `cargo deb` and install with `sudo dpkg -i target/debian/docker-env_0.1.0_amd64.deb`.

To set up simply update your `~/.docker/config.json` file to use the tool:

```json
{
  "credsStore": "docker-credential-manager"
}
```