# action skip-ci [![](https://github.com/meetup/action-skip-ci/workflows/Main/badge.svg)](https://github.com/meetup/action-skip-ci/actions)

> ⏭️ some commits are worth passing on

> **⚠️ Note:** To use this action, you must have access to the [GitHub Actions](https://github.com/features/actions) feature. GitHub Actions are currently only available in public beta. You can [apply for the GitHub Actions beta here](https://github.com/features/actions/signup/).

## 🤸 usage

By default, action-skip-ci will bail on workflow if a push event contains commits with the patterns

| message            |
|--------------------|
| `[skip ci]`        |
| `[skip actions]`   |
| `[skip actionsci]` |
| `[skip actions ci]`|
| `[ci skip]`        |
| `[actions skip]`   |
| `[actionsci skip]` |
| `[actions ci skip]`|

```yaml
on: push
name: CI
jobs:
  ci:
    name: build
    runs-on: ubuntu-latest
    steps:
    - name: Skip
      uses: docker://meetup/action-skip-ci:{docker-tag}
    - uses: actions/checkout@v1
    - run: echo 👍
```

## 🖍️ customize

You can customize the when to bail on commits using the `with.pattern` key
to provide your own regex matcher.

💡You can find more information on the supported regex syntax [here](https://docs.rs/regex/1.2.1/regex/#syntax)

```yaml
on: push
name: CI
jobs:
  ci:
    name: build
    runs-on: ubuntu-latest
    steps:
    - name: Skip
      uses: docker://meetup/action-skip-ci:{docker-tag}
      with:
        pattern: "\\[meh]\\"
    - uses: actions/checkout@v1
    - run: echo 👍
```


Meetup, Inc.