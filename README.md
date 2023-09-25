# MQTT-Broker

Basic demonstration how MQTT are implemented

## Starting broker

To start the broker run `$cargo run --bin broker`

## Producing message on some topic

`$ cargo run --bin producer tanks panzer`

## Consuming data from topic

Get value
`$ cargo run --bin consumer tanks`