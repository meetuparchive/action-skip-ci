# action skip-ci [![](https://github.com/meetup/action-skip-ci/workflows/Main/badge.svg)](https://github.com/meetup/action-skip-ci/actions)

> ⏭️ some commits are worth passing on

## 🤸 usage

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