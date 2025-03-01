# Commands
The following is a list of commands within this system.


## Create Account 
Requests the creation of a new bank account

### Emitted By
_This command is never emitted_

### Handled By
This is a list of entities that receive/process this command

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |


## Deposit Funds 
Requests the deposit of funds into a bank account

### Emitted By
_This command is never emitted_

### Handled By
This is a list of entities that receive/process this command

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |


## Initiate Interbank Transfer 
Requests the initiation of interbank transfer

### Emitted By
This is a list of entities that produce this command

| Entity | Description |
|---|:--|
| [bankaccount](./pm_index.md#bankaccount) | The process manager for a bank account |

### Handled By
This is a list of entities that receive/process this command

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |


## Release Reserved Funds 
Requests the release of reserved funds in a bank account

### Emitted By
This is a list of entities that produce this command

| Entity | Description |
|---|:--|
| [bankaccount](./pm_index.md#bankaccount) | The process manager for a bank account |

### Handled By
This is a list of entities that receive/process this command

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |


## Request Wire Transfer 
Requests the transfer of funds from one bank account to another

### Emitted By
_This command is never emitted_

### Handled By
This is a list of entities that receive/process this command

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |


## Reserve Funds 
Requests the reservation of funds in a bank account

### Emitted By
This is a list of entities that produce this command

| Entity | Description |
|---|:--|
| [bankaccount](./pm_index.md#bankaccount) | The process manager for a bank account |

### Handled By
This is a list of entities that receive/process this command

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |


## Withdraw Funds 
Requests the withdrawal of funds from a bank account

### Emitted By
_This command is never emitted_

### Handled By
This is a list of entities that receive/process this command

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |


## Withdraw Reserved Funds 
Requests the withdrawal of reserved funds from a bank account

### Emitted By
This is a list of entities that produce this command

| Entity | Description |
|---|:--|
| [bankaccount](./pm_index.md#bankaccount) | The process manager for a bank account |

### Handled By
This is a list of entities that receive/process this command

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |



---
_This file is automatically generated by a tool. Do not modify its contents manually_