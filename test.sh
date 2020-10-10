#!/bin/bash

assert() {
  expected="$1"
  input="$2"

  cargo run --quiet -- "$input" > tmp.s || {
      echo >&2 "error while trying to compile '$input'"
      exit
  }
  gcc -static -o tmp tmp.s || {
      echo >&2 "error while trying to link '$input'"
      exit
  }

  ./tmp
  actual="$?"

  if ! [ "$actual" = "$expected" ]
  then
    echo >&2 "'$input' => $expected expected, but got $actual"
    exit 1
  fi

  echo "'$input' => $actual"
}

fail() {
  expected="$1"
  input="$2"
  error="$3"

  err_msg="$(cargo run --quiet -- "$input" 2>&1 1>/dev/null)"
  if ! echo "$err_msg" | grep -q "$error"
  then
    echo >&2 "'$input' should yield the error '$error', got:"
    echo >&2 "$err_msg"
    exit 1
  fi

  echo "'$input' => $error"
}

assert 0 0
assert 42 42
assert 21 '5+20-4'

fail 0 '' 'the expression must be non empty'
fail 0 'a' 'expecting a number at the beggining of the expression'
fail 0 '3+' "unexpected end of input after '+'"
fail 0 '3+a' "expecting a number after '+' at index 2"
fail 0 '3-' "unexpected end of input after '-'"
fail 0 '3-a' "expecting a number after '-' at index 2"

rm tmp tmp.s 2>/dev/null
echo OK
