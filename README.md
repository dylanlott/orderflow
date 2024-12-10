<p align="center">
  <h3 align="center">orderflow</h3>

  <p align="center">
    an experimental order matching and filling engine in Rust
    <br/>
    <br/>
  </p>
</p>

![Contributors](https://img.shields.io/github/contributors/dylanlott/orderflow?color=dark-green) ![Issues](https://img.shields.io/github/issues/dylanlott/orderflow)

## Table Of Contents

- [Table Of Contents](#table-of-contents)
- [About The Project](#about-the-project)
- [Built With](#built-with)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Testing](#testing)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
  - [Creating A Pull Request](#creating-a-pull-request)
- [Authors](#authors)

## About The Project

This is a simple order matching and filling engine in Rust as an experiment in how Rust handles ownership and lifetimes in a complex and asynchronous process.

## Built With

Built with [Rust](https://www.rust-lang.org/) 🦀

## Getting Started

### Prerequisites

- Rust
- Cargo

### Installation

1. Clone the repo

2. Run the tests

```sh
cargo test
```

3. Run the application

```sh
cargo run
```

4. Build the binary

```sh
cargo build
```

## Usage

Don't use this in production, please. For experimentation only.

## Testing

To test the API, `cargo run` and then, in another terminal, fire off these curl commands to test post and get requests of the server.

- Create an order `POST /orders`

```sh
curl -X POST http://127.0.0.1:8080/orders -H "Content-Type: application/json" -d '{"is_buy": true, "price": 100, "quantity": 10, "priority": 1, "owner_id": 123}'
```

- GET a list of the open orders `GET /orders`

```sh
# TODO
```

## Roadmap

See the [open issues](https://github.com/dylanlott/orderflow/issues) for a list of proposed features (and known issues).

## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

- If you have suggestions for adding or removing projects, feel free to [open an issue](https://github.com/dylanlott/orderflow/issues/new) to discuss it, or directly create a pull request after you edit the *README.md* file with necessary changes.
- Please make sure you check your spelling and grammar.
- Create individual PR for each suggestion.
- Please also read through the [Code Of Conduct](https://github.com/dylanlott/orderflow/blob/main/CODE_OF_CONDUCT.md) before posting your first idea as well.

### Creating A Pull Request

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## Authors

- **d7t** - **- [d7t](https://github.com/dylanlott/) -**
