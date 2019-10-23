#!/bin/sh

set -uexo pipefail

cargo build
alias multibase='./target/debug/multibase'

INPUT_A=z76WGJzY2rXtSiZ8BDwU4VgcLqcMEm2dXdgVVS1QCZQUptZ5P8n5YCcnbuMUASYhVNihae7m8VeYvfViYf2KqTMVEH1BKNF6Xc5S2kPpBwsNos6egnrmDMxhtQppZjb47Mi2xG89jZm654uZUatDvfTCoDWuethfRHPSk81qn6od9zGxBxxAYyUPnY9Fs9QEQETm53AN9uk6erSAhJ2R3K8rosrBkSZbVhbzUJTPg22wpddVY8Xu3vhRVNpzyUvCEedg5EM6i7wE4G1CYsz7tbaApEF9aFRB92v4DoiY5GXGjwH5PhhGstJB9ySh9FyDfSYN8qRVVR7i5No2eBi3AjQ7cqaBiWkoSrCoQK7jJ4PyFsu3ZaAuUx8LAtkhaChmwfxH8E25LcTENJhFxqVnPd7f7Q3cUrFciYRqmg8eJsy1AahqbzJQ63n9RtekmwzqnMYrTwft6cLJJGeTSSxCCJ6HKnRtwE7jjDh6sB2ZeVj494VppdAVJBz2AAiZY9BBnCD8wUVgwqH3qchGRCuC2RugA4eQ9fUrR4Yuycac3caiaaay

# "echo -n" has inconsistent behavour across posix systems. we use printf instead
test "x$(printf "%s" $INPUT_A | multibase decode)" = "x$(multibase decode $INPUT_A)"

test "x$(multibase decode $INPUT_A | multibase base58btc)" = "x$INPUT_A"

INPUT_B=F4D756C74696261736520697320617765736F6D6521205C6F2F
INPUT_C=BJV2WY5DJMJQXGZJANFZSAYLXMVZW63LFEEQFY3ZP
INPUT_D=zYAjKoNbau5KiqmHPmSxYCvn66dA1vLmwbt
## the rust-multibase library, as of 0.6.0, does not yet support multibase encodings with padding
# INPUT_E="MTXVsdGliYXNlIGlzIGF3ZXNvbWUhIFxvLw==" 

test "x$(multibase decode $INPUT_B)" = "x$(multibase decode $INPUT_C)"
test "x$(multibase decode $INPUT_C)" = "x$(multibase decode $INPUT_D)"
# test "x$(multibase decode $INPUT_D)" = "x$(multibase decode $INPUT_E)"

echo SUCCESS
