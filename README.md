# futbladaj

👨‍💻 book a pitch from your terminal

## Motivation

Simple way to book a pitch in [Estádio Universitário de Lisboa](https://www.estadio.ulisboa.pt/webform/pedido-de-reserva-de-espacos) without having to fill in the form each time.

![](images/form.png)

It also served as an exercise to learn a bit more about Rust. ⚙️

## Usage

```
futbladaj 0.1
Daniel Serrano <danieljdserrano@protonmail.com>
book a pitch from your terminal

USAGE:
    futbladaj [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>    Sets a custom config file
```

We currently provide one way to book a pitch, via a configuration file written in YAML. E.g., `config/booking.yml`:

```yaml
---
day: 1
month: 1
year: 2020
start_hour: 21
start_minute: 00
end_hour: 22
end_minute: 00
username: "Jorge Jesus"
email: "email@email.com"
fiscal_number: "123 123 123"
phone: "91 123 12 12"
address: "Rua do Ouro"
postcode: "1234-123"
location: "Lisbon"
```

You can use a given configuration file as follows:

```
futbladaj -c config/booking.yml
[SUCCESS] form submission successful
```

## Contributing

Bug reports and pull requests are welcome on GitHub at [`dnlserrano/futbladaj`](https://github.com/dnlserrano/futbladaj).

## License

    Copyright © 2019-present Daniel Serrano <danieljdserrano at protonmail>

    This work is free. You can redistribute it and/or modify it under the
    terms of the MIT License. See the LICENSE file for more details.

Made in Portugal :portugal: by [dnlserrano](https://dnlserrano.dev)
