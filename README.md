### Getting started

This project uses *Visual Studio Code Remote - Containers* as a way to have developer environments
as code.

https://code.visualstudio.com/docs/remote/containers

Using VSCode - Install the plugin and select *run inside container* to get the full rust and ink! smart contract
developoment environment.

### Testing the contract

Once inside the dev container you should be able to run the following.

`cargo +nightly test`

### Building

`cargo +nightly contract build`

To get the metadata

`cargo +nightly contract generate-metadata`