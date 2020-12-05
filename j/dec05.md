# Dec 05 in J

The decoding can be done very concisely using the `#.`
[Base 2](https://code.jsoftware.com/wiki/Vocabulary/numberdot) verb:

```
   #. 'B' = 'FBFBBFF'
44
```

In fact, the seat id is simply the decoding of the whole string with 'B' and 'R'
treated as ones. There's no need to explicitly multiply.

How do we express testing whether each character is a 'B' or an 'R'? One way is
with a hook like this:

```
IsBorR =: 'R'&= +. 'B' &=
   IsBorR 'BFB'
1 0 1
   IsBorR 'BFBRLR'
1 0 1 1 0 1
   #. IsBorR 'BFBRLR'
45
   #. IsBorR 'FBFBBFFRLR'
357
```

So to decode one line:

```
   Decode =: #. & ('R' &= +. 'B' &=)
   Decode 'FBFBBFFRLR'
357
```

In fact, checking for the 1-bit values of `R` or `B` can be simpler using the
[`e.` "member"](https://code.jsoftware.com/wiki/Vocabulary/edot#dyadic) verb (or
more mnemonically maybe "exists").
