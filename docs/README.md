# Holochain + GraphQL Starter Kit

Can't wait for official GraphQL support? Get started right away with this example app thanks to the power of WASM!

## About

While official Holochain support for GraphQL is coming it is still some time away. By compiling the Rust GraphQL server Juniper and exposing a single zome function `graphql` we can create a zome that acts as a distributed graphql server.

At the moment there is a fair bit of boilerplate code required. Entities must be defined twice, once as a holochain entry and once as a graphql object. There are plans for a macro that creates both from a single declaration.

## Building the DNA

Ensure you have the holochain CLI `hc` version 0.0.6-alpha on your path and run

```
hc package
```

## Running the DNA in the conductor

Ensure you have the holochain donductor `holochain` version 0.0.6-alpha on your path and run

```
holochain -c ./conductor-config.toml
```

This will ask for passphrase. Just press enter to start the conductor.

## Connecting the UI

The simplest way to start the UI is to use the node dev server. First make sure the conductor is running then run the following

```
cd ui
npm install
npm run start
```

This will open up the UI on `http://localhost:3000`. The UI is configured to connect to the conductor via HTTP on port 3001. 

The UI is a GraphiQL interface. You can browser the auto-generated documentation for the server on the right panel and type your own queries to execute on the left. 

![ui-screnshot](./ui-screenshot.png?raw=true)

Try pasting the following to get started (make sure you run them in the correct order):

```
{ apiVersion }
```

```
{
  rootWidget {
    address
    description
  }
}
```

```
mutation {
  addWidget(description: "brand new widget") {
    address
  }
}
```

```
mutation {
  addSubwidgetAsChild(parentAddress: "QmXmZwSpqKJ9ctDSeBvKiCfHqCi7c8M3zdcmQpBgAaU95k", description: "first child subwidget") {
    address,
    description
  }
}
```

```
{
  rootWidget {
    description
    subwidgets { description }
  }
}
```

## Running the tests

The tests for the DNA can be run from the project root by running

```
hc test
```

## Built With

* [Holochain](https://holochain.org/) - Distributed web framework
* [Juniper](https://github.com/graphql-rust/juniper) - Rust Graphql Server
* [GraphiQL](https://github.com/graphql/graphiql) - Interactive GraphQL browser

## Authors

* **Willem Olding** - https://github.com/willemolding

## License

This project is licensed under the GPL-3 License - see the [LICENSE.md](LICENSE.md) file for details



