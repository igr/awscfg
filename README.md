# AWS Config file parser

Sometimes you need informations from the config file.

## Usage

```shell
awscfg arn --profile $profile
```

Resolves ARN from the AWS profile that is configured to use `weep`. Usage:

```shell
profile=foo-name

aws --profile $profile sts get-caller-identity
arn=$(awscfg arn --profile $profile)
cmd=$(weep export $arn)
cmd=${cmd//"set -gx"/"export"}
cmd=${cmd//" \""/=\"}

eval $cmd
```
