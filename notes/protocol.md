## session stuff

```js
dttpv: dttp-0.1.0.
 -> ok.
 -> err: version_err, "...".
 -> err: ext_err, "...".
```

```js
opt: <session options json>.
 -> ok.
 -> err: option_unknown, "...".
```

## mote stuff

```js
have: <mote full spec>.
 -> ok.
 -> err: asdf, "..."
```

```js
have?: <mote spec>.
 -> yes.
 -> yes: [ <mote full spec>, ... ].
 -> no.
 -> err: asdf, "...".
```

```js
get: <mote spec>.
 -> ok: <mote json>.
 -> err: mote_not_found, "...".
```

```js
fetch: <mote spec>.
 -> ok: <mote json>.
 -> ok: [ hub_spec, ... ].
 -> err.
```

```js
want?: <mote spec>
 -> yes.
 -> no.
```

```js
take: <mote json>.
 -> ok.
 -> err.
```

## hub stuff

```js
self: <hub spec>.
 -> ok: <asdf>.
 -> err: asdf_err, "...".
```

```js
others?
 -> ok: [ hub_spec, ... ].
 -> err.
```

```js
profile: hub_spec.
 -> ok: <hub profile json>.
 -> err: hub_unknown, "...".
 -> err: mode_disabled, "...".
```

```js
asdf: asdf.
 -> ok: <asdf>.
 -> err: asdf_err, "...".
```
