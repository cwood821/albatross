# Albatross

Declarative HTTP tests for monitoring and security

![](https://github.com/cwood821/albatross/workflows/Build/badge.svg)

## Usage

### Configuration
```yaml
# albatross.yml
version: 1
hosts: 
  - host: https://christianwood.net
    tests:
      - path: /
        status: 200 
      - path: /admin
        status: 301 
```

### CLI
```bash
albatross --config albatross.yml 
```
Output:
```
Results:

```