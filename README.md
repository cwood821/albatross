# Albatross

Fast, declarative HTTP tests for monitoring and security

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