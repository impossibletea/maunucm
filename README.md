# Taunucm

## About

Command line utility `maunucm` (pronounced "typist") is made for parcing russian
text and converting it to ascii characters in a style made famous by early
SMS typing practices and Counter Strike usernames. Why? Because.

## Usage

Accepts `stdin`, outputs to `stdout`. Supports Russian characters. If you want
any additional cyrillic characters, ~~too bad~~ modify to your liking.

## Examples

```console
$ echo 'Русский кулхацкер' | maunucm
Pycckuu kyJIxau,kep
```

Simple dmenu script to convert on the fly:

```sh
#!/bin/sh
dmenu | maunucm | xclip -selection clipboard
```
