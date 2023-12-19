### Key Value Store

/ledger: Ledger Data(id, members, etc)
/ledger/contracts/{id}: Wasm Contracts
/ledger/members/{id}: Ledger members

### WASM Methods

/ledger/message/{id}: put, get, call
/ledger/command/{id}: put, get, call, random, sign 
/ledger/library/{id}: random, call

- incept : When incepting a new ledger
- mutate :  When mutating a ledger  
- add_contract: When a contract is added
- add_member: When a member is added
- add_member: When a member is added
- update_member: When a member is updated
- remove_member: When a member is removed
- create_request
- create_response
- verify_request
- verify_response

