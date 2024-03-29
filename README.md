<p align="center">
<img src="docs/assets/logo.svg" style="width: 250px;">
</p>

<h1>Ki-Kist</h1>

[![pipeline status](https://gitlab.com/gabyx/ki-kist/badges/main/pipeline.svg)](https://gitlab.com/gabyx/ki-kist/-/commits/main)

<!--toc:start-->

- [Overview](#overview)
  - [Project Structure](#project-structure)
  - [Architecture](#architecture)
  - [Requirements](#requirements)
  - [Quick Instructions](#quick-instructions)
    - [Deploy](#deploy)
    - [Shutdown](#shutdown)
  - [Locally Building Components](#locally-building-components)
  - [Deploying Components to the Cluster (Kubernetes)](#deploying-components-to-the-cluster-kubernetes)
  - [Development](#development)
    - [Tests](#tests)
    - [Debugging in Rust](#debugging-in-rust)
    - [Database Inspection](#database-inspection)
    - [Githooks](#githooks)
    - [CI/CD](#cicd)
      - [Gitlab](#gitlab)
    - [Testing API Calls](#testing-api-calls)

<!--toc:end-->

The project `ki-kist` (_/kiː ˈkɪst/_[^1]) is an unfinished proof-of-concept and
learning material to conceptualize a secure storage client/server architecture
to store asymmetric encryption/signing key pairs (public and private keys)
written in Rust with the following task in mind.

Provide the architecture and logic for a server and a client with the following
features:

- A **client** command-line interface (CLI) that:

  - Generates asymmetric key pairs (public and private key).
  - Registers multiple public keys and corresponding encrypted private keys on
    the server under an authenticated user.
  - Saves/retrieves public key and encrypted private keys from an authenticated
    user to/from the server.
  - Signs a document/message with a chosen private key from an authenticated
    user.
  - Verifies a signature for a specific username/email.

- A **server** that:

  - Stores & retrieves public keys.
  - Stores & retrieves private encrypted keys.

- Written in Rust. Why?
  [**Make sure to have read the note on the conclusion here.**](/docs/architecture.md#conclusion)

**Note: This is a proof-of-concept and a learning project. It should not be used
in production.**

[^1]:
    _ki-kist_ stands for _key chest/box_ borrowed from the English-Welsh term
    `cist`.

# Overview

The project consists of a mono-repo encompassing the following components:

- [`client`](components/client): A Rust command-line interface which enables the
  user to interact with the `api` component.
- [`api`](components/api): A Rust micro-service which fulfills the task of the
  `server` in the task description above.

## Project Structure

The project structure is outlined below and give you an overview about the
location of files:

- [`./components`](components): All components making up this mono-repo project.
  - [`./api`](components/api): The server micro-service which stores and loads
    public and encrypted private keys.
  - [`./client`](components/client): The command-line interface which can talk
    to the server `api`.
  - [`./common`](components/common): A common Rust library acting as glue
    between `api` and `client`.
- [`./manifests`](manifests): All Kubernetes (`k8s`) manifests which are needed
  to deploy the `api` and its accompanied micro-services (e.g database, reverse
  proxy etc.) to a Kubernetes cluster.
- [`./tools`](tools): Tools and scripts for best-practice tooling and CI/CD are
  located here.
  - [`.nvim`](.nvim): Neovim setup when entering this project. Needs plugin
    [`klen/nvim-config-local`](https://github.com/klen/nvim-config-local).
  - [`.githooks`](.githooks): Githooks setup, which also runs inside the CI to
    provide consistent formatting/linting in the whole development journey (all
    dockerized, see [setup instructions](#githooks).)
  - [`.gitlab`](.gitlab): CI setup with Gitlab which provides currently a
    `format` and a `build` step.

## Architecture

The architecture is described [in more details here](/docs/architecture.md).

## Requirements

On `NixOS` use the `flake.nix` by doing `nix develop --command zsh` inside the
root of the repo. This will setup an isolated development shell with all tools
installed.

On other systems you need the following essentials:

- [`just`](https://github.com/casey/just): A better `make` alternative.
- [`docker`](https://docs.docker.com/get-docker) or
  [`podman`](https://podman.io/docs/installation): Manage containers for
  virtualization and using the `kind` cluster.

and either

- [todo]: Using the [`.devcontainer`](.devcontainer) setup with VS Code or over
  the CLI with `just start-devcontainer` or
- you develop locally and have the following tools installed and on your `PATH`:

  - [`cargo`](https://www.rust-lang.org/tools/install): Rust toolchain with
    `rustup toolchain install nightly`.
  - `libpq`: The PostgreSQL C library. Normally comes with packages such as
    `postgres` on \*nix systems.
  - [`tilt`](https://docs.tilt.dev/install.html): Auto-deploy changes directly
    to a running Kubernetes cluster when working in the repository and get
    instance feedback.
  - [`kind`](https://kind.sigs.k8s.io/docs/user/quick-start): A Kubernetes
    cluster which runs in containers managed by `docker` or `podman`.
  - [`kustomize`](https://kubectl.docs.kubernetes.io/installation/kustomize):
    Rendering Kubernetes YAML manifests to specify
    resources/provisioning/deployments in the Kubernetes cluster.
  - [`httpie`](https://httpie.io/docs/cli/installation): A http client which is
    easier/more intuitive to use than `curl` \[optional\].
  - [`k9s`](https://k9scli.io/topics/install): A command-line tool to visualize
    what is running in your Kubernetes cluster \[optional\].

## Quick Instructions

The following walks you through starting up a local Kubernetes cluster with
`kind`, inspecting the cluster, accessing the `api` service with the `client`
and also shutting the cluster down again.

**Note**: All commands given in the following are safe to use (virtualized or
locally scoped to the repository) to use and **will only** minimally fiddle with
your system.

### Deploy

The easiest way to run the `api` and corresponding other deployments (database
etc.) is using `tilt` on a local Kubernetes cluster, such as `kind`. The tool
`kind` is only doing the following two simple isolated parts on source file
changes:

- Building docker images and pushing them to a registry.
- Auto applying Kubernetes manifests in [`./manifests`](manifests).

Start the `kind-kikist` cluster (context: `kind-kikist`, with a local image
registry :partying_face:) with

```shell
just create-cluster
```

**Note**: `kind` will write to your `kubectl` config file located in
`~/.kube/config` or otherwise set by `KUBECONFIG` env. variable.

You can now start `k9s` to inspect the state of the cluster. No `api` pods
should be running yet.

With `tilt` installed and the `kind` Kubernetes cluster running, deploy all the
`api` etc. with:

```shell
just deploy-up
```

Open the `tilt` web browser [`http://localhost:10350`](http://localhost:10350)
to see the log & status of all running components, notably `api` and `postgres`.

### Access With Client

Run the following

```shell
cd components/client
just client -h
```

to inspect what you can do.

#### Storing a Key Pair

```shell
credArgs=(--host "http://localhost:8080" \
          --access-token todo \
          --user-id demo)

just client store \
    ${credArgs[@]} \
    --passphrase-file <(echo "f73o6k&ntb9ojR@XmVgjazt%nae")
```

asdfasdf Note: The above `<(...)` is called process substitution to avoid
writing the password into a file. Drop the `--passphrase-file` to have an
interactive prompt.

#### Retrieving a Key Pair

```shell
just client get \
    ${credArgs[@]} \
    --passphrase-file passphrase.txt
    --key-id "<key-id>" \
    --passphrase-file passphrase.txt
```

#### Signing a Document

```shell
just client sign \
    ${credArgs[@]} \
    --key-id "<key-id>" \
    --file "README.md" \
    --passphrase-file passphrase.txt
```

This produces a signature file `README.md.sig`.

#### Verifying a Signed Document

```shell
just client verify \
    --key-id "<key-id>" \
    --file "README.md" \
    --file-signature "README.md.sig" \
```

### Shutdown

Killing the cluster is as simple as:

```shell
just delete-cluster
```

which will kill all resources and pods.

## Locally Building Components

All components can be build with e.g.

```shell
just [--set parallel true] build
```

which will run the build task over all components.

to build a single component `<component>` either run `just build` inside the
`components/<components>` directory or use

```shell
just component <component> build`
```

inside the repository root. All binaries are in the `target` directory inside
the repository root.

## Deploying Components to the Cluster (Kubernetes)

The tool `tilt` will run all services and build all docker containers to the
`ttl.sh` registry (ephemeral images).

It will watch for changes to any files (including the
[service manifests](manifests) and redeploy the services, configuration maps as
far as possible.

To start the loop run (after `just create-cluster`) do:

```shell
just deploy-up
```

which loads an optional user-defined settings file
[`manifests/.env.yaml`](manifests/.env.yaml.tmpl). You can use the local
registry for uploading the container images or `ttl.sh` and also configure if
you want to build a `debug` release for more log output on `trace` and `debug`
levels.

To remove all resources from the development cluster use:

```shell
just deploy-down
```

You can inspect continuously the state of the cluster with `k9s` and also watch
`tilt` what its doing by inspecting
[http://localhost:10350](http://localhost:10350).

## Development

### Tests

Note: Currently not a lot of test have been written. The goal is the following:

- **Unit Tests**: Should be written without real database access. That means we
  need to wrap our [`State`](../components/api/src/service/state.rs) for tests
  introducing a mock trait which uses a file database. Currently I don't know
  how to do that with Diesel.
- **Integration Tests**: Use the `client` be directly for integration tests
  against all endpoints.

### Debugging in Rust

- Either use VS Code with the rust extension or
- Debug in `neovim` as fancy-pancy as possible by using the
  [`nvim-dap.lua`](.nvim/nvim-dap.lua) file which is automatically loaded if you
  use the plugin `{ "klen/nvim-config-local" }` which will execute
  [`.nvim/nvim.lua`](.nvim/nvim.lua) when you open this repo in `nvim`. When you
  start the debugger (see plugin
  [`nvim-dap`](https://github.com/mfussenegger/nvim-dap)) it will prompt you
  which executable you want to debug.

### Database Inspection

Run `just start-db-tool` and make a connection with the `DATABASE_URL` in your
configured [components/api/.env](/components/api/.env.tmpl).

### Githooks

You can install Githooks by running the manual
[install here](https://github.com/gabyx/Githooks#quick-secure) and then running:

```shell
cd repository
git hooks install
git hooks config enable-containerized-hooks --global --set
```

in this repository. To show all running hooks run `git hooks list`. To disable
running hooks use either

- `GITHOOKS_DISABLE=1 <your-cmd>` or
- `git commit --no-verify ...` or
- `git hooks uninstall` to completely remove the hook run wrappers from
  `.git/hooks`.

### CI/CD

#### Gitlab

Either use the free Gitlab credits or start your own runner with `docker` by
running:

```shell
just start-gitlab-runner <token>
```

where the `<token>` is the Gitlab runner token obtained from setting up a
project specific runner in Gitlab. After starting the runner, the config mount
will be inside `.gitlab/local`.

### Testing API Calls

There is a simple `just test manual` command which tests some simple API calls
for manual debugging and investigations.
