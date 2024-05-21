# Service Listener

A CLI tool that polls services (sites) for new releases and, optionally, sends notifications to a Discord webhook.

## Supported sites

- [x] [Splice](https://splice.com) - listens for new packs

## Usage

```
A CLI tool that polls services (sites) for new releases and, optionally, sends notifications to a Discord webhook.

Usage: service-listener.exe [OPTIONS]

Options:
  -w, --webhook <WEBHOOK>
          The discord webhook to send notifications to (optional)
  -p, --splice-poll-rate <SPLICE_POLL_RATE>
          How often (per second) to poll for new packs on Splice [default: 60]
  -s, --disable-splice
          Disable the Splice listener
  -h, --help
          Print help
  -V, --version
          Print version
```

## Downloading / building the tool

You can download the tool for your OS from [latest releases](https://github.com/Stefanuk12/service-listener/releases/latest) or build manually. Steps for building manually are given below:
1. Install [Rust/Cargo via rustup](https://rustup.rs)
2. Clone the repository
3. `cd` into the repository
4. Run `cargo build --release`
5. The binary will be available at `target/release/service-listener`

## Running on your computer

Once you have the binary, `cd` into the directory where the binary is located and run `service-listener.exe` or `service-listener` (depending if you are on Windows or not).

This will list all of the options available for the tool. Repeat the command and provide any options and arguments as needed.

> [!NOTE]
> Here is a trick for automatically `cd` into the directory where the binary is located on Windows: [Video](https://youtu.be/bgSSJQolR0E?t=47)

## Running with [Docker](https://www.docker.com/)

You can also run the tool using [Docker](https://www.docker.com/). Here is a couple example commands (make sure you are `cd` into the directory where the [Dockerfile](https://github.com/Stefanuk12/service-listener/blob/master/Dockerfile) is located):

```bash
# Running with no arguments (defaults used)
docker run -it --rm ghcr.io/stefanuk12/service-listener:latest
```

```bash
# Running with webhook
docker run -it --rm ghcr.io/stefanuk12/service-listener:latest --webhook "https://discord.com/api/webhooks/1241502809552191509
/OXQnNf5iTFP0WBD420pnpEbXIk34PGCicsx7NuLWT6GEOL4mAJ_LjQRzBCiw00WFmqEt"
```

## Creating a Discord webhook

This tool supports sending notifications to a Discord webhook. To create a webhook, follow [this](https://www.youtube.com/watch?v=fKksxz2Gdnc) video up to the `0:52` mark. Copy the URL and provide it to the tool by using the `--webhook <WEBHOOK_URL>` option.

## Running 24/7 for free with Oracle Cloud

1. Follow this [tutorial](https://www.youtube.com/watch?v=g7sP33QtuxM) on setting up the server.
2. To install Docker on Oracle Linux 8, follow [this](https://oracle-base.com/articles/linux/docker-install-docker-on-oracle-linux-ol8) tutorial. Follow the `Install Docker`, `Finish Docker Setup` and `Docker Commands as Non-Root User` sections only.
3. Follow the instructions on [Running with Docker](#running-with-docker) section.