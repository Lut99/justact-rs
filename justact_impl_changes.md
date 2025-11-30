Changes are enumerated roughly in descending order of priority

1. Correct the impl

```
to_says_atom (α: agent) (a: atom): atom :=
      Node [fact_atom (Leaf α); Leaf (Lit "says"); a].
      
to_says_rule (α: agent) (r: rule): rule :=
    Rule (to_says_atom α (head r)) (body r).
    
add_says_head (α: agent) (sr: safe_rule): policy :=
    [sr ; to_says_safe_rule α sr].
    
    ...
    
extract1 (m: message): policy :=
	flat_map (add_says_head (author m)) (payload m).
	
extract: list message → policy :=
      flat_map extract1.
```

the case study must be tweaked to account for the removal of `within`

- agreements lose rule `X says Y if X within (X Z)` 
- agreement formalisation of authorisation relation changes:
  - before: relates (checker, task, message)
  - now: `Checker authorises Agent reads Data for Task `. 
    - Is it actually necessary for checkers to authorise *writes*?
      - each data identifies the tasks
    - We don't really have to authorise *writes* right? because data should only be written once
    - maybe some design space to consider here. Authorization should mention the task, right?
- scenario changes accordingly, updating how agents authorise data.

Update impl to reflect my minor changes to the definition of `action -> list message` and the basis property:

- there are two functions with the signature `action -> list message`:

  - `extra` is just the *field projection* of the action. I expect this to be the field of the struct in Rust. Intuition: this is arbitrarily chosen by the actor

  - `payload` is what was previously called *justification*. It is defined as: 

    ```
    payload (a: action): list message :=
        basis a :: reflect_actorship a :: extra a.
    ```

     where `reflect_actorship: action -> message` simply adds `actor Actor` where `Actor` is just the field of the action.

Fold agreements and times together:

- Delete `time` from the implementation.
- The runtime maintains a fully synchronised list/set of messages called agreements.
- The synchronous input stream accepts a command to entirely replace the agreements.
- Remove **time** rows from the nice diagrams you made.

**Bonus**: rename types and functions in the Rust impl (and thus, also paper snippets) to align better with terms in the paper.

- easy cases: actions have an *extra* field and a *payload* function
- harder cases: facts are an enum with variants *node* and *leaf* (in the paper I formalise them via the generic *rosetree* polymorphic type). atoms are rosetrees whose leaves are the enum `Var(String) | Lit(String)`. Maybe this would be a bit too annoying to change.